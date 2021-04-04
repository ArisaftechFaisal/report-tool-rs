use super::places::{Prefecture, Region};
use super::{FieldType, ComputedFieldType, Language};
use crate::errors::RustlyzerError;
use crate::helpers::Round;
use crate::helpers::*;
use anyhow::Result;
use chrono::prelude::*;
use hashbrown::HashMap;
use serde::{Deserialize, Serialize};
use serde_json::Value;
// use std::iter::Iterator;
// use std::process::Child;

pub mod computed;

#[derive(Serialize, Deserialize, Debug)]
pub struct Data {
    pub records: Vec<InputRecord>,
}

impl Data {
    pub fn from_csv(content: &str) -> Result<Self, RustlyzerError> {
        let mut rdr = csv::ReaderBuilder::new()
            .has_headers(true)
            .delimiter(b',')
            .double_quote(true)
            .escape(Some(b'\\'))
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

#[derive(Serialize, Deserialize, Debug)]
pub struct InputRecord {
    id: String,
    user_id: String,
    campaign_id: String,
    price: u32,
    bonus_point: u32,
    status: PurchaseStatus,
    created_at: String,
    #[serde(rename = "updated at")]
    updated_at: String,
    email: String,
    nickname: String,
    gender: Gender,
    birth_year: u16,
    // job: String,
    job: Job,
    prefecture: Prefecture,
    marital_status: MaritalStatus,
    children: u16,
    household_income_min: u64,
    household_income_max: u64,
    #[serde(flatten, deserialize_with = "custom_fields_de")]
    custom_fields: HashMap<usize, Option<Value>>,
}

impl InputRecord {
    // pub fn get_field_variant_as_string(&self, field: &FieldType, meta: &Meta, lng:Language)
    //     -> Result<String, RustlyzerError>
    // {
    //     match field {
    //         FieldType::AgeGroup1060 => {
    //            self.records
    //         },
    //         _ => Err(RustlyzerError::WrongArgument)
    //     }
    // }

    /// Returns custom field value for field.
    /// - `field` has to be of field type `FieldType::Custom(_)`
    /// - For `FieldType::Custom(i)`, `i` has to exist in records
    pub fn get_custom_field(
        &self,
        field: &FieldType,
    ) -> Result<&Option<Value>, RustlyzerError> {
         match field {
            FieldType::Custom(i) => {
                if let Some(res) = self.custom_fields.get(i) {
                    Ok(res)
                } else {
                    Err(RustlyzerError::CustomFieldNotInRecords)
                }
            }
            _ => Err(RustlyzerError::WrongArgument),
        }
    }

    pub fn get_field_value_as_str(
        &self,
        field: &FieldType,
        lng: Language,
        created_year: u16,
    ) -> Result<String, RustlyzerError> {
        match field {
            FieldType::Custom(_) => self.get_custom_field_str(field, lng),
            _ => Ok(self.get_static_field_str(field, lng, created_year)),
        }
    }

    pub fn validate_birth_year(&self, created_year: u16, row: usize) -> Result<(), RustlyzerError>{
        let age = (created_year as i16 - self.birth_year as i16);
        if age < 0 {
            return Err(RustlyzerError::InvalidDataError {
                field: "birth_year".to_string(),
                val: self.birth_year.to_string(),
                row: Some(row + 2)
            });
        }
        Ok(())
    }

    fn get_custom_field_str(
        &self,
        field: &FieldType,
        lng: Language,
    ) -> Result<String, RustlyzerError> {
        let res = self.get_custom_field(field)?;
        if let Some(res) = res {
            Ok(self.get_value_inner_str(res))
        } else {
            Ok(format!("NULL"))
        }
    }

    fn get_static_field_str(&self, field: &FieldType, lng: Language, created_year: u16) -> String {
        match lng {
            _ => match field {
                FieldType::Id => self.id.clone(),
                FieldType::UserId => self.user_id.clone(),
                FieldType::CreatedAt => self.created_at.clone(),
                // FieldType::Gender => self.get_gender_str(lng),
                FieldType::Gender => self.gender.as_string(lng),
                FieldType::Prefecture => self.prefecture.as_string(lng),
                FieldType::Region => Region::from_prefecture(&self.prefecture).as_string(lng),
                FieldType::Age => self.get_age(created_year).to_string(),
                FieldType::AgeGroup => self.get_age_group_str(lng, created_year),
                FieldType::AgeGroup1060 => self.get_age_group_1060(created_year).as_string(lng),
                FieldType::AgeGroup1070 => self.get_age_group_1070(created_year).as_string(lng),
                FieldType::Job => self.job.as_str(lng).to_string(),
                FieldType::MaritalStatus => self.get_marital_status_str(lng),
                FieldType::Children => self.get_children_str(lng),
                FieldType::MaritalStatusAndChildren => {
                    self.get_marital_status_and_children_str(lng)
                }
                FieldType::YearlyIncome => self.get_income_range_str(lng),
                FieldType::Custom(_) | FieldType::Computed(_) => panic!("Called \
                get_static_field_str for \
                FieldType::Custom"),
            },
        }
    }

    fn get_value_inner_str(&self, val: &Value) -> String {
        match val {
            Value::String(val) => val.to_owned(),
            Value::Number(val) => format!("{}", val),
            Value::Array(arr) => {
                let mut narr = Vec::<String>::new();
                for nval in arr.iter() {
                    narr.push(self.get_value_inner_str(nval));
                }
                format!("{:?}", narr)
            }
            Value::Bool(val) => format!("{:?}", val),
            Value::Null => format!("null"), // TODO: need to change this
            Value::Object(val) => format!("{:?}", val),
        }
    }

    // fn get_region(&self) -> Region {}

    fn get_gender_str(&self, lng: Language) -> String {
        match lng {
            Language::Ja => match self.gender {
                Gender::Male => "男性".to_owned(),
                Gender::Female => "女性".to_owned(),
            },
            Language::En => match self.gender {
                Gender::Male => "Male".to_owned(),
                Gender::Female => "Female".to_owned(),
            },
        }
    }

    fn get_age(&self, created_year: u16) -> u8 {
        // (Utc::now().year() as u16 - self.birth_year) as u8
        // println!("Birth year: {:?}", self.birth_year);
        let age = (created_year - self.birth_year) as u8;
        age
        // (created_year - self.birth_year) as u8
    }

    fn get_age_group_1060(&self, created_year: u16) -> AgeRange1060 {
        let age = (self.get_age(created_year) / 10) * 10;
        match age {
            0..20 => AgeRange1060::Under10s,
            20..30 => AgeRange1060::Group20s,
            30..40 => AgeRange1060::Group30s,
            40..50 => AgeRange1060::Group40s,
            50..60 => AgeRange1060::Group50s,
            60..=u8::MAX => AgeRange1060::Above60s
        }
    }

    fn get_age_group_1070(&self, created_year: u16) -> AgeRange1070 {
        let age = (self.get_age(created_year) / 10) * 10;
        match age {
            0..20 => AgeRange1070::Under10s,
            20..30 => AgeRange1070::Group20s,
            30..40 => AgeRange1070::Group30s,
            40..50 => AgeRange1070::Group40s,
            50..60 => AgeRange1070::Group50s,
            60..70 => AgeRange1070::Group60s,
            70..=u8::MAX => AgeRange1070::Above70s
        }
    }

    fn get_age_group(&self, created_year: u16) -> u8 {
        (self.get_age(created_year) / 10) * 10
    }

    fn get_age_group_str(&self, lng: Language, created_year: u16) -> String {
        match lng {
            Language::Ja => format!("{}代", self.get_age_group(created_year)),
            Language::En | _ => format!("{}s", self.get_age_group(created_year)),
        }
    }

    fn get_income_range_str(&self, lng: Language) -> String {
        // self.get_income_custom_range_str(lng, 1, 1_000_000)
        match (self.household_income_min, self.household_income_max) {
            (0..1_000_000, 0..1_000_000) => YearlyIncomeRange::Below1Mil.as_string(lng),
            (1_000_000..2_000_000, 1_000_000..2_000_000) => YearlyIncomeRange::Group1To2Mil.as_string(lng),
            (2_000_000..3_000_000, 2_000_000..3_000_000) => YearlyIncomeRange::Group2To3Mil.as_string(lng),
            (3_000_000..4_000_000, 3_000_000..4_000_000) => YearlyIncomeRange::Group3To4Mil.as_string(lng),
            (4_000_000..5_000_000, 4_000_000..5_000_000) => YearlyIncomeRange::Group4To5Mil
                .as_string(lng),
            (5_000_000..6_000_000, 5_000_000..6_000_000) => YearlyIncomeRange::Group5To6Mil
                .as_string(lng),
            (6_000_000..7_000_000, 6_000_000..7_000_000) => YearlyIncomeRange::Group6To7Mil
                .as_string(lng),
            (7_000_000..8_000_000, 7_000_000..8_000_000) => YearlyIncomeRange::Group7To8Mil
                .as_string(lng),
            (8_000_000..9_000_000, 8_000_000..9_000_000) => YearlyIncomeRange::Group8To9Mil
                .as_string(lng),
            (9_000_000..10_000_000, 9_000_000..10_000_000) => YearlyIncomeRange::Group9To10Mil
                .as_string(lng),
            (10_000_000..12_000_000, 10_000_000..12_000_000) => YearlyIncomeRange::Group10To12Mil
                .as_string(lng),
            (12_000_000..15_000_000, 12_000_000..15_000_000) => YearlyIncomeRange::Group12To15Mil
                .as_string(lng),
            (15_000_000..20_000_000, 15_000_000..20_000_000) => YearlyIncomeRange::Group15To20Mil
                .as_string(lng),
            (_, _ ) => YearlyIncomeRange::Above20Mil.as_string(lng),
        }
    }

    fn get_income_custom_range_str(&self, lng: Language, sfig: u8, min: u64) -> String {
        match lng {
            Language::Ja => {
                if self.household_income_max < min {
                    format!("{}万円未満", min / 10_000)
                } else {
                    format!(
                        "{}～{}万円未満",
                        self.household_income_min.floor(sfig) / 10_000,
                        self.household_income_max.ceil(sfig) / 10_000
                    )
                }
            }
            Language::En | _ => {
                if self.household_income_max < min {
                    format!("less than {}", min / 1000)
                } else {
                    format!(
                        "{}~{} thousand yen or less",
                        self.household_income_min.floor(sfig) / 1000,
                        self.household_income_max.ceil(sfig) / 1000
                    )
                }
            }
        }
    }

    fn get_children_str(&self, lng: Language) -> String {
        // match lng {
            // Language::Ja => format!("{}人", self.children),
            // _ => format!("{}", self.children),
        // }
        match self.children {
            0 => ChildrenRange::Group0.as_string(lng),
            1 => ChildrenRange::Group1.as_string(lng),
            2 => ChildrenRange::Group2.as_string(lng),
            3 => ChildrenRange::Group3.as_string(lng),
            _ => ChildrenRange::Above4.as_string(lng)
        }
    }

    fn get_marital_status_str(&self, lng: Language) -> String {
        match lng {
            _ => {
                if self.marital_status == MaritalStatus::Single {
                    String::from("未婚")
                } else {
                    String::from("既婚")
                }
            }
        }
    }

    fn get_marital_status_and_children_str(&self, lng: Language) -> String {
        let married_str = self.get_marital_status_str(lng);
        match lng {
            _ => {
                if self.children > 0 {
                    format!("{}(子あり)", married_str)
                } else {
                    format!("{}(子なし)", married_str)
                }
            }
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
enum PurchaseStatus {
    #[serde(rename = "purchased")]
    Purchased,
    // #[serde(rename = "not purchased")]
    // NotPurchased,
    #[serde(rename = "rejected")]
    Rejected,
    #[serde(rename = "evaluated")]
    Evaluated
}

impl EnumAttrs<PurchaseStatus> for PurchaseStatus {
    fn as_str(&self, lng: Language) -> &'static str {
        match lng {
            _ => {
                match self {
                    PurchaseStatus::Purchased => "Purchased",
                    PurchaseStatus::Rejected => "Rejected",
                    PurchaseStatus::Evaluated => "Evaluated"
                }
            }
        }
    }

    fn get_all() -> Vec<PurchaseStatus> {
        vec![
            PurchaseStatus::Purchased,
            PurchaseStatus::Rejected,
            PurchaseStatus::Evaluated
        ]
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
enum Gender {
    #[serde(alias = "male", alias = "男")]
    Male,
    #[serde(rename = "female", alias = "女")]
    Female,
}

impl EnumAttrs<Gender> for Gender {
    fn as_str(&self, lng: Language) -> &'static str {
        match lng {
            Language::Ja => {
                match self {
                   Gender::Male =>  "男性",
                    Gender::Female => "女性"
                }
            }
            Language::En | _ => {
                match self {
                    Gender::Male => "Male",
                    Gender::Female => "Female"
                }
            }
        }
    }

    fn get_all() -> Vec<Gender> {
        vec![
            Gender::Female,
            Gender::Male
        ]
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
enum MaritalStatus {
    #[serde(alias = "single", alias = "未婚")]
    Single,
    #[serde(alias = "married", alias = "既婚")]
    Married,
}

impl EnumAttrs<MaritalStatus> for MaritalStatus {
    fn as_str(&self, lng: Language) -> &'static str {
        match lng {
            Language::Ja => {
                match self {
                    MaritalStatus::Married => "既婚",
                    MaritalStatus::Single => "未婚"
                }
            }
            Language::En | _ => {
                match self {
                    MaritalStatus::Married => "Married",
                    MaritalStatus::Single => "Single"
                }
            }
        }
    }

    fn get_all() -> Vec<MaritalStatus> {
        vec![
            MaritalStatus::Married,
            MaritalStatus::Single
        ]
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
enum Job {
    #[serde(rename="専業主婦（主夫）")]
    FullTimeHousewife,
    #[serde(rename="パート・アルバイト")]
    PartTime,
    #[serde(rename="会社員（事務系）")]
    EmployeeOffice,
    #[serde(rename="会社員（その他）")]
    EmployeeOthers,
    #[serde(rename="会社員（技術系）")]
    EmployeeTech,
    #[serde(rename="無職")]
    Unemployed,
    #[serde(rename="学生")]
    Student,
    #[serde(rename="自営業")]
    SelfEmployed,
    #[serde(rename="自由業")]
    Freelancer,
    #[serde(rename="公務員")]
    CivilServant,
    #[serde(rename="経営者・役員")]
    Entrepreneur,
    #[serde(rename="その他")]
    Others
}

impl EnumAttrs<Job> for Job {
    // fn get_all_string(lng: Language) -> Vec<String> {
    //     Job::get_all().into_iter().map(|x| x.as_str(lng).to_string()).collect()
    // }

    fn get_all() -> Vec<Job> {
       vec![
           Job::FullTimeHousewife,
           Job::PartTime,
           Job::EmployeeOffice,
           Job::EmployeeOthers,
           Job::EmployeeTech,
           Job::Unemployed,
           Job::Student,
           Job::SelfEmployed,
           Job::Freelancer,
           Job::CivilServant,
           Job::Entrepreneur,
           Job::Others,
       ]
    }

    fn as_str(&self, lng: Language) -> &'static str {
        match lng {
            _ => match self {
                Job::FullTimeHousewife => "専業主婦（主夫）",
                Job::PartTime => "パート・アルバイト",
                Job::EmployeeOffice => "会社員（事務系）",
                Job::EmployeeOthers => "会社員（その他）",
                Job::EmployeeTech => "会社員（技術系）",
                Job::Unemployed => "無職",
                Job::Student => "学生",
                Job::SelfEmployed => "自営業",
                Job::Freelancer => "自由業",
                Job::CivilServant => "公務員",
                Job::Entrepreneur => "経営者・役員",
                Job::Others => "その他",
            }
        }
    }
}

fn custom_fields_de<'de, D>(de: D) -> Result<HashMap<usize, Option<Value>>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let input_map = HashMap::<String, Option<Value>>::deserialize(de)?;
    let mut map = HashMap::<usize, Option<Value>>::new();
    // let opt = opt.as_ref().map(Value);
    for (k, v) in &input_map {
        if let Some(id) = usize::extract_from_str(k) {
            let mut nv = v.clone();
            if let Some(val) = &nv {
                if let Value::String(s) = val {
                    if s == "" {
                        nv = None;
                    } else {
                        if let Ok(arr) = serde_json::from_str::<Vec<Value>>(s) {
                            // TODO: Might need to fix an array with one empty string, ie. [""]
                            nv = Some(Value::Array(arr));
                        }
                    }
                }
            }
            map.insert(id, nv);
        }
    }
    Ok(map)
}

// Extra enums
pub enum AgeRange1060 {
    Under10s,
    Group20s,
    Group30s,
    Group40s,
    Group50s,
    Above60s
}

impl EnumAttrs<AgeRange1060> for AgeRange1060 {
    fn as_str(&self, lng: Language) -> &'static str {
        match lng {
            _ => {
                match self {
                    AgeRange1060::Under10s => "10代以下",
                    AgeRange1060::Group20s => "20代",
                    AgeRange1060::Group30s => "30代",
                    AgeRange1060::Group40s => "40代",
                    AgeRange1060::Group50s => "50代",
                    AgeRange1060::Above60s => "60代以上",
                }
            }
        }
    }

    fn get_all() -> Vec<AgeRange1060> {
        vec![
            AgeRange1060::Under10s,
            AgeRange1060::Group20s,
            AgeRange1060::Group30s,
            AgeRange1060::Group40s,
            AgeRange1060::Group50s,
            AgeRange1060::Above60s,
        ]
    }
}

pub enum AgeRange1070 {
    Under10s,
    Group20s,
    Group30s,
    Group40s,
    Group50s,
    Group60s,
    Above70s
}

impl EnumAttrs<AgeRange1070> for AgeRange1070 {
    fn as_str(&self, lng: Language) -> &'static str {
        match lng {
            _ => {
                match self {
                    AgeRange1070::Under10s => "10代以下",
                    AgeRange1070::Group20s => "20代",
                    AgeRange1070::Group30s => "30代",
                    AgeRange1070::Group40s => "40代",
                    AgeRange1070::Group50s => "50代",
                    AgeRange1070::Group60s => "60代",
                    AgeRange1070::Above70s => "70代以上",
                }
            }
        }
    }

    fn get_all() -> Vec<AgeRange1070> {
        vec![
            AgeRange1070::Under10s,
            AgeRange1070::Group20s,
            AgeRange1070::Group30s,
            AgeRange1070::Group40s,
            AgeRange1070::Group50s,
            AgeRange1070::Group60s,
            AgeRange1070::Above70s,
        ]
    }
}

pub enum ChildrenRange {
    Group0,
    Group1,
    Group2,
    Group3,
    Above4
}

impl EnumAttrs<ChildrenRange> for ChildrenRange {
    fn as_str(&self, lng: Language) -> &'static str {
        match lng {
            _ => match self {
                ChildrenRange::Group0 => "0人",
                ChildrenRange::Group1 => "1人",
                ChildrenRange::Group2 => "2人",
                ChildrenRange::Group3 => "3人",
                ChildrenRange::Above4 => "4人以上",
            }
        }
    }

    fn get_all() -> Vec<ChildrenRange> {
        vec![
            ChildrenRange::Group0,
            ChildrenRange::Group1,
            ChildrenRange::Group2,
            ChildrenRange::Group3,
            ChildrenRange::Above4,
        ]
    }
}

enum YearlyIncomeRange {
    Below1Mil,
    Group1To2Mil,
    Group2To3Mil,
    Group3To4Mil,
    Group4To5Mil,
    Group5To6Mil,
    Group6To7Mil,
    Group7To8Mil,
    Group8To9Mil,
    Group9To10Mil,
    Group10To12Mil,
    Group12To15Mil,
    Group15To20Mil,
    Above20Mil
}

impl EnumAttrs<YearlyIncomeRange> for YearlyIncomeRange {
    fn as_str(&self, lng: Language) -> &'static str {
        match lng {
            _ => match self {
                YearlyIncomeRange::Below1Mil => "100万円未満",
                YearlyIncomeRange::Group1To2Mil => "100～200万円未満",
                YearlyIncomeRange::Group2To3Mil => "200～300万円未満",
                YearlyIncomeRange::Group3To4Mil => "300～400万円未満",
                YearlyIncomeRange::Group4To5Mil => "400～500万円未満",
                YearlyIncomeRange::Group5To6Mil => "500～600万円未満",
                YearlyIncomeRange::Group6To7Mil => "600～700万円未満",
                YearlyIncomeRange::Group7To8Mil => "700～800万円未満",
                YearlyIncomeRange::Group8To9Mil => "800～900万円未満",
                YearlyIncomeRange::Group9To10Mil => "900～1000万円未満",
                YearlyIncomeRange::Group10To12Mil => "1000～1200万円未満",
                YearlyIncomeRange::Group12To15Mil => "1200～1500万円未満",
                YearlyIncomeRange::Group15To20Mil => "1500～2000万円未満",
                YearlyIncomeRange::Above20Mil => "2000万円以上",
            }
        }
    }

    fn get_all() -> Vec<YearlyIncomeRange> {
        vec![
            YearlyIncomeRange::Below1Mil,
            YearlyIncomeRange::Group1To2Mil,
            YearlyIncomeRange::Group2To3Mil,
            YearlyIncomeRange::Group3To4Mil,
            YearlyIncomeRange::Group4To5Mil,
            YearlyIncomeRange::Group5To6Mil,
            YearlyIncomeRange::Group6To7Mil,
            YearlyIncomeRange::Group7To8Mil,
            YearlyIncomeRange::Group8To9Mil,
            YearlyIncomeRange::Group9To10Mil,
            YearlyIncomeRange::Group10To12Mil,
            YearlyIncomeRange::Group12To15Mil,
            YearlyIncomeRange::Group15To20Mil,
            YearlyIncomeRange::Above20Mil
        ]
    }
}

