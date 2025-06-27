use serde::{Deserialize, Serialize};
use serde_json::Value;
use serde_with::{serde_as, skip_serializing_none};
use crate::json_coercion::{AsString, AsEnum};
use super::enums::AgentType;

#[cfg(feature = "utoipa")]
use utoipa::ToSchema;

/// Object: Extended Identifier UIDs
/// This object contains the unique ID from a source or technology provider specified
/// by the ExtendedIdentifiers.source field.
#[serde_as]
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "utoipa", derive(ToSchema))]
pub struct ExtendedIdentifiersUids {
    /// Cookie or platform-native identifier.
    #[serde_as(as = "AsString")]
    pub id: String,

    /// The agent type. Refer to List: Agent Types.
    #[serde_as(as = "Option<AsEnum<AgentType>>")]
    pub atype: Option<AgentType>,

    /// Optional vendor-specific extensions.
    pub ext: Option<Value>,
}