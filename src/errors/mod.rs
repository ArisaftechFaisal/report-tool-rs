use serde::{Deserialize, Serialize};
use std::convert::From;
use thiserror::Error;
use std::option::NoneError;
use std::num::ParseIntError;

#[derive(Error, Debug, Deserialize)]
pub enum RustlyzerError {
    #[error("Wrong format for metadata json")]
    MetadataWrongFormat,
    #[error("Wrong format for csv input data: {0}")]
    CsvInputWrongFormat(String),
    #[error("Index out of range")]
    IndexOutOfRange,
    #[error("Header not defined")]
    HeaderNotDefined,
    #[error("Custom header not found in metadata")]
    CustomHeaderNotDefined,
    #[error("Custom field does not exist in records")]
    CustomFieldNotInRecords,
    #[error("Wrong arguments in function call")]
    WrongArgument,
    #[error("Option key not in custom field options")]
    KeyNotInOptions,
    #[error("IO Error: {0}")]
    IoError(String),
    #[error("XLSX Error: {0}")]
    XlsxError(String),
    #[error("Item does not exist in vec")]
    NoneError,
    #[error("Integer number couldn't be parsed from string")]
    ParseIntError
}

impl From<csv::Error> for RustlyzerError {
    fn from(err: csv::Error) -> Self {
        RustlyzerError::CsvInputWrongFormat(format!("{:?}", err))
    }
}

// impl std::convert::From<RustlyzerError> for PyErr {
//     fn from(err: RustlyzerError) -> PyErr {
//         PyOSError::new_err(err.to_string())
//     }
// }

impl From<std::io::Error> for RustlyzerError {
    fn from(err: std::io::Error) -> Self {
        RustlyzerError::IoError((err.to_string()))
    }
}

impl From<xlsxwriter::XlsxError> for RustlyzerError {
    fn from(err: xlsxwriter::XlsxError) -> Self {
        RustlyzerError::XlsxError(err.to_string())
    }
}

impl From<NoneError> for RustlyzerError {
    fn from(err: NoneError) -> Self { RustlyzerError::NoneError}
}

impl From<ParseIntError> for RustlyzerError {
    fn from(err: ParseIntError) -> Self { RustlyzerError::ParseIntError }
}
