use crate::punishments::Punishment;
use crate::stings::Sting;
use crate::userinfo::UserInfo;
use serde_json::Value;
use strum::{IntoStaticStr, VariantNames};

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct PermissionCheckData {
    pub perm: kittycat::perms::Permission,
    pub user_id: serenity::all::UserId,
    pub user_info: UserInfo,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(tag = "action")]
pub enum ModerationAction {
    Kick {
        member: serenity::all::Member, // The target to kick
    },
    TempBan {
        user: serenity::all::User, // The target to ban
        duration: u64,             // Duration, in seconds
        prune_dmd: u8,
    },
    Ban {
        user: serenity::all::User, // The target to ban
        prune_dmd: u8,
    },
    Unban {
        user: serenity::all::User, // The target to unban
    },
    Timeout {
        member: serenity::all::Member, // The target to timeout
        duration: u64,                 // Duration, in seconds
    },
    Prune {
        user: Option<serenity::all::User>,
        prune_opts: serde_json::Value,
        channels: Vec<serenity::all::ChannelId>,
    },
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct ModerationStartEventData {
    pub correlation_id: uuid::Uuid, // This will also be sent on ModerationEndEventData to correlate the events while avoiding duplication of data
    pub action: ModerationAction,
    pub author: serenity::all::Member,
    pub num_stings: i32,
    pub reason: Option<String>,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct ModerationEndEventData {
    pub correlation_id: uuid::Uuid, // Will correlate with a ModerationStart's event data
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub enum ExternalKeyUpdateEventDataAction {
    Create,
    Update,
    Delete,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct ExternalKeyUpdateEventData {
    pub key_modified: String,
    pub author: serenity::all::UserId,
    pub action: ExternalKeyUpdateEventDataAction,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(tag = "op")]
pub enum TemplateSettingExecuteEventDataAction {
    View {
        filters: indexmap::IndexMap<String, Value>,
    },
    Create {
        fields: indexmap::IndexMap<String, Value>,
    },
    Update {
        fields: indexmap::IndexMap<String, Value>,
    },
    Delete {
        fields: indexmap::IndexMap<String, Value>,
    },
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct TemplateSettingExecuteEventData {
    pub template_id: String,
    pub setting_id: String,
    pub correlation_id: uuid::Uuid, // A response from this must include a "correlation_id" field with this value so
    pub action: TemplateSettingExecuteEventDataAction,
    pub author: serenity::all::UserId,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct ScheduledExecutionEventData {
    pub id: String,
    pub data: serde_json::Value,
    pub run_at: chrono::DateTime<chrono::Utc>,
}

// TODO Later
//#[derive(Debug, serde::Serialize, serde::Deserialize)]
//pub struct TemplatePageRequestEventData {
//    pub target_template: Option<String>,
//    pub author: serenity::all::UserId,
//}

#[derive(Debug, serde::Serialize, serde::Deserialize, IntoStaticStr, VariantNames)]
#[must_use]
pub enum AntiraidEvent {
    /// An on startup event is fired when a set of templates are modified
    ///
    /// The inner Vec<String> is the list of templates modified/reloaded
    OnStartup(Vec<String>),

    /// A permission check event is fired when a permission check is done
    PermissionCheckExecute(PermissionCheckData),

    /// A moderation start event is fired prior to the execution of a moderation action
    ModerationStart(ModerationStartEventData),

    /// A moderation end event is fired after the execution of a moderation action
    ///
    /// Note that this event is not guaranteed to be fired (e.g. the action fails, jobserver timeout etc.)
    ModerationEnd(ModerationEndEventData),

    /// A key external modify event. Fired when a key is modified externally
    ExternalKeyUpdate(ExternalKeyUpdateEventData),

    /// A template setting execute event. Fired when a template setting is executed
    TemplateSettingExecute(TemplateSettingExecuteEventData),

    /// Fired when a scheduled execution is executed
    ScheduledExecution(ScheduledExecutionEventData),

    // TODO Later
    // A template page request event. Fired when a template page is requested
    //
    // E.g. when user opens dashboard etc
    //TemplatePageRequest(TemplatePageRequestEventData),
}

impl std::fmt::Display for AntiraidEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s: &'static str = self.into();
        write!(f, "{}", s)
    }
}

impl AntiraidEvent {
    /// Returns the variant names
    pub fn variant_names() -> &'static [&'static str] {
        Self::VARIANTS
    }

    /// Convert the event's inner data to a JSON value
    pub fn to_value(&self) -> Result<serde_json::Value, serde_json::Error> {
        match self {
            AntiraidEvent::OnStartup(templates) => serde_json::to_value(templates),
            AntiraidEvent::PermissionCheckExecute(data) => serde_json::to_value(data),
            AntiraidEvent::ModerationStart(data) => serde_json::to_value(data),
            AntiraidEvent::ModerationEnd(data) => serde_json::to_value(data),
            AntiraidEvent::ExternalKeyUpdate(data) => serde_json::to_value(data),
            AntiraidEvent::TemplateSettingExecute(data) => serde_json::to_value(data),
            AntiraidEvent::ScheduledExecution(data) => serde_json::to_value(data),
        }
    }

    /// Returns the author of the event
    pub fn author(&self) -> Option<String> {
        match self {
            AntiraidEvent::OnStartup(_) => None,
            AntiraidEvent::PermissionCheckExecute(pce) => Some(pce.user_id.to_string()),
            AntiraidEvent::ModerationStart(data) => Some(data.author.user.id.to_string()),
            AntiraidEvent::ModerationEnd(_) => None,
            AntiraidEvent::ExternalKeyUpdate(data) => Some(data.author.to_string()),
            AntiraidEvent::TemplateSettingExecute(data) => Some(data.author.to_string()),
            AntiraidEvent::ScheduledExecution(_) => None, // Scheduled executions inherently have no author
        }
    }
}
