use serde::{Deserialize, Serialize};
use serde_json::Value;
use serde_with::{serde_as, skip_serializing_none};
use crate::json_coercion::{AsString, AsEnum};
use super::enums::{CategoryTaxonomy, CreativeAttribute};

#[cfg(feature = "utoipa")]
use utoipa::ToSchema;

/// Object: Restrictions
/// This object allows lists of restrictions on ad responses to be specified including
/// specific content categories, advertisers, ads pertaining to specific apps, or
/// creative attributes.
#[serde_as]
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "utoipa", derive(ToSchema))]
pub struct Restrictions {
    /// The taxonomy in use. Refer to List: Category Taxonomies.
    #[serde_as(as = "Option<AsEnum<CategoryTaxonomy>>")]
    pub cattax: Option<CategoryTaxonomy>,

    /// Array of allowed content categories using the taxonomy indicated in cattax.
    #[serde_as(as = "Option<Vec<AsString>>")]
    pub cat: Option<Vec<String>>,

    /// Array of blocked content categories using the taxonomy indicated in cattax.
    /// If both cat and bcat are present, cat takes precedence.
    #[serde_as(as = "Option<Vec<AsString>>")]
    pub bcat: Option<Vec<String>>,

    /// Array of blocked advertisers by their domains (e.g., "ford.com").
    #[serde_as(as = "Option<Vec<AsString>>")]
    pub badv: Option<Vec<String>>,

    /// Array of blocked apps for ads; list of platform-specific application identifiers.
    #[serde_as(as = "Option<Vec<AsString>>")]
    pub bapp: Option<Vec<String>>,

    /// Array of blocked creative attributes. Refer to List: Creative Attributes.
    #[serde_as(as = "Option<Vec<AsEnum<CreativeAttribute>>>")]
    pub battr: Option<Vec<CreativeAttribute>>,

    /// Optional vendor-specific extensions.
    pub ext: Option<Value>,
}