use super::{InputRecord, Data, FieldType,
            ComputedFieldType, Region, Language};
use super::enums::{AgeRange1060, YearlyIncomeRange, ChildrenRange, Job, MaritalStatus, Gender };
use hashbrown::HashMap;
use super::{Meta, CustomFieldVariant};
use crate::errors::RustlyzerError;
use serde_json::Value;
use indexmap::map::IndexMap;
use crate::helpers::EnumAttrs;
use std::ptr::null;

impl Data {
    pub fn get_static_computed_field_vals(&self, field:&FieldType, year: u16, lng: Language)
        -> Vec<String>
    {
        match lng {
            _ => match field {
                FieldType::Computed(computed) => {
                    let mut list = Vec::<String>::new();
                    match computed {
                        // Labels
                        ComputedFieldType::AgeGroup1060AggregateLabel => vec![
                            "10代以下".to_string(),
                            "20代".to_string(),
                            "30代".to_string(),
                            "40代".to_string(),
                            "50代".to_string(),
                            "60代以上".to_string(),
                        ],
                        ComputedFieldType::AgeGroup1070AggregateLabel => vec![
                            "10代以下".to_string(),
                            "20代".to_string(),
                            "30代".to_string(),
                            "40代".to_string(),
                            "50代".to_string(),
                            "60代".to_string(),
                            "70代以上".to_string(),
                        ],
                        ComputedFieldType::GenderAggregateLabel => vec![
                            "女性".to_string(),
                            "男性".to_string(),
                        ],
                        ComputedFieldType::MaritalStatusAggregateLabel => vec![
                            "既婚".to_string(),
                            "未婚".to_string(),
                        ],
                        ComputedFieldType::ChildrenAggregateLabel => vec![
                            "0人".to_string(),
                            "1人".to_string(),
                            "2人".to_string(),
                            "3人".to_string(),
                            "4人以上".to_string(),
                        ],
                        ComputedFieldType::JobAggregateLabel => Job::get_all_string(lng),
                        ComputedFieldType::RegionAggregateLabel => Region::get_all_string(lng),
                        ComputedFieldType::YearlyIncomeAggregateLabel => vec![
                            "100万円未満".to_string(),
                            "100～200万円未満".to_string(),
                            "200～300万円未満".to_string(),
                            "300～400万円未満".to_string(),
                            "400～500万円未満".to_string(),
                            "500～600万円未満".to_string(),
                            "600～700万円未満".to_string(),
                            "700～800万円未満".to_string(),
                            "800～900万円未満".to_string(),
                            "900～1000万円未満".to_string(),
                            "1000～1200万円未満".to_string(),
                            "1200～1500万円未満".to_string(),
                            "1500～2000万円未満".to_string(),
                            "2000万円以上".to_string(),
                        ],

                        // Values
                        ComputedFieldType::AgeGroup1060AggregateValue => {
                            let mut arr: [usize; 6] = [0; 6];
                            for record in &self.records {
                                match record.get_age(year) {
                                    0..20 => arr[0] += 1,
                                    20..30 => arr[1] += 1,
                                    30..40 => arr[2] += 1,
                                    40..50 => arr[3] += 1,
                                    50..60 => arr[4] += 1,
                                    60..= u8::MAX => arr[5] += 1,
                                }
                            }
                            arr.to_vec().into_iter().map(|x| x.to_string())
                               .collect::<Vec<String>>()
                        },
                        ComputedFieldType::AgeGroup1070AggregateValue => {
                            let mut arr: [usize; 7] = [0; 7];
                            for record in &self.records {
                                match record.get_age(year) {
                                    0..20 => arr[0] += 1,
                                    20..30 => arr[1] += 1,
                                    30..40 => arr[2] += 1,
                                    40..50 => arr[3] += 1,
                                    50..60 => arr[4] += 1,
                                    50..70 => arr[5] += 1,
                                    70..=u8::MAX => arr[6] += 1,
                                }
                            }
                            arr.to_vec().into_iter().map(|x| x.to_string())
                               .collect::<Vec<String>>()
                        },
                        ComputedFieldType::GenderAggregateValue => {
                            let mut arr: [usize; 2] = [0; 2];
                            for record in &self.records {
                                match &record.gender {
                                    Gender::Female => arr[0] += 1,
                                    Gender::Male => arr[1] += 1
                                }
                            }
                            arr.to_vec().into_iter().map(|x| x.to_string()).collect()
                        },
                        ComputedFieldType::MaritalStatusAggregateValue => {
                            let mut arr: [usize; 2] = [0; 2];
                            for record in &self.records {
                                match &record.marital_status {
                                    MaritalStatus::Married => arr[0] += 1,
                                    MaritalStatus::Single => arr[1] += 1,
                                }
                            }
                            arr.to_vec().into_iter().map(|x| x.to_string()).collect()
                        },
                        ComputedFieldType::ChildrenAggregateValue => {
                            let mut arr: [usize; 5] = [0; 5];
                            for record in &self.records {
                                match record.children {
                                    0 => arr[0] += 1,
                                    1 => arr[1] += 1,
                                    2 => arr[2] += 1,
                                    3 => arr[3] += 1,
                                    4..=u16::MAX => arr[4] += 1,
                                }
                            }
                            arr.to_vec().into_iter().map(|x| x.to_string()).collect()
                        },
                        ComputedFieldType::JobAggregateValue => {
                            let jobs = Job::get_all();
                            let mut vec = vec![0usize; jobs.len()];
                            for record in &self.records {
                                for (i, job) in jobs.iter().enumerate() {
                                    if job == &record.job {
                                        vec[i] += 1;
                                        break;
                                    }
                                }
                            }
                            vec.into_iter().map(|x| x.to_string()).collect()
                        },
                        ComputedFieldType::RegionAggregateValue => {
                            let regions = Region::get_all();
                            let mut vec = vec![0usize; regions.len()];
                            for record in &self.records {
                                for (i, region) in regions.iter().enumerate() {
                                    if region == &Region::from_prefecture(&record.prefecture) {
                                        vec[i] += 1;
                                        break;
                                    }
                                }
                            }
                            vec.into_iter().map(|x| x.to_string()).collect()
                        },
                        ComputedFieldType::YearlyIncomeAggregateValue => {
                            let mut arr: [usize; 14] = [0; 14];
                            for record in &self.records {
                                match (record.household_income_min, record.household_income_max) {
                                    (0..1_000_000, 0..1_000_000) => arr[0] += 1,
                                    (1_000_000..2_000_000, 1_000_000..2_000_000) => arr[1] += 1,
                                    (2_000_000..3_000_000, 2_000_000..3_000_000) => arr[2] += 1,
                                    (3_000_000..4_000_000, 3_000_000..4_000_000) => arr[3] += 1,
                                    (4_000_000..5_000_000, 4_000_000..5_000_000) => arr[4] += 1,
                                    (5_000_000..6_000_000, 5_000_000..6_000_000) => arr[5] += 1,
                                    (6_000_000..7_000_000, 6_000_000..7_000_000) => arr[6] += 1,
                                    (7_000_000..8_000_000, 7_000_000..8_000_000) => arr[7] += 1,
                                    (8_000_000..9_000_000, 8_000_000..9_000_000) => arr[8] += 1,
                                    (9_000_000..10_000_000, 9_000_000..10_000_000) => arr[9] += 1,
                                    (10_000_000..12_000_000, 10_000_000..12_000_000) => arr[10] += 1,
                                    (12_000_000..15_000_000, 12_000_000..15_000_000) => arr[11] += 1,
                                    (15_000_000..20_000_000, 15_000_000..20_000_000) => arr[12] += 1,
                                    (_, _ ) => arr[13] += 1,
                                }
                            }
                            arr.to_vec().into_iter().map(|x| x.to_string()).collect()
                        },

                        // Others
                        _ => Vec::<String>::new()
                    }
                },
                _ => panic!("Called get_computed_field_labels on non-computed field.")
            }
        }
    }

    pub fn get_field_variants_as_string(&self, field: &FieldType, meta: &Meta, lng: Language)
        -> Result<Vec<String>, RustlyzerError>
    {
        match field {
            FieldType::AgeGroup1060 => Ok(AgeRange1060::get_all_string(lng)),
            FieldType::Gender => Ok(Gender::get_all_string(lng)),
            FieldType::MaritalStatus => Ok(MaritalStatus::get_all_string(lng)),
            FieldType::Children => Ok(ChildrenRange::get_all_string(lng)),
            FieldType::Job => Ok(Job::get_all_string(lng)),
            FieldType::Region => Ok(Region::get_all_string(lng)),
            FieldType::YearlyIncome => Ok(YearlyIncomeRange::get_all_string(lng)),
            FieldType::Custom(custom_index) => {
                meta.get_custom_field_option_values(&field)
            }
            _ => Err(RustlyzerError::WrongArgument)
        }
    }

    pub fn get_self_count_distribution(&self, field: &FieldType, meta: &Meta, lng: Language,
                                       created_year: u16)
        -> Result<FreqPerc, RustlyzerError>
    {
        let mut freq_perc = FreqPerc::new();
        // if *field == FieldType::Custom(5) { println!("      -- step181"); }
        let mut null_vals = 0usize;
        let mut map = IndexMap::<String, usize>::new();
        let variants = self.get_field_variants_as_string(&field, &meta, lng)?;
        // if *field == FieldType::Custom(5) { println!("      -- step182"); }
        variants.into_iter().for_each(|s| { map.entry(s).or_insert(0);});
        // if *field == FieldType::Custom(5) { println!("      -- step183"); }
        match field {
            FieldType::AgeGroup1060 |
            FieldType::Gender |
            FieldType::MaritalStatus |
            FieldType::Children |
            FieldType::Job |
            FieldType::Region |
            FieldType::YearlyIncome => {
                for record in self.records.iter() {
                    let record_val = record.get_static_field_str(&field, lng, created_year);
                    if record_val == "NULL" {null_vals += 1; continue; }
                    map.entry(record_val)
                       .and_modify(|e| { *e += 1;});
                }
            },
            FieldType::Custom(custom_index) =>  {
                    // let key = meta.get_custom_field_option_key(&field, &value)?;
                for record in self.records.iter() {
                    match record.custom_fields.get(custom_index).ok_or(RustlyzerError::NoneError)? {
                        Some(Value::String(s)) => {
                            map.entry(s.to_string())
                               .and_modify(|e| { *e += 1; });
                        },
                        Some(Value::Number(n)) => {
                            map.entry(n.to_string())
                               .and_modify(|e| { *e += 1; });
                        },
                        Some(Value::Bool(b)) => {
                            map.entry(b.to_string())
                               .and_modify(|e| { *e += 1; });
                        },
                        Some(Value::Array(arr)) => {
                            if arr.len() == 0 {
                                null_vals += 1;
                                continue;
                            }
                            // println!("{:?}", arr);
                            let mut iter = arr.iter();
                            while let Some(Value::String(s)) = iter.next() {
                                // println!("{}",s);
                                map.entry(s.to_string())
                                   .and_modify(|e| { *e += 1; });
                            }
                        },
                        None => { null_vals += 1; continue; }
                        _ => panic!("Abnormal value in custom field!")
                    }
                }
            }
            _ => {
                return Err(RustlyzerError::WrongArgument);
            }
        }
        // if *field == FieldType::Custom(5) { println!("      -- step184"); }
        freq_perc.freq = map.values().cloned().collect::<Vec<usize>>();
        // if *field == FieldType::Custom(5) { println!("      -- step185"); }
        let cnt = self.records.len() - null_vals;
        // if *field == FieldType::Custom(5) { println!("      -- step186"); }
        let perc = freq_perc.freq.iter().map(|n| (n.to_owned() as f64 / cnt as f64) * 100f64)
            .collect::<Vec<f64>>();
        // if *field == FieldType::Custom(5) { println!("      -- step187"); }
        freq_perc.perc = perc;
        // if *field == FieldType::Custom(5) { println!("      -- step188"); }
        Ok(freq_perc)
    }

    /// Returns field variant distribution for field_base variants
    /// - `field_base`: Distribution is mapped on variants of this field
    /// - `field_secondary`: Variant to be mapped
    /// - `value`: Value of `field_secondary`
    pub fn get_count_distribution(&self, field_base: &FieldType,
                                  field_secondary: &FieldType,
                                  value: &String,
                                  meta: &Meta,
                                  created_year: u16,
                                  lng: Language)
        -> Result<Vec<usize>, RustlyzerError>
    {
        let mut map = IndexMap::<String, usize>::new();
        let base_fields = self.get_field_variants_as_string(&field_base, &meta, lng)?;
        base_fields.into_iter().for_each(|s| { map.entry(s).or_insert(0);});
        match field_base {
            FieldType::AgeGroup1060 |
            FieldType::Gender |
            FieldType::MaritalStatus |
            FieldType::Children |
            FieldType::Job |
            FieldType::Region |
            FieldType::YearlyIncome => match field_secondary{
                FieldType::AgeGroup1060 |
                FieldType::Gender |
                FieldType::MaritalStatus |
                FieldType::Children |
                FieldType::Job |
                FieldType::Region |
                FieldType::YearlyIncome => {
                    for record in self.records.iter() {
                        if record.get_static_field_str(&field_secondary, lng, created_year) == *value {
                            map.entry(record.get_static_field_str(&field_base, lng, created_year))
                               .and_modify(|e| { *e += 1;});
                        }
                    }
                },
                FieldType::Custom(custom_index) => {
                   // let key = meta.get_custom_field_option_key(&field, &value)?;
                    for record in self.records.iter() {
                        let does_exist: bool= match record.custom_fields.get(custom_index).ok_or(RustlyzerError::NoneError)? {
                            Some(Value::String(s)) => if s == value { true } else { false },
                            Some(Value::Number(n)) => if n.to_string() == *value { true } else
                            {false},
                            Some(Value::Bool(b)) => if b.to_string() == *value { true } else
                            {false},
                            Some(Value::Array(arr)) => {
                                let mut inner_bool = false;
                                let mut iter = arr.iter();
                                while let Some(Value::String(s)) = iter.next() {
                                   if s == value {
                                       inner_bool = true;
                                   }
                                }
                                inner_bool
                            },
                            None => false,
                            _ => panic!("Abnormal value in custom field")
                        };
                        if does_exist {
                            map.entry(record
                                .get_static_field_str(&field_base, lng, created_year))
                               .and_modify(|e| { *e += 1;});
                        }
                    }
                }
                _ => panic!("Wrong argument"),
            },
            FieldType::Custom(base_custom_index) => match field_secondary {
                FieldType::AgeGroup1060 |
                FieldType::Gender |
                FieldType::MaritalStatus |
                FieldType::Children |
                FieldType::Job |
                FieldType::Region |
                FieldType::YearlyIncome => {
                   for record in self.records.iter() {
                        if record.get_static_field_str(&field_secondary, lng, created_year) == *value {
                            // map.entry(record.get_static_field_str(&field_base, lng, created_year))
                            //    .and_modify(|e| { *e += 1;});
                            self.increment_field_freq_in_map(
                                field_base,
                                meta,
                                lng,
                                created_year,
                                &mut map,
                                record
                            )?;
                        }
                   }
                },
                FieldType::Custom(custom_index) => {
                    // let key = meta.get_custom_field_option_key(&field, &value)?;
                    for record in self.records.iter() {
                        let does_exist: bool = match record.custom_fields.get(custom_index).ok_or(RustlyzerError::NoneError)? {
                            Some(Value::String(s)) => if s == value { true } else { false },
                            Some(Value::Number(n)) => if n.to_string() == *value { true } else {
                                false },
                            Some(Value::Bool(b)) => if b.to_string() == *value { true } else {
                                false },
                            Some(Value::Array(arr)) => {
                                let mut inner_bool = false;
                                let mut iter = arr.iter();
                                while let Some(Value::String(s)) = iter.next() {
                                    if s == value { inner_bool = true; }
                                }
                                inner_bool
                            },
                            None => false,
                            x => panic!("Abnormal value in custom field: {:?}", x)
                        };
                        if does_exist {
                            self.increment_field_freq_in_map(
                                &field_base,
                                meta,
                                lng,
                                created_year,
                                &mut map,
                                record
                            )?;
                        }
                    }
                },
                _ => panic!("Wrong argument")
            },
            _ => {
                return Err(RustlyzerError::WrongArgument);
            }
        }
        Ok(map.values().cloned().collect::<Vec<usize>>())
    }

    pub fn get_perc_distribution(&self, field_base: &FieldType,
                                  field_secondary: &FieldType,
                                  value: &String,
                                  meta: &Meta,
                                  created_year: u16,
                                  lng: Language)
        -> Result<Vec<String>, RustlyzerError>
    {
        let cnt_dist = self.get_count_distribution(
            field_base,
            field_secondary,
            value,
            meta,
            created_year,
            lng
        )?;
        let total = cnt_dist.iter().sum::<usize>() as f64;
        Ok(cnt_dist
            .into_iter()
            .map(|v| format!("{:.2}", (v as f64 / total) * 100f64))
            .collect::<Vec<String>>())
    }

    fn increment_field_freq_in_map(
        &self,
        field: &FieldType,
        meta: &Meta,
        lng: Language,
        created_year: u16,
        map: &mut IndexMap<String, usize>,
        record: &InputRecord
    ) -> Result<(), RustlyzerError> {
        match field {
            FieldType::AgeGroup1060 |
            FieldType::Gender |
            FieldType::MaritalStatus |
            FieldType::Children |
            FieldType::Job |
            FieldType::Region |
            FieldType::YearlyIncome => {
                map.entry(record.get_static_field_str(&field, lng, created_year))
                   .and_modify(|e| { *e += 1;});
                Ok(())
            },
            FieldType::Custom(custom_index) => {
                if meta.is_custom_field_multiselect(field) {
                    // println!("is multiselect");
                    match record.custom_fields.get(custom_index).ok_or(RustlyzerError::NoneError)? {
                        Some(Value::Array(arr)) => {
                            let mut iter = arr.iter();
                            while let Some(Value::String(s)) = iter.next() {
                                map.entry(s.to_owned()).and_modify(|e| { *e += 1 });
                            }
                        },
                        None => (),
                        x => panic!("Abnormal value in custom field: {:?}", x)
                    }
                } else {
                    map.entry(record
                        .get_custom_field_str(&field, lng)?)
                       .and_modify(|e| { *e += 1; });
                }
                Ok(())
            },
            x => unimplemented!("increment_field_freq_in_map isn't implemented for field: {:?}", x)
        }

    }


    pub fn get_static_computed_field_footer(&self, field: &FieldType) -> String {
       match field {
           FieldType::Computed(computed) => match computed {
               // Total for static field values
              ComputedFieldType::AgeGroup1060AggregateValue |
               ComputedFieldType::AgeGroup1070AggregateValue |
               ComputedFieldType::GenderAggregateValue |
               ComputedFieldType::MaritalStatusAggregateValue |
               ComputedFieldType::ChildrenAggregateValue |
               ComputedFieldType::JobAggregateValue |
               ComputedFieldType::RegionAggregateValue |
               ComputedFieldType::YearlyIncomeAggregateValue => self.records.len().to_string(),

               // Total string
               ComputedFieldType::AgeGroup1060AggregateLabel |
               ComputedFieldType::AgeGroup1070AggregateLabel |
               ComputedFieldType::GenderAggregateLabel |
               ComputedFieldType::MaritalStatusAggregateLabel |
               ComputedFieldType::ChildrenAggregateLabel |
               ComputedFieldType::JobAggregateLabel |
               ComputedFieldType::RegionAggregateLabel |
               ComputedFieldType::YearlyIncomeAggregateLabel => "計".to_string(),

               _ => "".to_string(),
           },
           _ => panic!("Called get_computed_field_total on non-computed field.")
       }
    }

    pub fn get_custom_field_total(&self, field: &FieldType, meta: &Meta) -> 
        usize {
            let mut total: usize = 0;
            match field {
                FieldType::Custom(_) => {
                    for record in self.records.iter() {
                        if let Ok(Some(x)) = record.get_custom_field(field) {
                            match x {
                                Value::Null => continue,
                                _ => total += 1
                            }
                        }
                    }
                },
                _ => panic!("Called get_custom_field_total on non-custom-computed field.")
            }
            total
        }

    pub fn get_custom_field_map(&self, field: &FieldType, meta: &Meta) ->
            Result<IndexMap<String, usize>, RustlyzerError> {
        let time = std::time::Instant::now();
        match field {
            FieldType::Custom(custom_index) => {
                let keys = meta.get_custom_field_option_keys(field)?;
                let mut map = IndexMap::new();
                for key in keys {
                    map.entry(meta.get_custom_field_option_value(field, &key)?).or_insert
                    (0);
                }
                for record in self.records.iter() {
                    if let Ok(Some(x)) = record.get_custom_field(field) {
                        match x {
                            Value::String(s) => {map.entry(s.to_owned()).and_modify(|e| *e += 1);
                                ()},
                            Value::Number(n) => {map.entry(n.to_string())
                                .and_modify(|e|
                                *e += 1); ()},
                            Value::Bool(b) => {map.entry(b.to_string()).and_modify(|e| *e += 1);
                                ()},
                            Value::Array(arr) => {
                                for val in arr.iter() {
                                    if let Value::String(arr_s) = val {
                                        map.entry(arr_s.to_owned()).and_modify(|e| *e += 1);
                                    }
                                }
                                // while let Value::String(s2) = arr.iter().next() {
                                //     map.entry(s.to_owned())
                                // }
                            },
                            _ => panic!("Undefined error")
                        }
                    }
                }
                let time = time.elapsed().as_millis();
                // println!("Custom field map for {:?}: {}", field, time);
                Ok(map)
            },
            _ => panic!("Called get_custom_field_map on non-custom-computed field.")
        }
    }

}

pub struct FreqPerc {
    pub freq: Vec<usize>,
    pub perc: Vec<f64>,
}

impl FreqPerc {
    fn new() -> Self {
        FreqPerc{freq: Vec::<usize>::new(), perc: Vec::<f64>::new()}
    }
}