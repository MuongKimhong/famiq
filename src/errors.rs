use thiserror::Error;

#[derive(Debug, Error, PartialEq)]
pub enum StylesFileError {
    #[error("Failed to read the file")]
    ReadStylesFileFail,

    #[error("Styles file does not exist")]
    StylesFileDoesNotExist,

    #[error("Failed to convert styles to struct")]
    ReadStylesFromFileToStructFail,
}
