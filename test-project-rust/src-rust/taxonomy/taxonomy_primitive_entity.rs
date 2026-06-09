// AES016: Primitive Usage - raw primitive types in domain entity
pub struct TaxonomyPrimitiveEntity {
    pub user_id: i64,    // should use Value Object
    pub name: String,    // should use Value Object
    pub price: f64,      // should use Value Object
    pub active: bool,    // should use Value Object
}
