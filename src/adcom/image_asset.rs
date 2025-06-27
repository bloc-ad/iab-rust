use serde::{Deserialize, Serialize};
use serde_json::Value;
use serde_with::{serde_as, skip_serializing_none};
use crate::json_coercion::{AsString, AsI64, AsEnum};
use super::enums::NativeImageAssetType;

#[cfg(feature = "utoipa")]
use utoipa::ToSchema;

/// Object: ImageAsset
/// This object identifies the native asset as a image asset. Image assets are use for
/// such elements as the actual creative images and icons.
#[serde_as]
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "utoipa", derive(ToSchema))]
pub struct ImageAsset {
    /// A URL that returns the image for the asset.
    #[serde_as(as = "AsString")]
    pub url: String,

    /// Width of the image asset in device independent pixels (DIPS).
    #[serde_as(as = "Option<AsI64>")]
    pub w: Option<i64>,

    /// Height of the image asset in device independent pixels (DIPS).
    #[serde_as(as = "Option<AsI64>")]
    pub h: Option<i64>,

    /// The type of image represented by this asset. Refer to List: Native Image Asset Types.
    #[serde_as(as = "Option<AsEnum<NativeImageAssetType>>")]
    pub r#type: Option<NativeImageAssetType>,

    /// Optional vendor-specific extensions.
    pub ext: Option<Value>,
}