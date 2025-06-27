use serde::{Deserialize, Serialize};
use serde_json::Value;
use serde_with::{skip_serializing_none, serde_as};
use crate::json_coercion::{AsString, AsF64};

#[cfg(feature = "utoipa")]
use utoipa::ToSchema;

/// Offers insight into the impression, like viewability or CTR.
#[serde_as]
#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "utoipa", derive(ToSchema))]
pub struct Metric {
    /// Type of metric being presented.
    #[serde(rename = "type")]
    #[serde_as(as = "AsString")]
    pub type_: String, // Renamed to avoid keyword conflict
    /// Value of the metric (probabilities 0.0–1.0).
    #[serde_as(as = "AsF64")]
    pub value: f64,
    /// Source of the value. Recommended.
    #[serde_as(as = "Option<AsString>")]
    pub vendor: Option<String>,
    /// Placeholder for exchange-specific extensions.
    pub ext: Option<Value>,
}