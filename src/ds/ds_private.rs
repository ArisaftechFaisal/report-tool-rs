use serde_json::Value;
use super::field::{FieldType, ComputedFieldType};
use super::table::{Column, Header, Table};
use super::DataSet;
use super::meta::CustomFieldVariant;
use crate::errors::RustlyzerError;

impl DataSet {
    pub(super) fn get_static_computed_table_1(&self, fields: Vec<FieldType>) -> Result<Table, RustlyzerError> {
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

    pub(super) fn get_static_computed_table_2(&self, fields: Vec<FieldType>) -> Result<Table, RustlyzerError> {
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

    pub(super) fn custom_fields_with_options_all(&self) -> Vec<FieldType> {
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

    pub(super) fn custom_fields_except_html(&self) -> Vec<FieldType> {
        self.meta
            .custom_fields
            .iter()
            .filter(|(k,v)| {
                match v.variant {
                    CustomFieldVariant::Html {..} => false,
                    _ => true
                }
            })
            .map(|(k, _)| FieldType::Custom((k.to_owned())))
            .collect::<Vec<FieldType>>()
    }

    pub(super) fn custom_computed_fields_all(&self) -> Vec<FieldType> {
        self.meta
            .custom_fields
            .keys()
            .map(|k| FieldType::Computed(ComputedFieldType::Custom(k.to_owned())))
            .collect::<Vec<FieldType>>()
    }

    pub(super) fn get_cols_for_field(&self, field: &FieldType) -> Result<Vec<Column>, RustlyzerError> {
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

    pub(super) fn init_col_for_field(&self, field: &FieldType) -> Result<Column, RustlyzerError> {
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

    pub(super) fn init_field_col_tuple_for_fields(
        &self,
        fields: Vec<FieldType>,
    ) -> Result<Vec<(FieldType, Column)>, RustlyzerError> {
        fields
            .iter()
            .map(|&f| self.init_field_col_tuple_for_field(f))
            .collect::<Result<Vec<(FieldType, Column)>, RustlyzerError>>()
    }

    pub(super) fn init_field_col_tuple_for_field(
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


    pub(super) fn get_crosstab_primary_base_cols(&self, field: &FieldType) -> Result<Vec<Column>,
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

    pub(super) fn get_crosstab_base_cols(&self, field: &FieldType) -> Result<Vec<Column>, RustlyzerError> {
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

    pub(super) fn get_crosstab_field_title(&self, field: &FieldType) -> Result<String, RustlyzerError> {
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

    pub(super) fn get_crosstab_field_type(&self, field: &FieldType) -> Result<String, RustlyzerError> {
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

    pub(super) fn get_crosstab_field_label(&self, field: &FieldType) -> Result<String, RustlyzerError> {
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