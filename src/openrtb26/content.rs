use serde::{Deserialize, Serialize};
use serde_json::Value;
use serde_with::{skip_serializing_none, serde_as};
use crate::json_coercion::{AsString, AsI64, AsEnum};
use crate::defaults::{default_optional_cattax_nine, default_optional_cattax_one};
use crate::adcom;
use super::{Producer, Data, Network, Channel};

#[cfg(feature = "utoipa")]
use utoipa::ToSchema;

/// Describes the content in which the impression appears.
#[serde_as]
#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[cfg_attr(feature = "utoipa", derive(ToSchema))]
pub struct Content {
    /// ID uniquely identifying the content.
    #[serde_as(as = "Option<AsString>")]
    pub id: Option<String>,
    /// Episode number.
    #[serde_as(as = "Option<AsI64>")]
    pub episode: Option<i64>,
    /// Content title.
    #[serde_as(as = "Option<AsString>")]
    pub title: Option<String>,
    /// Content series.
    #[serde_as(as = "Option<AsString>")]
    pub series: Option<String>,
    /// Content season.
    #[serde_as(as = "Option<AsString>")]
    pub season: Option<String>,
    /// Artist credited with the content.
    #[serde_as(as = "Option<AsString>")]
    pub artist: Option<String>,
    /// Genre describing the content.
    #[serde_as(as = "Option<AsString>")]
    pub genre: Option<String>,
    /// Taxonomy used for 'genres'. Refer to `AdCOM 1.0` List: Category Taxonomies.
    #[serde_as(as = "Option<AsEnum<adcom::enums::CategoryTaxonomy>>")]
    #[serde(default="default_optional_cattax_nine")]
    pub gtax: Option<adcom::enums::CategoryTaxonomy>,
    /// Array of unique IDs for content genre. Taxonomy defined by gtax field.
    #[serde_as(as = "Option<Vec<AsString>>")]
    pub genres: Option<Vec<String>>,
    /// Album the content belongs to (typically audio).
    #[serde_as(as = "Option<AsString>")]
    pub album: Option<String>,
    /// International Standard Recording Code (ISO-3901).
    #[serde_as(as = "Option<AsString>")]
    pub isrc: Option<String>,
    /// Details about the content Producer.
    pub producer: Option<Producer>,
    /// URL of the content.
    #[serde_as(as = "Option<AsString>")]
    pub url: Option<String>,
    /// Taxonomy in use for 'cat'. Refer to `AdCOM` List: Category Taxonomies.
    #[serde_as(as = "Option<AsEnum<adcom::enums::CategoryTaxonomy>>")]
    #[serde(default="default_optional_cattax_one")]
    pub cattax: Option<adcom::enums::CategoryTaxonomy>,
    /// Array of IAB Tech Lab content categories describing the content.
    #[serde_as(as = "Option<Vec<AsString>>")]
    pub cat: Option<Vec<String>>,
    /// Production quality. Refer to `AdCOM 1.0` List: Production Qualities.
    #[serde_as(as = "Option<AsEnum<adcom::enums::ProductionQuality>>")]
    pub prodq: Option<adcom::enums::ProductionQuality>,
    /// Type of content. Refer to `AdCOM 1.0` List: Content Contexts.
    #[serde_as(as = "Option<AsEnum<adcom::enums::ContentContext>>")]
    pub context: Option<adcom::enums::ContentContext>,
    /// Content rating (e.g., MPAA).
    #[serde_as(as = "Option<AsString>")]
    pub contentrating: Option<String>,
    /// User rating of the content.
    #[serde_as(as = "Option<AsString>")]
    pub userrating: Option<String>,
    /// Media rating per IQG guidelines. Refer to `AdCOM 1.0` List: Media Ratings.
    #[serde_as(as = "Option<AsEnum<adcom::enums::MediaRating>>")]
    pub qagmediarating: Option<adcom::enums::MediaRating>,
    /// Comma separated list of keywords describing the content.
    #[serde_as(as = "Option<AsString>")]
    pub keywords: Option<String>,
    /// Array of keywords describing the content.
    #[serde_as(as = "Option<Vec<AsString>>")]
    pub kwarray: Option<Vec<String>>,
    /// 0 = not live, 1 = content is live.
    #[serde_as(as = "Option<AsI64>")]
    pub livestream: Option<i64>,
    /// 0 = indirect, 1 = direct.
    #[serde_as(as = "Option<AsI64>")]
    pub sourcerelationship: Option<i64>,
    /// Length of content in seconds (video/audio).
    #[serde_as(as = "Option<AsI64>")]
    pub len: Option<i64>,
    /// Content language using ISO-639-1-alpha-2.
    #[serde_as(as = "Option<AsString>")]
    pub language: Option<String>,
    /// Content language using IETF BCP 47.
    #[serde_as(as = "Option<AsString>")]
    pub langb: Option<String>,
    /// Indicator if content is embeddable (0=no, 1=yes).
    #[serde_as(as = "Option<AsI64>")]
    pub embeddable: Option<i64>,
    /// Additional content data via Data objects.
    pub data: Option<Vec<Data>>,
    /// Details about the network the content is on.
    pub network: Option<Network>,
    /// Details about the channel the content is on.
    pub channel: Option<Channel>,
    /// Placeholder for exchange-specific extensions.
    pub ext: Option<Value>,
}
