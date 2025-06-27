use serde::{Deserialize, Serialize};
use serde_json::Value;
use serde_with::{skip_serializing_none, serde_as};
use crate::json_coercion::AsString;
use super::Segment;

#[cfg(feature = "utoipa")]
use utoipa::ToSchema;

/// Container for specifying additional data about a related object.
#[serde_as]
#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[cfg_attr(feature = "utoipa", derive(ToSchema))]
pub struct Data {
    /// Exchange-specific ID for the data provider.
    #[serde_as(as = "Option<AsString>")]
    pub id: Option<String>,
    /// Exchange-specific name for the data provider.
    #[serde_as(as = "Option<AsString>")]
    pub name: Option<String>,
    /// Array of Extended Content IDs for video/audio content.
    #[serde_as(as = "Option<Vec<AsString>>")]
    pub cids: Option<Vec<String>>,
    /// Array of Segment objects containing actual data values.
    pub segment: Option<Vec<Segment>>,
    /// Placeholder for exchange-specific extensions.
    pub ext: Option<Value>,
}
