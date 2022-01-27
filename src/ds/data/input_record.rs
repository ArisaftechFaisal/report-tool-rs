use hashbrown::HashMap;
use serde_json::Value;
use crate::errors::RustlyzerError;
use super::{Language, Prefecture, Region, FieldType};
use super::enums::{PurchaseStatus, YearlyIncomeRange, Job, MaritalStatus, Gender, AgeRange1060,
                   AgeRange1070,
                   ChildrenRange};
use crate::helpers::{EnumAttrs, Round, ExtractFromStr};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct InputRecord {
    id: String,
    user_id: String,
    campaign_id: String,
    price: u32,
    bonus_point: u32,
    pub(crate) status: PurchaseStatus,
    created_at: String,
    #[serde(rename = "updated at")]
    updated_at: String,
    email: String,
    nickname: String,
    pub(crate) gender: Gender,
    birth_year: u16,
    // job: String,
    pub(crate) job: Job,
    pub(crate) prefecture: Prefecture,
    pub(crate) marital_status: MaritalStatus,
    pub(crate) children: u16,
    pub(crate) household_income_min: u64,
    pub(crate) household_income_max: u64,
    #[serde(flatten, deserialize_with = "custom_fields_de")]
    pub(crate) custom_fields: HashMap<usize, Option<Value>>,
}

impl InputRecord {
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

    pub(crate) fn get_custom_field_str(
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

    pub(crate) fn get_static_field_str(&self, field: &FieldType, lng: Language, created_year: u16) -> String {
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
                format!("{}", "[\"".to_owned() + narr.join("\",\"").as_str() + "\"]")
            }
            Value::Bool(val) => format!("{}", val),
            Value::Null => format!("NULL"), // TODO: need to change this
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

    pub(crate) fn get_age(&self, created_year: u16) -> u8 {
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

    pub(crate) fn get_age_group_1070(&self, created_year: u16) -> AgeRange1070 {
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
        self.get_income_range().as_string(lng)
    }
    pub(crate) fn get_income_range(&self) -> YearlyIncomeRange {
        // self.get_income_custom_range_str(lng, 1, 1_000_000)
        match (self.household_income_min, self.household_income_max) {
            (0..1_000_000, 0..1_000_000) => YearlyIncomeRange::Below1Mil,
            (1_000_000..2_000_000, 1_000_000..2_000_000) => YearlyIncomeRange::Group1To2Mil,
            (2_000_000..3_000_000, 2_000_000..3_000_000) => YearlyIncomeRange::Group2To3Mil,
            (3_000_000..4_000_000, 3_000_000..4_000_000) => YearlyIncomeRange::Group3To4Mil,
            (4_000_000..5_000_000, 4_000_000..5_000_000) => YearlyIncomeRange::Group4To5Mil
                ,
            (5_000_000..6_000_000, 5_000_000..6_000_000) => YearlyIncomeRange::Group5To6Mil
                ,
            (6_000_000..7_000_000, 6_000_000..7_000_000) => YearlyIncomeRange::Group6To7Mil
                ,
            (7_000_000..8_000_000, 7_000_000..8_000_000) => YearlyIncomeRange::Group7To8Mil
                ,
            (8_000_000..9_000_000, 8_000_000..9_000_000) => YearlyIncomeRange::Group8To9Mil
                ,
            (9_000_000..10_000_000, 9_000_000..10_000_000) => YearlyIncomeRange::Group9To10Mil
                ,
            (10_000_000..12_000_000, 10_000_000..12_000_000) => YearlyIncomeRange::Group10To12Mil
                ,
            (12_000_000..15_000_000, 12_000_000..15_000_000) => YearlyIncomeRange::Group12To15Mil
                ,
            (15_000_000..20_000_000, 15_000_000..20_000_000) => YearlyIncomeRange::Group15To20Mil
                ,
            (_, _ ) => YearlyIncomeRange::Above20Mil,
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
        self.get_children_range().as_string(lng)
    }

    pub(crate) fn get_children_range(&self) -> ChildrenRange {
        match self.children {
            0 => ChildrenRange::Group0,
            1 => ChildrenRange::Group1,
            2 => ChildrenRange::Group2,
            3 => ChildrenRange::Group3,
            _ => ChildrenRange::Above4
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

// Custom deserialization
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