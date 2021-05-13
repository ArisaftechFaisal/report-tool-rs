use super::{FieldType, ComputedFieldType, Language};
use crate::errors::RustlyzerError;
use crate::helpers::Round;
use crate::helpers::*;
use anyhow::Result;
use chrono::prelude::*;
use hashbrown::HashMap;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use super::meta::{Meta, CustomFieldVariant};
// use std::iter::Iterator;
// use std::process::Child;

pub mod computed;
pub mod input_record;
pub mod enums;

use enums::{Prefecture, Region};
use input_record::InputRecord;

#[derive(Serialize, Deserialize, Debug)]
pub struct Data {
    pub(crate) records: Vec<InputRecord>,
}

impl Data {
    pub fn from_csv(content: &str) -> Result<Self, RustlyzerError> {
        let mut rdr = csv::ReaderBuilder::new()
            .has_headers(true)
            .delimiter(b',')
            .double_quote(true)
            .from_reader(content.as_bytes());
        let mut records: Vec<InputRecord> = Vec::new();
        for result in rdr.deserialize() {
            let record: InputRecord = result?;
            // println!("{:?}", record);
            records.push(record);
        }
        Ok(Data { records })
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }


    pub fn get_static_field_title(&self, field: &FieldType, lng: Language) -> &'static str {
        match lng {
            _ => match field {
                FieldType::Id => "投稿id",
                FieldType::UserId => "ユーザid",
                FieldType::CreatedAt => "投稿日",
                FieldType::Gender => "性別",
                FieldType::Prefecture => "現住所",
                FieldType::Region => "地域",
                FieldType::Age => "年齢",
                FieldType::AgeGroup => "年代",
                FieldType::AgeGroup1060 => "年代(10代以下～60代以上)",
                FieldType::AgeGroup1070 => "年代(10代以下～70代以上)",
                FieldType::Job => "職業",
                FieldType::MaritalStatus => "未既婚",
                FieldType::Children => "子供の人数",
                FieldType::MaritalStatusAndChildren => "未既婚×子有無",
                FieldType::YearlyIncome => "世帯年収",
                FieldType::Custom(_) => {
                    panic!("Called get_static_field_title for FieldType::Custom")
                },
                FieldType::Computed(computed) => {
                    match computed {
                        ComputedFieldType::AgeGroup1060AggregateLabel |
                        ComputedFieldType::AgeGroup1070AggregateLabel =>
                            "年代ラベル",
                        ComputedFieldType::GenderAggregateLabel => "性別ラベル",
                        ComputedFieldType::MaritalStatusAggregateLabel => "未既婚ラベル",
                        ComputedFieldType::ChildrenAggregateLabel => "子供の人数ラベル",
                        ComputedFieldType::JobAggregateLabel => "職業ラベル",
                        ComputedFieldType::RegionAggregateLabel => "地域ラベル",
                        ComputedFieldType::YearlyIncomeAggregateLabel => "世帯年収ラベル",

                        ComputedFieldType::AgeGroup1060AggregateValue |
                        ComputedFieldType::AgeGroup1070AggregateValue |
                        ComputedFieldType::GenderAggregateValue |
                        ComputedFieldType::MaritalStatusAggregateValue |
                        ComputedFieldType::ChildrenAggregateValue |
                        ComputedFieldType::JobAggregateValue |
                        ComputedFieldType::RegionAggregateValue |
                        ComputedFieldType::YearlyIncomeAggregateValue => "値",

                        ComputedFieldType::AgeGroup1060AggregateDisplay |
                        ComputedFieldType::AgeGroup1070AggregateDisplay |
                        ComputedFieldType::GenderAggregateDisplay |
                        ComputedFieldType::MaritalStatusAggregateDisplay |
                        ComputedFieldType::ChildrenAggregateDisplay => "表示用値",

                        ComputedFieldType::JobAggregateGraphLabel |
                        ComputedFieldType::RegionAggregateGraphLabel |
                        ComputedFieldType::YearlyIncomeAggregateGraphLabel => "グラフ用ラベル",

                        ComputedFieldType::JobAggregatePercentage |
                        ComputedFieldType::RegionAggregatePercentage |
                        ComputedFieldType::YearlyIncomeAggregatePercentage => "割合",
                        _ => panic!("Called get_static_field_title on non-static computed field.")
                    }
                }
            },
        }
    }
}