use serde::{Deserialize, Serialize};
use serde_json::Value;
use serde_with::{skip_serializing_none, serde_as};
use crate::json_coercion::{AsI64, AsF64};
use crate::defaults::default_zero_f64;

#[cfg(feature = "utoipa")]
use utoipa::ToSchema;

/// Allows specifying price floors for video/audio creatives based on duration ranges.
#[serde_as]
#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[cfg_attr(feature = "utoipa", derive(ToSchema))]
pub struct DurFloors {
    /// Low end of duration range (seconds). If missing, unbounded.
    #[serde_as(as = "Option<AsI64>")]
    pub mindur: Option<i64>,
    /// High end of duration range (seconds). If missing, unbounded.
    #[serde_as(as = "Option<AsI64>")]
    pub maxdur: Option<i64>,
    /// Minimum bid (CPM) for this duration range. Defaults to Imp.bidfloor if outside ranges.
    #[serde_as(as = "Option<AsF64>")]
    #[serde(default="default_zero_f64")]
    pub bidfloor: Option<f64>,
    /// Placeholder for vendor specific extensions.
    pub ext: Option<Value>,
}
