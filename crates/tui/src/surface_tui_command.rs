use crate::surface_file_list_view::FileListView;
use crate::surface_path_screen::PathScreen;
use crate::surface_preview_view::PreviewView;
use crate::surface_shortcut_component::ShortcutComponent;
use crate::surface_status_component::StatusComponent;
use crate::surface_tree_view::TreeView;
use crossterm::event;
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers, MouseEvent, MouseEventKind};
use crossterm::terminal::{
    disable_raw_mode, enable_raw_mode, size as terminal_size, EnterAlternateScreen,
    LeaveAlternateScreen,
};
use ratatui::backend::CrosstermBackend;
use ratatui::layout::{Constraint, Direction, Layout};
use ratatui::Terminal;
use shared::tui::contract_tui_aggregate::ITuiAggregate;
use shared::tui::taxonomy_state_vo::AppState;
use shared::tui::taxonomy_tui_event::TuiEvent;
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
    tui_aggregate: Arc<dyn ITuiAggregate>,
}

impl TuiCommandSurface {
    pub fn new(tui_aggregate: Arc<dyn ITuiAggregate>) -> Self {
        Self { tui_aggregate }
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
        if let Ok((_, h)) = terminal_size() {
            state.terminal_height = h;
        }
        state.show_path_dialog = false;
        self.tui_aggregate.load_directory(&mut state, &cwd);

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
        loop {
            terminal.draw(|frame| {
                let area = frame.area();

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
            })?;

            if event::poll(Duration::from_millis(50))? {
                let crossterm_event = event::read()?;
                let tui_event = from_crossterm_event(crossterm_event, state);
                self.tui_aggregate.handle_event(state, tui_event);
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
    let ctrl = key.modifiers.contains(KeyModifiers::CONTROL);

    if ctrl {
        return match key.code {
            KeyCode::Char('q') | KeyCode::Char('c') => TuiEvent::Quit,
            KeyCode::Char('s') => TuiEvent::ActionSecurity,
            KeyCode::Char('d') => TuiEvent::ActionDuplicates,
            KeyCode::Char('p') => TuiEvent::ActionDependencies,
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
            KeyCode::Esc => TuiEvent::Quit,
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
        KeyCode::Tab => TuiEvent::FocusNext,
        KeyCode::BackTab => TuiEvent::FocusPrev,
        KeyCode::Char('c') => TuiEvent::ActionCheck,
        KeyCode::Char('s') => TuiEvent::ActionScan,
        KeyCode::Char('f') => TuiEvent::ActionFix,
        KeyCode::Char('t') => TuiEvent::ActionCi,
        KeyCode::Char('w') => TuiEvent::ActionWatch,
        KeyCode::Char('o') => TuiEvent::ActionOrphan,
        KeyCode::Char('d') => TuiEvent::ActionDoctor,
        KeyCode::Char('i') => TuiEvent::ActionInit,
        KeyCode::Char('I') => TuiEvent::ActionInstall,
        KeyCode::Char('m') => TuiEvent::ActionMcpConfig,
        KeyCode::Char('C') => TuiEvent::ActionConfigShow,
        KeyCode::Char('H') => TuiEvent::ActionInstallHook,
        KeyCode::Char('U') => TuiEvent::ActionUninstallHook,
        KeyCode::Char('a') => TuiEvent::ActionAdapters,
        KeyCode::Char('v') => TuiEvent::ActionVersion,
        KeyCode::Char('?') => TuiEvent::ToggleHelp,
        KeyCode::Char('/') => TuiEvent::ToggleSearch,
        KeyCode::Esc => TuiEvent::None,
        _ => TuiEvent::None,
    }
}

fn from_mouse_event(mouse: MouseEvent) -> TuiEvent {
    match mouse.kind {
        MouseEventKind::Down(crossterm::event::MouseButton::Left) => {
            TuiEvent::MouseClick(mouse.column, mouse.row)
        }
        MouseEventKind::ScrollUp => TuiEvent::MouseScrollUp,
        MouseEventKind::ScrollDown => TuiEvent::MouseScrollDown,
        _ => TuiEvent::None,
    }
}

fn render_header(state: &AppState, frame: &mut ratatui::Frame, area: ratatui::layout::Rect) {
    use ratatui::style::{Color, Style};
    use ratatui::text::{Line, Span};
    use ratatui::widgets::Paragraph;

    let line = Line::from(vec![
        Span::styled(" lint-arwaky TUI ", Style::default().fg(Color::Cyan)),
        Span::styled("\u{2502} ", Style::default().fg(Color::DarkGray)),
        Span::styled("Path: ", Style::default().fg(Color::DarkGray)),
        Span::styled(&state.current_dir, Style::default().fg(Color::White)),
        Span::styled("  ", Style::default()),
        Span::styled("[q] Quit", Style::default().fg(Color::DarkGray)),
    ]);

    let paragraph = Paragraph::new(line);
    frame.render_widget(paragraph, area);
}
