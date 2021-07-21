/*!
This is a crate for creating [API](https://docs.mathpix.com/?shell#introduction) requests for Mathpix.
`MathpixAPI` uses the `reqwest` crate for making the requests and `serde_json` for serializing the
structure to `JSON`.

> The MathpixOCR API is a JSON API for extracting text from images and digital ink
> inputs. Unlike other OCR API's, MathpixOCR has 1rst class support for scientific
> notation, as used in chemistry, math, physics, computer science, economics, and other
> STEM subjects.

> If you have any questions or problems, please send us an email at [support@mathpix.com](mailto:support@mathpix.com).
*/

// TODO: Add examples for making requests to different endpoints. <23-05-21, kunzaatko> //

#![allow(clippy::upper_case_acronyms)]

/// Common part of the URL for all the API endpoints
pub const MATHPIX_APIURL: &str = "https://api.mathpix.com/v3/";

// pub mod endpoint; {{{
/**
Endpoints that the API provides. This module implements a
structure for every endpoint that adheres to what the enpoint expects the request to look like.
*/
pub mod endpoint; //}}}

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
