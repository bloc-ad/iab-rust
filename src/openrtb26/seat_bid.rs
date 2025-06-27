use serde::{Deserialize, Serialize};
use serde_json::Value;
use serde_with::{skip_serializing_none, serde_as};
use crate::json_coercion::{AsString, AsI64};
use crate::defaults::default_zero;
use super::Bid;

#[cfg(feature = "utoipa")]
use utoipa::ToSchema;

/// Collection of bids from a specific bidder seat.
#[serde_as]
#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[cfg_attr(feature = "utoipa", derive(ToSchema))]
pub struct SeatBid {
    /// Array of 1+ Bid objects.
    pub bid: Vec<Bid>,
    /// ID of the buyer seat on whose behalf this bid is made.
    #[serde_as(as = "Option<AsString>")]
    pub seat: Option<String>,
    /// 0 = impressions can be won individually; 1 = must be won/lost as group.
    #[serde_as(as = "Option<AsI64>")]
    #[serde(default="default_zero")]
    pub group: Option<i64>,
    /// Placeholder for bidder-specific extensions.
    pub ext: Option<Value>,
}
