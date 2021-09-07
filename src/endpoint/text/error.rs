pub use super::super::shared_objects::request::{Base64ImageError, ConfidenceThresholdError};
use reqwest;
use serde_json;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum TextError {
    #[error("SerializationError: {0}")]
    Serialization(#[from] serde_json::Error),
    #[error("SrcError: {0}")]
    Src(#[from] Base64ImageError),
    #[error("RequestError: {0}")]
    Request(#[from] reqwest::Error),
    #[error("OptionsError: {0}")]
    Options(#[from] TextOptionsError),
}

impl From<std::convert::Infallible> for TextError {
    fn from(_: std::convert::Infallible) -> Self {
        unreachable!()
    }
}

#[derive(Error, Debug)]
pub enum TextOptionsError {
    #[error("BadOption: {0}")]
    BadOption(#[from] BadOptionError),
    #[error("LogicalFalacy: {0}")]
    LogicalFallacy(#[from] LogicalFallacyError),
    #[error("UnreasonableOptions: {0}")]
    UnreasonableOptions(#[from] UnreasonableOptionsError),
}

#[derive(Error, Debug)]
pub enum UnreasonableOptionsError {
    #[error("NoAlphabetsAllowed: There should be atleast one alphabet allowed.")]
    NoAlphabetsAllowed,
}

#[derive(Error, Debug)]
pub enum LogicalFallacyError {
    #[error(
        "AlphabetsAllowedLogicalFallacy: {true_alphabet} and {false_alphabet} are both being set."
    )]
    AlphabetsAllowedLogicalFallacy {
        false_alphabet: String,
        true_alphabet: String,
    },
}

#[derive(Error, Debug)]
pub enum BadOptionError {
    #[error(
        "BadTextFormat: {0} is not available as a text format. Possible options are {:?}.",
        TEXT_FORMATS
    )]
    TextFormat(String),
    #[error(
        "BadDataOption: {0} is not available as data option. Possible options are {:?}.",
        DATA_OPTIONS
    )]
    DataOption(String),
    #[error("BadAlphabetAllowed: {0} is not available as an allowed alphabet. Possible options are {:?}.", ALPHABETS_ALLOWED)]
    AlphabetAllowed(String),
    #[error("BadConfidenceThreshold: {0}")]
    ConfidenceThreshold(#[from] ConfidenceThresholdError),
}

const TEXT_FORMATS: &[&str] = &["text", "data", "html", "latex_styled"];
const DATA_OPTIONS: &[&str] = &[
    "include_asciimath",
    "{no/!}include_asciimath",
    "include_latex",
    "{no/!}include_latex",
    "include_mathml",
    "{no/!}include_mathml",
    "include_svg",
    "{no/!}include_svg",
    "include_table_html",
    "{no/!}include_table_html",
    "include_tsv",
    "{no/!}include_tsv",
];
const ALPHABETS_ALLOWED: &[&str] = &[
    "en", "{no/!}en", "hi", "{no/!}hi", "zh", "{no/!}zh", "ja", "{no/!}ja", "ko", "{no/!}ko", "ru",
    "{no/!}ru", "th", "{no/!}th",
];
