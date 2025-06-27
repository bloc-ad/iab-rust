use serde::{Deserialize, Serialize};
use serde_json::Value;
use serde_with::{serde_as, skip_serializing_none};
use crate::json_coercion::{AsString, AsEnum};
use super::enums::CategoryTaxonomy;

#[cfg(feature = "utoipa")]
use utoipa::ToSchema;

/// Object: Producer
/// This object defines the producer of the content in which the ad will be shown. This is
/// particularly useful when the content is syndicated and may be distributed through
/// different channels and thus when the producer and publisher are not necessarily the
/// same entity.
#[serde_as]
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "utoipa", derive(ToSchema))]
pub struct Producer {
    /// Vendor-specific unique producer identifier. Useful if content is syndicated
    /// and may be posted on a site using multiple sellers. Can be used to look up seller.json.
    #[serde_as(as = "Option<AsString>")]
    pub id: Option<String>,

    /// Displayable name of the producer.
    #[serde_as(as = "Option<AsString>")]
    pub name: Option<String>,

    /// The taxonomy in use. Refer to List: Category Taxonomies.
    #[serde_as(as = "Option<AsEnum<CategoryTaxonomy>>")]
    pub cattax: Option<CategoryTaxonomy>,

    /// Array of content categories that describe the producer using the taxonomy
    /// identified in cattax.
    #[serde_as(as = "Option<Vec<AsString>>")]
    pub cat: Option<Vec<String>>,

    /// Highest level domain of the producer (e.g., "producer.com").
    #[serde_as(as = "Option<AsString>")]
    pub domain: Option<String>,

    /// Optional vendor-specific extensions.
    pub ext: Option<Value>,
}
