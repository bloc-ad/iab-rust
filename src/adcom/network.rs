use serde::{Deserialize, Serialize};
use serde_json::Value;
use serde_with::{serde_as, skip_serializing_none};
use crate::json_coercion::AsString;

#[cfg(feature = "utoipa")]
use utoipa::ToSchema;

/// Object: Network
/// This object describes the network an ad will be displayed on. A network is defined as
/// the parent entity of the channel object's entity for the purposes of organizing
/// advertising channels hierarchically. Examples are companies that own and/or license
/// a collection of content channels (e.g., Viacom, Discovery, CBS, WarnerMedia, Turner
/// and others).
#[serde_as]
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "utoipa", derive(ToSchema))]
pub struct Network {
    /// A unique identifier assigned by the publisher.
    #[serde_as(as = "Option<AsString>")]
    pub id: Option<String>,

    /// Network the content is on (e.g., a TV network like "ABC").
    #[serde_as(as = "Option<AsString>")]
    pub name: Option<String>,

    /// The primary domain of the network (e.g., "abc.com" in the case of the network ABC).
    #[serde_as(as = "Option<AsString>")]
    pub domain: Option<String>,

    /// Optional vendor-specific extensions.
    pub ext: Option<Value>,
}
