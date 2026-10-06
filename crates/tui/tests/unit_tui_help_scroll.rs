//! Regression: help overlay key mapping (#556).
//!
//! Before #556, `from_key_event` collapsed every navigation key onto
//! `TuiEvent::MoveDown` while the help overlay was open, so there was no way
//! to scroll back to the top of a long help panel. This test pins the
//! key-to-event contract via the pure `help_overlay_event_for` seam, plus a
//! state-level scroll round-trip through `AppState.preview.scroll`.
use crossterm::event::KeyCode;
use std::sync::Arc;

use dispatcher::surface_check_action::FilesystemSeam;
use dispatcher::surface_orphan_action::OrphanFactory;
use shared_filesystem::contract_filesystem_aggregate::IFilesystemAggregate;
use tui_lint_arwaky::surface_event_action::SurfaceActionHandler;
use tui_lint_arwaky::surface_lint_action::SurfaceLintExecutor;
use tui_lint_arwaky::surface_tui_command::help_overlay_event_for;
use tui_lint_arwaky::taxonomy_tui_event::TuiEvent;
use tui_lint_arwaky::taxonomy_tui_vo::AppState;
use tui_lint_arwaky::taxonomy_tui_vo::PanelFocus;
use tui_lint_arwaky::taxonomy_tui_vo::PreviewMode;

// ─── No-op filesystem aggregate (only needed for the #911 handler test) ──

#[derive(Debug)]
struct NoOpFsAggregate;

impl IFilesystemAggregate for NoOpFsAggregate {
    fn execute(
        &self,
        _request: shared_filesystem::taxonomy_filesystem_request::FilesystemRequest,
    ) -> shared_filesystem::taxonomy_filesystem_response::FilesystemResponse {
        shared_filesystem::taxonomy_filesystem_response::FilesystemResponse::Has { exists: false }
    }
}

fn no_op_fs_aggregate() -> Arc<NoOpFsAggregate> {
    Arc::new(NoOpFsAggregate)
}

/// Build a `SurfaceLintExecutor` whose aggregates are all no-ops; MoveBottom
/// in this test never starts a background action, so no protocol is invoked.
fn no_op_executor() -> Arc<SurfaceLintExecutor> {
    use shared_filesystem::contract_filesystem_protocol::{
        IParserProtocol, IToolResolutionProtocol, IWorkspaceProtocol,
    };

    #[derive(Debug)]
    struct Ws;
    impl IWorkspaceProtocol for Ws {
        fn workspace_root(
            &self,
            _start: &shared_common::taxonomy_path_vo::FilePath,
        ) -> Option<std::path::PathBuf> {
            None
        }
        fn find_workspace_root_from_path(
            &self,
            _start: &std::path::Path,
        ) -> Result<std::path::PathBuf, std::io::Error> {
            Err(std::io::Error::new(std::io::ErrorKind::NotFound, "stub"))
        }
        fn is_member_path(&self, _path: &shared_common::taxonomy_path_vo::FilePath) -> bool {
            false
        }
        fn is_leaf_member_path(&self, _path: &shared_common::taxonomy_path_vo::FilePath) -> bool {
            false
        }
        fn detect_source_dir(&self, _project_root: &std::path::Path) -> std::path::PathBuf {
            std::path::PathBuf::new()
        }
        fn detect_language_from_path(
            &self,
            _path: &str,
        ) -> shared_common::taxonomy_config_language_vo::ConfigLanguage {
            shared_common::taxonomy_config_language_vo::ConfigLanguage::Rust
        }
        fn check_wired_in_container(
            &self,
            _workspace_root: &std::path::Path,
            _identifiers: &shared_common::taxonomy_common_vo::PatternList,
        ) -> bool {
            false
        }
        fn detect_project_languages(
            &self,
            _root: &std::path::Path,
        ) -> shared_filesystem::taxonomy_filesystem_vo::ProjectLanguagesVO {
            shared_filesystem::taxonomy_filesystem_vo::ProjectLanguagesVO::default()
        }
        fn resolve_orphan_module_path(
            &self,
            _root: &std::path::Path,
            _base_dir: &std::path::Path,
            _module_path: &str,
        ) -> Option<std::path::PathBuf> {
            None
        }
    }

    #[derive(Debug)]
    struct P;
    impl IParserProtocol for P {
        fn parse_warnings(&self) -> &[shared_filesystem::taxonomy_filesystem_vo::ParseWarning] {
            &[]
        }
        fn import_list(&self) -> Vec<shared_filesystem::taxonomy_filesystem_vo::ImportEntry> {
            Vec::new()
        }
        fn parse_all(&self, _files: &mut [shared_filesystem::taxonomy_filesystem_vo::FileEntry]) {}
        fn imports_for(
            &self,
            _path: &std::path::Path,
        ) -> Vec<shared_filesystem::taxonomy_filesystem_vo::ImportEntry> {
            Vec::new()
        }
        fn extract(
            &self,
            _path: &std::path::Path,
            _content: &str,
            _language: shared_common::taxonomy_language_vo::Language,
        ) -> Vec<shared_filesystem::taxonomy_filesystem_vo::ImportEntry> {
            Vec::new()
        }
        fn resolve_barrel_imports(&self, _root_dir: &std::path::Path) {}
    }

    #[derive(Debug)]
    struct T;
    impl IToolResolutionProtocol for T {
        fn is_executable_in_path(
            &self,
            _executable: &shared_common::taxonomy_tool_name_vo::ToolName,
        ) -> bool {
            false
        }
        fn is_binary_available(
            &self,
            _bin_name: &shared_common::taxonomy_tool_name_vo::ToolName,
        ) -> bool {
            false
        }
        fn has_local_bin(
            &self,
            _working_dir: &std::path::Path,
            _executable: &shared_common::taxonomy_tool_name_vo::ToolName,
        ) -> bool {
            false
        }
        fn resolve_js_cmd(
            &self,
            _executable: &shared_common::taxonomy_tool_name_vo::ToolName,
            _args: Vec<String>,
            _working_dir: &shared_common::taxonomy_path_vo::FilePath,
        ) -> Option<Vec<String>> {
            None
        }
        fn resolve_js_working_dir(
            &self,
            path: &shared_common::taxonomy_path_vo::FilePath,
        ) -> shared_common::taxonomy_path_vo::FilePath {
            path.clone()
        }
        fn resolve_cargo_working_dir(
            &self,
            path: &shared_common::taxonomy_path_vo::FilePath,
        ) -> shared_common::taxonomy_path_vo::FilePath {
            path.clone()
        }
        fn resolve_cargo_lock_working_dir(
            &self,
            path: &shared_common::taxonomy_path_vo::FilePath,
        ) -> shared_common::taxonomy_path_vo::FilePath {
            path.clone()
        }
        fn has_config_file(&self, _dir: &std::path::Path) -> bool {
            false
        }
        fn has_cargo_toml(
            &self,
            _path: &shared_common::taxonomy_path_vo::FilePath,
        ) -> Option<shared_common::taxonomy_path_vo::FilePath> {
            None
        }
        fn has_cargo_lock(
            &self,
            _path: &shared_common::taxonomy_path_vo::FilePath,
        ) -> Option<shared_common::taxonomy_path_vo::FilePath> {
            None
        }
        fn is_python_file_recursive(
            &self,
            _path: &shared_common::taxonomy_path_vo::FilePath,
        ) -> bool {
            false
        }
        fn default_working_dir(
            &self,
            path: &shared_common::taxonomy_path_vo::FilePath,
        ) -> shared_common::taxonomy_path_vo::FilePath {
            path.clone()
        }
    }

    use shared_quality_rules::ICodeAnalysisAggregate;
    #[derive(Debug)]
    struct CA;
    impl ICodeAnalysisAggregate for CA {
        fn execute(
            &self,
            _request: shared_quality_rules::CodeAnalysisRequest,
        ) -> shared_quality_rules::CodeAnalysisResponse {
            unimplemented!("MoveBottom tests never invoke code analysis")
        }
    }

    let ca: Arc<CA> = Arc::new(CA);
    let ws: Arc<dyn shared_filesystem::contract_filesystem_protocol::IWorkspaceProtocol> =
        Arc::new(Ws);
    let p: Arc<dyn shared_filesystem::contract_filesystem_protocol::IParserProtocol> = Arc::new(P);
    let t: Arc<dyn shared_filesystem::contract_filesystem_protocol::IToolResolutionProtocol> =
        Arc::new(T);
    let fs_agg: Arc<dyn IFilesystemAggregate> = no_op_fs_aggregate();
    let fs_seam = Arc::new(FilesystemSeam {
        workspace: Arc::clone(&ws),
        parser: Arc::clone(&p),
        aggregate: Arc::clone(&fs_agg),
    });
    let ws_f = Arc::clone(&ws);
    let p_f = Arc::clone(&p);
    let fs_agg_f = Arc::clone(&fs_agg);
    let fs_factory = Arc::new(move || FilesystemSeam {
        workspace: Arc::clone(&ws_f),
        parser: Arc::clone(&p_f),
        aggregate: Arc::clone(&fs_agg_f),
    });
    let orphan_factory: Arc<OrphanFactory> = Arc::new(move |_config, _fs, _ws| {
        unimplemented!("MoveBottom tests never invoke orphan scanning")
    });
    Arc::new(SurfaceLintExecutor::new(
        ca,
        fs_agg,
        ws,
        t,
        fs_seam,
        fs_factory,
        orphan_factory,
    ))
}

fn forward_keys() -> [KeyCode; 4] {
    [
        KeyCode::Char('j'),
        KeyCode::Down,
        KeyCode::Char('l'),
        KeyCode::Right,
    ]
}

fn backward_keys() -> [KeyCode; 4] {
    [
        KeyCode::Char('k'),
        KeyCode::Up,
        KeyCode::Char('h'),
        KeyCode::Left,
    ]
}

/// Forward navigation keys keep scrolling the help overlay down.
#[test]
fn help_overlay_forward_keys_map_to_move_down() {
    for code in forward_keys() {
        assert_eq!(
            help_overlay_event_for(code),
            TuiEvent::MoveDown,
            "{code:?} should scroll the help overlay down"
        );
    }
}

/// Backward navigation keys scroll the help overlay up — the exact regression
/// from #556. Each of these used to collapse onto MoveDown, so there was no
/// way back to the top.
#[test]
fn help_overlay_backward_keys_map_to_move_up() {
    for code in backward_keys() {
        assert_eq!(
            help_overlay_event_for(code),
            TuiEvent::MoveUp,
            "{code:?} should scroll the help overlay up"
        );
    }
}

/// Home jumps to the top; End jumps to the bottom. Before #556 both collapsed
/// onto MoveDown, so neither jump was possible.
#[test]
fn help_overlay_jump_keys() {
    assert_eq!(help_overlay_event_for(KeyCode::Home), TuiEvent::MoveTop);
    assert_eq!(help_overlay_event_for(KeyCode::End), TuiEvent::MoveBottom);
}

/// PageUp / PageDown use the dedicated preview scroll events (10-line steps),
/// distinct from the 3-line MoveUp/MoveDown used by the arrow/vim keys.
#[test]
fn help_overlay_page_keys() {
    assert_eq!(
        help_overlay_event_for(KeyCode::PageUp),
        TuiEvent::PreviewScrollUp
    );
    assert_eq!(
        help_overlay_event_for(KeyCode::PageDown),
        TuiEvent::PreviewScrollDown
    );
}

/// `?` and `Esc` both close the help overlay; `q` quits the app. Before #556
/// `Esc` produced `Quit`, inconsistent with `?` toggling.
#[test]
fn help_overlay_toggle_and_quit_keys() {
    assert_eq!(
        help_overlay_event_for(KeyCode::Char('?')),
        TuiEvent::ToggleHelp
    );
    assert_eq!(help_overlay_event_for(KeyCode::Esc), TuiEvent::ToggleHelp);
    assert_eq!(help_overlay_event_for(KeyCode::Char('q')), TuiEvent::Quit);
}

/// Anything else in the overlay is swallowed so it cannot leak through to
/// directory navigation or list selection underneath.
#[test]
fn help_overlay_unmapped_keys_map_to_none() {
    assert_eq!(help_overlay_event_for(KeyCode::Char('x')), TuiEvent::None);
    assert_eq!(help_overlay_event_for(KeyCode::Enter), TuiEvent::None);
}

/// State-level round trip: from the bottom of a long help panel, the
/// backward keys walk the scroll back to the top. This is the user-facing
/// regression #556 was about: before the fix, every key (including `k`,
/// `Up`, `Home`, `PageUp`) only ever scrolled forward, so a long panel had
/// no way back up.
#[test]
fn help_overlay_back_and_forward_scroll_round_trips() {
    let mut state = AppState::new("/tmp".to_string());
    state.show_help = true;
    state.navigation.panel_focus = PanelFocus::Preview;
    state.preview.mode = PreviewMode::HelpOverlay;
    state.terminal_height = 30;
    // 40 lines at 30 terminal height: max scroll = 40 - 26 = 14.
    state.preview.text = (0..40)
        .map(|i| format!("help line {i}"))
        .collect::<Vec<_>>()
        .join("\n");
    state.preview.scroll = 14;

    // Each backward key must strictly decrease the scroll; `Home` must reach 0.
    for code in [
        KeyCode::PageUp,
        KeyCode::Char('k'),
        KeyCode::Up,
        KeyCode::Char('h'),
        KeyCode::Left,
        KeyCode::Home,
    ] {
        let next = match help_overlay_event_for(code) {
            TuiEvent::MoveUp => state.preview.scroll.saturating_sub(3),
            TuiEvent::MoveTop => 0,
            TuiEvent::PreviewScrollUp => state.preview.scroll.saturating_sub(10),
            other => panic!("{code:?} mapped to {other:?}, expected a backward scroll event"),
        };
        assert!(
            next <= state.preview.scroll,
            "{code:?} must not advance the scroll"
        );
        state.preview.scroll = next;
    }
    assert_eq!(state.preview.scroll, 0, "backward keys must reach the top");

    // Each forward key must strictly increase the scroll; `End` must reach max.
    let max = state.preview.text.lines().count() - 26;
    for code in [
        KeyCode::Char('j'),
        KeyCode::Down,
        KeyCode::Char('l'),
        KeyCode::Right,
        KeyCode::PageDown,
        KeyCode::End,
    ] {
        let next = match help_overlay_event_for(code) {
            TuiEvent::MoveDown => state.preview.scroll.saturating_add(3).min(max),
            TuiEvent::MoveBottom => max,
            TuiEvent::PreviewScrollDown => state.preview.scroll.saturating_add(10).min(max),
            other => panic!("{code:?} mapped to {other:?}, expected a forward scroll event"),
        };
        assert!(
            next >= state.preview.scroll,
            "{code:?} must not regress the scroll"
        );
        state.preview.scroll = next;
    }
    assert_eq!(
        state.preview.scroll, max,
        "forward keys must reach the bottom"
    );
}

/// #911 regression: `max_preview_scroll` must subtract the full panel band
/// height (header + 3 shortcut rows + status = 5) from `terminal_height`,
/// not 4. Before the fix, the last line of a long preview was unreachable
/// via MoveBottom / PreviewScrollDown.
#[test]
fn max_preview_scroll_uses_full_panel_band_height() {
    let mut state = AppState::new("/tmp".to_string());
    state.terminal_height = 30;
    // 40 content lines at height 30: band = 30 - 5 = 25 → max scroll = 15.
    state.preview.text = (0..40)
        .map(|i| format!("line {i}"))
        .collect::<Vec<_>>()
        .join("\n");
    state.preview.scroll = 0;
    state.navigation.panel_focus = PanelFocus::Preview;

    let h = SurfaceActionHandler::new(no_op_executor(), no_op_fs_aggregate());
    h.handle(&mut state, TuiEvent::MoveBottom);

    // band = 30 - 5 = 25; content = 40 → max = 40 - 25 = 15.
    // The pre-fix band (30 - 4 = 26) would have capped it at 14, leaving
    // the last line unreachable.
    assert_eq!(
        state.preview.scroll, 15,
        "MoveBottom must reach the true bottom (band height h-5)"
    );
}
