use criterion::{Criterion, criterion_group, criterion_main};

fn bench_role_audit(c: &mut Criterion) {
    c.bench_function("role_container_creation", |b| {
        b.iter(|| {
            role_rules_lint_arwaky::root_role_rules_container::RoleContainer::new_with_config(
                Default::default(),
            )
        });
    });
}

criterion_group!(benches, bench_role_audit);
criterion_main!(benches);
