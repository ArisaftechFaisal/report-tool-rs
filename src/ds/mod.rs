use crate::errors::RustlyzerError;
use csv::ReaderBuilder;
use serde_json::Value;
use std::collections::HashMap;
// use anyhow::Result;

pub mod data;
pub mod field;
pub mod meta;
pub mod places;
pub mod table;
use data::{Data, InputRecord};
use field::FieldType;
use meta::{CustomFieldVariant, Meta};
use table::{Column, Header, Table};
use crate::ds::field::ComputedFieldType;
use csv::Trim::Fields;
use crate::ds::table::{TableWithMeta, SpecialCase};

pub struct DataSet {
    pub meta: Meta, // fields: HashMap<>
    pub config: DataSetConfig,
    pub data: Data,
}

impl DataSet {
    pub fn from_data(meta_str: &str, config: DataSetConfig, content_str: &str) -> Result<Self,
        RustlyzerError> {
        let meta = Meta::from_json(meta_str)?;
        let data = Data::from_csv(content_str)?;
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
        fields.extend(self.custom_fields_with_options_all());
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
        fields.extend(self.custom_fields_with_options_all());
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
        for field in fields.iter() {
            if let Ok(map) = self.data.get_custom_field_map(field, &self.meta) {
            let col_labels = map.keys().cloned().collect::<Vec<String>>();
            let col_values = map.values().cloned().collect::<Vec<usize>>();
            let mut col_percentage = Vec::<f64>::new();
            let total = col_values.iter().sum::<usize>() as f64;
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
            // println!("step18");
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
        // println!("{:?}", fields);
        let mut tables = Vec::<TableWithMeta>::new();
        for field_base in fields.iter() {
            // println!("Field base: {:?}", field_base);
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
            // tables.push(Table::new(cols_primary));

            // Secondary columns (crosstabs)
            for field_secondary in fields.iter() {
                // println!("    Field Secondary: {:?}", field_secondary);
                let variants_secondary = self.data.get_field_variants_as_string(
                    field_secondary,
                    &self.meta,
                    self.config.lng
                )?;
                // println!("      -- step1");
                let mut cols = self.get_crosstab_base_cols(field_secondary)?;
                // println!("      -- step2");
                let mut cols_data = Vec::<Column>::new();
                variants.iter().for_each(|v| cols_data.push(
                    Column::new(Header::new(v.to_string(), true), None, variants_secondary.len())
                ));
                // println!("      -- step3");
                for variant_secondary in variants_secondary.iter() {
                    // println!("    --Variant Secondary: {:?}", variant_secondary);
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
                    // println!("      -- Row length: {}", row.len());
                    // println!("      -- Variants length: {}", variants.len());
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
                // tables.push(Table::new(cols));
            }
        }
        Ok(tables)
    }

}

impl DataSet {
    fn get_static_computed_table_1(&self, fields: Vec<FieldType>) -> Result<Table, RustlyzerError> {
        // Check if right call
        if fields.len() != 3 { return Err(RustlyzerError::WrongArgument) }

        let mut col_vec = Vec::<Column>::new();
        for field in fields.iter() {
            let header = Header::new(self.data.get_static_field_title(field, self.config.lng)
                                         .to_string(),
                                     true);
            let footer = Some(self.data.get_static_computed_field_footer(field));
            let contents = self.data.get_static_computed_field_vals(field, self.config.created_year,
                                                                    self.config.lng);
            let col = Column::from_contents(header, contents, footer);
            col_vec.push(col);
        }
        // Display
        col_vec.get_mut(2)?.header.highlight = false;
        let mut display_contents = Vec::<String>::new();
        for val in col_vec.get(1)?.contents.iter() {
            display_contents.push(format!("{}件", val));
        }
        col_vec.get_mut(2)?.contents = display_contents;
        Ok(Table::new(col_vec))
    }

    fn get_static_computed_table_2(&self, fields: Vec<FieldType>) -> Result<Table, RustlyzerError> {
        // Check if right call
        if fields.len() != 4 { return Err(RustlyzerError::WrongArgument) }

        let mut col_vec = Vec::<Column>::new();
        for field in fields.iter() {
            let header = Header::new(self.data.get_static_field_title(field, self.config.lng)
                                         .to_string(),
                                     true);
            let footer = Some(self.data.get_static_computed_field_footer(field));
            let contents = self.data.get_static_computed_field_vals(field, self.config.created_year,
                                                                    self.config.lng);
            let col = Column::from_contents(header, contents, footer);
            col_vec.push(col);
        }
        // Graph Display & Percentage
        col_vec.get_mut(2)?.header.highlight = false;
        col_vec.get_mut(3)?.header.highlight = false;
        let total: f64 = self.data.records.len() as f64;
        let mut graph_display_contents = Vec::<String>::new();
        let mut percentage_contents = Vec::<String>::new();
        for (i, val) in col_vec.get(1)?.contents.iter().enumerate() {
            let num_val = val.parse::<usize>()?;
            graph_display_contents.push(format!("{}(n={})", col_vec.get(0)?.contents.get(i)
               ?, num_val));
            percentage_contents.push(format!("{:.1}%", (num_val as f64/ total) * 100.0));
        }
        col_vec.get_mut(2)?.contents = graph_display_contents;
        col_vec.get_mut(3)?.contents = percentage_contents;
        col_vec.get_mut(3)?.footer = Some("100.0%".to_string());


        Ok(Table::new(col_vec))
    }

    fn custom_fields_with_options_all(&self) -> Vec<FieldType> {
        self.meta
            .custom_fields
            .iter()
            .filter(|(k, v)| {
                match v.variant {
                    CustomFieldVariant::Html {..}
                    | CustomFieldVariant::Text {..}
                    | CustomFieldVariant::TextArea {..} => false,
                    _ => true
                }
            })
            .map(|(k, _)| FieldType::Custom(k.to_owned()))
            .collect::<Vec<FieldType>>()
    }

    fn custom_computed_fields_all(&self) -> Vec<FieldType> {
        self.meta
            .custom_fields
            .keys()
            .map(|k| FieldType::Computed(ComputedFieldType::Custom(k.to_owned())))
            .collect::<Vec<FieldType>>()
    }

    fn get_cols_for_field(&self, field: &FieldType) -> Result<Vec<Column>, RustlyzerError> {
        let capacity = self.data.len();
        if self.meta.is_custom_field_multiselect(field) {
            let option_keys = self.meta.get_custom_field_option_keys(field)?;
            let mut cols = Vec::<Column>::with_capacity(option_keys.len());
            // cols.push(self.init_col_for_field(field)?);
            // let mut base_col = cols.get_mut(0).unwrap(); // always succeed
            let mut base_col = self.init_col_for_field(field)?;
            for record in self.data.records.iter() {
                base_col.contents.push(record.get_field_value_as_str(
                    field,
                    self.config.lng,
                    self.config.created_year,
                )?);
            }
            for key in option_keys.iter() {
                let option_val = self.meta.get_custom_field_option_value(field, key)?;
                let mut col = Column::new(Header::new(option_val.to_owned(), true), None, capacity);
                for record in self.data.records.iter() {
                    // let vec_option = record.get_custom_field(field, self.config.lng)?;
                    match record.get_custom_field(field)? {
                        Some(Value::Array(values)) => {
                            if let Some(_) = values.iter().find(|&val| {
                                if let Value::String(s) = val {
                                    if *s == *key {
                                        return true;
                                    }
                                }
                                false
                            }) {
                                col.contents.push(String::from("1"));
                            } else {
                                col.contents.push(String::from("0"));
                            }
                        }
                        _ => col.contents.push(String::from("0")),
                    }
                }
                cols.push(col);
            }
            cols.insert(0, base_col);
            // println!("{}", cols.len());
            Ok(cols)
        } else {
            let mut col = self.init_col_for_field(field)?;
            for record in self.data.records.iter() {
                match record.get_field_value_as_str(
                    field,
                    self.config.lng,
                    self.config.created_year,
                ) {
                    Ok(s) => col.contents.push(s),
                    Err(err) => return Err(err),
                }
            }
            Ok(vec![col])
        }
    }

    fn init_col_for_field(&self, field: &FieldType) -> Result<Column, RustlyzerError> {
        let capacity = self.data.len();
        match field {
            FieldType::Custom(_) => Ok(Column::new(
                Header::new(
                    self.meta.get_custom_field_label(field)?,
                    false
                    // if let Ok(CustomFieldVariant::MultiSelect { .. }) =
                    //     self.meta.get_custom_field_variant(field)
                    // {
                    //     true
                    // } else {
                    //     false
                    // },
                ),
                None,
                capacity,
            )),
            _ => Ok(Column::new(
                Header::new(
                    self.data
                        .get_static_field_title(&field, self.config.lng)
                        .to_string(),
                    false,
                ),
                None,
                capacity,
            )),
        }
    }

    fn init_field_col_tuple_for_fields(
        &self,
        fields: Vec<FieldType>,
    ) -> Result<Vec<(FieldType, Column)>, RustlyzerError> {
        fields
            .iter()
            .map(|&f| self.init_field_col_tuple_for_field(f))
            .collect::<Result<Vec<(FieldType, Column)>, RustlyzerError>>()
    }

    fn init_field_col_tuple_for_field(
        &self,
        field: FieldType,
    ) -> Result<(FieldType, Column), RustlyzerError> {
        let capacity = self.data.len();
        match field {
            FieldType::Custom(_) => Ok((
                field.to_owned(),
                Column::new(
                    Header::new(
                        self.meta.get_custom_field_label(&field)?,
                        if let Ok(CustomFieldVariant::MultiSelect { .. }) =
                            self.meta.get_custom_field_variant(&field)
                        {
                            true
                        } else {
                            false
                        },
                    ),
                    None,
                    capacity,
                ),
            )),
            _ => Ok((
                field.to_owned(),
                Column::new(
                    Header::new(
                        self.data
                            .get_static_field_title(&field, self.config.lng)
                            .to_string(),
                        false,
                    ),
                    None,
                    capacity,
                ),
            )),
        }
    }


    fn get_crosstab_primary_base_cols(&self, field: &FieldType) -> Result<Vec<Column>,
        RustlyzerError> {
        let header = Header::new(self.get_crosstab_field_title(field)?, true);
        let contents = Vec::<String>::new();
        let col_1 = Column::from_contents(header, contents, None);

        let header = Header::new(self.get_crosstab_field_type(field)?, true);
        let contents = Vec::<String>::new();
        let col_2 = Column::from_contents(header, contents, None);

        let header = Header::new(self.get_crosstab_field_label(field)?, true);
        let contents = Vec::<String>::new();
        let col_3 = Column::from_contents(header, contents, None);

        let header = Header::new("選択肢".to_string(), true);
        let contents = vec!["件数".to_string(), "割合".to_string()];
        let col_4 = Column::from_contents(header, contents, None);
        Ok(vec![col_1, col_2, col_3, col_4])
    }

    fn get_crosstab_base_cols(&self, field: &FieldType) -> Result<Vec<Column>, RustlyzerError> {
        // if *field == FieldType::Custom(5) { println!("      -- step11"); }
        let header = Header::new(self.get_crosstab_field_title(field)?, false);
        // if *field == FieldType::Custom(5) { println!("      -- step12"); }
        let contents = vec![
            self.get_crosstab_field_type(field)?,
            self.get_crosstab_field_label(field)?
        ];
        // if *field == FieldType::Custom(5) { println!("      -- step13"); }
        let col_1 = Column::from_contents(header, contents, None);

        // if *field == FieldType::Custom(5) { println!("      -- step14"); }
        let header = Header::new("選択肢".to_string(), false);
        // if *field == FieldType::Custom(5) { println!("      -- step15"); }
        let contents = self.data.get_field_variants_as_string(field, &self.meta, self.config.lng)?;
        // if *field == FieldType::Custom(5) { println!("      -- step16"); }
        let len = contents.len();
        // if *field == FieldType::Custom(5) { println!("      -- step17"); }
        let col_2 = Column::from_contents(header, contents, None);
        // if *field == FieldType::Custom(5) { println!("      -- step18"); }

        let freq_perc = self.data.get_self_count_distribution(
            field,
            &self.meta,
            self.config.lng,
            self.config.created_year
        )?;

        // if *field == FieldType::Custom(5) { println!("      -- step19"); }
        let header = Header::new("件数".to_string(), false);
        // if *field == FieldType::Custom(5) { println!("      -- step20"); }
        let contents = freq_perc.freq.iter().map(|&n| n.to_string()).collect::<Vec<String>>();
        // if *field == FieldType::Custom(5) { println!("      -- step21"); }
        let col_3 = Column::from_contents(header, contents, None);
        // if *field == FieldType::Custom(5) { println!("      -- step22"); }

        let header = Header::new("割合".to_string(), false);
        // if *field == FieldType::Custom(5) { println!("      -- step23"); }
        let contents = freq_perc.perc.iter().map(|&n| format!("{:.2}%", n))
            .collect::<Vec<String>>();
        // if *field == FieldType::Custom(5) { println!("      -- step24"); }
        let col_4 = Column::from_contents(header, contents, None);
        // if *field == FieldType::Custom(5) { println!("      -- step25"); }

        Ok(vec![col_1, col_2, col_3, col_4])
    }

    fn get_crosstab_field_title(&self, field: &FieldType) -> Result<String, RustlyzerError> {
        let title = match field {
            FieldType::AgeGroup1060 => "age_range".to_string(),
            FieldType::Gender => "gender".to_string(),
            FieldType::MaritalStatus => "marital_status".to_string(),
            FieldType::Children => "children".to_string(),
            FieldType::Job => "job".to_string(),
            FieldType::Region => "region".to_string(),
            FieldType::YearlyIncome => "household_income".to_string(),
            FieldType::Custom(i) => format!("field{}", i),
            _ => return Err(RustlyzerError::WrongArgument)
        };
        Ok(title)
    }

    fn get_crosstab_field_type(&self, field: &FieldType) -> Result<String, RustlyzerError> {
       let type_string = match field {
            FieldType::AgeGroup1060 |
            FieldType::Gender |
            FieldType::MaritalStatus |
            FieldType::Children |
            FieldType::Job |
            FieldType::Region |
            FieldType::YearlyIncome => "プルダウン".to_string(),
            FieldType::Custom(i) => self.meta.get_custom_field_type_str(field, self.config.lng).unwrap(),
            _ => return Err(RustlyzerError::WrongArgument)
        };
        Ok(type_string)
    }

    fn get_crosstab_field_label(&self, field: &FieldType) -> Result<String, RustlyzerError> {
       let label = match field {
           FieldType::AgeGroup1060 => "年代".to_string(),
           FieldType::Gender => "性別".to_string(),
           FieldType::MaritalStatus => "未既婚".to_string(),
           FieldType::Children => "子供の人数".to_string(),
           FieldType::Job => "職業".to_string(),
           FieldType::Region => "地域".to_string(),
           FieldType::YearlyIncome => "年収".to_string(),
           FieldType::Custom(i) => self.meta.get_custom_field_label(field).unwrap(),
           _ => return Err(RustlyzerError::WrongArgument)
       };
       Ok(label)
    }

}

#[derive(Debug, Clone, Copy)]
pub struct DataSetConfig {
    lng: Language,
    created_year: u16,
}

impl DataSetConfig {
    /// Create config to be used for DataSet operations.
    ///
    /// ### Arguments
    ///
    /// - `language` - Desired language for data representation
    ///     - Possible values: "en", "ja"
    ///     - Incorrect values will result in "en"
    pub fn new(language: String, created_year: u16) -> Self {
        const LNG_JA: &'static str = "ja";
        const LNG_EN: &'static str = "en";

        let lng: Language;
        match language.as_ref() {
            LNG_JA => lng = Language::Ja,
            _ => lng = Language::En,
        }
        // let created_date = chrono::DateTime::parse_from_str(format!("{} Jan 01", year), "%Y %b %d").unwrap();
        DataSetConfig { lng, created_year }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Language {
    En,
    Ja,
}

#[derive(Debug, Clone, Copy)]
pub enum CrosstabType {
    N,
    Perc
}