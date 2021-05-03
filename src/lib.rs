use reqwest::header;

/// Common part of the URL for all the api requests
const BASEURL: &str = "https://api.mathpix.com/v3/";

mod process;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
