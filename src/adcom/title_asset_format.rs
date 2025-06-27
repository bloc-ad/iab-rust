use serde::{Deserialize, Serialize};
use serde_json::Value;
use serde_with::{serde_as, skip_serializing_none};
use crate::json_coercion::AsI64;

#[cfg(feature = "utoipa")]
use utoipa::ToSchema;

/// Object: TitleAssetFormat
/// This object is used to provide native asset format specifications for a title element.
/// Title element asset formats must have a text length specification.
#[serde_as]
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "utoipa", derive(ToSchema))]
pub struct TitleAssetFormat {
    /// The maximum allowed length of the title value.
    /// Recommended lengths are 25, 90, or 140.
    #[serde_as(as = "AsI64")]
    pub len: i64,

    /// Optional vendor-specific extensions.
    pub ext: Option<Value>,
}