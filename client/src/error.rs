#[derive(Clone, Debug, PartialEq)]
pub enum Error {
    RequestError,
}

impl From<reqwest::Error> for Error {
    fn from(item: reqwest::Error) -> Self {
        item.into()
    }
}
