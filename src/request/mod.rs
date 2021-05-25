/// Common part of the URL for all the API endpoints
pub const MATHPIX_APIURL: &str = "https://api.mathpix.com/v3/";

// pub mod body; {{{
/// The body for the endpoints that the API provides all look different. This module implements a
/// structure for every endpoint that adheres to what the enpoint expects to be the body of the
/// request.
pub mod body; //}}}

// pub mod header; {{{
/// Module for creating the header of requests.
/// ```text
///  MathpixOCR uses API keys to allow access to the API. You can find your API keys on
///  your account dashboard at https://accounts.mathpix.com/ocr-api.
///
///  MathpixOCR expects for the API key to be included in all API requests to the server
///  via HTTP Basic Auth. Expected set of HTTP headers is shown on the right.
/// ```
///  The header structure that the API requires looks like this:
///  ```json
///  {
///     "content-type": "application/json",
///     "app_id": "YOUR_APP_ID",
///     "app_key": "YOUR_APP_KEY"
///  }
///  ```
pub mod header; //}}}

use reqwest;
use std::convert::TryInto;
use std::future::Future;

pub use body::Body;
pub use header::Header;

#[derive(Debug)]
pub struct Request {
    header: Header,
    body: Body,
}

impl Request {
    /// Get the url that is appropriate for the endpoint
    pub fn url(&self) -> reqwest::Url {
        let mut url = String::from(MATHPIX_APIURL);
        url.push_str(match &self.body {
            Body::Batch(_) => "batch",
            Body::LaTeX(_) => "latex",
            Body::PDF(_) => "pdf",
            Body::Strokes(_) => "strokes",
            Body::Text(_) => "text",
        });
        reqwest::Url::parse(&url).unwrap()
    }

    /// Post the the request to the correct endpoint
    pub fn post(
        &self,
        client: reqwest::Client,
    ) -> impl Future<Output = Result<reqwest::Response, reqwest::Error>> {
        let header_map: reqwest::header::HeaderMap = self.header.clone().try_into().unwrap();
        client
            .post(self.url())
            .headers(header_map)
            .json(&self.body)
            .send()
    }
}

// TESTS {{{
#[cfg(test)]
mod test {
    use super::body::text::{self, TextBody, TextBodyOptions};
    use super::{Body, Header, Request};
    use std::convert::TryInto;
    use std::path::PathBuf;

    #[test]
    fn request_url() {
        let image: text::Base64Image =
            PathBuf::from("./test/assets/test_encode_base64.jpg".to_string())
                .try_into()
                .unwrap();
        let mut alphabets_allowed = text::AlphabetsAllowed::default();
        alphabets_allowed
            .allow(vec!["ru".to_string(), "en".to_string()])
            .unwrap();
        let data_options = text::DataOptions {
            include_asciimath: Some(true),
            include_latex: Some(false),
            include_mathml: None,
            include_svg: None,
            include_table_html: None,
            include_tsv: None,
        };
        let src = text::Src::Image(image);

        let text_body_opts = TextBodyOptions {
            metadata: None,
            formats: Some(vec![text::TextFormats::Text, text::TextFormats::Data]),
            alphabets_allowed: Some(alphabets_allowed),
            auto_rotate_confidence_threshold: Some(1.),
            confidence_threshold: Some(1.),
            confidence_rate_threshold: Some(1.),
            data_options: Some(data_options),
            include_detected_alphabets: Some(true),
            include_geometry_data: Some(false),
            include_inchi: Some(true),
            include_line_data: Some(false),
            include_smiles: Some(true),
            include_word_data: Some(false),
            rm_fonts: Some(true),
            rm_spaces: Some(false),
            numbers_default_to_math: None,
        };
        let text_body = TextBody {
            src,
            options: text_body_opts,
        };
        let body = Body::Text(text_body);
        let header = Header {
            app_id: "mathpix_id".to_string(),
            app_key: "mathpix_key".to_string(),
        };
        let request = Request { header, body };
        let url = request.url();
        let expected = reqwest::Url::parse("https://api.mathpix.com/v3/text").unwrap();
        assert_eq!(url, expected);
    }
}
// }}}
