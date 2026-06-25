use crate::contract_tui_aggregate::ITuiAggregate;
use crate::surface_file_list_view::FileListView;
use crate::surface_path_screen::PathScreen;
use crate::surface_preview_view::PreviewView;
use crate::surface_shortcut_component::ShortcutComponent;
use crate::surface_status_component::StatusComponent;
use crate::surface_tree_view::TreeView;
use crate::taxonomy_state_vo::AppState;
use crate::taxonomy_tui_event::TuiEvent;
use crossterm::event;
use crossterm::terminal::{
    disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen,
};
use ratatui::backend::CrosstermBackend;
use ratatui::layout::{Constraint, Direction, Layout};
use ratatui::Terminal;
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
        let mut state = AppState::new(cwd);

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
                let tui_event = TuiEvent::from_crossterm_event(crossterm_event);
                self.tui_aggregate.handle_event(state, tui_event);
            }

            if state.should_quit {
                break;
            }
        }
        Ok(())
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
