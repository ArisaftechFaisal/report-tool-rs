use crate::errors::RustlyzerError;
use csv::ReaderBuilder;
use serde_json::Value;
use std::collections::HashMap;
// use anyhow::Result;

mod ds_private;
mod ds_validation;
pub mod config;
pub mod data;
pub mod field;
pub mod meta;
pub mod table;

pub use config::{DataSetConfig, Language};
use data::{Data, input_record::InputRecord};
use field::FieldType;
use meta::{CustomFieldVariant, Meta};
use table::{Column, Header, Table};
use crate::ds::field::ComputedFieldType;
use csv::Trim::Fields;
use crate::ds::table::{TableWithMeta, SpecialCase};
use crate::ds::config::IncludeCriteria::PurchaseStatusInclude;
use crate::ds::data::enums::PurchaseStatus;
use crate::ds::config::IncludeCriteria;

pub struct DataSet {
    pub meta: Meta, // fields: HashMap<>
    pub config: DataSetConfig,
    pub data: Data,
}

impl DataSet {
    pub fn from_data(meta_str: &str, config: DataSetConfig, content_str: &str) -> Result<Self,
        RustlyzerError> {
        let meta = Meta::from_json(meta_str)?;
        let mut data = Data::from_csv(content_str)?;
        let data = DataSet::validate_and_filter(data, &meta, &config)?;
        Ok(DataSet { meta, config, data })
    }



    pub fn get_fkc_raw_table(&self) -> Result<Table, RustlyzerError> {
        let mut fields = vec![
            FieldType::Id,
            FieldType::UserId,
            FieldType::CreatedAt,
            FieldType::Gender,
            FieldType::Prefecture,
            FieldType::Region,
            FieldType::Age,
            FieldType::AgeGroup,
            FieldType::Job,
            FieldType::MaritalStatus,
            FieldType::Children,
            FieldType::YearlyIncome,
        ];
        fields.extend(self.custom_fields_except_html());
        let mut cols = Vec::<Column>::new();
        for field in fields.iter() {
            cols.extend(self.get_cols_for_field(field)?)
        }
        Ok(Table::new(cols))
    }

    pub fn get_it_raw_table(&self) -> Result<Table, RustlyzerError> {
        let mut fields = vec![
            FieldType::Id,
            FieldType::UserId,
            FieldType::CreatedAt,
            FieldType::Gender,
            FieldType::Prefecture,
            FieldType::Region,
            FieldType::Age,
            FieldType::AgeGroup,
            FieldType::AgeGroup1060,
            FieldType::AgeGroup1070,
            FieldType::Job,
            FieldType::MaritalStatus,
            FieldType::Children,
            FieldType::MaritalStatusAndChildren,
            FieldType::YearlyIncome,
        ];
        fields.extend(self.custom_fields_except_html());
        let mut cols = Vec::<Column>::new();
        for field in fields.iter() {
            cols.extend(self.get_cols_for_field(field)?)
        }
        Ok(Table::new(cols))
    }

    pub fn get_user_graph_table_age1060(&self) -> Result<Table, RustlyzerError> {
        let fields = vec![
            FieldType::Computed(ComputedFieldType::AgeGroup1060AggregateLabel),
            FieldType::Computed(ComputedFieldType::AgeGroup1060AggregateValue),
            FieldType::Computed(ComputedFieldType::AgeGroup1060AggregateDisplay),
        ];
        self.get_static_computed_table_1(fields)
    }

    pub fn get_user_graph_table_age1070(&self) -> Result<Table, RustlyzerError> {
        let fields = vec![
            FieldType::Computed(ComputedFieldType::AgeGroup1070AggregateLabel),
            FieldType::Computed(ComputedFieldType::AgeGroup1070AggregateValue),
            FieldType::Computed(ComputedFieldType::AgeGroup1070AggregateDisplay),
        ];
        self.get_static_computed_table_1(fields)
    }

    pub fn get_user_graph_table_gender(&self) -> Result<Table, RustlyzerError> {
        let fields = vec![
            FieldType::Computed(ComputedFieldType::GenderAggregateLabel),
            FieldType::Computed(ComputedFieldType::GenderAggregateValue),
            FieldType::Computed(ComputedFieldType::GenderAggregateDisplay),
        ];
        self.get_static_computed_table_1(fields)
    }

    pub fn get_user_graph_table_marital(&self) -> Result<Table, RustlyzerError> {
        let fields = vec![
            FieldType::Computed(ComputedFieldType::MaritalStatusAggregateLabel),
            FieldType::Computed(ComputedFieldType::MaritalStatusAggregateValue),
            FieldType::Computed(ComputedFieldType::MaritalStatusAggregateDisplay),
        ];
        self.get_static_computed_table_1(fields)
    }

    pub fn get_user_graph_table_children(&self) -> Result<Table, RustlyzerError> {
        let fields = vec![
            FieldType::Computed(ComputedFieldType::ChildrenAggregateLabel),
            FieldType::Computed(ComputedFieldType::ChildrenAggregateValue),
            FieldType::Computed(ComputedFieldType::ChildrenAggregateDisplay),
        ];
        self.get_static_computed_table_1(fields)
    }

    pub fn get_user_graph_table_job(&self) -> Result<Table, RustlyzerError> {
       let fields = vec![
           FieldType::Computed(ComputedFieldType::JobAggregateLabel),
           FieldType::Computed(ComputedFieldType::JobAggregateValue),
           FieldType::Computed(ComputedFieldType::JobAggregateGraphLabel),
           FieldType::Computed(ComputedFieldType::JobAggregatePercentage),
       ];

        self.get_static_computed_table_2(fields)
    }

    pub fn get_user_graph_table_region(&self) -> Result<Table, RustlyzerError> {
        let fields = vec![
            FieldType::Computed(ComputedFieldType::RegionAggregateLabel),
            FieldType::Computed(ComputedFieldType::RegionAggregateValue),
            FieldType::Computed(ComputedFieldType::RegionAggregateGraphLabel),
            FieldType::Computed(ComputedFieldType::RegionAggregatePercentage),
        ];

        self.get_static_computed_table_2(fields)
    }

    pub fn get_user_graph_table_income(&self) -> Result<Table, RustlyzerError> {
        let fields = vec![
            FieldType::Computed(ComputedFieldType::YearlyIncomeAggregateLabel),
            FieldType::Computed(ComputedFieldType::YearlyIncomeAggregateValue),
            FieldType::Computed(ComputedFieldType::YearlyIncomeAggregateGraphLabel),
            FieldType::Computed(ComputedFieldType::YearlyIncomeAggregatePercentage),
        ];

        self.get_static_computed_table_2(fields)
    }

    pub fn get_aggregate_tables(&self) -> Result<Vec<TableWithMeta>, RustlyzerError> {
        let fields = self.custom_fields_with_options_all();
        let mut table_with_meta_vec = Vec::<TableWithMeta>::new();
        // Total number of user responses
        let total = self.data.len() as f64;
        for field in fields.iter() {
            if let Ok(map) = self.data.get_custom_field_map(field, &self.meta) {
            let col_labels = map.keys().cloned().collect::<Vec<String>>();
            let col_values = map.values().cloned().collect::<Vec<usize>>();
            let mut col_percentage = Vec::<f64>::new();
            col_values.iter().for_each(|v| col_percentage.push((*v as f64 / total) * 100.0));
            let table = TableWithMeta::new(
                vec![
                    self.meta.get_custom_field_title(field)?,
                    self.meta.get_custom_field_type_str(field, self.config.lng)?,
                    self.meta.get_custom_field_label(field)?
                ],
                Table::new(vec![
                Column::from_contents(Header::new("選択肢".to_string(), false), col_labels, None),
                Column::from_contents(Header::new("件数".to_string(), false), col_values.into_iter
                ().map(|val| val.to_string()).collect(),
                                      None),
                Column::from_contents(Header::new("割合".to_string(), false), col_percentage
                    .into_iter().map(|val| format!("{:.2}%", val)).collect(), None),
            ]));
            table_with_meta_vec.push(table);
            }

        }
        Ok(table_with_meta_vec)
    }

    pub fn get_crosstab_tables(&self, crosstab_type: CrosstabType) -> Result<Vec<TableWithMeta>,
        RustlyzerError> {
        let mut fields = vec![
            FieldType::AgeGroup1060,
            FieldType::Gender,
            FieldType::MaritalStatus,
            FieldType::Children,
            FieldType::Job,
            FieldType::Region,
            FieldType::YearlyIncome,
        ];
        fields.append(&mut self.custom_fields_with_options_all());
        let mut tables = Vec::<TableWithMeta>::new();
        for field_base in fields.iter() {
            let variants = self.data.get_field_variants_as_string(
                field_base,
                &self.meta,
                self .config.lng)?;
            // Primary col (all golden column)
            let mut cols_primary = self.get_crosstab_primary_base_cols(field_base).unwrap();
            let freq_perc = self.data.get_self_count_distribution(
                field_base,
                &self.meta,
                self.config.lng,
                self.config.created_year
            )?;
            for (i, variant) in variants.iter().enumerate() {
               let header = Header::new(variant.to_owned(), true);
                let contents = vec![
                    freq_perc.freq.get(i).unwrap().to_string(),
                    format!("{:.2}%", freq_perc.perc.get(i).unwrap())
                ];
                cols_primary.push(Column::from_contents(
                    header,
                    contents,
                    None
                ));
            }
            tables.push(TableWithMeta::with_special_case(
                SpecialCase::SpaceAndHighlightOn4,
                Table::new(cols_primary)
            ));

            // Secondary columns (crosstabs)
            for field_secondary in fields.iter() {
                let variants_secondary = self.data.get_field_variants_as_string(
                    field_secondary,
                    &self.meta,
                    self.config.lng
                )?;
                let mut cols = self.get_crosstab_base_cols(field_secondary)?;
                let mut cols_data = Vec::<Column>::new();
                variants.iter().for_each(|v| cols_data.push(
                    Column::new(Header::new(v.to_string(), true), None, variants_secondary.len())
                ));
                for variant_secondary in variants_secondary.iter() {
                    let row: Vec<String>;
                    if let CrosstabType::N = crosstab_type {
                        row = self.data.get_count_distribution(
                            field_base,
                            field_secondary,
                            variant_secondary,
                            &self.meta,
                            self.config.created_year,
                            self.config.lng
                        )?.into_iter().map(|v| v.to_string()).collect::<Vec<String>>();
                    } else {
                        row = self.data.get_perc_distribution(
                            field_base,
                            field_secondary,
                            variant_secondary,
                            &self.meta,
                            self.config.created_year,
                            self.config.lng
                        )?;
                    }
                    if row.len() != variants.len() { panic!("Wrong variant length!")}
                    row.iter().enumerate().for_each(|(i, val)| {
                        cols_data.get_mut(i).unwrap().contents.push(val.to_owned())
                    });
                }
                cols.append(&mut cols_data);
                tables.push(TableWithMeta::with_special_case(
                    SpecialCase::None,
                    Table::new(cols)
                ));
            }
        }
        Ok(tables)
    }

}


#[derive(Debug, Clone, Copy)]
pub enum CrosstabType {
    N,
    Perc
}