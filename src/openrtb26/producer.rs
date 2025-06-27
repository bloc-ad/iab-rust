use serde::{Deserialize, Serialize};
use serde_json::Value;
use serde_with::{skip_serializing_none, serde_as};
use crate::json_coercion::{AsString, AsEnum};
use crate::defaults::default_one_cattax;
use crate::adcom;

#[cfg(feature = "utoipa")]
use utoipa::ToSchema;

/// Defines the producer of the content.
#[serde_as]
#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[cfg_attr(feature = "utoipa", derive(ToSchema))]
pub struct Producer {
    /// Content producer or originator ID.
    #[serde_as(as = "Option<AsString>")]
    pub id: Option<String>,
    /// Content producer or originator name.
    #[serde_as(as = "Option<AsString>")]
    pub name: Option<String>,
    /// Taxonomy in use for categories. Refer to `AdCOM` List: Category Taxonomies.
    #[serde_as(as = "Option<AsEnum<adcom::enums::CategoryTaxonomy>>")]
    #[serde(default="default_one_cattax")]
    pub cattax: Option<adcom::enums::CategoryTaxonomy>,
    /// Array of IAB Tech Lab content categories describing the producer.
    #[serde_as(as = "Option<Vec<AsString>>")]
    pub cat: Option<Vec<String>>,
    /// Highest level domain of the content producer.
    #[serde_as(as = "Option<AsString>")]
    pub domain: Option<String>,
    /// Placeholder for exchange-specific extensions.
    pub ext: Option<Value>,
}
