use serde::{Deserialize, Serialize};
use serde_json::Value;
use serde_with::{serde_as, skip_serializing_none};
use crate::json_coercion::{AsString, AsEnum};
use super::display_placement::DisplayPlacement;
use super::enums::CompanionType;

#[cfg(feature = "utoipa")]
use utoipa::ToSchema;

/// Object: Companion
/// This object is used in audio and video placements to specify an associated or so-called
/// "companion" display ad. Video and audio placements can specify an array of companion ads.
#[serde_as]
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "utoipa", derive(ToSchema))]
pub struct Companion {
    /// Identifier of the companion ad; usually starts at 1, increasing with each
    /// additional companion.
    #[serde_as(as = "Option<AsString>")]
    pub id: Option<String>,

    /// Indicates the companion banner rendering mode relative to the associated video
    /// or audio ad, where 0 = concurrent, 1 = end-card. For a VAST compliant video
    /// player, only concurrent companions can be rendered. Refer to List: Companion Types.
    #[serde_as(as = "Option<AsEnum<CompanionType>>")]
    pub vcm: Option<CompanionType>,

    /// Display placement object representing the companion. Refer to Object: DisplayPlacement.
    pub display: Option<DisplayPlacement>,

    /// Optional vendor-specific extensions.
    pub ext: Option<Value>,
}