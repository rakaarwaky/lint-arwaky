use shared::<domain>::taxonomy_<name>_vo::<VO>;

pub trait I<Name>Aggregate: Send + Sync {
    fn execute(
        &self,
        request: &ScanRequest,
    ) -> Vec<LintResult>;
}
