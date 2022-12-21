/*
 * OneSignal
 *
 * A powerful way to send personalized messages at scale and build effective customer engagement strategies. Learn more at onesignal.com
 *
 * The version of the OpenAPI document: 1.0.2
 * Contact: devrel@onesignal.com
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct UpdateLiveActivityRequest {
    /// Type of live activity
    #[serde(rename = "name")]
    pub name: NameType,
    #[serde(rename = "event")]
    pub event: EventType,
    #[serde(rename = "event_updates")]
    pub event_updates: serde_json::Value,
    /// Timestamp; only allowed if event is \"end\"
    #[serde(rename = "dismiss_at", skip_serializing_if = "Option::is_none")]
    pub dismiss_at: Option<f32>,
}

impl UpdateLiveActivityRequest {
    pub fn new(name: NameType, event: EventType, event_updates: serde_json::Value) -> UpdateLiveActivityRequest {
        UpdateLiveActivityRequest {
            name,
            event,
            event_updates,
            dismiss_at: None,
        }
    }
}

/// Type of live activity
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum NameType {
    #[serde(rename = "headings")]
    Headings,
    #[serde(rename = "contents")]
    Contents,
    #[serde(rename = "ios_sound")]
    IosSound,
    #[serde(rename = "priority_level")]
    PriorityLevel,
}

impl Default for NameType {
    fn default() -> NameType {
        Self::Headings
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum EventType {
    #[serde(rename = "update")]
    Update,
    #[serde(rename = "end")]
    End,
}

impl Default for EventType {
    fn default() -> EventType {
        Self::Update
    }
}

