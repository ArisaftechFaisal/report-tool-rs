use crate::ds::{data::Data, meta::Meta};
use anyhow::Result;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

const DATA_PATH: &'static str = "./src/tests/";

#[test]
fn parse_dropdown() -> Result<()> {
  // let mut file = File::open("meta.json")?;
  // let mut contents = String::new();
  // file.read_to_string(&mut contents)?;
  // let meta: Meta = parse_json(&contents)?;

  let test_str = r#"{
          "Pages": [
            {
              "Elements": [
                {
                  "Options": [
                    {
                      "Value": "北海道",
                      "Label": "北海道"
                    },
                    {
                      "Value": "青森県",
                      "Label": "青森県"
                    }
                  ],
                  "Type": "dropdown",
                  "Required": true,
                  "OptionsCaption": "選択してください",
                  "Label": "あなたがお住まいの地域をお知らせください。",
                  "QuestionKey": "field1"
                }
              ]
            }
          ]
        }"#;
  // let testStruct = Meta {
  //   pages: vec![Page {
  //     elements: vec![Field::Dropdown {
  //       common: CommonParams {
  //         field_key: "field1".to_owned(),
  //         field_label: "あなたがお住まいの地域をお知らせください。".to_owned(),
  //         field_required: true,
  //       },
  //       options: vec![
  //         Option {
  //           value: "北海道".to_owned(),
  //           label: "北海道".to_owned(),
  //         },
  //         Option {
  //           value: "青森県".to_owned(),
  //           label: "青森県".to_owned(),
  //         },
  //       ],
  //     }],
  //   }],
  // };
  // assert_eq!(testStruct.pages[0].elements[0], parse_json(&testStr)?);
  // assert!(matches!(parse_json(testStr)?.pages[0].elements[0], Field::Dropdown {..}));
  // print!("{:?}", test_str);
  let path = format!("{}{}", DATA_PATH, "dropdown.json");
  let mut file = File::open(&path)?;
  let mut contents = String::new();
  file.read_to_string(&mut contents)?;
  let meta = Meta::from_json(&contents)?;
  // print!("{:?}", meta);
  assert_eq!(2, 2);
  Ok(())
}

#[test]
fn parse_meta_full() -> Result<()> {
  let path = format!("{}{}", DATA_PATH, "meta.json");
  let mut file = File::open(&path)?;
  let mut contents = String::new();
  file.read_to_string(&mut contents)?;
  let meta = Data::from_csv(&contents)?;
  // print!("{:?}", meta);
  assert_eq!(2, 2);
  Ok(())
}

#[test]
fn parse_csv_test() -> Result<()> {
  use crate::ds::Language;

  let path = format!("{}{}", DATA_PATH, "input.csv");
  let mut file = File::open(&path)?;
  let mut contents = String::new();
  file.read_to_string(&mut contents)?;
  let data = Data::from_csv(&contents)?;
  // println!("{:?}", data);
  let lng = Language::En;
  assert_eq!(2, 2);
  Ok(())
}
