use serde::Deserialize;
use serde::Serialize;

/// Represents a sting on AntiRaid
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Sting {
    /// The sting ID
    pub id: uuid::Uuid,
    /// Src of the sting, this can be useful to store the source of a sting
    pub src: Option<String>,
    /// The number of stings
    pub stings: i32,
    /// The reason for the stings (optional)
    pub reason: Option<String>,
    /// The reason the stings were voided
    pub void_reason: Option<String>,
    /// The guild ID the sting targets
    pub guild_id: serenity::all::GuildId,
    /// The creator of the sting
    pub creator: StingTarget,
    /// The target of the sting
    pub target: StingTarget,
    /// The state of the sting
    pub state: StingState,
    /// When the sting was created
    pub created_at: chrono::DateTime<chrono::Utc>,
    /// When the sting expires as a chrono duration
    pub duration: Option<std::time::Duration>,
    /// The data/metadata present within the sting, if any
    pub sting_data: Option<serde_json::Value>,
    /// The handle log encountered while handling the sting
    pub handle_log: serde_json::Value,
}

/// Data required to create a sting
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StingCreate {
    /// Src of the sting, this can be useful to store the source of the sting
    pub src: Option<String>,
    /// The number of stings
    pub stings: i32,
    /// The reason for the stings (optional)
    pub reason: Option<String>,
    /// The reason the stings were voided
    pub void_reason: Option<String>,
    /// The guild ID the sting targets
    pub guild_id: serenity::all::GuildId,
    /// The creator of the sting
    pub creator: StingTarget,
    /// The target of the sting
    pub target: StingTarget,
    /// The state of the sting
    pub state: StingState,
    /// When the sting expires as a chrono duration
    pub duration: Option<std::time::Duration>,
    /// The data/metadata present within the sting, if any
    pub sting_data: Option<serde_json::Value>,
}

impl StingCreate {
    pub fn to_sting(self, id: uuid::Uuid, created_at: chrono::DateTime<chrono::Utc>) -> Sting {
        Sting {
            id,
            src: self.src,
            stings: self.stings,
            reason: self.reason,
            void_reason: self.void_reason,
            guild_id: self.guild_id,
            creator: self.creator,
            target: self.target,
            state: self.state,
            created_at,
            duration: self.duration,
            sting_data: self.sting_data,
            handle_log: serde_json::Value::Null,
        }
    }
}

/// A sting target (either user or system)
#[derive(Debug, Clone, Copy)]
pub enum StingTarget {
    /// The sting was created by a user
    User(serenity::all::UserId),
    /// The sting was created by the system
    System,
}

impl std::fmt::Display for StingTarget {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            StingTarget::User(user_id) => write!(f, "user:{}", user_id),
            StingTarget::System => write!(f, "system"),
        }
    }
}

impl std::str::FromStr for StingTarget {
    type Err = crate::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == "system" {
            Ok(StingTarget::System)
        } else {
            let user_id = s
                .strip_prefix("user:")
                .ok_or_else(|| format!("Invalid sting creator: {}", s))?;
            Ok(StingTarget::User(
                user_id
                    .parse()
                    .map_err(|e| format!("Invalid user ID: {}", e))?,
            ))
        }
    }
}

// Serde impls for StingTarget
impl Serialize for StingTarget {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        serializer.serialize_str(&format!("{}", self))
    }
}

impl<'de> Deserialize<'de> for StingTarget {
    fn deserialize<D>(deserializer: D) -> Result<StingTarget, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        StingTarget::from_str(&s).map_err(serde::de::Error::custom)
    }
}

#[derive(Hash, Default, Debug, Clone, Copy, PartialEq)]
pub enum StingState {
    #[default]
    Active,
    Voided,
    Handled,
}

impl std::fmt::Display for StingState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            StingState::Active => write!(f, "active"),
            StingState::Voided => write!(f, "voided"),
            StingState::Handled => write!(f, "handled"),
        }
    }
}

impl std::str::FromStr for StingState {
    type Err = crate::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "active" => Ok(StingState::Active),
            "voided" => Ok(StingState::Voided),
            "handled" => Ok(StingState::Handled),
            _ => Err(format!("Invalid sting state: {}", s).into()),
        }
    }
}

// Serde impls for StingState
impl Serialize for StingState {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        serializer.serialize_str(&format!("{}", self))
    }
}

impl<'de> Deserialize<'de> for StingState {
    fn deserialize<D>(deserializer: D) -> Result<StingState, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        StingState::from_str(&s).map_err(serde::de::Error::custom)
    }
}

/// An aggregate of stings
pub struct StingAggregate {
    /// Src of the sting, this can be useful if a module wants to store the source of the sting
    pub src: Option<String>,
    /// The target of the sting
    pub target: StingTarget,
    /// The total number of stings matching this aggregate
    pub total_stings: i64,
}

impl StingAggregate {
    /// Returns the sum of all total stings in the aggregate
    pub fn total_stings(vec: Vec<StingAggregate>) -> i64 {
        vec.iter().map(|x| x.total_stings).sum()
    }

    /// Returns the total stings per-user
    ///
    /// Returns (user_id_map, system_stings)
    pub fn total_stings_per_user(
        vec: Vec<StingAggregate>,
    ) -> (std::collections::HashMap<serenity::all::UserId, i64>, i64) {
        let mut map = std::collections::HashMap::new();

        let mut system_stings = 0;

        for sting in vec {
            match sting.target {
                StingTarget::System => {
                    system_stings += sting.total_stings;
                }
                StingTarget::User(user_id) => {
                    *map.entry(user_id).or_insert(0) += sting.total_stings;
                }
            }
        }

        // Add system stings to each user
        for (_, total_stings) in map.iter_mut() {
            *total_stings += system_stings;
        }

        (map, system_stings)
    }
}
