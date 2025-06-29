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
pub struct KeyExpiryEvent {
    pub id: String,
    pub key: String,
    pub scopes: Vec<String>,
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

    /// A key external modify event. Fired when a key is modified externally
    ExternalKeyUpdate(ExternalKeyUpdateEventData),

    /// A template setting execute event. Fired when a template setting is executed
    TemplateSettingExecute(TemplateSettingExecuteEventData),

    /// Fired when a key expires within the key-value store
    KeyExpiry(KeyExpiryEvent),
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
            AntiraidEvent::ExternalKeyUpdate(data) => serde_json::to_value(data),
            AntiraidEvent::TemplateSettingExecute(data) => serde_json::to_value(data),
            AntiraidEvent::KeyExpiry(data) => serde_json::to_value(data),
        }
    }

    /// Returns the author of the event
    pub fn author(&self) -> Option<String> {
        match self {
            AntiraidEvent::OnStartup(_) => None,
            AntiraidEvent::ExternalKeyUpdate(data) => Some(data.author.to_string()),
            AntiraidEvent::TemplateSettingExecute(data) => Some(data.author.to_string()),
            AntiraidEvent::KeyExpiry(_) => None, // Key expiries inherently have no author
        }
    }
}
