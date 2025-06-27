use serde::{Deserialize, Serialize};
use serde_json::Value;
use serde_with::{serde_as, skip_serializing_none};
use crate::json_coercion::{AsString, AsI64, AsEnum};
use super::enums::{ApiFramework, CreativeSubtypeAudioVideo};

#[cfg(feature = "utoipa")]
use utoipa::ToSchema;

/// Object: Video
/// This object provides additional detail about an ad specifically for video ads.
#[serde_as]
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "utoipa", derive(ToSchema))]
pub struct Video {
    /// Mime type(s) of the ad creative(s) (e.g., "video/mp4").
    #[serde_as(as = "Option<Vec<AsString>>")]
    pub mime: Option<Vec<String>>,

    /// API required by the ad if applicable. Refer to List: API Frameworks.
    #[serde_as(as = "Option<Vec<AsEnum<ApiFramework>>>")]
    pub api: Option<Vec<ApiFramework>>,

    /// Subtype of video creative. Refer to List: Creative Subtypes - Audio/Video.
    #[serde_as(as = "Option<AsEnum<CreativeSubtypeAudioVideo>>")]
    pub ctype: Option<CreativeSubtypeAudioVideo>,

    /// Duration of the video creative in seconds.
    #[serde_as(as = "Option<AsI64>")]
    pub dur: Option<i64>,

    /// Video markup (e.g., VAST).
    /// Note that including both adm and curl is not recommended.
    #[serde_as(as = "Option<AsString>")]
    pub adm: Option<String>,

    /// Optional means of retrieving markup by reference; a URL that returns
    /// video markup (e.g., VAST). If this ad is matched to a Placement
    /// specification, the Placement.curlx attribute indicates if this markup
    /// retrieval option is supported.
    /// Note that including both adm and curl is not recommended.
    #[serde_as(as = "Option<AsString>")]
    pub curl: Option<String>,

    /// Optional vendor-specific extensions.
    pub ext: Option<Value>,
}