use serde::Serialize;
use std::convert::TryInto;

mod error;
mod options;
mod response;

pub use super::shared_objects::request::{
    AlphabetsAllowed, Base64Image, DataOptions, ImageSrc, MetaData,
};
use super::MathpixEndpoint;
use error::TextError;
pub use options::{TextFormats, TextOptions};
use response::TextResponse;

// Text {{{
#[derive(Serialize, Debug)]
/// This structs contains the possible items that the _text_ endpoint accepts
pub struct Text {
    /// > Image data, or public URL where image is located
    pub src: ImageSrc,
    /// Configuration options for the _text_ endpoint
    #[serde(flatten)]
    pub options: TextOptions,
} //}}}

impl MathpixEndpoint for Text {
    type Src = ImageSrc;
    type Error = TextError;
    type Options = TextOptions;
    type Response = TextResponse;

    fn new<S>(options: Option<Self::Options>, src: S) -> Result<Self, Self::Error> {
        // TODO:  <21-07-21, kunzaatko> //
        todo!()
    }

    fn send_request<H: Into<super::AuthHeader>, F>(&self, header: H) -> F
    where
        F: std::future::Future<Output = Result<Self::Response, Self::Error>>,
    {
        // TODO:  <21-07-21, kunzaatko> //
        todo!()
    }

    fn url(&self) -> reqwest::Url {
        // TODO:  <21-07-21, kunzaatko> //
        todo!()
    }

    fn to_request<H: Into<super::AuthHeader>>(
        &self,
        header: H,
    ) -> Result<reqwest::Request, Self::Error> {
        // TODO:  <21-07-21, kunzaatko> //
        todo!()
    }

    fn to_request_builder(&self) -> reqwest::RequestBuilder {
        // TODO:  <21-07-21, kunzaatko> //
        todo!()
    }

    fn options(&self) -> Self::Options {
        // TODO:  <21-07-21, kunzaatko> //
        todo!()
    }

    fn src(&self) -> Self::Src {
        // TODO:  <21-07-21, kunzaatko> //
        todo!()
    }
}

// TESTS {{{
#[cfg(test)]
mod test {}
//}}}
