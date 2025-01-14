pub mod punishments;
pub mod stings;
pub mod userinfo;

pub type Error = Box<dyn std::error::Error + Send + Sync>; // This is constant and should be copy pasted
