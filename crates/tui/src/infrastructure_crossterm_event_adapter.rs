use crossterm::event::{Event, KeyCode, KeyEvent, KeyModifiers, MouseEvent, MouseEventKind};
use shared::tui::taxonomy_tui_event::TuiEvent;

pub struct EventAdapter;

impl EventAdapter {
    pub fn from_crossterm_event(event: Event) -> TuiEvent {
        match event {
            Event::Key(key) => Self::from_key_event(key),
            Event::Mouse(mouse) => Self::from_mouse_event(mouse),
            Event::Resize(w, h) => TuiEvent::Resize(w, h),
            _ => TuiEvent::None,
        }
    }

    fn from_key_event(key: KeyEvent) -> TuiEvent {
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
            KeyCode::Esc => TuiEvent::SearchCancel,
            KeyCode::Char(ch) => {
                if ch == '\n' {
                    TuiEvent::SearchConfirm
                } else {
                    TuiEvent::SearchInput(ch)
                }
            }
            KeyCode::Backspace => TuiEvent::SearchBackspace,
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
}
