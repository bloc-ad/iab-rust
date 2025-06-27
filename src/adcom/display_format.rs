use serde::{Deserialize, Serialize};
use serde_json::Value;
use serde_with::{serde_as, skip_serializing_none};
use crate::json_coercion::AsI64;

#[cfg(feature = "utoipa")]
use utoipa::ToSchema;

/// Object: DisplayFormat
/// This object represents an allowed set of parameters for a banner display ad and often
/// appears as an array when multiple sizes are permitted.
#[serde_as]
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "utoipa", derive(ToSchema))]
pub struct DisplayFormat {
    /// Absolute width of the creative in units specified by DisplayPlacement.unit.
    #[serde_as(as = "Option<AsI64>")]
    pub w: Option<i64>,

    /// Absolute height of the creative in units specified by DisplayPlacement.unit.
    #[serde_as(as = "Option<AsI64>")]
    pub h: Option<i64>,

    /// Relative width of the creative when expressing size as a ratio, in which case
    /// both wratio and hratio must be included.
    #[serde_as(as = "Option<AsI64>")]
    pub wratio: Option<i64>,

    /// Relative height of the creative when expressing size as a ratio, in which case
    /// both wratio and hratio must be included.
    #[serde_as(as = "Option<AsI64>")]
    pub hratio: Option<i64>,

    /// Minimum requested width of the creative in units specified by DisplayPlacement.unit.
    /// This field is optional. If included, it indicates that the creative is a native ad
    /// element that should scale between the size indicated by this field and the w/h fields.
    #[serde_as(as = "Option<AsI64>")]
    pub wmin: Option<i64>,

    /// Minimum requested height of the creative in units specified by DisplayPlacement.unit.
    /// This field is optional. If included, it indicates that the creative is a native ad
    /// element that should scale between the size indicated by this field and the w/h fields.
    #[serde_as(as = "Option<AsI64>")]
    pub hmin: Option<i64>,

    /// Optional vendor-specific extensions.
    pub ext: Option<Value>,
}
