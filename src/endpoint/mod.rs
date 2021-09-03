use crate::header::AuthHeader;
use async_trait::async_trait;
use reqwest;
use std::convert::TryInto;

/**
Object that are shared in multiple endpoints. Now consists of `ImageSrc` (and `Base64Image`),
`MetaData`, `DataOptions` and `CallBack` types.
*/
mod shared_objects;

macro_rules! field_builder {
    ($field_name: ident, $field_type: ty) => {
        pub fn $field_name(&mut self, val: $field_type) -> &mut Self {
            self.$field_name = Some(val);
            self
        }
    };
}

// pub mod batch; {{{
/**
Module for constructing the _batch_ endpoint request, associated response structure and error handling for the _batch_ endpoint.

> The Mathpix API supports processing multiple images in a single POST request to
> a different endpoint: /v3/batch. The request body may contain all the /v3/latex
> parameters except src and must contain a urls parameter. The request may contain
> an additonal callback parameter to receive results after all the images in the
> batch have been processed.
*/
pub mod batch; //}}}

// pub mod latex; {{{
/**
Module for constructing the _latex_ endpoint request, associated response structure and error handling for the _latex_ endpoint.

> This is an older endpoint that was developed when Mathpix could only read math
> equations, before we had full text OCR.
>
> We recommend using v3/text or v3/strokes instead, if you want to handle text and
> math together.
>
> There are some benefits to v3/latex when it comes to ignoring everything but the
> main equation in the image. If you have a solver app that only handles math and not
> text, you should consider using v3/latex, as it contains special math equation cropping.
>
> Mathpix supports image recognition for jpg and png images. Images are encoded by base64
> and sent inside JSON requests.
*/
pub mod latex; //}}}

// pub mod pdf; {{{
/**
Module for constructing the _pdf_ endpoint request, associated response structure and error handling for the _pdf_ endpoint.

> Mathpix supports PDF processing for scientific documents.
>
> Supported outputs:
>
> mmd file (Mathpix Markdown spec)
> docx file (compatible with MS Office, Google Docs, Libre Office)
> LaTeX zip file (includes images)
>
> Disclaimer: don't expect good results with fancy formatting as might be encountered in
> textbooks. The PDF processing feature is designed to work with scientific documents,
> both single and double columned.
> The PDF processing as currently exists was designed to work with PDF documents found
> on ArXiv.
*/
pub mod pdf; //}}}

// pub mod strokes; {{{
/**
Module for constructing the _strokes_ endpoint request, associated response structure and error handling for the _strokes_ endpoint.

> Mathpix supports handwriting recognition for strokes coordinates.
>
> The v3/strokes endpoint is in beta but provides a service able to transform handwritten
> strokes into its transcript of text and math.
>
> This endpoint is very convenient for users that were generating images of handwritten
> math and text and then using the service v3/text, since with v3/strokes no image generation
> is required, the request payload is smaller and therefore it results in faster response
> time.
>
> The LaTeX of the recognized handwriting is returned inside inline delimiters \( ... \) and
> block mode delimiters \[ .... \]. Lines are separated with \n newline characters. In some
> cases (e.g. multiple choice equations) we will try to flatten horizontally aligned content
> into different lines in order to keep the markup simple.
*/
pub mod strokes; //}}}

// pub mod text; {{{
/**
Module for constructing the _text_ endpoint request, associated response structure and error handling for the _text_ endpoint.

> The v3/text endpoint extracts text, and optionally derived data / HTML, from images.
>
> The text outputs follow mathpix-markdown conventions, including math mode Latex inside
> inline delimiters \( ... \) and block mode delimiters \[ .... \]. Lines are separated
> with \n newline characters. In some cases (eg multiple choice equations) we will try
> flatten horizontally aligned content into different lines in order to keep the markup
> simple.
>
> We also provide structured data outputs via the data and html output options. The data
> output returns a list of extracted formats (such as tsv for tables, or asciimath for
> equations). The html output provides annotated HTML and can be parsed via HTML / XML
> parsers.
*/
pub mod text; //}}}

/// The main library user interface for any of the Mathpix endpoints
#[async_trait]
pub trait MathpixEndpoint
where
    Self: Sized,
    Self::Options: Default, // there should be a corresponding default that is the same as the API server default for options
    Self::Error: std::error::Error,
    Self::Options: serde::Serialize,
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
    fn new<S, E>(options: Option<Self::Options>, src: S) -> Result<Self, Self::Error>
    where
        S: TryInto<Self::Src, Error = E>,
        Self::Error: From<E>,
        Self: Sized;

    /// Return the source that is meant for OCR
    fn src(&mut self) -> Option<&mut Self::Src>;

    /// Return the options that are to be sent for the OCR
    fn options(&mut self) -> &mut Self::Options;

    /**
    Send an API request to the Mathpix server with the given header.

    > __NOTE:__ The header is needed for every request due to authentication of the API
    > certificate[^certificate] for the given user. It is done by the mathpix server.
    >
    > [^certificate]: There is a free license for the API certificate available with limited request
    > numbers. For further information see the [mathpix accounts website](https://accounts.mathpix.com/ocr-api).
    */
    async fn send_request<H, Fut, F>(&self, header: H) -> Result<Self::Response, Self::Error>
    where
        H: Into<self::AuthHeader> + std::marker::Send,
        reqwest::Response: Into<Self::Response>,
        reqwest::Error: Into<Self::Error>;
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
