use crate::surface_confirm_screen::ConfirmScreen;
use crate::surface_event_action::SurfaceActionHandler;
use crate::surface_file_list_view::FileListView;
use crate::surface_path_screen::PathScreen;
use crate::surface_preview_view::PreviewView;
use crate::surface_shortcut_component::ShortcutComponent;
use crate::surface_status_component::StatusComponent;
use crate::surface_tree_view::TreeView;
use crossterm::event;
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers, MouseEvent, MouseEventKind};
use crossterm::terminal::{
    EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode,
};
use ratatui::Terminal;
use ratatui::backend::CrosstermBackend;
use ratatui::layout::{Alignment, Constraint, Direction, Layout};
use ratatui::style::{Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::Paragraph;
use shared::tui::{AppState, ScanUpdate, TuiEvent};

use std::io::stdout;
use std::sync::Arc;
use std::time::Duration;

struct RenderViews {
    file_list: FileListView,
    preview: PreviewView,
    tree: TreeView,
    path_screen: PathScreen,
    confirm_screen: ConfirmScreen,
    shortcuts: ShortcutComponent,
    status: StatusComponent,
}

impl RenderViews {
    fn new() -> Self {
        Self {
            file_list: FileListView::new(),
            preview: PreviewView::new(),
            tree: TreeView::new(),
            path_screen: PathScreen::new(),
            confirm_screen: ConfirmScreen::new(),
            shortcuts: ShortcutComponent::new(),
            status: StatusComponent::new(),
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

        disable_raw_mode()?;
        crossterm::execute!(stdout(), LeaveAlternateScreen)?;
        crossterm::execute!(stdout(), crossterm::event::DisableMouseCapture)?;

        result
    }

    fn event_loop(
        &self,
        terminal: &mut Terminal<CrosstermBackend<std::io::Stdout>>,
        state: &mut AppState,
        views: &RenderViews,
    ) -> anyhow::Result<()> {
        let mut scan_rx: Option<std::sync::mpsc::Receiver<ScanUpdate>> = None;

        loop {
            // --- Poll scan progress (non-blocking) ---
            if state.scanning
                && let Some(ref rx) = scan_rx
            {
                self.action_handler.poll_scan(state, rx);
            }
            // --- Poll pending background global action (non-blocking) ---
            if state.action_pending {
                self.action_handler.poll_pending_background_action(state);
            }

            terminal.draw(|frame| {
                let area = frame.area();

                // W5: guard — refuse to draw the full layout on a too-small terminal.
                if area.height < 15 || area.width < 40 {
                    let message = "Terminal too small — resize to at least 40x15";
                    let line = Line::from(vec![Span::styled(
                        message,
                        Style::default()
                            .fg(crate::utility_tui_theme::KEY)
                            .add_modifier(Modifier::BOLD),
                    )]);
                    let paragraph = Paragraph::new(line)
                        .style(Style::default().bg(crate::utility_tui_theme::BACKGROUND))
                        .alignment(Alignment::Center);
                    frame.render_widget(paragraph, area);
                    return;
                }

                if state.show_path_dialog {
                    views.path_screen.render(state, frame, area);
                    return;
                }

                let main_layout = Layout::default()
                    .direction(Direction::Vertical)
                    .constraints([
                        Constraint::Length(1),
                        Constraint::Min(10),
                        Constraint::Length(3),
                        Constraint::Length(1),
                    ])
                    .split(area);

                render_header(state, frame, main_layout[0]);

                let panel_layout = Layout::default()
                    .direction(Direction::Horizontal)
                    .constraints([
                        Constraint::Percentage(20),
                        Constraint::Percentage(35),
                        Constraint::Percentage(45),
                    ])
                    .split(main_layout[1]);

                views.tree.render(state, frame, panel_layout[0]);
                views.file_list.render(state, frame, panel_layout[1]);
                views.preview.render(state, frame, panel_layout[2]);

                views.shortcuts.render(state, frame, main_layout[2]);
                views.status.render(state, frame, main_layout[3]);

                // I5 confirm gate (UX-1-01): the modal renders on top of the
                // panel layout whenever a destructive action awaits confirmation.
                if state.pending_confirm.is_some() {
                    views.confirm_screen.render(state, frame, area);
                }
            })?;

            if event::poll(Duration::from_millis(50))? {
                let crossterm_event = event::read()?;
                let tui_event = from_crossterm_event(crossterm_event, state);
                crate::surface_logging_controller::record(&tui_event);

                // --- Intercept ActionScan: spawn background thread ---
                if matches!(tui_event, TuiEvent::ActionScan) {
                    if !state.scanning
                        && let Some(rx) = self.action_handler.start_scan(state)
                    {
                        scan_rx = Some(rx);
                    } else if state.scanning {
                        // Feedback instead of silence when a second scan is requested.
                        state.set_status("Scan already running — press Esc to cancel");
                    }
                } else if state.scanning
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
                    // Block long-running actions while a scan is in progress,
                    // but never silently (UX-5-01: disabled state needs feedback).
                    state.set_status("Scan in progress — press Esc to cancel");
                } else {
                    self.action_handler.handle(state, tui_event);
                }
            }

            if state.should_quit {
                break;
            }
        }
        Ok(())
    }
}

fn from_crossterm_event(event: event::Event, state: &AppState) -> TuiEvent {
    match event {
        event::Event::Key(key) => from_key_event(key, state),
        event::Event::Mouse(mouse) => from_mouse_event(mouse),
        event::Event::Resize(w, h) => TuiEvent::Resize(w, h),
        _ => TuiEvent::None,
    }
}

/// Pure keymap: translate a crossterm key event (plus modal state) into a
/// TuiEvent. Modal gates are evaluated top-down: confirm gate → help overlay →
/// path dialog → search mode → normal browsing. Public so the gates are
/// unit-testable from tests/ (see unit_tui_surface_tui_command.rs).
pub fn from_key_event(key: KeyEvent, state: &AppState) -> TuiEvent {
    let ctrl = key.modifiers.contains(KeyModifiers::CONTROL);

    // Confirm gate (UX-1-01): while a destructive action awaits confirmation,
    // ALL input routes to the pending decision — nothing else may fire.
    if state.pending_confirm.is_some() {
        return match key.code {
            KeyCode::Char('y') | KeyCode::Enter => TuiEvent::ConfirmAction,
            KeyCode::Char('n') | KeyCode::Esc => TuiEvent::CancelConfirm,
            _ => TuiEvent::None,
        };
    }

    if ctrl {
        return match key.code {
            KeyCode::Char('q') => TuiEvent::Quit,
            KeyCode::Char('s') => TuiEvent::ActionSecurity,
            KeyCode::Char('p') => TuiEvent::ActionDependencies,
            KeyCode::Char('y') => TuiEvent::CopyToFile,
            _ => TuiEvent::None,
        };
    }

    // Help overlay gate (UX-2-03): only overlay keys reach the handler while
    // help is open, so lint actions can't fire behind the overlay.
    if state.show_help {
        return match key.code {
            KeyCode::Char('?') | KeyCode::Esc => TuiEvent::ToggleHelp,
            _ => TuiEvent::None,
        };
    }

    // Path dialog: ALL input goes to path editing when dialog is visible
    if state.show_path_dialog {
        return match key.code {
            KeyCode::Char(ch) => TuiEvent::PathInput(ch),
            KeyCode::Backspace => TuiEvent::PathBackspace,
            KeyCode::Enter => TuiEvent::PathConfirm,
            KeyCode::Tab => TuiEvent::PathUseCurrent,
            // UX-1-02: Esc quits only on the first-run prompt; afterwards it
            // cancels the dialog and keeps the current project root.
            KeyCode::Esc => {
                if state.dialog_is_first_run {
                    TuiEvent::Quit
                } else {
                    TuiEvent::PathCancel
                }
            }
            _ => TuiEvent::None,
        };
    }

    // Search mode: character and edit keys go to search
    if state.search_mode {
        return match key.code {
            KeyCode::Char(ch) => TuiEvent::SearchInput(ch),
            KeyCode::Backspace => TuiEvent::SearchBackspace,
            KeyCode::Enter => TuiEvent::SearchConfirm,
            KeyCode::Esc => TuiEvent::SearchCancel,
            _ => TuiEvent::None,
        };
    }

    // Normal mode: navigation and action keys
    match key.code {
        KeyCode::Char('q') => TuiEvent::Quit,
        KeyCode::Char('j') | KeyCode::Down => TuiEvent::MoveDown,
        KeyCode::Char('k') | KeyCode::Up => TuiEvent::MoveUp,
        KeyCode::Char('h') | KeyCode::Left => TuiEvent::NavigateBack,
        KeyCode::Char('l') | KeyCode::Right | KeyCode::Enter => TuiEvent::NavigateForward,
        KeyCode::Home => TuiEvent::MoveTop,
        KeyCode::End => TuiEvent::MoveBottom,
        KeyCode::PageUp => TuiEvent::PreviewScrollUp,
        KeyCode::PageDown => TuiEvent::PreviewScrollDown,
        KeyCode::Tab => TuiEvent::FocusNext,
        KeyCode::BackTab => TuiEvent::FocusPrev,
        KeyCode::Char('c') => TuiEvent::ActionCheck,
        KeyCode::Char('s') => TuiEvent::ActionScan,
        // UX-4-01: `f` = dry-run fix, `F` = live fix. Crossterm reports a shifted
        // letter as Char('F') on standard terminals — matching Char('f') plus a
        // SHIFT check silently drops the key almost everywhere, so match both
        // spellings explicitly (the SHIFT variant is covered by the 'F' arm too).
        KeyCode::Char('f') => TuiEvent::ActionFix,
        KeyCode::Char('F') => TuiEvent::ActionFixLive,
        // `x` runs the security scan (^S freezes most terminals via XOFF).
        KeyCode::Char('x') => TuiEvent::ActionSecurity,
        KeyCode::Char('t') => TuiEvent::ActionCi,
        KeyCode::Char('w') => TuiEvent::ActionWatch,
        KeyCode::Char('o') => TuiEvent::ActionOrphan,
        // `r` re-opens the project root dialog (I4).
        KeyCode::Char('r') => TuiEvent::ChangeProjectRoot,
        KeyCode::Char('d') => TuiEvent::ActionDoctor,
        KeyCode::Char('i') => TuiEvent::ActionInit,
        KeyCode::Char('I') => TuiEvent::ActionInstall,
        KeyCode::Char('m') => TuiEvent::ActionMcpConfig,
        KeyCode::Char('C') => TuiEvent::ActionConfigShow,
        KeyCode::Char('H') => TuiEvent::ActionInstallHook,
        KeyCode::Char('U') => TuiEvent::ActionUninstallHook,
        KeyCode::Char('a') => TuiEvent::ActionAdapters,
        KeyCode::Char('v') => TuiEvent::ActionVersion,
        KeyCode::Char('y') => TuiEvent::CopyToClipboard,
        KeyCode::Char('?') => TuiEvent::ToggleHelp,
        KeyCode::Char('/') => TuiEvent::ToggleSearch,
        // Esc cancels an in-flight scan before it quits the TUI.
        KeyCode::Esc => {
            if state.scanning {
                TuiEvent::CancelScan
            } else {
                TuiEvent::Quit
            }
        }
        _ => TuiEvent::None,
    }
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
            Style::default().fg(crate::utility_tui_theme::HEADER),
        ),
        Span::styled(
            "\u{2502} ",
            Style::default().fg(crate::utility_tui_theme::SEPARATOR),
        ),
        Span::styled(
            "Path: ",
            Style::default().fg(crate::utility_tui_theme::SEPARATOR),
        ),
        Span::styled(
            &state.current_dir,
            Style::default().fg(crate::utility_tui_theme::LABEL),
        ),
        Span::styled("  ", Style::default()),
        Span::styled(
            "[q/Esc] Quit",
            Style::default().fg(crate::utility_tui_theme::SEPARATOR),
        ),
    ]);

    let paragraph = Paragraph::new(line);
    frame.render_widget(paragraph, area);
}
