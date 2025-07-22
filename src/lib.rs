pub mod ar_event;
pub mod setting;

pub type Error = Box<dyn std::error::Error + Send + Sync>; // This is constant and should be copy pasted
