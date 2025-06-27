use serde::{Deserialize, Serialize};
use serde_json::Value;
use serde_with::{serde_as, skip_serializing_none};
use crate::json_coercion::AsString;
use super::segment::Segment;

#[cfg(feature = "utoipa")]
use utoipa::ToSchema;

/// Object: Data
/// The data and segment objects together allow additional data about the related object
/// (e.g., user, content) to be specified. This data may be from multiple sources whether
/// from the exchange itself or third parties as specified by the id attribute. When in use,
/// vendor-specific IDs should be communicated among the parties.
#[serde_as]
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "utoipa", derive(ToSchema))]
pub struct Data {
    /// Vendor-specific ID of the data provider.
    #[serde_as(as = "Option<AsString>")]
    pub id: Option<String>,

    /// Vendor-specific displayable name of the data provider.
    #[serde_as(as = "Option<AsString>")]
    pub name: Option<String>,

    /// Array of data segment objects. Refer to Object: Segment.
    pub segment: Option<Vec<Segment>>,

    /// Optional vendor-specific extensions.
    pub ext: Option<Value>,
}