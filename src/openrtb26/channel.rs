use serde::{Deserialize, Serialize};
use serde_json::Value;
use serde_with::{serde_as, skip_serializing_none};
use crate::json_coercion::AsString;

#[cfg(feature = "utoipa")]
use utoipa::ToSchema;

#[serde_as]
#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "utoipa", derive(ToSchema))]
pub struct Channel {
    /// A unique identifier assigned by the publisher. This may not be a unique identifier across all supply sources.
    #[serde_as(as = "AsString")]
    pub id: String,
    /// The name of the channel.
    #[serde_as(as = "Option<AsString>")]
    pub name: Option<String>,
    /// The domain of the channel (e.g., "mysite.foo.com").
    #[serde_as(as = "Option<AsString>")]
    pub domain: Option<String>,
    /// Placeholder for exchange-specific extensions to OpenRTB.
    pub ext: Option<Value>,
}
