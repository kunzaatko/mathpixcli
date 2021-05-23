use mime::{Mime, APPLICATION_JSON};
use reqwest::header::{HeaderMap, HeaderName, HeaderValue};
use serde::Serialize;
use std::convert::TryFrom;

#[derive(Serialize, Debug, Clone)]
/// Struct storing the `"app_id"` and `"app_key"` for authentication when using the API.
pub struct Header {
    pub app_id: String,
    pub app_key: String,
}

impl Header {
    const CONTENT_TYPE: Mime = APPLICATION_JSON;
    // TODO: Add function for adding values to HeaderMap without having to construct one <03-05-21, kunzaatko> //
}

impl TryFrom<Header> for HeaderMap {
    //{{{
    type Error = reqwest::header::InvalidHeaderValue;

    fn try_from(val: Header) -> Result<Self, Self::Error> {
        let mut map = HeaderMap::with_capacity(3);

        map.insert(
            HeaderName::from_static("content-type"),
            HeaderValue::from_str(Header::CONTENT_TYPE.essence_str()).unwrap(), // essence_str of APPLICATION_JSON is "application/json"
        );

        let app_id = HeaderValue::from_str(&val.app_id)?;
        map.insert(HeaderName::from_static("app_id"), app_id);

        let app_key = HeaderValue::from_str(&val.app_key)?;
        map.insert(HeaderName::from_static("app_key"), app_key);

        Ok(map)
    }
} //}}}

// TESTS {{{
#[cfg(test)]
mod tests {
    use super::Header;
    use reqwest::header::{HeaderMap, HeaderValue};
    use std::convert::TryFrom;

    #[test]
    fn try_from_header() {
        //{{{
        let header = Header {
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
} //}}}
