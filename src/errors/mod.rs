use serde::{Deserialize, Serialize};
use std::convert::From;
use thiserror::Error;
use std::num::ParseIntError;

#[derive(Error, Debug, Deserialize, Serialize)]
pub enum RustlyzerError {
    #[error("Wrong format for metadata json: {0}")]
    MetadataWrongFormat(String),
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
    ParseIntError,
    #[error("Invalid {field}: {val} at row {:?}", if let Some(r) = .row {r.to_string()} else
    {"unknown".to_string()})]
    InvalidDataError {
        field: String,
        val: String,
        row: Option<usize>
    },
    #[error("Invalid config error for item {config_item}: Got value: {val}, expected \
    one of {:?}", .expected_values)]
    InvalidConfigValError {
        config_item: String,
        val: String,
        expected_values: Vec<String>
    },
    #[error("Invalid config item: {0}")]
    InvalidConfigItemError(String),
    #[error("Internal error: {0}")]
    InternalError(String)
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

impl From<ParseIntError> for RustlyzerError {
    fn from(err: ParseIntError) -> Self { RustlyzerError::ParseIntError }
}
