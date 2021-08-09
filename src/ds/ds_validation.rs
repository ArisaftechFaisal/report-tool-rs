use super::{DataSet, DataSetConfig, IncludeCriteria};
use super::field::{FieldType, ComputedFieldType};
use super::table::{Column, Header, Table};
use super::meta::{Meta, CustomFieldVariant, CustomField};
use super::data::{input_record::InputRecord, Data};
use crate::errors::RustlyzerError;
use serde_json::Value;
use crate::helpers::strings::into_clean_string;

impl DataSet {
    pub(super) fn validate_and_filter(mut data: Data, meta: &Meta, config: &DataSetConfig) ->
                                                                                      Result<Data,
    RustlyzerError> {
       let mut n_records = Vec::<InputRecord>::with_capacity(data.records.len()/2);
        for (row, mut record) in data.records.into_iter().enumerate() {
           record.validate_birth_year(config.created_year, row)?;
            let mut would_include = true;
            DataSet::validate_meta_data(&mut record, meta, row)?;
            let include_map = IncludeCriteria::map_from_vec(&config.includes);
            for (cat_string, items) in include_map.into_iter() {
                would_include = false; // Needs to matched and made false to NOT ignore
                for item in items.into_iter() {
                    match item {
                        IncludeCriteria::PurchaseStatusInclude(cond) => if record.status == cond {
                          would_include = true;
                          break;
                        },
                        IncludeCriteria::MaritalStatusInclude(cond) => if record.marital_status == cond {
                           would_include = true;
                           break;
                        },
                       IncludeCriteria::GenderInclude(cond) => if record.gender == cond {
                           would_include = true;
                           break;
                       },
                       IncludeCriteria::ChildrenRangeInclude(cond) => if record.get_children_range() ==
                           cond {
                           would_include = true;
                           break;
                       },
                       IncludeCriteria::JobInclude(cond) => if record.job == cond {
                           would_include = true;
                           break;
                       },
                       IncludeCriteria::AgeRange1070Include(cond) => if record.get_age_group_1070
                       (config.created_year) == cond {
                           would_include = true;
                           break;
                       },
                      IncludeCriteria::YearlyIncomeRangeInclude(cond) => if record.get_income_range()
                          == cond {
                          would_include = true;
                          break;
                      },
                       IncludeCriteria::PrefectureInclude(cond) => if record.prefecture == cond {
                           would_include = true;
                           break;
                       }
                    }
                }
                if would_include == false { break; }
            }
            DataSet::clean_data(&mut record);

            if would_include {
                n_records.push(record);
            }
        }
       data.records = n_records;
        Ok(data)
       //  data.records.iter().enumerate().map(|(row,record)| {
       //      record.validate_birth_year(config.created_year, row).or_else()
       //  });
       // Ok(())
    }

    fn validate_meta_data(mut record: &mut InputRecord, meta: &Meta, row_number: usize) -> Result<
        (), RustlyzerError> {
        for (&index, custom_field) in meta.custom_fields.iter() {
            // "Required" validation breaks for most input files, so abandoned
            {
            // if custom_field.required == true {
            //     match record.custom_fields.get(index) {
            //         Some(None) | None => return Err(RustlyzerError::InvalidDataError {
            //             val: "null".to_string(),
            //             field: format!("field{}", index),
            //             row: Some(row_number)
            //         }),
            //         _ => ()
            //     }
            // }
            }
            match record.custom_fields.get_mut(&index) {
                None => { record.custom_fields.insert(index, None); },
                _ => (),
            }
        }
        Ok(())
    }

    fn clean_data(mut record: &mut InputRecord) -> () {
        for (_, v) in record.custom_fields.iter_mut() {
            match &v {
                Some(val) => match val {
                   Value::String(s) => {
                     *v = Some(Value::String(into_clean_string(s)));
                     ()
                    },
                    _ => ()
                },
                None => ()
            }
        }
    }
}