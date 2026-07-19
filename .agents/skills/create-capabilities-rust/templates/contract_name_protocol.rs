use shared::<domain>::taxonomy_<name>_vo::<VO>;

pub trait I<Name>Protocol: Send + Sync {
    fn method_name(
        &self,
        param: &VO,
    );
}
