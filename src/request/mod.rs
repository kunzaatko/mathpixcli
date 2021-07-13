/// Common part of the URL for all the API endpoints
pub const MATHPIX_APIURL: &str = "https://api.mathpix.com/v3/";

// pub mod body; {{{
/**
The body for the endpoints that the API provides all look different. This module implements a
structure for every endpoint that adheres to what the enpoint expects to be the body of the
request.
*/
pub mod body; //}}}

// pub mod header; {{{
/**
Module for creating the header of requests.

> MathpixOCR uses API keys to allow access to the API. You can find your API keys on
> your account dashboard at <https://accounts.mathpix.com/ocr-api>.

> MathpixOCR expects for the API key to be included in all API requests to the server
> via HTTP Basic Auth. Expected set of HTTP headers is shown on the right.
>
> The header structure that the API requires looks like this:
> ```json
> {
>    "content-type": "application/json",
>    "app_id": "YOUR_APP_ID",
>    "app_key": "YOUR_APP_KEY"
> }
> ```
*/
pub mod header; //}}}

use reqwest;
use std::convert::{TryFrom, TryInto};
use std::future::Future;

/// The main library user interface for any of the Mathpix endpoints
pub trait MathpixEndpoint
where
    Self: Sized,
    Self::Options: Default, // there should be a corresponding default that is the same as the API server default for options
    Self::Error: std::error::Error,
    Self::Options: Sized + serde::Serialize,
    // Self::Response: Sized + serde::Deserialize<'a> + TryFrom<reqwest::Response>,
    <Self::Response as TryFrom<reqwest::Response>>::Error: Into<Self::Error>,
    <Self as MathpixEndpoint>::Error: From<std::convert::Infallible>,
    <Self as MathpixEndpoint>::Response: From<reqwest::Response>,
{
    /// What can be sent through to the endpoint to OCR.
    type Src;
    /// Possible configuration options for the endpoint OCR request.
    type Options;
    /**
    Type that describes the anticipated response fields for the particular endpoint OCR
    result.
    */
    type Response;
    /**
    Error type for the particular endpoint. It describes for each endpoint the possible OCR
    errors that can be recieved from the server as well as errors that can occur when manipulating or creating the
    request.
    */
    type Error;

    /**
    Create a new reqwest for the given endpoint using an nonobligatory `options` parameter. If
    - `options` is `Some(Self::Options)` then they are used in the constructor
    - `options` is `None` then the default options are used
    */
    fn new<S: TryInto<Self::Src>>(
        options: Option<Self::Options>,
        src: S,
    ) -> Result<Self, Self::Error>
    where
        <S as TryInto<Self::Src>>::Error: Into<Self::Error>;

    /// Return the source that is meant for OCR
    fn src(&self) -> Self::Src;

    /// Return the options that are to be sent for the OCR
    fn options(&self) -> Self::Options;

    /**
    Send an API request to the Mathpix server with the given header.

    > __NOTE:__ The header is needed for every request due to authentication of the API
    > certificate[^certificate] for the given user. It is done by the mathpix server.
    >
    > [^certificate]: There is a free license for the API certificate available with limited request
    > numbers. For further information see the [mathpix accounts website](https://accounts.mathpix.com/ocr-api).
    */
    fn send_request<H: Into<self::AuthHeader>, F>(&self, header: H) -> F
    where
        F: Future<Output = Result<Self::Response, Self::Error>>;

    /**
    Create a `reqwest::Request` from `self` with the `header`

    > __NOTE:__ It should only be necessary to use this method when you want to do something in the weeds
    without it being possible to use `Self::send_request`. One meaningful use could be if you
    wanted to send your requests through something like a VPN and add some more headers to the
    request. Then you would need to have the request itself instead of the future output.
    */
    fn to_request<H: Into<self::AuthHeader>>(
        &self,
        header: H,
    ) -> Result<reqwest::Request, Self::Error>
    where
        // TODO:  <05-07-21, kunzaatko> //
        <Self as MathpixEndpoint>::Error: From<reqwest::Error>,
    {
        let headers: reqwest::header::HeaderMap = header.into().into();
        Ok(self.to_request_builder().headers(headers).build()?)
    }

    /**
    Create a `reqwest::RequestBuilder` from `self`

    This could be usefull if you do not want to add the header right away.
    */
    fn to_request_builder(&self) -> reqwest::RequestBuilder;

    /**
    Return the URL that is associated with the request

    > __NOTE:__ It does not have to be the same for the same endpoint. (See PDF)
    */
    fn url(&self) -> reqwest::Url;
}

pub use body::Body;
pub use header::AuthHeader;

#[derive(Debug)]
pub struct Request {
    header: AuthHeader,
    body: Body,
}

use body::pdf::PDFSrc;

impl Request {
    /// Get the url that is appropriate for the endpoint
    pub fn url(&self) -> reqwest::Url {
        let mut url = String::from(MATHPIX_APIURL);
        url.push_str(match &self.body {
            Body::Batch(_) => "batch",
            Body::LaTeX(_) => "latex",
            Body::PDF(pdf) => match pdf.src {
                PDFSrc::Path(_) => "pdf-file",
                PDFSrc::Url(_) => "pdf",
            },
            Body::Strokes(_) => "strokes",
            Body::Text(_) => "text",
        });
        reqwest::Url::parse(&url).unwrap()
    }

    ///// Post the the request to the correct endpoint
    // pub fn post(
    //     &self,
    //     client: reqwest::Client,
    // ) -> impl Future<Output = Result<reqwest::Response, reqwest::Error>> {
    //     let header_map: reqwest::header::HeaderMap = self.header.clone().try_into().unwrap();
    //     if let  = self.body {
    //         client.post(self.url()).headers(header_map).form(&self.)
    //     } else {
    //         client
    //             .post(self.url())
    //             .headers(header_map)
    //             .json(&self.body)
    //             .send()
    //     }
    // }
}

// TESTS {{{
#[cfg(test)]
mod test {
    use super::body::text::{self, TextBody, TextBodyOptions};
    use super::{AuthHeader, Body, Request};
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
        let src = text::ImageSrc::Image(image);

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
        let header = AuthHeader {
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
