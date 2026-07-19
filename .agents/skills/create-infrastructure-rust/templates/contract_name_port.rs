use shared::<domain>::taxonomy_<name>_vo::<VO>;

pub trait I<Name>Port: Send + Sync {
    fn method_name(
        &self,
        param: &VO,
    ) -> Result<VO, Error>;
}
