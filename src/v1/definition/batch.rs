use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Batch<T> {
    pub(in crate::v1) offset: u64,
    pub(in crate::v1) next: Option<u64>,
    #[serde(default)]
    pub(in crate::v1) data: Vec<T>,
}
