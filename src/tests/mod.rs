#![cfg(test)]

// mod parse_input_test;
use crate::ds::{field::FieldType, DataSet, DataSetConfig, Language};
use std::fs::File;
use std::io::*;
mod ds_test;
mod static_fields_test;
mod table_test;
#[test]
fn sum_as_string_test() {
    assert_eq!(some_sample::sum_as_string(2, 2), "4");
}

// Helpers
fn test_static_field_str(ind: usize, field: FieldType, expected: &'static str) {
    let ds = get_test_ds();
    let lng = Language::Ja;
    let created_year = 2020u16;
    assert_eq!(
        ds.data.records[ind]
            .get_field_value_as_str(&field, lng, created_year)
            .unwrap(),
        expected
    );
}

fn get_fkc_custom_fields(ds: &DataSet) -> Vec<FieldType> {
    ds.meta
        .custom_fields
        .keys()
        .map(|k| FieldType::Custom(k.to_owned()))
        .collect::<Vec<FieldType>>()
}

fn get_fkc_static_fields() -> [FieldType; 11] {
    [
        FieldType::Id,
        FieldType::UserId,
        FieldType::Gender,
        FieldType::Prefecture,
        FieldType::Region,
        FieldType::Age,
        FieldType::AgeGroup,
        FieldType::Job,
        FieldType::MaritalStatus,
        FieldType::Children,
        FieldType::YearlyIncome,
    ]
}

fn get_test_ds() -> DataSet {
    const DATA_PATH: &'static str = "./src/tests/";
    let path_meta = format!("{}{}", DATA_PATH, "meta.json");
    let path_data = format!("{}{}", DATA_PATH, "input.csv");

    let mut meta = String::new();
    let mut data = String::new();

    File::open(&path_meta)
        .unwrap()
        .read_to_string(&mut meta)
        .unwrap();
    File::open(&path_data)
        .unwrap()
        .read_to_string(&mut data)
        .unwrap();

    let config = DataSetConfig::new(String::from("ja"), 2020u16);

    DataSet::from_data(meta.as_ref(), config, data.as_ref()).unwrap()
}
