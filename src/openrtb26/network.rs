use serde::{Deserialize, Serialize};
use serde_json::Value;
use serde_with::{skip_serializing_none, serde_as};
use crate::json_coercion::AsString;

#[cfg(feature = "utoipa")]
use utoipa::ToSchema;

/// Describes the network an ad will be displayed on.
#[serde_as]
#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "utoipa", derive(ToSchema))]
pub struct Network {
    /// Unique identifier assigned by the publisher.
    #[serde_as(as = "Option<AsString>")]
    pub id: Option<String>,
    /// Network name.
    #[serde_as(as = "Option<AsString>")]
    pub name: Option<String>,
    /// Primary domain of the network. Recommend TLD+1.
    #[serde_as(as = "Option<AsString>")]
    pub domain: Option<String>,
    /// Placeholder for exchange-specific extensions.
    pub ext: Option<Value>,
}
