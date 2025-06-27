use serde::{Deserialize, Serialize};
use serde_json::Value;
use serde_with::{skip_serializing_none, serde_as};
use crate::json_coercion::AsString;

#[cfg(feature = "utoipa")]
use utoipa::ToSchema;

/// Identifies a device's browser/component or platform/OS using User-Agent Client Hints.
#[serde_as]
#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "utoipa", derive(ToSchema))]
pub struct BrandVersion {
    /// Brand identifier.
    #[serde_as(as = "AsString")]
    pub brand: String,
    /// Sequence of version components.
    #[serde_as(as = "Option<Vec<AsString>>")]
    pub version: Option<Vec<String>>,
    /// Placeholder for vendor specific extensions.
    pub ext: Option<Value>,
}