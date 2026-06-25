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
use crossterm::ExecutableCommand;
use ratatui::backend::CrosstermBackend;
use ratatui::layout::{Constraint, Direction, Layout};
use ratatui::Terminal;
use std::io::stdout;
use std::sync::Arc;
use std::time::Duration;

pub struct TuiCommandSurface {
    tui_aggregate: Arc<dyn ITuiAggregate>,
}

impl TuiCommandSurface {
    pub fn new(tui_aggregate: Arc<dyn ITuiAggregate>) -> Self {
        Self { tui_aggregate }
    }

    pub fn run(&self) -> anyhow::Result<()> {
        enable_raw_mode()?;
        stdout().execute(EnterAlternateScreen)?;
        stdout().execute(crossterm::event::EnableMouseCapture)?;

        let backend = CrosstermBackend::new(stdout());
        let mut terminal = Terminal::new(backend)?;

        let cwd = std::env::current_dir()
            .map(|p| p.to_string_lossy().to_string())
            .unwrap_or_else(|_| ".".to_string());
        let mut state = AppState::new(cwd);

        let file_list_view = FileListView::new();
        let preview_view = PreviewView::new();
        let tree_view = TreeView::new();
        let path_screen = PathScreen::new();
        let shortcut_bar = ShortcutComponent::new();
        let status_bar = StatusComponent::new();

        let result = self.event_loop(
            &mut terminal,
            &mut state,
            &file_list_view,
            &preview_view,
            &tree_view,
            &path_screen,
            &shortcut_bar,
            &status_bar,
        );

        disable_raw_mode()?;
        stdout().execute(LeaveAlternateScreen)?;
        stdout().execute(crossterm::event::DisableMouseCapture)?;

        result
    }

    #[allow(clippy::too_many_arguments)]
    fn event_loop(
        &self,
        terminal: &mut Terminal<CrosstermBackend<std::io::Stdout>>,
        state: &mut AppState,
        file_list_view: &FileListView,
        preview_view: &PreviewView,
        tree_view: &TreeView,
        path_screen: &PathScreen,
        shortcut_bar: &ShortcutComponent,
        status_bar: &StatusComponent,
    ) -> anyhow::Result<()> {
        loop {
            terminal.draw(|frame| {
                let area = frame.area();

                if state.show_path_dialog {
                    path_screen.render(state, frame, area);
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

                tree_view.render(state, frame, panel_layout[0]);
                file_list_view.render(state, frame, panel_layout[1]);
                preview_view.render(state, frame, panel_layout[2]);

                shortcut_bar.render(state, frame, main_layout[2]);
                status_bar.render(state, frame, main_layout[3]);
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
