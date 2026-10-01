//! Headless behavioural coverage for every TUI view.
//! These tests exercise real widget rendering instead of only checking that a
//! type can be named (FE-5-01).

use ratatui::Terminal;
use ratatui::backend::TestBackend;
use shared_tui::{AesLayer, AppState, FileEntry, PanelFocus, PreviewMode};
use tui_lint_arwaky::surface_file_list_view::FileListView;
use tui_lint_arwaky::surface_path_screen::PathScreen;
use tui_lint_arwaky::surface_preview_view::PreviewView;
use tui_lint_arwaky::surface_shortcut_component::ShortcutComponent;
use tui_lint_arwaky::surface_status_component::StatusComponent;
use tui_lint_arwaky::surface_tree_view::TreeView;

fn buffer_text(terminal: &Terminal<TestBackend>) -> String {
    terminal
        .backend()
        .buffer()
        .content
        .iter()
        .map(|cell| cell.symbol())
        .collect()
}

fn file(name: &str, full_path: &str) -> FileEntry {
    FileEntry {
        name: name.to_string(),
        full_path: full_path.to_string(),
        is_dir: false,
        layer: AesLayer::None,
        violation_count: 0,
        extension: "rs".to_string(),
        size_bytes: 1,
    }
}

#[test]
fn file_list_renders_search_empty_state_with_panel_chrome() {
    let mut state = AppState::new("/project".to_string());
    state.path_dialog.visible = false;
    state
        .navigation
        .entries
        .push(file("main.rs", "/project/main.rs"));
    state.search.mode = true;
    state.search.query = "missing".to_string();
    state.compute_filtered_indices();

    let mut terminal = Terminal::new(TestBackend::new(50, 10)).unwrap();
    terminal
        .draw(|frame| FileListView::new().render(&state, frame, frame.area()))
        .unwrap();
    let text = buffer_text(&terminal);
    assert!(text.contains("Search: missing"));
    assert!(text.contains("No matches"));
}

#[test]
fn tree_renders_leaf_of_a_deep_path() {
    let mut state = AppState::new("/project".to_string());
    state.path_dialog.visible = false;
    state.navigation.current_dir = "/project/a/b/c/d/e/f".to_string();
    let mut terminal = Terminal::new(TestBackend::new(30, 6)).unwrap();
    terminal
        .draw(|frame| TreeView::new().render(&state, frame, frame.area()))
        .unwrap();
    assert!(buffer_text(&terminal).contains("f/"));
}

#[test]
fn preview_renders_content_and_title() {
    let mut state = AppState::new("/project".to_string());
    state.path_dialog.visible = false;
    state.preview.mode = PreviewMode::LintResults;
    state.preview.text = "lint result".to_string();
    let mut terminal = Terminal::new(TestBackend::new(40, 8)).unwrap();
    terminal
        .draw(|frame| PreviewView::new().render(&state, frame, frame.area()))
        .unwrap();
    let text = buffer_text(&terminal);
    assert!(text.contains("Lint Results"));
    assert!(text.contains("lint result"));
}

#[test]
fn shortcut_bar_renders_bindings_from_the_central_table() {
    let mut state = AppState::new("/project".to_string());
    state.path_dialog.visible = false;
    state.preview.mode = PreviewMode::ActionOutput;
    let mut terminal = Terminal::new(TestBackend::new(240, 4)).unwrap();
    terminal
        .draw(|frame| ShortcutComponent::new().render(&state, frame, frame.area()))
        .unwrap();
    let text = buffer_text(&terminal);
    // Action-output context uses the same central bindings with their result labels.
    assert!(text.contains("c:re-check"));
    assert!(text.contains("?:help"));
}

#[test]
fn status_bar_renders_state_values() {
    let mut state = AppState::new("/project".to_string());
    state.path_dialog.visible = false;
    state.status_message = "Ready for checks".to_string();
    let mut terminal = Terminal::new(TestBackend::new(70, 2)).unwrap();
    terminal
        .draw(|frame| StatusComponent::new().render(&state, frame, frame.area()))
        .unwrap();
    assert!(buffer_text(&terminal).contains("Ready for checks"));
}

#[test]
fn path_dialog_renders_input_and_controls() {
    let mut state = AppState::new("/project".to_string());
    state.path_dialog.visible = true;
    state.path_dialog.input = "/tmp/workspace".to_string();
    let mut terminal = Terminal::new(TestBackend::new(70, 20)).unwrap();
    terminal
        .draw(|frame| PathScreen::new().render(&state, frame, frame.area()))
        .unwrap();
    let text = buffer_text(&terminal);
    assert!(text.contains("Enter Project Path"));
    assert!(text.contains("/tmp/workspace"));
}

#[test]
fn app_state_navigation_and_search_are_real_mutations() {
    let mut state = AppState::new("/project".to_string());
    state.navigation.panel_focus = PanelFocus::FileList;
    state.navigation.entries = vec![
        file("one.rs", "/project/one.rs"),
        file("two.rs", "/project/two.rs"),
    ];
    state.select_next();
    assert_eq!(state.navigation.selected_index, 1);
    state.search.mode = true;
    state.search.query = "two".to_string();
    state.compute_filtered_indices();
    assert_eq!(state.search.filtered_indices, vec![1]);
    assert_eq!(state.navigation.selected_index, 1);
}
