#[cfg(feature = "utoipa")]
use utoipa::ToSchema;

/// List: Pod Sequence
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "utoipa", derive(ToSchema))]
pub enum PodSequence {
    LastPosition,
    AnyPosition,
    FirstPosition,
    Unknown(i64),
}

impl Default for PodSequence {
    fn default() -> Self {
        PodSequence::AnyPosition
    }
}

impl From<i64> for PodSequence {
    fn from(value: i64) -> Self {
        match value {
            -1 => PodSequence::LastPosition,
            0 => PodSequence::AnyPosition,
            1 => PodSequence::FirstPosition,
            _ => PodSequence::Unknown(value),
        }
    }
}

impl From<PodSequence> for i64 {
    fn from(value: PodSequence) -> Self {
        match value {
            PodSequence::LastPosition => -1,
            PodSequence::AnyPosition => 0,
            PodSequence::FirstPosition => 1,
            PodSequence::Unknown(v) => v,
        }
    }
}

// Use the macro to implement Serialize and Deserialize
crate::impl_serde_for_enum!(PodSequence);