use std::str::FromStr;

use serde::{Deserialize, Serialize};
use serenity::all::{GuildId, UserId};

/// A punishment
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Punishment {
    /// The ID of the applied punishment
    pub id: uuid::Uuid,
    /// Src of the sting, this can be useful if a module wants to store the source of the sting
    pub src: Option<String>,
    /// The guild id of the applied punishment
    pub guild_id: GuildId,
    /// The punishment string
    pub punishment: String,
    /// Creator of the punishment
    pub creator: PunishmentTarget,
    /// The target of the punishment
    pub target: PunishmentTarget,
    /// The handle log encountered while handling the punishment
    pub handle_log: serde_json::Value,
    /// When the punishment was created
    pub created_at: chrono::DateTime<chrono::Utc>,
    /// Duration of the punishment
    pub duration: Option<std::time::Duration>,
    /// The reason for the punishment
    pub reason: String,
    /// The state of the sting
    pub state: PunishmentState,
    /// Extra misc data
    pub data: Option<serde_json::Value>,
}

/// Data required to create a punishment
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PunishmentCreate {
    /// Src of the sting, this can be useful if a module wants to store the source of the sting
    pub src: Option<String>,
    /// The guild id of the applied punishment
    pub guild_id: GuildId,
    /// The punishment string
    pub punishment: String,
    /// Creator of the punishment
    pub creator: PunishmentTarget,
    /// The target of the punishment
    pub target: PunishmentTarget,
    /// The handle log encountered while handling the punishment
    pub handle_log: serde_json::Value,
    /// Duration of the punishment
    pub duration: Option<std::time::Duration>,
    /// The reason for the punishment
    pub reason: String,
    /// The state of the punishment
    pub state: PunishmentState,
    /// Extra misc data
    pub data: Option<serde_json::Value>,
}

impl PunishmentCreate {
    pub fn to_punishment(
        self,
        id: uuid::Uuid,
        created_at: chrono::DateTime<chrono::Utc>,
    ) -> Punishment {
        Punishment {
            id,
            created_at,
            src: self.src,
            guild_id: self.guild_id,
            punishment: self.punishment,
            creator: self.creator,
            target: self.target,
            handle_log: self.handle_log,
            duration: self.duration,
            reason: self.reason,
            data: self.data,
            state: self.state,
        }
    }
}

/// A punishment target (either user or system)
#[derive(Debug, Clone, Copy)]
pub enum PunishmentTarget {
    /// The punishment was created by a user
    User(UserId),
    /// The punishment was created by the system
    System,
}

impl std::fmt::Display for PunishmentTarget {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PunishmentTarget::User(user_id) => write!(f, "user:{}", user_id),
            PunishmentTarget::System => write!(f, "system"),
        }
    }
}

impl std::str::FromStr for PunishmentTarget {
    type Err = crate::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == "system" {
            Ok(PunishmentTarget::System)
        } else {
            let user_id = s
                .strip_prefix("user:")
                .ok_or_else(|| format!("Invalid sting creator: {}", s))?;
            Ok(PunishmentTarget::User(
                user_id
                    .parse()
                    .map_err(|e| format!("Invalid user ID: {}", e))?,
            ))
        }
    }
}

// Serde impls for PunishmentTarget
impl Serialize for PunishmentTarget {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        serializer.serialize_str(&format!("{}", self))
    }
}

impl<'de> Deserialize<'de> for PunishmentTarget {
    fn deserialize<D>(deserializer: D) -> Result<PunishmentTarget, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        PunishmentTarget::from_str(&s).map_err(serde::de::Error::custom)
    }
}

#[derive(Hash, Default, Debug, Clone, Copy, PartialEq)]
pub enum PunishmentState {
    #[default]
    Active,
    Voided,
    Handled,
}

impl std::fmt::Display for PunishmentState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PunishmentState::Active => write!(f, "active"),
            PunishmentState::Voided => write!(f, "voided"),
            PunishmentState::Handled => write!(f, "handled"),
        }
    }
}

impl std::str::FromStr for PunishmentState {
    type Err = crate::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "active" => Ok(PunishmentState::Active),
            "voided" => Ok(PunishmentState::Voided),
            "handled" => Ok(PunishmentState::Handled),
            _ => Err(format!("Invalid punishment state: {}", s).into()),
        }
    }
}

// Serde impls for StingState
impl Serialize for PunishmentState {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        serializer.serialize_str(&format!("{}", self))
    }
}

impl<'de> Deserialize<'de> for PunishmentState {
    fn deserialize<D>(deserializer: D) -> Result<PunishmentState, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        PunishmentState::from_str(&s).map_err(serde::de::Error::custom)
    }
}
