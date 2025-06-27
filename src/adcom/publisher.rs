use serde::{Deserialize, Serialize};
use serde_json::Value;
use serde_with::{serde_as, skip_serializing_none};
use crate::json_coercion::{AsString, AsEnum};
use super::enums::CategoryTaxonomy;

#[cfg(feature = "utoipa")]
use utoipa::ToSchema;

/// Object: Publisher
/// This object describes the publisher of the media in which the ad will be displayed.
/// The publisher is typically the seller in an OpenRTB transaction.
#[serde_as]
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "utoipa", derive(ToSchema))]
pub struct Publisher {
    /// Vendor-specific unique publisher identifier, as used in ads.txt files.
    #[serde_as(as = "Option<AsString>")]
    pub id: Option<String>,

    /// Displayable name of the publisher.
    #[serde_as(as = "Option<AsString>")]
    pub name: Option<String>,

    /// The taxonomy in use. Refer to List: Category Taxonomies.
    #[serde_as(as = "Option<AsEnum<CategoryTaxonomy>>")]
    pub cattax: Option<CategoryTaxonomy>,

    /// Array of content categories that describe the publisher using the taxonomy
    /// indicated in cattax.
    #[serde_as(as = "Option<Vec<AsString>>")]
    pub cat: Option<Vec<String>>,

    /// Highest level domain of the publisher (e.g., "publisher.com").
    #[serde_as(as = "Option<AsString>")]
    pub domain: Option<String>,

    /// Optional vendor-specific extensions.
    pub ext: Option<Value>,
}