use serde::Serialize;
use std::convert::TryInto;

mod error;
mod options;
mod response;

pub use super::shared_objects::request::{
    AlphabetsAllowed, Base64Image, DataOptions, ImageSrc, MetaData,
};
use super::{super::MATHPIX_APIURL, MathpixEndpoint};
use async_trait::async_trait;
use error::TextError;
pub use options::{TextFormats, TextOptions};
use reqwest;
use response::TextResponse;

// Text {{{
#[derive(Serialize, Debug)]
/// This structs contains the possible items that the _text_ endpoint accepts
pub struct Text {
    /// > Image data, or public URL where image is located
    pub src: Option<ImageSrc>,
    /// Configuration options for the _text_ endpoint
    #[serde(flatten)]
    pub options: TextOptions,
} //}}}

#[async_trait]
impl MathpixEndpoint for Text {
    //{{{
    type Src = ImageSrc;
    type Error = TextError;
    type Options = TextOptions;
    type Response = TextResponse;

    fn new<S, E>(options: Option<Self::Options>, src: S) -> Result<Self, Self::Error>
    where
        S: TryInto<ImageSrc, Error = E>,
        Self::Error: From<E>,
        Self: Sized,
    {
        let text_options = if let Some(options) = options {
            options
        } else {
            Self::Options::default()
        };
        let text_src: Self::Src = src.try_into()?;
        Ok(Self {
            src: Some(text_src),
            options: text_options,
        })
    }

    fn url(&self) -> reqwest::Url {
        let mut url_str = MATHPIX_APIURL.to_string();
        url_str.push_str("text");
        reqwest::Url::parse(&url_str).unwrap()
    }

    async fn send_request<H, Fut, F>(&self, header: H) -> Result<Self::Response, Self::Error>
    where
        H: Into<super::AuthHeader> + std::marker::Send,
        reqwest::Response: Into<Self::Response>,
        reqwest::Error: Into<Self::Error>,
    {
        let client = reqwest::Client::new();
        let request = self.to_request(header)?;
        client.execute(request).await.map(|f| Ok(f.into()))?
    }

    fn to_request<H: Into<super::AuthHeader>>(
        &self,
        header: H,
    ) -> Result<reqwest::Request, Self::Error> {
        Ok(self
            .to_request_builder()
            .headers(header.into().into())
            .build()?)
    }

    fn to_request_builder(&self) -> reqwest::RequestBuilder {
        reqwest::Client::new().post(self.url()).json(self)
    }

    fn options(&mut self) -> &mut Self::Options {
        &mut self.options
    }

    fn src(&mut self) -> Option<&mut Self::Src> {
        if let Some(src) = &mut self.src {
            Some(src)
        } else {
            None
        }
    }
} //}}}

// TESTS {{{
#[cfg(test)]
mod test {}
//}}}
