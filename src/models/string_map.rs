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
pub struct StringMap {
    /// Text in English.  Will be used as a fallback
    #[serde(rename = "en", skip_serializing_if = "Option::is_none")]
    pub en: Option<String>,
    /// Text in Arabic.
    #[serde(rename = "ar", skip_serializing_if = "Option::is_none")]
    pub ar: Option<String>,
    /// Text in Bosnian.
    #[serde(rename = "bs", skip_serializing_if = "Option::is_none")]
    pub bs: Option<String>,
    /// Text in Bulgarian.
    #[serde(rename = "bg", skip_serializing_if = "Option::is_none")]
    pub bg: Option<String>,
    /// Text in Catalan.
    #[serde(rename = "ca", skip_serializing_if = "Option::is_none")]
    pub ca: Option<String>,
    /// Text in Chinese (Simplified).
    #[serde(rename = "zh-Hans", skip_serializing_if = "Option::is_none")]
    pub zh_hans: Option<String>,
    /// Text in Chinese (Traditional).
    #[serde(rename = "zh-Hant", skip_serializing_if = "Option::is_none")]
    pub zh_hant: Option<String>,
    /// Alias for zh-Hans.
    #[serde(rename = "zh", skip_serializing_if = "Option::is_none")]
    pub zh: Option<String>,
    /// Text in Croatian.
    #[serde(rename = "hr", skip_serializing_if = "Option::is_none")]
    pub hr: Option<String>,
    /// Text in Czech.
    #[serde(rename = "cs", skip_serializing_if = "Option::is_none")]
    pub cs: Option<String>,
    /// Text in Danish.
    #[serde(rename = "da", skip_serializing_if = "Option::is_none")]
    pub da: Option<String>,
    /// Text in Dutch.
    #[serde(rename = "nl", skip_serializing_if = "Option::is_none")]
    pub nl: Option<String>,
    /// Text in Estonian.
    #[serde(rename = "et", skip_serializing_if = "Option::is_none")]
    pub et: Option<String>,
    /// Text in Finnish.
    #[serde(rename = "fi", skip_serializing_if = "Option::is_none")]
    pub fi: Option<String>,
    /// Text in French.
    #[serde(rename = "fr", skip_serializing_if = "Option::is_none")]
    pub fr: Option<String>,
    /// Text in Georgian.
    #[serde(rename = "ka", skip_serializing_if = "Option::is_none")]
    pub ka: Option<String>,
    /// Text in German.
    #[serde(rename = "de", skip_serializing_if = "Option::is_none")]
    pub de: Option<String>,
    /// Text in Greek.
    #[serde(rename = "el", skip_serializing_if = "Option::is_none")]
    pub el: Option<String>,
    /// Text in Hindi.
    #[serde(rename = "hi", skip_serializing_if = "Option::is_none")]
    pub hi: Option<String>,
    /// Text in Hebrew.
    #[serde(rename = "he", skip_serializing_if = "Option::is_none")]
    pub he: Option<String>,
    /// Text in Hungarian.
    #[serde(rename = "hu", skip_serializing_if = "Option::is_none")]
    pub hu: Option<String>,
    /// Text in Indonesian.
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Text in Italian.
    #[serde(rename = "it", skip_serializing_if = "Option::is_none")]
    pub it: Option<String>,
    /// Text in Japanese.
    #[serde(rename = "ja", skip_serializing_if = "Option::is_none")]
    pub ja: Option<String>,
    /// Text in Korean.
    #[serde(rename = "ko", skip_serializing_if = "Option::is_none")]
    pub ko: Option<String>,
    /// Text in Latvian.
    #[serde(rename = "lv", skip_serializing_if = "Option::is_none")]
    pub lv: Option<String>,
    /// Text in Lithuanian.
    #[serde(rename = "lt", skip_serializing_if = "Option::is_none")]
    pub lt: Option<String>,
    /// Text in Malay.
    #[serde(rename = "ms", skip_serializing_if = "Option::is_none")]
    pub ms: Option<String>,
    /// Text in Norwegian.
    #[serde(rename = "nb", skip_serializing_if = "Option::is_none")]
    pub nb: Option<String>,
    /// Text in Polish.
    #[serde(rename = "pl", skip_serializing_if = "Option::is_none")]
    pub pl: Option<String>,
    /// Text in Persian.
    #[serde(rename = "fa", skip_serializing_if = "Option::is_none")]
    pub fa: Option<String>,
    /// Text in Portugese.
    #[serde(rename = "pt", skip_serializing_if = "Option::is_none")]
    pub pt: Option<String>,
    /// Text in Punjabi.
    #[serde(rename = "pa", skip_serializing_if = "Option::is_none")]
    pub pa: Option<String>,
    /// Text in Romanian.
    #[serde(rename = "ro", skip_serializing_if = "Option::is_none")]
    pub ro: Option<String>,
    /// Text in Russian.
    #[serde(rename = "ru", skip_serializing_if = "Option::is_none")]
    pub ru: Option<String>,
    /// Text in Serbian.
    #[serde(rename = "sr", skip_serializing_if = "Option::is_none")]
    pub sr: Option<String>,
    /// Text in Slovak.
    #[serde(rename = "sk", skip_serializing_if = "Option::is_none")]
    pub sk: Option<String>,
    /// Text in Spanish.
    #[serde(rename = "es", skip_serializing_if = "Option::is_none")]
    pub es: Option<String>,
    /// Text in Swedish.
    #[serde(rename = "sv", skip_serializing_if = "Option::is_none")]
    pub sv: Option<String>,
    /// Text in Thai.
    #[serde(rename = "th", skip_serializing_if = "Option::is_none")]
    pub th: Option<String>,
    /// Text in Turkish.
    #[serde(rename = "tr", skip_serializing_if = "Option::is_none")]
    pub tr: Option<String>,
    /// Text in Ukrainian.
    #[serde(rename = "uk", skip_serializing_if = "Option::is_none")]
    pub uk: Option<String>,
    /// Text in Vietnamese.
    #[serde(rename = "vi", skip_serializing_if = "Option::is_none")]
    pub vi: Option<String>,
}

impl StringMap {
    pub fn new() -> StringMap {
        StringMap {
            en: None,
            ar: None,
            bs: None,
            bg: None,
            ca: None,
            zh_hans: None,
            zh_hant: None,
            zh: None,
            hr: None,
            cs: None,
            da: None,
            nl: None,
            et: None,
            fi: None,
            fr: None,
            ka: None,
            de: None,
            el: None,
            hi: None,
            he: None,
            hu: None,
            id: None,
            it: None,
            ja: None,
            ko: None,
            lv: None,
            lt: None,
            ms: None,
            nb: None,
            pl: None,
            fa: None,
            pt: None,
            pa: None,
            ro: None,
            ru: None,
            sr: None,
            sk: None,
            es: None,
            sv: None,
            th: None,
            tr: None,
            uk: None,
            vi: None,
        }
    }
}


