use serde::{Deserialize, Serialize};
use serde_json::Value;
use serde_with::{skip_serializing_none, serde_as};
use crate::json_coercion::{AsString, AsI64, AsF64, AsEnum};
use crate::defaults::{default_one_cattax, default_zero_slotinpod};
use crate::adcom;

#[cfg(feature = "utoipa")]
use utoipa::ToSchema;

/// An offer to buy a specific impression.
#[serde_as]
#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[cfg_attr(feature = "utoipa", derive(ToSchema))]
pub struct Bid {
    /// Bidder generated bid ID.
    #[serde_as(as = "AsString")]
    pub id: String,
    /// ID of the Imp object in the related bid request.
    #[serde_as(as = "AsString")]
    pub impid: String,
    /// Bid price expressed as CPM.
    #[serde_as(as = "AsF64")]
    pub price: f64,
    /// Win notice URL. Macros supported.
    #[serde_as(as = "Option<AsString>")]
    pub nurl: Option<String>,
    /// Billing notice URL. Macros supported.
    #[serde_as(as = "Option<AsString>")]
    pub burl: Option<String>,
    /// Loss notice URL. Macros supported.
    #[serde_as(as = "Option<AsString>")]
    pub lurl: Option<String>,
    /// Optional ad markup. Supersedes win notice markup. Macros supported.
    #[serde_as(as = "Option<AsString>")]
    pub adm: Option<String>,
    /// ID of a preloaded ad.
    #[serde_as(as = "Option<AsString>")]
    pub adid: Option<String>,
    /// Advertiser domain for block list checking.
    #[serde_as(as = "Option<Vec<AsString>>")]
    pub adomain: Option<Vec<String>>,
    /// Store ID of the app.
    #[serde_as(as = "Option<AsString>")]
    pub bundle: Option<String>,
    /// URL to image representative of campaign for ad quality/safety checking.
    #[serde_as(as = "Option<AsString>")]
    pub iurl: Option<String>,
    /// Campaign ID for ad quality checking.
    #[serde_as(as = "Option<AsString>")]
    pub cid: Option<String>,
    /// Creative ID for ad quality checking.
    #[serde_as(as = "Option<AsString>")]
    pub crid: Option<String>,
    /// Tactic ID for reporting. Meaning coordinated a priori.
    #[serde_as(as = "Option<AsString>")]
    pub tactic: Option<String>,
    /// Taxonomy in use for 'cat'. Refer to `AdCOM 1.0` List: Category Taxonomies.
    #[serde_as(as = "Option<AsEnum<adcom::enums::CategoryTaxonomy>>")]
    #[serde(default="default_one_cattax")]
    pub cattax: Option<adcom::enums::CategoryTaxonomy>,
    /// IAB Tech Lab content categories of the creative.
    #[serde_as(as = "Option<Vec<AsString>>")]
    pub cat: Option<Vec<String>>,
    /// Set of attributes describing the creative. Refer to `AdCOM 1.0` List: Creative Attributes.
    #[serde_as(as = "Option<Vec<AsEnum<adcom::enums::CreativeAttribute>>>")]
    pub attr: Option<Vec<adcom::enums::CreativeAttribute>>,
    /// List of supported APIs for the markup. Refer to `AdCOM 1.0` List: API Frameworks.
    #[serde_as(as = "Option<Vec<AsEnum<adcom::enums::ApiFramework>>>")]
    pub apis: Option<Vec<adcom::enums::ApiFramework>>,
    /// NOTE: Deprecated in favor of apis.
    #[deprecated(note = "Deprecated in favor of apis")]
    #[serde_as(as = "Option<AsEnum<adcom::enums::ApiFramework>>")]
    pub api: Option<adcom::enums::ApiFramework>,
    /// Video response protocol. Refer to `AdCOM 1.0` List: Creative Subtypes - Audio/Video.
    #[serde_as(as = "Option<AsEnum<adcom::enums::CreativeSubtypeAudioVideo>>")]
    pub protocol: Option<adcom::enums::CreativeSubtypeAudioVideo>,
    /// Creative media rating per IQG guidelines. Refer to `AdCOM 1.0` List: Media Ratings.
    #[serde_as(as = "Option<AsEnum<adcom::enums::MediaRating>>")]
    pub qagmediarating: Option<adcom::enums::MediaRating>,
    /// Language of creative using ISO-639-1-alpha-2.
    #[serde_as(as = "Option<AsString>")]
    pub language: Option<String>,
    /// Language of creative using IETF BCP 47.
    #[serde_as(as = "Option<AsString>")]
    pub langb: Option<String>,
    /// Reference to deal.id if this bid pertains to a PMP deal.
    #[serde_as(as = "Option<AsString>")]
    pub dealid: Option<String>,
    /// Width of the creative in DIPS.
    #[serde_as(as = "Option<AsI64>")]
    pub w: Option<i64>,
    /// Height of the creative in DIPS.
    #[serde_as(as = "Option<AsI64>")]
    pub h: Option<i64>,
    /// Relative width of creative for ratio size (Flex Ads).
    #[serde_as(as = "Option<AsI64>")]
    pub wratio: Option<i64>,
    /// Relative height of creative for ratio size (Flex Ads).
    #[serde_as(as = "Option<AsI64>")]
    pub hratio: Option<i64>,
    /// Advisory seconds bidder willing to wait between auction and impression.
    #[serde_as(as = "Option<AsI64>")]
    pub exp: Option<i64>,
    /// Duration of video/audio creative in seconds.
    #[serde_as(as = "Option<AsI64>")]
    pub dur: Option<i64>,
    /// Type of creative markup: 1=Banner, 2=Video, 3=Audio, 4=Native.
    #[serde_as(as = "Option<AsI64>")]
    pub mtype: Option<i64>,
    /// Indicates bid eligibility for specific position within video/audio pod. Refer to `AdCOM 1.0` List: Slot Position in Pod.
    #[serde_as(as = "Option<AsEnum<adcom::enums::SlotPositionInPod>>")]
    #[serde(default="default_zero_slotinpod")]
    pub slotinpod: Option<adcom::enums::SlotPositionInPod>,
    /// Placeholder for bidder-specific extensions.
    pub ext: Option<Value>,
}