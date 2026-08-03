use criterion::{Criterion, criterion_group, criterion_main};
use shared::role_rules::IRoleAuditAggregate;

fn bench_role_audit(c: &mut Criterion) {
    c.bench_function("role_container_creation", |b| {
        b.iter(|| {
            let fs =
                filesystem::root_filesystem_container::FilesystemContainer::new().orchestrator();
            role_rules_lint_arwaky::root_role_rules_container::RoleRulesContainer::new(fs)
        });
    });
}

criterion_group!(benches, bench_role_audit);
criterion_main!(benches);
