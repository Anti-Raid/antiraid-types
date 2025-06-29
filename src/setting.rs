#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(tag = "type")]
pub enum ColumnType {
    /// A single valued column (scalar)
    Scalar {
        /// The value type
        inner: InnerColumnType,
    },
    /// An array column
    Array {
        /// The inner type of the array
        inner: InnerColumnType,
    },
}

/// Note: this is merely a hint used for styling the website
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(tag = "type")]
pub enum InnerColumnType {
    String {
        min_length: Option<usize>,
        max_length: Option<usize>,
        allowed_values: Vec<String>, // If empty, all values are allowed
        kind: String, // e.g. uuid, textarea, channel, user, role, interval, timestamp etc.
    },
    Integer {},
    Float {},
    BitFlag {
        /// The bit flag values
        values: indexmap::IndexMap<String, i64>,
    },
    Boolean {},
    Json {
        style: String, // e.g. templateref etc.
    },
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(tag = "type")]
pub enum ColumnSuggestion {
    Static { suggestions: Vec<String> },
    None {},
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Column {
    /// The ID of the column on the database
    pub id: String,

    /// The friendly name of the column
    pub name: String,

    /// The description of the column
    pub description: String,

    /// The type of the column
    pub column_type: ColumnType,

    /// Whether or not the column is a primary key
    pub primary_key: bool,

    /// Whether or not the column is nullable
    pub nullable: bool,

    /// Suggestions to display
    pub suggestions: ColumnSuggestion,

    /// A secret field that is not shown to the user
    pub secret: bool,

    /// Whether the field is readonly. Readonly fields may or may not be sent to the server
    pub readonly: Vec<OperationType>,
}

#[derive(Debug, Clone, Copy, PartialEq, Hash, Eq, serde::Serialize, serde::Deserialize)]
#[allow(dead_code)]
pub enum OperationType {
    View,
    Create,
    Update,
    Delete,
}

impl std::fmt::Display for OperationType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OperationType::View => write!(f, "View"),
            OperationType::Create => write!(f, "Create"),
            OperationType::Update => write!(f, "Update"),
            OperationType::Delete => write!(f, "Delete"),
        }
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Setting {
    /// The ID of the option
    pub id: String,

    /// The name of the option
    pub name: String,

    /// The description of the option
    pub description: String,

    /// Title template, used for the title of the embed
    pub title_template: String,

    /// The columns for this option
    pub columns: Vec<Column>,

    /// The supported operations for this option
    pub operations: Vec<OperationType>,
}
