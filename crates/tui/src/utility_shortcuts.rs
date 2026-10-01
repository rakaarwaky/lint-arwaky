//! Single source of truth for TUI key bindings.
//!
//! The dispatcher, shortcut bar, and help overlay all consume this table. A
//! binding therefore cannot be changed in one surface while remaining stale in
//! another.

use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use shared_tui::TuiEvent;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Trigger {
    Plain(char),
    Ctrl(char),
    Code(KeyCode),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ShortcutAction {
    Quit,
    MoveDown,
    MoveUp,
    NavigateBack,
    NavigateForward,
    MoveTop,
    MoveBottom,
    PreviewScrollUp,
    PreviewScrollDown,
    FocusNext,
    FocusPrev,
    ActionCheck,
    ActionScan,
    ActionFix,
    ActionFixLive,
    ActionCi,
    ActionWatch,
    ActionOrphan,
    ActionDoctor,
    ActionInit,
    ActionInstall,
    ActionMcpConfig,
    ActionConfigShow,
    ActionInstallHook,
    ActionUninstallHook,
    ActionAdapters,
    ActionVersion,
    ActionDependencies,
    ActionSecurity,
    CopyToClipboard,
    CopyToFile,
    ToggleHelp,
    ToggleSearch,
}

impl ShortcutAction {
    pub fn to_event(self) -> TuiEvent {
        match self {
            Self::Quit => TuiEvent::Quit,
            Self::MoveDown => TuiEvent::MoveDown,
            Self::MoveUp => TuiEvent::MoveUp,
            Self::NavigateBack => TuiEvent::NavigateBack,
            Self::NavigateForward => TuiEvent::NavigateForward,
            Self::MoveTop => TuiEvent::MoveTop,
            Self::MoveBottom => TuiEvent::MoveBottom,
            Self::PreviewScrollUp => TuiEvent::PreviewScrollUp,
            Self::PreviewScrollDown => TuiEvent::PreviewScrollDown,
            Self::FocusNext => TuiEvent::FocusNext,
            Self::FocusPrev => TuiEvent::FocusPrev,
            Self::ActionCheck => TuiEvent::ActionCheck,
            Self::ActionScan => TuiEvent::ActionScan,
            Self::ActionFix => TuiEvent::ActionFix,
            Self::ActionFixLive => TuiEvent::ActionFixLive,
            Self::ActionCi => TuiEvent::ActionCi,
            Self::ActionWatch => TuiEvent::ActionWatch,
            Self::ActionOrphan => TuiEvent::ActionOrphan,
            Self::ActionDoctor => TuiEvent::ActionDoctor,
            Self::ActionInit => TuiEvent::ActionInit,
            Self::ActionInstall => TuiEvent::ActionInstall,
            Self::ActionMcpConfig => TuiEvent::ActionMcpConfig,
            Self::ActionConfigShow => TuiEvent::ActionConfigShow,
            Self::ActionInstallHook => TuiEvent::ActionInstallHook,
            Self::ActionUninstallHook => TuiEvent::ActionUninstallHook,
            Self::ActionAdapters => TuiEvent::ActionAdapters,
            Self::ActionVersion => TuiEvent::ActionVersion,
            Self::ActionDependencies => TuiEvent::ActionDependencies,
            Self::ActionSecurity => TuiEvent::ActionSecurity,
            Self::CopyToClipboard => TuiEvent::CopyToClipboard,
            Self::CopyToFile => TuiEvent::CopyToFile,
            Self::ToggleHelp => TuiEvent::ToggleHelp,
            Self::ToggleSearch => TuiEvent::ToggleSearch,
        }
    }
}

pub struct Shortcut {
    pub triggers: &'static [Trigger],
    pub action: ShortcutAction,
    pub key_label: &'static str,
    pub label: &'static str,
    pub result_label: &'static str,
    pub section: &'static str,
    pub normal_row: Option<usize>,
    pub result_row: Option<usize>,
}

// Keep aliases in one record: dispatch matches every trigger while rendering
// shows the compact key label once.
pub const SHORTCUTS: &[Shortcut] = &[
    Shortcut { triggers: &[Trigger::Plain('q'), Trigger::Ctrl('q'), Trigger::Code(KeyCode::Esc)], action: ShortcutAction::Quit, key_label: "q", label: "quit", result_label: "quit", section: "General", normal_row: Some(2), result_row: Some(2) },
    Shortcut { triggers: &[Trigger::Plain('j'), Trigger::Code(KeyCode::Down)], action: ShortcutAction::MoveDown, key_label: "j/↓", label: "down", result_label: "scroll", section: "Navigation", normal_row: Some(0), result_row: Some(0) },
    Shortcut { triggers: &[Trigger::Plain('k'), Trigger::Code(KeyCode::Up)], action: ShortcutAction::MoveUp, key_label: "k/↑", label: "up", result_label: "scroll", section: "Navigation", normal_row: Some(0), result_row: Some(0) },
    Shortcut { triggers: &[Trigger::Plain('h'), Trigger::Code(KeyCode::Left)], action: ShortcutAction::NavigateBack, key_label: "h/←", label: "back", result_label: "back", section: "Navigation", normal_row: Some(0), result_row: Some(2) },
    Shortcut { triggers: &[Trigger::Plain('l'), Trigger::Code(KeyCode::Right), Trigger::Code(KeyCode::Enter)], action: ShortcutAction::NavigateForward, key_label: "l/→/⏎", label: "open", result_label: "open", section: "Navigation", normal_row: Some(0), result_row: None },
    Shortcut { triggers: &[Trigger::Code(KeyCode::Home)], action: ShortcutAction::MoveTop, key_label: "Home", label: "top", result_label: "top", section: "Navigation", normal_row: Some(0), result_row: Some(0) },
    Shortcut { triggers: &[Trigger::Code(KeyCode::End)], action: ShortcutAction::MoveBottom, key_label: "End", label: "bottom", result_label: "bottom", section: "Navigation", normal_row: Some(0), result_row: Some(0) },
    Shortcut { triggers: &[Trigger::Code(KeyCode::PageUp)], action: ShortcutAction::PreviewScrollUp, key_label: "PgUp", label: "scroll up", result_label: "scroll up", section: "Navigation", normal_row: Some(2), result_row: Some(0) },
    Shortcut { triggers: &[Trigger::Code(KeyCode::PageDown)], action: ShortcutAction::PreviewScrollDown, key_label: "PgDn", label: "scroll down", result_label: "scroll down", section: "Navigation", normal_row: Some(2), result_row: Some(0) },
    Shortcut { triggers: &[Trigger::Code(KeyCode::Tab)], action: ShortcutAction::FocusNext, key_label: "Tab", label: "focus next", result_label: "focus next", section: "Navigation", normal_row: Some(2), result_row: Some(2) },
    Shortcut { triggers: &[Trigger::Code(KeyCode::BackTab)], action: ShortcutAction::FocusPrev, key_label: "BackTab", label: "focus prev", result_label: "focus prev", section: "Navigation", normal_row: Some(2), result_row: Some(2) },
    Shortcut { triggers: &[Trigger::Plain('c')], action: ShortcutAction::ActionCheck, key_label: "c", label: "check", result_label: "re-check", section: "Actions", normal_row: Some(0), result_row: Some(0) },
    Shortcut { triggers: &[Trigger::Plain('s')], action: ShortcutAction::ActionScan, key_label: "s", label: "scan", result_label: "scan", section: "Actions", normal_row: Some(0), result_row: Some(0) },
    Shortcut { triggers: &[Trigger::Plain('f')], action: ShortcutAction::ActionFix, key_label: "f", label: "fix (dry)", result_label: "fix (dry)", section: "Actions", normal_row: Some(0), result_row: Some(0) },
    Shortcut { triggers: &[Trigger::Plain('F')], action: ShortcutAction::ActionFixLive, key_label: "F", label: "fix (live)", result_label: "fix (live)", section: "Actions", normal_row: Some(0), result_row: Some(0) },
    Shortcut { triggers: &[Trigger::Plain('t')], action: ShortcutAction::ActionCi, key_label: "t", label: "ci", result_label: "ci", section: "Actions", normal_row: Some(0), result_row: Some(0) },
    Shortcut { triggers: &[Trigger::Plain('w')], action: ShortcutAction::ActionWatch, key_label: "w", label: "watch (CLI only)", result_label: "watch (CLI only)", section: "Actions", normal_row: Some(0), result_row: Some(1) },
    Shortcut { triggers: &[Trigger::Plain('o')], action: ShortcutAction::ActionOrphan, key_label: "o", label: "orphan", result_label: "orphan", section: "Actions", normal_row: Some(0), result_row: Some(1) },
    Shortcut { triggers: &[Trigger::Plain('d')], action: ShortcutAction::ActionDoctor, key_label: "d", label: "doctor", result_label: "doctor", section: "Setup", normal_row: Some(0), result_row: Some(1) },
    Shortcut { triggers: &[Trigger::Plain('i')], action: ShortcutAction::ActionInit, key_label: "i", label: "init", result_label: "init", section: "Setup", normal_row: Some(0), result_row: Some(1) },
    Shortcut { triggers: &[Trigger::Plain('I')], action: ShortcutAction::ActionInstall, key_label: "I", label: "install", result_label: "install", section: "Setup", normal_row: Some(1), result_row: Some(1) },
    Shortcut { triggers: &[Trigger::Plain('m')], action: ShortcutAction::ActionMcpConfig, key_label: "m", label: "mcp", result_label: "mcp", section: "Setup", normal_row: Some(1), result_row: Some(1) },
    Shortcut { triggers: &[Trigger::Plain('C')], action: ShortcutAction::ActionConfigShow, key_label: "C", label: "config", result_label: "config", section: "Setup", normal_row: Some(1), result_row: Some(1) },
    Shortcut { triggers: &[Trigger::Plain('H')], action: ShortcutAction::ActionInstallHook, key_label: "H", label: "hook", result_label: "hook", section: "Setup", normal_row: Some(1), result_row: Some(1) },
    Shortcut { triggers: &[Trigger::Plain('U')], action: ShortcutAction::ActionUninstallHook, key_label: "U", label: "unhook", result_label: "unhook", section: "Setup", normal_row: Some(1), result_row: Some(1) },
    Shortcut { triggers: &[Trigger::Plain('a')], action: ShortcutAction::ActionAdapters, key_label: "a", label: "adapters", result_label: "adapters", section: "Setup", normal_row: Some(1), result_row: Some(1) },
    Shortcut { triggers: &[Trigger::Plain('v')], action: ShortcutAction::ActionVersion, key_label: "v", label: "version", result_label: "version", section: "Setup", normal_row: Some(1), result_row: Some(1) },
    Shortcut { triggers: &[Trigger::Ctrl('s')], action: ShortcutAction::ActionDependencies, key_label: "^S", label: "deps", result_label: "deps", section: "Actions", normal_row: Some(2), result_row: Some(2) },
    Shortcut { triggers: &[Trigger::Ctrl('p')], action: ShortcutAction::ActionSecurity, key_label: "^P", label: "security", result_label: "security", section: "Actions", normal_row: Some(2), result_row: Some(2) },
    Shortcut { triggers: &[Trigger::Plain('y')], action: ShortcutAction::CopyToClipboard, key_label: "y", label: "copy", result_label: "copy", section: "General", normal_row: Some(2), result_row: Some(2) },
    Shortcut { triggers: &[Trigger::Ctrl('y')], action: ShortcutAction::CopyToFile, key_label: "^Y", label: "save to file", result_label: "save to file", section: "General", normal_row: Some(2), result_row: Some(2) },
    Shortcut { triggers: &[Trigger::Plain('?')], action: ShortcutAction::ToggleHelp, key_label: "?", label: "help", result_label: "help", section: "General", normal_row: Some(2), result_row: Some(2) },
    Shortcut { triggers: &[Trigger::Plain('/')], action: ShortcutAction::ToggleSearch, key_label: "/", label: "search", result_label: "search", section: "General", normal_row: Some(2), result_row: Some(2) },
];

fn trigger_matches(trigger: Trigger, key: &KeyEvent) -> bool {
    let ctrl = key.modifiers.contains(KeyModifiers::CONTROL);
    match trigger {
        Trigger::Plain(ch) => !ctrl && key.code == KeyCode::Char(ch),
        Trigger::Ctrl(ch) => ctrl && key.code == KeyCode::Char(ch),
        Trigger::Code(code) => !ctrl && key.code == code,
    }
}

pub fn action_for(key: &KeyEvent) -> Option<ShortcutAction> {
    SHORTCUTS.iter().find_map(|shortcut| {
        shortcut
            .triggers
            .iter()
            .copied()
            .any(|trigger| trigger_matches(trigger, key))
            .then_some(shortcut.action)
    })
}

pub fn rows_for_context(results: bool) -> [Vec<(&'static str, &'static str)>; 3] {
    let mut rows = [Vec::new(), Vec::new(), Vec::new()];
    for shortcut in SHORTCUTS {
        let row = if results {
            shortcut.result_row
        } else {
            shortcut.normal_row
        };
        if let Some(row) = row {
            rows[row].push((
                shortcut.key_label,
                if results { shortcut.result_label } else { shortcut.label },
            ));
        }
    }
    rows
}

pub fn help_text() -> String {
    let mut output = String::from("Keyboard shortcuts:\n\n");
    let mut section = "";
    for shortcut in SHORTCUTS {
        if shortcut.section != section {
            section = shortcut.section;
            output.push_str(&format!("{}:\n", section));
        }
        output.push_str(&format!("  {:<10} {}\n", shortcut.key_label, shortcut.label));
    }
    output
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_displayed_binding_has_a_dispatch_entry() {
        for shortcut in SHORTCUTS {
            assert!(shortcut.normal_row.is_some() || shortcut.result_row.is_some());
            assert!(shortcut.triggers.iter().any(|trigger| {
                SHORTCUTS.iter().any(|other| other.action == shortcut.action && other.triggers.contains(trigger))
            }));
        }
    }
}
