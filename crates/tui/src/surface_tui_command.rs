use crate::surface_confirm_component::ConfirmModal;
use crate::surface_event_action::SurfaceActionHandler;
use crate::surface_file_list_view::FileListView;
use crate::surface_path_screen::PathScreen;
use crate::surface_preview_view::PreviewView;
use crate::surface_shortcut_component::ShortcutComponent;
use crate::surface_status_component::StatusComponent;
use crate::surface_tree_view::TreeView;
use crate::{AppState, PanelFocus, ScanUpdate, TuiEvent};
use crossterm::event;
use crossterm::event::{KeyCode, KeyEvent, MouseEvent, MouseEventKind};
use crossterm::terminal::{
    EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode,
};
use ratatui::Terminal;
use ratatui::backend::CrosstermBackend;
use ratatui::layout::Alignment;
use ratatui::style::Style;
use ratatui::text::{Line, Span};
use ratatui::widgets::Paragraph;

use std::io::stdout;
use std::sync::Arc;
use std::time::Duration;

/// Restores the terminal on every exit path from [`TuiCommandSurface::run`].
///
/// Raw mode, the alternate screen, and mouse capture are process-wide terminal
/// state. `run` enabled all three and then restored them after the event loop,
/// but every `?` between the two points returns early — `Terminal::new`,
/// `terminal.draw`, `event::poll`, `event::read` — leaving the caller's shell
/// in raw mode on the alternate screen with no echo. A `Drop` guard cannot be
/// skipped by an early return or an unwind, so it owns the restore.
pub struct TerminalGuard {
    active: bool,
}

impl TerminalGuard {
    fn new() -> Self {
        Self { active: true }
    }

    /// Release the terminal exactly once. Called by `Drop`; callable directly
    /// so the success path restores before returning its result rather than at
    /// an unspecified point later in the caller.
    pub fn restore(&mut self) {
        if !self.active {
            return;
        }
        self.active = false;
        let _ = disable_raw_mode();
        let _ = crossterm::execute!(stdout(), LeaveAlternateScreen);
        let _ = crossterm::execute!(stdout(), crossterm::event::DisableMouseCapture);
    }
}

impl Drop for TerminalGuard {
    fn drop(&mut self) {
        self.restore();
    }
}

struct RenderViews {
    file_list: FileListView,
    preview: PreviewView,
    tree: TreeView,
    path_screen: PathScreen,
    shortcuts: ShortcutComponent,
    status: StatusComponent,
    confirm_modal: ConfirmModal,
}

impl RenderViews {
    fn new() -> Self {
        Self {
            file_list: FileListView::new(),
            preview: PreviewView::new(),
            tree: TreeView::new(),
            path_screen: PathScreen::new(),
            shortcuts: ShortcutComponent::new(),
            status: StatusComponent::new(),
            confirm_modal: ConfirmModal::new(),
        }
    }
}

pub struct TuiCommandSurface {
    action_handler: Arc<SurfaceActionHandler>,
}

impl TuiCommandSurface {
    pub fn new(action_handler: Arc<SurfaceActionHandler>) -> Self {
        Self { action_handler }
    }

    pub fn run(&self) -> anyhow::Result<()> {
        enable_raw_mode()?;
        crossterm::execute!(stdout(), EnterAlternateScreen)?;
        crossterm::execute!(stdout(), crossterm::event::EnableMouseCapture)?;
        // Armed immediately after the enables: everything below can return
        // early through `?`, and a dropped guard still restores the terminal.
        let mut guard = TerminalGuard::new();
        // A panic inside the render closure unwinds, which the guard handles —
        // but the default hook prints the message *before* unwinding, while the
        // alternate screen is still active, so the user never sees it. Restore
        // first, then let the default hook report.
        install_panic_restore_hook();

        let backend = CrosstermBackend::new(stdout());
        let mut terminal = Terminal::new(backend)?;

        let cwd = std::env::current_dir()
            .map(|p| p.to_string_lossy().to_string())
            .unwrap_or_else(|_| ".".to_string());
        let mut state = AppState::new(cwd.clone());

        // Initialize terminal_height so mouse clicks work from the start.
        // Without this, the h < 5 guard in handle_mouse_click drops ALL
        // clicks until the first Resize event arrives.
        if let Ok((w, h)) = crossterm::terminal::size() {
            state.terminal_height = h;
            state.terminal_width = w;
        }
        // FR-011: Path dialog shown on startup — user types project root or Tab for CWD.
        // Directory loading happens AFTER dialog confirmation (PathConfirm / PathUseCurrent).

        let views = RenderViews::new();
        let result = self.event_loop(&mut terminal, &mut state, &views);

        // Restore before returning so the caller's shell is usable the moment
        // `run` hands control back, not whenever the guard falls out of scope.
        guard.restore();

        result
    }

    fn event_loop(
        &self,
        terminal: &mut Terminal<CrosstermBackend<std::io::Stdout>>,
        state: &mut AppState,
        views: &RenderViews,
    ) -> anyhow::Result<()> {
        let mut scan_rx: Option<std::sync::mpsc::Receiver<ScanUpdate>> = None;
        // Ratatui only needs a new frame after state, terminal geometry, or a
        // background result changes. Polling remains active while idle so worker
        // messages still wake the renderer without a continuous draw loop.
        let mut needs_redraw = true;

        loop {
            // --- Poll scan progress (non-blocking) ---
            if state.scan.running
                && let Some(ref rx) = scan_rx
                && self.action_handler.poll_scan(state, rx)
            {
                needs_redraw = true;
            }
            // --- Poll pending background global action (non-blocking) ---
            if state.actions.pending && self.action_handler.poll_pending_background_action(state) {
                needs_redraw = true;
            }

            if needs_redraw {
                terminal.draw(|frame| {
                    let area = frame.area();

                    // W5: guard — refuse to draw the full layout on a too-small terminal.
                    let min_h = shared_tui::utility_tui_theme::MIN_TERMINAL_HEIGHT;
                    let min_w = shared_tui::utility_tui_theme::MIN_TERMINAL_WIDTH;
                    if area.height < min_h || area.width < min_w {
                        let message = too_small_message(area.width, area.height, min_w, min_h);
                        let line = Line::from(vec![Span::styled(
                            message,
                            Style::default()
                                .fg(shared_tui::utility_tui_theme::color(
                                    shared_tui::utility_tui_theme::KEY,
                                ))
                                .add_modifier(shared_tui::utility_tui_theme::EMPHASIS_HEADING),
                        )]);
                        let paragraph = Paragraph::new(line)
                            .style(Style::default().bg(shared_tui::utility_tui_theme::color(
                                shared_tui::utility_tui_theme::BACKGROUND,
                            )))
                            .wrap(ratatui::widgets::Wrap { trim: true })
                            .alignment(Alignment::Center);
                        frame.render_widget(paragraph, area);
                        return;
                    }

                    if state.path_dialog.visible {
                        views.path_screen.render(state, frame, area);
                        return;
                    }

                    let layout =
                        crate::surface_tui_layout::compute_panel_layout(area.width, area.height);

                    render_header(state, frame, layout.header);

                    if layout.three_column {
                        views.tree.render(state, frame, layout.tree);
                        views.file_list.render(state, frame, layout.file_list);
                        views.preview.render(state, frame, layout.preview);
                    } else {
                        // Narrow terminal: only the active panel fills the band.
                        // Tab/BackTab cycles between tree, file-list, preview.
                        match state.navigation.panel_focus {
                            PanelFocus::Tree => views.tree.render(state, frame, layout.band()),
                            PanelFocus::FileList => {
                                views.file_list.render(state, frame, layout.band())
                            }
                            PanelFocus::Preview => {
                                views.preview.render(state, frame, layout.band())
                            }
                        }
                    }

                    views.shortcuts.render(state, frame, layout.shortcuts);
                    views.status.render(state, frame, layout.status);

                    // A pending confirm is a gate, not a status line (#554): overlay a
                    // modal over every panel so it cannot be missed while browsing.
                    views.confirm_modal.render(state, frame, area);
                })?;
                needs_redraw = false;
            }

            if event::poll(Duration::from_millis(50))? {
                let crossterm_event = event::read()?;
                let tui_event = from_crossterm_event(crossterm_event, state);
                crate::surface_logging_controller::record(&tui_event);

                // --- Intercept ActionScan: spawn background thread ---
                // Both families claim one shared busy slot; a declined request
                // reports the holder on the status line instead of racing it.
                if matches!(tui_event, TuiEvent::ActionScan) {
                    if let Some(rx) = self.action_handler.start_scan(state) {
                        scan_rx = Some(rx);
                    }
                } else if state.scan.running
                    && matches!(
                        tui_event,
                        TuiEvent::ActionCheck
                            | TuiEvent::ActionFix
                            | TuiEvent::ActionFixLive
                            | TuiEvent::ActionCi
                            | TuiEvent::ActionOrphan
                            | TuiEvent::ActionSecurity
                            | TuiEvent::ActionDependencies
                    )
                {
                    // Block long-running actions while a scan is in progress
                    state.report_busy_slot();
                } else {
                    self.action_handler.handle(state, tui_event);
                }
                // A key, mouse, or resize event can alter any render-relevant
                // state; the next iteration paints the new frame once.
                needs_redraw = true;
            }

            if state.should_quit {
                break;
            }
        }
        Ok(())
    }
}

/// Key-to-event mapping while the help overlay is open (#556).
///
/// Each key keeps the directional meaning it has outside the overlay: `k`/`Up`
/// and `h`/`Left` scroll back, `j`/`Down` and `l`/`Right` scroll forward,
/// `Home` jumps to the top, `End` to the bottom, `PgUp`/`PgDn` page. `Esc`
/// closes the overlay like `?` does; `q` quits.
///
/// The overlay renders into the preview panel, so `MoveTop`/`MoveBottom` and
/// `PreviewScrollUp`/`PreviewScrollDown` act on `preview.scroll` while it is
/// open. Before #556 every navigation key collapsed onto `MoveDown`, leaving a
/// long help panel with no way back to the top.
pub fn help_overlay_event_for(code: KeyCode) -> TuiEvent {
    match code {
        KeyCode::Char('?') | KeyCode::Esc => TuiEvent::ToggleHelp,
        KeyCode::Char('q') => TuiEvent::Quit,
        KeyCode::Char('j') | KeyCode::Down => TuiEvent::MoveDown,
        KeyCode::Char('l') | KeyCode::Right => TuiEvent::MoveDown,
        KeyCode::Char('k') | KeyCode::Up => TuiEvent::MoveUp,
        KeyCode::Char('h') | KeyCode::Left => TuiEvent::MoveUp,
        KeyCode::Home => TuiEvent::MoveTop,
        KeyCode::End => TuiEvent::MoveBottom,
        KeyCode::PageUp => TuiEvent::PreviewScrollUp,
        KeyCode::PageDown => TuiEvent::PreviewScrollDown,
        _ => TuiEvent::None,
    }
}

/// Make a panic readable: restore the terminal, then report.
///
/// Without this the panic text landed on the alternate screen and vanished with
/// it, leaving a raw-mode shell and no diagnostic (#1326). The hook is
/// installed once per process and chains to the previous hook, so panic
/// payloads stay formatted exactly as before.
fn install_panic_restore_hook() {
    use std::sync::Once;
    static ONCE: Once = Once::new();
    ONCE.call_once(|| {
        let previous = std::panic::take_hook();
        std::panic::set_hook(Box::new(move |info| {
            let _ = disable_raw_mode();
            let _ = crossterm::execute!(stdout(), LeaveAlternateScreen);
            let _ = crossterm::execute!(stdout(), crossterm::event::DisableMouseCapture);
            previous(info);
        }));
    });
}

/// Copy for the too-small screen, chosen by what the terminal can actually show.
///
/// The fixed sentence was 45 columns wide while the guard fired below 40, so
/// the instruction to enlarge the terminal was always clipped by the terminal
/// it wanted enlarged (#1328). Name the current size and the target when they
/// fit, and fall back to a token that always renders.
pub fn too_small_message(width: u16, height: u16, min_w: u16, min_h: u16) -> String {
    let long = format!("{width}x{height} — need {min_w}x{min_h}");
    if long.chars().count() <= width as usize {
        return long;
    }
    let short = "too small";
    if short.chars().count() <= width as usize {
        return short.to_string();
    }
    "!".to_string()
}

fn from_crossterm_event(event: event::Event, state: &AppState) -> TuiEvent {
    match event {
        event::Event::Key(key) => from_key_event(key, state),
        event::Event::Mouse(mouse) => from_mouse_event(mouse),
        event::Event::Resize(w, h) => TuiEvent::Resize(w, h),
        _ => TuiEvent::None,
    }
}

pub fn from_key_event(key: KeyEvent, state: &AppState) -> TuiEvent {
    // --- Pending confirmation: only y/Enter = confirm, n/Esc = cancel (#354) ---
    if state.actions.pending_confirm.is_some() {
        return match key.code {
            KeyCode::Enter | KeyCode::Char('y') | KeyCode::Char('Y') => TuiEvent::ConfirmAction,
            KeyCode::Esc | KeyCode::Char('n') | KeyCode::Char('N') => TuiEvent::CancelConfirm,
            _ => TuiEvent::None,
        };
    }

    // --- Help overlay: ?/Esc toggle, q quit, directional navigation (#359, #556) ---
    if state.show_help {
        return help_overlay_event_for(key.code);
    }

    // Path dialog: ALL input goes to path editing when dialog is visible
    if state.path_dialog.visible {
        return match key.code {
            KeyCode::Char(ch) => TuiEvent::PathInput(ch),
            KeyCode::Backspace => TuiEvent::PathBackspace,
            KeyCode::Left => TuiEvent::PathMoveLeft,
            KeyCode::Right => TuiEvent::PathMoveRight,
            KeyCode::Home => TuiEvent::PathMoveStart,
            KeyCode::End => TuiEvent::PathMoveEnd,
            KeyCode::Enter => TuiEvent::PathConfirm,
            KeyCode::Tab => TuiEvent::PathUseCurrent,
            // I4: Esc dismisses the dialog and returns to the tree (not quit) (#355).
            KeyCode::Esc => TuiEvent::PathCancel,
            _ => TuiEvent::None,
        };
    }

    // Search mode: character and edit keys go to search
    if state.search.mode {
        return match key.code {
            KeyCode::Char(ch) => TuiEvent::SearchInput(ch),
            KeyCode::Backspace => TuiEvent::SearchBackspace,
            KeyCode::Enter => TuiEvent::SearchConfirm,
            KeyCode::Esc => TuiEvent::SearchCancel,
            _ => TuiEvent::None,
        };
    }

    // --- A running scan owns Esc, after every overlay has had its turn ---
    // `CancelScan` had a handler, a cancel token, and a `ScanUpdate::Cancelled`
    // path, but no producer: no key ever mapped to it. Esc was bound to Quit,
    // so the product's only cancellable operation could not be cancelled and
    // the "habit" key killed the process instead (#1329).
    //
    // This sits below the confirm/help/path/search branches on purpose: while
    // one of those is open, Esc belongs to the overlay, not to the scan.
    if state.scan.running && key.code == KeyCode::Esc {
        return TuiEvent::CancelScan;
    }

    // Normal mode is resolved from the shared shortcut table. Contextual
    // dialog/search/help bindings above remain explicit because they shadow
    // normal actions while those overlays are active.
    crate::surface_bindings_store::action_for(&key)
        .map(crate::surface_bindings_store::ShortcutAction::to_event)
        .unwrap_or(TuiEvent::None)
}

fn from_mouse_event(mouse: MouseEvent) -> TuiEvent {
    match mouse.kind {
        MouseEventKind::Down(crossterm::event::MouseButton::Left) => {
            TuiEvent::MouseClick(mouse.column, mouse.row)
        }
        MouseEventKind::Drag(crossterm::event::MouseButton::Left) => {
            TuiEvent::MouseDrag(mouse.column, mouse.row)
        }
        MouseEventKind::ScrollUp => TuiEvent::MouseScrollUp(mouse.column, mouse.row),
        MouseEventKind::ScrollDown => TuiEvent::MouseScrollDown(mouse.column, mouse.row),
        _ => TuiEvent::None,
    }
}

fn render_header(state: &AppState, frame: &mut ratatui::Frame, area: ratatui::layout::Rect) {
    let line = Line::from(vec![
        Span::styled(
            " lint-arwaky TUI ",
            Style::default().fg(shared_tui::utility_tui_theme::color(
                shared_tui::utility_tui_theme::HEADER,
            )),
        ),
        Span::styled(
            "\u{2502} ",
            Style::default().fg(shared_tui::utility_tui_theme::color(
                shared_tui::utility_tui_theme::SEPARATOR,
            )),
        ),
        Span::styled(
            "Path: ",
            Style::default().fg(shared_tui::utility_tui_theme::color(
                shared_tui::utility_tui_theme::SEPARATOR,
            )),
        ),
        Span::styled(
            &state.navigation.current_dir,
            Style::default().fg(shared_tui::utility_tui_theme::color(
                shared_tui::utility_tui_theme::LABEL,
            )),
        ),
        Span::styled("  ", Style::default()),
        Span::styled(
            "[q/Esc] Quit",
            Style::default().fg(shared_tui::utility_tui_theme::color(
                shared_tui::utility_tui_theme::SEPARATOR,
            )),
        ),
    ]);

    let paragraph = Paragraph::new(line);
    frame.render_widget(paragraph, area);
}
