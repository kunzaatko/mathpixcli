//! This is a crate for creating [API](https://docs.mathpix.com/?shell#introduction) requests for Mathpix.
//! `MathpixAPI` uses the `reqwest` crate for making the requests and `serde_json` for serializing the
//! structure to `JSON`.
//!
//! > The MathpixOCR API is a JSON API for extracting text from images and digital ink
//! > inputs. Unlike other OCR API's, MathpixOCR has 1rst class support for scientific
//! > notation, as used in chemistry, math, physics, computer science, economics, and other
//! > STEM subjects.
//!
//! > If you have any questions or problems, please send us an email at [support@mathpix.com](mailto:support@mathpix.com).
// TODO: Add examples for making requests to different endpoints. <23-05-21, kunzaatko> //

#![allow(clippy::upper_case_acronyms)]

/// Module for making Mathpix API requests
pub mod request;

/// Module specifying the expected response
pub mod response;
