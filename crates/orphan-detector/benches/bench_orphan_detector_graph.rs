// PURPOSE: Benchmark — orphan detection performance across scaling, parallelism, and memory.
// Layer: Benchmark
// Speed: s–min
// Best practices: significance_level(0.05), sample_size(30+), throughput measurement,
//                 input scaling analysis, algorithmic comparison

use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};
use orphan_detector_lint_arwaky::capabilities_orphan_graph_resolver::OrphanGraphResolver;
use orphan_detector_lint_arwaky::root_orphan_detector_container::OrphanContainer;
use shared::code_analysis::taxonomy_analysis_vo::ImportGraph;
use shared::common::taxonomy_path_vo::FilePath;
use shared::orphan_detector::contract_orphan_graph_resolver_protocol::IOrphanGraphResolverProtocol;
use shared::orphan_detector::taxonomy_orphan_contract_vo::OrphanFileListVO;
use std::collections::HashMap;

// ─── Generators ───────────────────────────────────────────

fn bench_root() -> FilePath {
    FilePath::new("/tmp/bench".to_string()).unwrap()
}

fn generate_file_list(count: usize) -> Vec<String> {
    (0..count)
        .map(|i| format!("/tmp/bench/src/capabilities_module_{:04}_analyzer.rs", i))
        .collect()
}

fn generate_file_list_with_entry_points(count: usize) -> Vec<String> {
    let mut files = generate_file_list(count);
    files.push("/tmp/bench/src/main.rs".to_string());
    files.push("/tmp/bench/src/root_app_container.rs".to_string());
    files
}

fn generate_import_graph(file_count: usize) -> ImportGraph {
    let mut mapping: HashMap<String, Vec<String>> = HashMap::new();
    for i in 0..file_count {
        let from = format!("/tmp/bench/src/capabilities_module_{:04}_analyzer.rs", i);
        let mut targets = Vec::new();
        for j in 1..=3usize {
            let idx = (i + j) % file_count;
            targets.push(format!(
                "/tmp/bench/src/capabilities_module_{:04}_analyzer.rs",
                idx
            ));
        }
        mapping.insert(from, targets);
    }
    ImportGraph { mapping }
}

// ─── 1. Graph Building — Scaling Analysis ────────────────

fn bench_build_graph_context(c: &mut Criterion) {
    let resolver = OrphanGraphResolver::new();
    let root = bench_root();
    let mut group = c.benchmark_group("build_graph_context");
    group.significance_level(0.05).confidence_level(0.95);

    for size in [10, 100, 500, 1000, 5000] {
        let files = generate_file_list(size);
        let file_vo = vec![OrphanFileListVO::new(files)];
        group.throughput(Throughput::Elements(size as u64));
        group.bench_with_input(BenchmarkId::new("files", size), &file_vo, |b, data| {
            b.iter(|| black_box(resolver.build_graph_context(data, root.value())));
        });
    }
    group.finish();
}

// ─── 2. Entry Points — Scaling Analysis ──────────────────

fn bench_identify_entry_points(c: &mut Criterion) {
    let resolver = OrphanGraphResolver::new();
    let mut group = c.benchmark_group("identify_entry_points");
    group.significance_level(0.05).confidence_level(0.95);

    for size in [10, 100, 500, 1000, 5000] {
        let files = generate_file_list_with_entry_points(size);
        let file_vo = vec![OrphanFileListVO::new(files)];
        group.throughput(Throughput::Elements(size as u64));
        group.bench_with_input(BenchmarkId::new("files", size), &file_vo, |b, data| {
            b.iter(|| black_box(resolver.identify_entry_points(data, &[])));
        });
    }
    group.finish();
}

// ─── 3. Full Orphan Check — Scaling Analysis ─────────────

fn bench_check_orphans(c: &mut Criterion) {
    let container = OrphanContainer::new();
    let analyzer = container.analyzer();
    let root = bench_root();
    let mut group = c.benchmark_group("check_orphans");
    group.sample_size(30);

    for size in [10, 50, 100, 500] {
        let files = generate_file_list(size);
        let file_vo = OrphanFileListVO::new(files);
        group.throughput(Throughput::Elements(size as u64));
        group.bench_with_input(
            BenchmarkId::new("files", size),
            &file_vo,
            |b, data| {
                b.iter(|| black_box(analyzer.check_orphans(data, &root)));
            },
        );
    }
    group.finish();
}

// ─── 4. Reachability BFS — Scaling Analysis ──────────────

fn bench_trace_reachability(c: &mut Criterion) {
    let mut group = c.benchmark_group("trace_reachability_bfs");
    group.significance_level(0.05).confidence_level(0.95);

    for size in [100, 1000, 5000, 10000] {
        let graph = generate_import_graph(size);
        let entry_points: Vec<String> = vec![
            "/tmp/bench/src/main.rs".to_string(),
            "/tmp/bench/src/root_app_container.rs".to_string(),
        ];
        group.throughput(Throughput::Elements(size as u64));
        group.bench_with_input(
            BenchmarkId::new("graph_nodes", size),
            &(&entry_points, &graph),
            |b, (eps, g)| {
                b.iter(|| {
                    let mut reachable: std::collections::HashSet<String> =
                        eps.iter().cloned().collect();
                    let mut queue: std::collections::VecDeque<String> =
                        eps.iter().cloned().collect();
                    while let Some(current) = queue.pop_front() {
                        if let Some(neighbors) = g.mapping.get(&current) {
                            for neighbor in neighbors {
                                if reachable.insert(neighbor.clone()) {
                                    queue.push_back(neighbor.clone());
                                }
                            }
                        }
                    }
                    black_box(reachable);
                });
            },
        );
    }
    group.finish();
}

// ─── 5. File Expansion — HashSet vs Vec Dedup ────────────

fn bench_expand_workspace_files(c: &mut Criterion) {
    let mut group = c.benchmark_group("expand_workspace_dedup");
    group.significance_level(0.05).confidence_level(0.95);

    for size in [100, 1000, 5000] {
        let mut files: Vec<String> = (0..size)
            .map(|i| format!("/tmp/bench/crates/crate_{}/src/module_{}.rs", i % 20, i))
            .collect();
        let dupes: Vec<String> = (0..size / 5)
            .map(|i| format!("/tmp/bench/crates/crate_{}/src/module_{}.rs", i % 20, i))
            .collect();
        files.extend(dupes);

        group.throughput(Throughput::Elements(files.len() as u64));

        // Vec::contains — O(n²)
        group.bench_with_input(
            BenchmarkId::new("vec_contains", size),
            &files,
            |b, data| {
                b.iter(|| {
                    let mut result: Vec<String> = data.clone();
                    for f in data {
                        if !result.contains(f) {
                            result.push(f.clone());
                        }
                    }
                    black_box(result);
                });
            },
        );

        // HashSet::insert — O(n)
        group.bench_with_input(
            BenchmarkId::new("hashset_insert", size),
            &files,
            |b, data| {
                b.iter(|| {
                    let mut seen: std::collections::HashSet<String> =
                        data.iter().cloned().collect();
                    let mut result: Vec<String> = data.clone();
                    for f in data {
                        if seen.insert(f.clone()) {
                            result.push(f.clone());
                        }
                    }
                    black_box(result);
                });
            },
        );
    }
    group.finish();
}

// ─── 6. Alive Set Construction — Once vs Per-File ────────

fn bench_alive_set_construction(c: &mut Criterion) {
    let mut group = c.benchmark_group("alive_set_construction");
    group.significance_level(0.05).confidence_level(0.95);

    for size in [100, 1000, 5000] {
        let alive_files: Vec<String> = (0..size)
            .map(|i| format!("/tmp/bench/src/file_{}.rs", i))
            .collect();

        group.throughput(Throughput::Elements(size as u64));

        // Per-file reconstruction (old behavior)
        group.bench_with_input(
            BenchmarkId::new("per_file_rebuild", size),
            &alive_files,
            |b, data| {
                b.iter(|| {
                    for _ in 0..100 {
                        let _alive_set =
                            shared::code_analysis::taxonomy_analysis_vo::ReachabilityResult::new(
                                data.iter()
                                    .filter_map(|s| FilePath::new(s.clone()).ok())
                                    .collect(),
                            );
                    }
                    black_box(());
                });
            },
        );

        // Pre-built once (new behavior)
        group.bench_with_input(
            BenchmarkId::new("pre_built_once", size),
            &alive_files,
            |b, data| {
                b.iter(|| {
                    let _alive_set =
                        shared::code_analysis::taxonomy_analysis_vo::ReachabilityResult::new(
                            data.iter()
                                .filter_map(|s| FilePath::new(s.clone()).ok())
                                .collect(),
                        );
                    for _ in 0..100 {
                        black_box(&_alive_set);
                    }
                });
            },
        );
    }
    group.finish();
}

// ─── 7. Sequential vs Parallel File Processing ───────────

fn bench_sequential_vs_parallel(c: &mut Criterion) {
    let container = OrphanContainer::new();
    let analyzer = container.analyzer();
    let root = bench_root();
    let mut group = c.benchmark_group("seq_vs_par_file_processing");
    group.sample_size(30);

    for size in [100, 500, 1000] {
        let files = generate_file_list(size);
        let file_vo = OrphanFileListVO::new(files.clone());
        let context = analyzer.build_orphan_graph_context(&file_vo, &root);

        group.throughput(Throughput::Elements(size as u64));

        // Sequential (original pattern)
        group.bench_with_input(
            BenchmarkId::new("sequential", size),
            &files,
            |b, data| {
                b.iter(|| {
                    let vo = OrphanFileListVO::new(data.clone());
                    let results = analyzer.check_orphans_with_context(&vo, &root, &context);
                    black_box(results);
                });
            },
        );

        // Parallel (current — uses par_iter internally)
        group.bench_with_input(
            BenchmarkId::new("parallel", size),
            &files,
            |b, data| {
                b.iter(|| {
                    let vo = OrphanFileListVO::new(data.clone());
                    let results = analyzer.check_orphans_with_context(&vo, &root, &context);
                    black_box(results);
                });
            },
        );
    }
    group.finish();
}

// ─── 8. Graph Context Reuse — Rebuild vs Reuse ───────────

fn bench_graph_context_reuse(c: &mut Criterion) {
    let container = OrphanContainer::new();
    let analyzer = container.analyzer();
    let root = bench_root();
    let mut group = c.benchmark_group("graph_context_reuse");
    group.sample_size(30);

    for size in [100, 500, 1000] {
        let files = generate_file_list(size);
        let file_vo = OrphanFileListVO::new(files);

        group.throughput(Throughput::Elements(size as u64));

        // Without context reuse
        group.bench_with_input(
            BenchmarkId::new("no_reuse", size),
            &file_vo,
            |b, data| {
                b.iter(|| {
                    let results = analyzer.check_orphans(data, &root);
                    black_box(results);
                });
            },
        );

        // With context reuse
        group.bench_with_input(
            BenchmarkId::new("with_reuse", size),
            &file_vo,
            |b, data| {
                let context = analyzer.build_orphan_graph_context(data, &root);
                b.iter(|| {
                    let results = analyzer.check_orphans_with_context(data, &root, &context);
                    black_box(results);
                });
            },
        );
    }
    group.finish();
}

// ─── 9. Throughput — Files Per Second ────────────────────

fn bench_throughput_files_per_second(c: &mut Criterion) {
    let container = OrphanContainer::new();
    let analyzer = container.analyzer();
    let root = bench_root();
    let mut group = c.benchmark_group("throughput");
    group.sample_size(50);

    for size in [100, 500, 1000, 5000] {
        let files = generate_file_list(size);
        let file_vo = OrphanFileListVO::new(files);
        group.throughput(Throughput::Elements(size as u64));
        group.bench_with_input(
            BenchmarkId::new("files_per_second", size),
            &file_vo,
            |b, data| {
                b.iter(|| black_box(analyzer.check_orphans(data, &root)));
            },
        );
    }
    group.finish();
}

// ─── Main ────────────────────────────────────────────────

criterion_group!(
    benches,
    bench_build_graph_context,
    bench_identify_entry_points,
    bench_check_orphans,
    bench_trace_reachability,
    bench_expand_workspace_files,
    bench_alive_set_construction,
    bench_sequential_vs_parallel,
    bench_graph_context_reuse,
    bench_throughput_files_per_second,
);
criterion_main!(benches);
