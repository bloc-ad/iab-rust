use serde::{Deserialize, Serialize};
use serde_json::Value;
use serde_with::{serde_as, skip_serializing_none};
use crate::json_coercion::{AsString, AsI64, AsEnum};
use super::enums::{OperatingSystem, UserAgentSource};

#[cfg(feature = "utoipa")]
use utoipa::ToSchema;

/// Object: UserAgent
/// Structured user agent information if both the User-Agent header and the Sec-CH-UA-*
/// headers are available. If only the User-Agent header is available, this object should
/// be omitted and the device.ua field should be used.
#[serde_as]
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "utoipa", derive(ToSchema))]
pub struct UserAgent {
    /// The user agent's execution platform or browser.
    #[serde_as(as = "Option<AsString>")]
    pub browser: Option<String>,

    /// The user agent's execution platform or browser version.
    #[serde_as(as = "Option<AsString>")]
    pub browserver: Option<String>,

    /// The user agent's operating system platform (e.g., "Windows", "Android", etc.).
    #[serde_as(as = "Option<AsString>")]
    pub platform: Option<String>,

    /// The user agent's operating system. Refer to List: Operating Systems.
    #[serde_as(as = "Option<AsEnum<OperatingSystem>>")]
    pub os: Option<OperatingSystem>,

    /// The user agent's operating system version.
    #[serde_as(as = "Option<AsString>")]
    pub osver: Option<String>,

    /// 1 if the user agent explicitly signals a preference for a "mobile" version
    /// of the content, if available; else 0.
    #[serde_as(as = "Option<AsI64>")]
    pub mobile: Option<i64>,

    /// The user agent's underlying CPU architecture, such as "x86" or "arm".
    #[serde_as(as = "Option<AsString>")]
    pub arch: Option<String>,

    /// The user agent's device model. For example, "Pixel 2 XL".
    #[serde_as(as = "Option<AsString>")]
    pub model: Option<String>,

    /// The source of data used to create this object, as a UserAgentSource value.
    #[serde_as(as = "Option<AsEnum<UserAgentSource>>")]
    pub source: Option<UserAgentSource>,

    /// Optional vendor-specific extensions.
    pub ext: Option<Value>,
}
