use crate::surface_event_action::SurfaceActionHandler;
use crate::surface_file_list_view::FileListView;
use crate::surface_path_screen::PathScreen;
use crate::surface_preview_view::PreviewView;
use crate::surface_shortcut_component::ShortcutComponent;
use crate::surface_status_component::StatusComponent;
use crate::surface_tree_view::TreeView;
use crossterm::event;
use crossterm::event::{KeyCode, KeyEvent, MouseEvent, MouseEventKind};
use crossterm::terminal::{
    EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode,
};
use ratatui::Terminal;
use ratatui::backend::CrosstermBackend;
use ratatui::layout::{Alignment, Constraint, Direction, Layout};
use ratatui::style::{Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::Paragraph;
use shared_tui::{AppState, ScanUpdate, TuiEvent};

use std::io::stdout;
use std::sync::Arc;
use std::time::Duration;

struct RenderViews {
    file_list: FileListView,
    preview: PreviewView,
    tree: TreeView,
    path_screen: PathScreen,
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
                    let min_h = crate::utility_tui_theme::MIN_TERMINAL_HEIGHT;
                    let min_w = crate::utility_tui_theme::MIN_TERMINAL_WIDTH;
                    if area.height < min_h || area.width < min_w {
                        let message = format!(
                            "Terminal too small — resize to at least {}x{}",
                            min_w, min_h
                        );
                        let line = Line::from(vec![Span::styled(
                            message,
                            Style::default()
                                .fg(crate::utility_tui_theme::color(
                                    crate::utility_tui_theme::KEY,
                                ))
                                .add_modifier(Modifier::BOLD),
                        )]);
                        let paragraph = Paragraph::new(line)
                            .style(Style::default().bg(crate::utility_tui_theme::color(
                                crate::utility_tui_theme::BACKGROUND,
                            )))
                            .alignment(Alignment::Center);
                        frame.render_widget(paragraph, area);
                        return;
                    }

                    if state.path_dialog.visible {
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
                })?;
                needs_redraw = false;
            }

            if event::poll(Duration::from_millis(50))? {
                let crossterm_event = event::read()?;
                let tui_event = from_crossterm_event(crossterm_event, state);
                crate::surface_logging_controller::record(&tui_event);

                // --- Intercept ActionScan: spawn background thread ---
                if matches!(tui_event, TuiEvent::ActionScan) {
                    if !state.scan.running
                        && let Some(rx) = self.action_handler.start_scan(state)
                    {
                        scan_rx = Some(rx);
                    }
                    // Ignore if already scanning
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

fn from_crossterm_event(event: event::Event, state: &AppState) -> TuiEvent {
    match event {
        event::Event::Key(key) => from_key_event(key, state),
        event::Event::Mouse(mouse) => from_mouse_event(mouse),
        event::Event::Resize(w, h) => TuiEvent::Resize(w, h),
        _ => TuiEvent::None,
    }
}

fn from_key_event(key: KeyEvent, state: &AppState) -> TuiEvent {
    // --- Pending confirmation: only y/Enter = confirm, n/Esc = cancel (#354) ---
    if state.actions.pending_confirm.is_some() {
        return match key.code {
            KeyCode::Enter | KeyCode::Char('y') | KeyCode::Char('Y') => TuiEvent::ConfirmAction,
            KeyCode::Esc | KeyCode::Char('n') | KeyCode::Char('N') => TuiEvent::CancelConfirm,
            _ => TuiEvent::None,
        };
    }

    // --- Help overlay: only ? (toggle), q/Esc (quit), navigation allowed (#359) ---
    if state.show_help {
        return match key.code {
            KeyCode::Char('?') => TuiEvent::ToggleHelp,
            KeyCode::Char('q') | KeyCode::Esc => TuiEvent::Quit,
            KeyCode::Char('j')
            | KeyCode::Down
            | KeyCode::Char('k')
            | KeyCode::Up
            | KeyCode::Char('h')
            | KeyCode::Left
            | KeyCode::Char('l')
            | KeyCode::Right
            | KeyCode::Home
            | KeyCode::End
            | KeyCode::PageUp
            | KeyCode::PageDown => TuiEvent::MoveDown,
            _ => TuiEvent::None,
        };
    }

    // Path dialog: ALL input goes to path editing when dialog is visible
    if state.path_dialog.visible {
        return match key.code {
            KeyCode::Char(ch) => TuiEvent::PathInput(ch),
            KeyCode::Backspace => TuiEvent::PathBackspace,
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

    // Normal mode is resolved from the shared shortcut table. Contextual
    // dialog/search/help bindings above remain explicit because they shadow
    // normal actions while those overlays are active.
    crate::surface_shortcut_bindings::action_for(&key)
        .map(crate::surface_shortcut_bindings::ShortcutAction::to_event)
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
            Style::default().fg(crate::utility_tui_theme::color(
                crate::utility_tui_theme::HEADER,
            )),
        ),
        Span::styled(
            "\u{2502} ",
            Style::default().fg(crate::utility_tui_theme::color(
                crate::utility_tui_theme::SEPARATOR,
            )),
        ),
        Span::styled(
            "Path: ",
            Style::default().fg(crate::utility_tui_theme::color(
                crate::utility_tui_theme::SEPARATOR,
            )),
        ),
        Span::styled(
            &state.navigation.current_dir,
            Style::default().fg(crate::utility_tui_theme::color(
                crate::utility_tui_theme::LABEL,
            )),
        ),
        Span::styled("  ", Style::default()),
        Span::styled(
            "[q/Esc] Quit",
            Style::default().fg(crate::utility_tui_theme::color(
                crate::utility_tui_theme::SEPARATOR,
            )),
        ),
    ]);

    let paragraph = Paragraph::new(line);
    frame.render_widget(paragraph, area);
}
