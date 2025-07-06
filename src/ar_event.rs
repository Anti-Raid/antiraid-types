use serde_json::Value;
use strum::{IntoStaticStr, VariantNames};

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
pub struct GetSettingsEvent {
    pub author: serenity::all::UserId,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct SettingExecuteEvent {
    /// The ID of the setting being executed
    pub id: String,
    /// The author of the event
    pub author: serenity::all::UserId,
    /// The operation being performed on the setting
    pub op: String,
    /// The fields of the operation. May be a map or list of fields
    pub fields: Value,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct KeyExpiryEvent {
    pub id: String,
    pub key: String,
    pub scopes: Vec<String>,
}

#[derive(Debug, serde::Serialize, serde::Deserialize, IntoStaticStr, VariantNames)]
#[must_use]
pub enum AntiraidEvent {
    /// An on startup event is fired when a set of templates are modified
    ///
    /// The inner Vec<String> is the list of templates modified/reloaded
    OnStartup(Vec<String>),

    /// A key external modify event. Fired when a key is modified externally
    ExternalKeyUpdate(ExternalKeyUpdateEventData),

    /// Fired when a key expires within the key-value store
    KeyExpiry(KeyExpiryEvent),

    /// A GetSettings event. Fired when settings are requested by the user
    ///
    /// E.g. when user opens dashboard etc
    GetSettings(GetSettingsEvent),

    /// A ExecuteSetting event. Fired when a setting is executed by the user
    ExecuteSetting(SettingExecuteEvent),
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
            AntiraidEvent::ExternalKeyUpdate(data) => serde_json::to_value(data),
            AntiraidEvent::KeyExpiry(data) => serde_json::to_value(data),
            AntiraidEvent::GetSettings(data) => serde_json::to_value(data),
            AntiraidEvent::ExecuteSetting(data) => serde_json::to_value(data),
        }
    }

    /// Returns the author of the event
    pub fn author(&self) -> Option<String> {
        match self {
            AntiraidEvent::OnStartup(_) => None,
            AntiraidEvent::ExternalKeyUpdate(data) => Some(data.author.to_string()),
            AntiraidEvent::KeyExpiry(_) => None, // Key expiries inherently have no author
            AntiraidEvent::GetSettings(data) => Some(data.author.to_string()),
            AntiraidEvent::ExecuteSetting(data) => Some(data.author.to_string()),
        }
    }
}
