use thiserror::Error;

use crate::<domain>::taxonomy_<name>_vo::<VO>;

#[derive(Debug, Error)]
pub enum <Name>Error {
    #[error("Error message: {0}")]
    Variant(#[source] std::io::Error),
}
