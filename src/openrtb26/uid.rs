use serde::{Deserialize, Serialize};
use serde_json::Value;
use serde_with::{skip_serializing_none, serde_as};
use crate::json_coercion::{AsString, AsEnum};
use crate::adcom;

#[cfg(feature = "utoipa")]
use utoipa::ToSchema;

/// A single user identifier provided as part of extended identifiers.
#[serde_as]
#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[cfg_attr(feature = "utoipa", derive(ToSchema))]
pub struct UID {
    /// The identifier for the user.
    #[serde_as(as = "Option<AsString>")]
    pub id: Option<String>,
    /// Type of user agent the ID is from. Highly recommended. Refer to `AdCOM 1.0` List: Agent Types.
    #[serde_as(as = "Option<AsEnum<adcom::enums::AgentType>>")]
    pub atype: Option<adcom::enums::AgentType>,
    /// Placeholder for vendor specific extensions.
    pub ext: Option<Value>,
}