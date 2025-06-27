use serde::{Deserialize, Serialize};
use serde_json::Value;
use serde_with::{skip_serializing_none, serde_as};
use crate::json_coercion::{AsString, AsI64};
use super::SupplyChainNode;

#[cfg(feature = "utoipa")]
use utoipa::ToSchema;

/// Represents the chain of entities involved in the direct flow of payment for inventory.
#[serde_as]
#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "utoipa", derive(ToSchema))]
pub struct SupplyChain {
    /// Flag indicating if chain contains all nodes back to owner (0=no, 1=yes).
    #[serde_as(as = "AsI64")]
    pub complete: i64,
    /// Array of `SupplyChainNode` objects in order.
    pub nodes: Vec<SupplyChainNode>,
    /// Version of the supply chain specification.
    #[serde_as(as = "AsString")]
    pub ver: String,
    /// Placeholder for exchange-specific extensions.
    pub ext: Option<Value>,
}
