use serde::{Deserialize, Serialize};
use serde_json::Value;
use serde_with::{skip_serializing_none, serde_as};
use crate::json_coercion::{AsString, AsF64, AsEnum};
use crate::adcom;

#[cfg(feature = "utoipa")]
use utoipa::ToSchema;

/// Represents the impression multiplier for DOOH/CTV.
#[serde_as]
#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[cfg_attr(feature = "utoipa", derive(ToSchema))]
pub struct Qty {
    /// Quantity of billable events if purchased.
    #[serde_as(as = "AsF64")]
    pub multiplier: f64,
    /// Source type of quantity measurement. Refer to `AdCOM 1.0` List: Multiplier Measurement Source Types. Recommended.
    #[serde_as(as = "Option<AsEnum<adcom::enums::DoohMultiplierMeasurementSourceType>>")]
    pub sourcetype: Option<adcom::enums::DoohMultiplierMeasurementSourceType>,
    /// Top-level business domain of measurement vendor. Required if sourcetype=1.
    #[serde_as(as = "Option<AsString>")]
    pub vendor: Option<String>,
    /// Placeholder for vendor specific extensions.
    pub ext: Option<Value>,
}