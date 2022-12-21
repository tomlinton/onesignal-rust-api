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
pub struct InvalidIdentifierError {
    /// Returned if using include_external_user_ids
    #[serde(rename = "invalid_external_user_ids", skip_serializing_if = "Option::is_none")]
    pub invalid_external_user_ids: Option<Vec<String>>,
    /// Returned if using include_player_ids and some were valid and others were not.
    #[serde(rename = "invalid_player_ids", skip_serializing_if = "Option::is_none")]
    pub invalid_player_ids: Option<Vec<String>>,
}

impl InvalidIdentifierError {
    pub fn new() -> InvalidIdentifierError {
        InvalidIdentifierError {
            invalid_external_user_ids: None,
            invalid_player_ids: None,
        }
    }
}


