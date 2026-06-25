use crate::taxonomy_state_vo::PanelFocus;
use crossterm::event::{Event, KeyCode, KeyEvent, KeyModifiers, MouseEvent, MouseEventKind};

pub const DEFAULT_FOCUS: PanelFocus = PanelFocus::FileList;

#[derive(Debug, Clone, PartialEq)]
pub enum TuiEvent {
    MoveUp,
    MoveDown,
    MoveTop,
    MoveBottom,
    NavigateBack,
    NavigateForward,
    FocusNext,
    FocusPrev,
    ActionCheck,
    ActionScan,
    ActionFix,
    ActionCi,
    ActionWatch,
    ActionOrphan,
    ActionSecurity,
    ActionDuplicates,
    ActionDependencies,
    ActionDoctor,
    ActionInit,
    ActionInstall,
    ActionMcpConfig,
    ActionConfigShow,
    ActionInstallHook,
    ActionUninstallHook,
    ActionAdapters,
    ActionVersion,
    ToggleHelp,
    ToggleSearch,
    SearchInput(char),
    SearchBackspace,
    SearchConfirm,
    SearchCancel,
    PathInput(char),
    PathBackspace,
    PathConfirm,
    PathUseCurrent,
    Quit,
    Resize(u16, u16),
    MouseClick(u16, u16),
    MouseScrollUp,
    MouseScrollDown,
    Tick,
    None,
}

impl TuiEvent {
    pub fn from_crossterm_event(event: Event) -> Self {
        match event {
            Event::Key(key) => Self::from_key_event(key),
            Event::Mouse(mouse) => Self::from_mouse_event(mouse),
            Event::Resize(w, h) => TuiEvent::Resize(w, h),
            _ => TuiEvent::None,
        }
    }

    fn from_key_event(key: KeyEvent) -> Self {
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

    fn from_mouse_event(mouse: MouseEvent) -> Self {
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
