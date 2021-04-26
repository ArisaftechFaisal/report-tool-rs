use super::{DataSet, DataSetConfig, IgnoreCriteria};
use super::field::{FieldType, ComputedFieldType};
use super::table::{Column, Header, Table};
use super::meta::CustomFieldVariant;
use super::data::{input_record::InputRecord, Data};
use crate::errors::RustlyzerError;

impl DataSet {
    pub(super) fn validate_and_filter(mut data: Data, config: &DataSetConfig) -> Result<Data,
    RustlyzerError> {
       let mut n_records = Vec::<InputRecord>::with_capacity(data.records.len()/2);
        for (row, record) in data.records.into_iter().enumerate() {
           record.validate_birth_year(config.created_year, row)?;
            let mut would_ignore: bool = false;
            for &ignore in config.ignores.iter() {
               match ignore {
                  IgnoreCriteria::PurchaseStatusIgnore(cond) => if record.status == cond {
                      would_ignore = true;
                      break;
                  },
                   IgnoreCriteria::MaritalStatusIgnore(cond) => if record.marital_status == cond {
                       would_ignore = true;
                       break;
                   },
                   IgnoreCriteria::GenderIgnore(cond) => if record.gender == cond {
                       would_ignore = true;
                       break;
                   },
                   IgnoreCriteria::ChildrenRangeIgnore(cond) => if record.get_children_range() ==
                       cond {
                       would_ignore = true;
                       break;
                   },
                   IgnoreCriteria::JobIgnore(cond) => if record.job == cond {
                       would_ignore = true;
                       break;
                   },
                   IgnoreCriteria::AgeRange1070Ignore(cond) => if record.get_age_group_1070
                   (config.created_year) == cond {
                       would_ignore = true;
                       break;
                   },
                  IgnoreCriteria::YearlyIncomeRangeIgnore(cond) => if record.get_income_range()
                      == cond {
                      would_ignore = true;
                      break;
                  },
                   IgnoreCriteria::PrefectureIgnore(cond) => if record.prefecture == cond {
                       would_ignore = true;
                       break;
                   }
               }
            }
            if !would_ignore {
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
}