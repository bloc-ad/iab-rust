use serde::{Deserialize, Serialize};
use serde_json::Value;
use serde_with::{serde_as, skip_serializing_none};
use crate::json_coercion::AsString;

#[cfg(feature = "utoipa")]
use utoipa::ToSchema;

/// Object: Channel
/// This object describes the channel an ad will be displayed on. A channel is defined as the
/// entity that curates a content library among one or more publishers. Examples of channels
/// are specific streaming services, TV stations, radio stations, or web properties. These
/// entities offer specific content collections and are oftentimes branded.
#[serde_as]
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "utoipa", derive(ToSchema))]
pub struct Channel {
    /// Vendor-specific unique identifier of the channel.
    #[serde_as(as = "Option<AsString>")]
    pub id: Option<String>,

    /// Displayable name of the channel.
    #[serde_as(as = "Option<AsString>")]
    pub name: Option<String>,

    /// The domain or URL of the channel (e.g., mystation.com).
    #[serde_as(as = "Option<AsString>")]
    pub domain: Option<String>,

    /// Optional vendor-specific extensions.
    pub ext: Option<Value>,
}
