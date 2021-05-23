/// Common part of the URL for all the API endpoints
pub const MATHPIX_APIURL: &str = "https://api.mathpix.com/v3/";

/// The body for the endpoints that the API provides all look different. This module implements a
/// structure for every endpoint that adheres to what the enpoint expects to be the body of the
/// request.
pub mod body;

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
pub mod header;
