use serde::{Deserialize, Serialize};
use serde_json::Value;
use serde_with::{skip_serializing_none, serde_as};
use crate::json_coercion::AsString;

#[cfg(feature = "utoipa")]
use utoipa::ToSchema;

/// Key-value pairs conveying specific units of data within a Data object.
#[serde_as]
#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[cfg_attr(feature = "utoipa", derive(ToSchema))]
pub struct Segment {
    /// ID of the data segment specific to the data provider.
    #[serde_as(as = "Option<AsString>")]
    pub id: Option<String>,
    /// Name of the data segment specific to the data provider.
    #[serde_as(as = "Option<AsString>")]
    pub name: Option<String>,
    /// String representation of the data segment value.
    #[serde_as(as = "Option<AsString>")]
    pub value: Option<String>,
    /// Placeholder for exchange-specific extensions.
    pub ext: Option<Value>,
}
