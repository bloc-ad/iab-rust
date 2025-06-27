use serde::{Deserialize, Serialize};
use serde_json::Value;
use serde_with::{skip_serializing_none, serde_as};
use crate::json_coercion::{AsString, AsF64, AsI64};
use crate::defaults::{default_usd, default_zero};
use super::DurFloors;

#[cfg(feature = "utoipa")]
use utoipa::ToSchema;

/// Constitutes a specific deal struck between a buyer and a seller.
#[serde_as]
#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[cfg_attr(feature = "utoipa", derive(ToSchema))]
pub struct Deal {
    /// Unique identifier for the direct deal.
    #[serde_as(as = "AsString")]
    pub id: String,
    /// Minimum bid for this impression expressed in CPM.
    #[serde_as(as = "Option<AsF64>")]
    pub bidfloor: Option<f64>,
    /// Currency for bidfloor (ISO-4217 alpha codes).
    #[serde_as(as = "Option<AsString>")]
    #[serde(default="default_usd")]
    pub bidfloorcur: Option<String>,
    /// Optional override of overall auction type (1=First Price, 2=Second Price Plus, 3=Deal Price).
    #[serde_as(as = "Option<AsI64>")]
    pub at: Option<i64>,
    /// Allowed list of buyer seats for this deal.
    #[serde_as(as = "Option<Vec<AsString>>")]
    pub wseat: Option<Vec<String>>,
    /// Array of advertiser domains allowed for this deal.
    #[serde_as(as = "Option<Vec<AsString>>")]
    pub wadomain: Option<Vec<String>>,
    /// Indicates if deal is 'guaranteed' (0 = not guaranteed, 1 = guaranteed).
    #[serde_as(as = "Option<AsI64>")]
    #[serde(default="default_zero")]
    pub guar: Option<i64>,
    /// Minimum CPM per second for video/audio opportunities.
    #[serde_as(as = "Option<AsF64>")]
    pub mincpmpersec: Option<f64>,
    /// Container for floor price by duration information (video/audio).
    pub durfloors: Option<Vec<DurFloors>>,
    /// Placeholder for exchange-specific extensions.
    pub ext: Option<Value>,
}