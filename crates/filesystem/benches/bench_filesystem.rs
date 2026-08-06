// Benchmark tests for filesystem — AST parsing, file I/O, directory scanning, graph construction.
// Best practices: significance_level(0.05), sample_size(30+), throughput measurement
use criterion::{BenchmarkId, Criterion, Throughput, criterion_group, criterion_main};
use filesystem_lint_arwaky::capabilities_ast_parser::ASTParser;
use filesystem_lint_arwaky::capabilities_dependency_graph::DependencyGraph;
use filesystem_lint_arwaky::capabilities_filesystem_io::CapabilitiesFileSystemIO;
use filesystem_lint_arwaky::capabilities_tool_resolution::CapabilitiesToolResolution;
use filesystem_lint_arwaky::capabilities_workspace_root_finder::CapabilitiesWorkspace;
use shared::common::taxonomy_common_vo::PatternList;
use shared::common::taxonomy_language_vo::Language;
use shared::common::taxonomy_path_vo::FilePath;
use shared::filesystem::contract_filesystem_io_protocol::IFileSystemIOProtocol;
use shared::filesystem::contract_graph_protocol::IGraphProtocol;
use shared::filesystem::contract_parser_protocol::IParserProtocol;
use shared::filesystem::contract_tool_resolution_protocol::IToolResolutionProtocol;
use shared::filesystem::contract_workspace_protocol::IWorkspaceProtocol;
use shared::filesystem::taxonomy_filesystem_vo::{
    DefinitionEntry, FileEntry, ImportEntry, ImportType,
};
use std::path::PathBuf;
use tempfile::TempDir;

fn generate_rust_files(n: usize) -> Vec<FileEntry> {
    (0..n)
        .map(|i| {
            let content = format!(
                "use std::collections::HashMap;\npub struct Item_{} {{ value: u64 }}\nimpl Item_{} {{ pub fn new(v: u64) -> Self {{ Self {{ value: v }} }} }}\n",
                i, i
            );
            FileEntry {
                path: PathBuf::from(format!("/bench/file_{}.rs", i)),
                extension: "rs".to_string(),
                language: Language::Rust,
                size: content.len() as u64,
                content,
                parse_ok: false,
                parse_metadata: None,
            }
        })
        .collect()
}

fn bench_parse_small(c: &mut Criterion) {
    let mut group = c.benchmark_group("parse_small");
    group.significance_level(0.05).confidence_level(0.95);
    group.sample_size(30);

    group.bench_function("10_files", |b| {
        b.iter(|| {
            let parser = ASTParser::new();
            let mut files = generate_rust_files(10);
            parser.parse_all(&mut files);
            std::hint::black_box(&files);
        });
    });

    group.bench_function("50_files", |b| {
        b.iter(|| {
            let parser = ASTParser::new();
            let mut files = generate_rust_files(50);
            parser.parse_all(&mut files);
            std::hint::black_box(&files);
        });
    });

    group.finish();
}

fn bench_parse_large(c: &mut Criterion) {
    let mut group = c.benchmark_group("parse_large");
    group.significance_level(0.05).confidence_level(0.95);

    for n in [100, 500, 1000] {
        group.throughput(Throughput::Elements(n as u64));
        group.bench_with_input(BenchmarkId::new("rust_files", n), &n, |b, &n| {
            b.iter(|| {
                let parser = ASTParser::new();
                let mut files = generate_rust_files(n);
                parser.parse_all(&mut files);
                std::hint::black_box(&files);
            });
        });
    }

    group.finish();
}

fn bench_extract_imports(c: &mut Criterion) {
    let mut group = c.benchmark_group("extract_imports");
    group.sample_size(30);

    let rust_code = "use std::collections::HashMap;\nuse std::io::Read;\nuse crate::module::Item;\npub fn foo() {}\n";
    let py_code = "import os\nimport sys\nfrom pathlib import Path\ndef hello(): pass\n";
    let ts_code = "import { foo } from './bar';\nimport * as utils from './utils';\nexport function hello() {}\n";

    group.bench_function("rust", |b| {
        let parser = ASTParser::new();
        b.iter(|| {
            std::hint::black_box(parser.extract(
                &PathBuf::from("/bench.rs"),
                rust_code,
                Language::Rust,
            ))
        });
    });

    group.bench_function("python", |b| {
        let parser = ASTParser::new();
        b.iter(|| {
            std::hint::black_box(parser.extract(
                &PathBuf::from("/bench.py"),
                py_code,
                Language::Python,
            ))
        });
    });

    group.bench_function("typescript", |b| {
        let parser = ASTParser::new();
        b.iter(|| {
            std::hint::black_box(parser.extract(
                &PathBuf::from("/bench.ts"),
                ts_code,
                Language::TypeScript,
            ))
        });
    });

    group.finish();
}

fn bench_file_io(c: &mut Criterion) {
    let mut group = c.benchmark_group("file_io");
    group.sample_size(30);

    let tmp = TempDir::new().unwrap();
    let file = tmp.path().join("bench.txt");
    let content = "x".repeat(4096);

    let io = CapabilitiesFileSystemIO::with_default_timing();
    group.bench_function("write_4k", |b| {
        b.iter(|| {
            io.write_string(&file, &content).unwrap();
        });
    });

    group.bench_function("read_4k", |b| {
        io.write_string(&file, &content).unwrap();
        b.iter(|| {
            std::hint::black_box(io.read_to_string(&file).unwrap());
        });
    });

    group.bench_function("scan_directory_100", |b| {
        let scan_dir = tmp.path().join("scan_bench");
        std::fs::create_dir_all(&scan_dir).unwrap();
        for i in 0..100 {
            std::fs::write(scan_dir.join(format!("file_{}.rs", i)), "fn f() {}").unwrap();
        }
        b.iter(|| {
            std::hint::black_box(
                io.scan_directory_with_ignored(&scan_dir, &PatternList::default()),
            );
        });
    });

    group.finish();
}

fn bench_graph_construction(c: &mut Criterion) {
    let mut group = c.benchmark_group("graph_construction");
    group.significance_level(0.05).confidence_level(0.95);

    for n in [10, 50, 200] {
        group.throughput(Throughput::Elements(n as u64));
        group.bench_with_input(BenchmarkId::new("build_graph", n), &n, |b, &n| {
            let files: Vec<FileEntry> = (0..n)
                .map(|i| FileEntry {
                    path: PathBuf::from(format!("/g/file_{}.rs", i)),
                    extension: "rs".to_string(),
                    language: Language::Rust,
                    size: 0,
                    content: String::new(),
                    parse_ok: true,
                    parse_metadata: None,
                })
                .collect();
            let imports: Vec<ImportEntry> = (1..n)
                .map(|i| ImportEntry {
                    source_file: PathBuf::from(format!("/g/file_{}.rs", i)),
                    raw_path: format!("/g/file_{}.rs", i - 1),
                    resolved_path: Some(PathBuf::from(format!("/g/file_{}.rs", i - 1))),
                    import_type: ImportType::Use,
                    language: Language::Rust,
                    is_dynamic: false,
                    is_resolved: true,
                    symbols: vec![],
                    is_reexport: false,
                    is_wildcard: false,
                })
                .collect();
            let definitions: Vec<DefinitionEntry> = (0..n)
                .map(|i| DefinitionEntry {
                    name: format!("Type_{}", i),
                    file_path: PathBuf::from(format!("/g/file_{}.rs", i)),
                    language: Language::Rust,
                })
                .collect();

            b.iter(|| {
                let graph = DependencyGraph::new();
                graph.build_graph(&imports, &files, &definitions, &[]);
                std::hint::black_box(graph.stats());
            });
        });
    }

    group.finish();
}

fn bench_graph_queries(c: &mut Criterion) {
    let mut group = c.benchmark_group("graph_queries");
    group.sample_size(30);

    let n = 500;
    let files: Vec<FileEntry> = (0..n)
        .map(|i| FileEntry {
            path: PathBuf::from(format!("/q/file_{}.rs", i)),
            extension: "rs".to_string(),
            language: Language::Rust,
            size: 0,
            content: String::new(),
            parse_ok: true,
            parse_metadata: None,
        })
        .collect();
    let imports: Vec<ImportEntry> = (1..n)
        .map(|i| ImportEntry {
            source_file: PathBuf::from(format!("/q/file_{}.rs", i)),
            raw_path: format!("/q/file_{}.rs", i - 1),
            resolved_path: Some(PathBuf::from(format!("/q/file_{}.rs", i - 1))),
            import_type: ImportType::Use,
            language: Language::Rust,
            is_dynamic: false,
            is_resolved: true,
            symbols: vec![],
            is_reexport: false,
            is_wildcard: false,
        })
        .collect();
    let graph = DependencyGraph::new();
    graph.build_graph(&imports, &files, &[], &[]);

    group.bench_function("dependents", |b| {
        b.iter(|| {
            std::hint::black_box(graph.dependents(&PathBuf::from(format!("/q/file_{}.rs", n / 2))));
        });
    });

    group.bench_function("dependencies", |b| {
        b.iter(|| {
            std::hint::black_box(
                graph.dependencies(&PathBuf::from(format!("/q/file_{}.rs", n / 2))),
            );
        });
    });

    group.bench_function("reachable", |b| {
        b.iter(|| {
            std::hint::black_box(graph.reachable(
                &PathBuf::from("/q/file_0.rs"),
                &PathBuf::from(format!("/q/file_{}.rs", n - 1)),
            ));
        });
    });

    group.finish();
}

fn bench_workspace_detection(c: &mut Criterion) {
    let mut group = c.benchmark_group("workspace_detection");
    group.sample_size(30);

    let tmp = TempDir::new().unwrap();
    std::fs::write(
        tmp.path().join("Cargo.toml"),
        "[workspace]\nmembers=[\"crates/*\"]\n",
    )
    .unwrap();
    let ws = CapabilitiesWorkspace::new();

    group.bench_function("workspace_root", |b| {
        let fp = FilePath::new(tmp.path().to_string_lossy().to_string()).unwrap();
        b.iter(|| {
            std::hint::black_box(ws.workspace_root(&fp));
        });
    });

    group.bench_function("detect_language", |b| {
        b.iter(|| {
            std::hint::black_box(ws.detect_language_from_path("src/main.rs"));
            std::hint::black_box(ws.detect_language_from_path("module.py"));
            std::hint::black_box(ws.detect_language_from_path("index.ts"));
        });
    });

    group.finish();
}

fn bench_tool_resolution(c: &mut Criterion) {
    let mut group = c.benchmark_group("tool_resolution");
    group.sample_size(30);

    let tool = CapabilitiesToolResolution::new();
    let sh_name = shared::filesystem::taxonomy_filesystem_vo::ToolName::new("sh").unwrap();

    group.bench_function("is_binary_available", |b| {
        b.iter(|| {
            std::hint::black_box(tool.is_binary_available(&sh_name));
        });
    });

    group.bench_function("has_config_file", |b| {
        let tmp = TempDir::new().unwrap();
        std::fs::write(tmp.path().join("Cargo.toml"), "").unwrap();
        b.iter(|| {
            std::hint::black_box(tool.has_config_file(tmp.path()));
        });
    });

    group.finish();
}

criterion_group!(
    benches,
    bench_parse_small,
    bench_parse_large,
    bench_extract_imports,
    bench_file_io,
    bench_graph_construction,
    bench_graph_queries,
    bench_workspace_detection,
    bench_tool_resolution,
);
criterion_main!(benches);
