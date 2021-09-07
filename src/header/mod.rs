use mime::{Mime, APPLICATION_JSON};
use reqwest::header::{HeaderMap, HeaderName, HeaderValue};
use serde::Serialize;

#[derive(Serialize, Debug, Clone, PartialEq)]
/// Struct storing the `"app_id"` and `"app_key"` for authentication when using the API.
pub struct AuthHeader {
    pub app_id: String,
    pub app_key: String,
}

impl AuthHeader {
    const CONTENT_TYPE: Mime = APPLICATION_JSON;
    // TODO: Add function for adding values to HeaderMap without having to construct one <03-05-21, kunzaatko> //
    pub fn new<ID: ToString, KEY: ToString>(id: ID, key: KEY) -> Self {
        AuthHeader {
            app_id: id.to_string(),
            app_key: key.to_string(),
        }
    }
}

/* impl TryFrom<AuthHeader> for HeaderMap {
    //{{{
    type Error = reqwest::header::InvalidHeaderValue;

    fn try_from(val: AuthHeader) -> Result<Self, Self::Error> {
        let mut map = HeaderMap::with_capacity(3);

        map.insert(
            HeaderName::from_static("content-type"),
            HeaderValue::from_str(AuthHeader::CONTENT_TYPE.essence_str()).unwrap(), // essence_str of APPLICATION_JSON is "application/json"
        );

        let app_id = HeaderValue::from_str(&val.app_id)?;
        map.insert(HeaderName::from_static("app_id"), app_id);

        let app_key = HeaderValue::from_str(&val.app_key)?;
        map.insert(HeaderName::from_static("app_key"), app_key);

        Ok(map)
    }
} //}}} */

// TODO: This could fail but I do not consider the scenario <05-07-21, kunzaatko> //
impl From<AuthHeader> for HeaderMap {
    fn from(val: AuthHeader) -> Self {
        let mut map = HeaderMap::with_capacity(3);

        map.insert(
            HeaderName::from_static("content-type"),
            HeaderValue::from_str(AuthHeader::CONTENT_TYPE.essence_str()).unwrap(), // essence_str of APPLICATION_JSON is "application/json"
        );

        let app_id = HeaderValue::from_str(&val.app_id).unwrap();
        map.insert(HeaderName::from_static("app_id"), app_id);

        let app_key = HeaderValue::from_str(&val.app_key).unwrap();
        map.insert(HeaderName::from_static("app_key"), app_key);

        map
    }
}

// TESTS {{{
#[cfg(test)]
mod header_tests {
    use super::AuthHeader;
    use reqwest::header::{HeaderMap, HeaderValue};
    use std::convert::TryFrom;

    #[test]
    fn try_from_header() {
        //{{{
        let header = AuthHeader {
            app_id: "nevypustsupyven_gmail_com_24325g_26c684".to_owned(),
            app_key: "29f1253cb23b8se13fgd".to_owned(),
        };

        let map = <HeaderMap<HeaderValue>>::try_from(header).unwrap();
        for (&header_key, &header_val) in
            (&["content-type", "app_id", "app_key"]).into_iter().zip(&[
                "application/json",
                "nevypustsupyven_gmail_com_24325g_26c684",
                "29f1253cb23b8se13fgd",
            ])
        {
            assert!(map[header_key] == header_val)
        }
    } //}}}

    #[test]
    fn new_auth_header() {
        //{{{
        let header = AuthHeader::new(
            "nevypustsupyven_gmail_com_24325g_26c684",
            "29f1253cb23b8se13fgd",
        );
        let expected = AuthHeader {
            app_id: "nevypustsupyven_gmail_com_24325g_26c684".to_string(),
            app_key: "29f1253cb23b8se13fgd".to_string(),
        };

        assert_eq!(header, expected)
    } //}}}
} //}}}
