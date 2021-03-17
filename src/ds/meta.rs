// use anyhow::{Context, Result};
use super::FieldType;
use crate::errors::RustlyzerError;
use crate::helpers::*;
use hashbrown::HashMap;
use indexmap::IndexMap;
use serde::{Deserialize, Serialize};
use serde_json;
use serde_json::Value;
use std::iter::FromIterator;
use crate::ds::Language;

/// Meta struct containing the metadata for the custom fields.
/// Metadata is contained in pages: Vec<Page>
#[derive(Debug, Clone)]
pub struct Meta {
    pub custom_fields: IndexMap<usize, CustomField>,
}

impl Meta {
    pub fn from_json(content: &str) -> Result<Self, RustlyzerError> {
        match serde_json::from_str::<MetaInput>(content) {
            Ok(metadata) => {
                // println!("Parsed JSON struct: {:?}", metadata);
                let mut custom_fields_hashmap = HashMap::<usize, CustomField>::new();
                for page in metadata.pages.into_iter() {
                    for field in page.elements.into_iter() {
                        if let Some(id) = usize::extract_from_str(&field.key) {
                            match &field.variant {
                                CustomFieldVariant::Html { .. } => (),
                                _ => {
                                    custom_fields_hashmap.insert(id, field);
                                    ()
                                }
                            }
                        }
                    }
                }
                let mut keys = custom_fields_hashmap
                    .keys()
                    .cloned()
                    .collect::<Vec<usize>>();
                keys.sort();
                let mut custom_fields = IndexMap::<usize, CustomField>::new();
                for k in keys.into_iter() {
                    custom_fields.insert(k, custom_fields_hashmap.remove(&k).unwrap());
                }
                // println!("{:#?}", custom_fields);
                Ok(Meta { custom_fields })
            }
            Err(e) => {
                // println!("{:?}", e);
                Err(RustlyzerError::MetadataWrongFormat)
            }
        }
    }

    pub fn get_custom_field_title(&self, field: &FieldType) -> Result<String, RustlyzerError> {
        match field {
            FieldType::Custom(i) => Ok(format!("field{}", i)),
            _ => Err(RustlyzerError::WrongArgument)
        }
    }

    pub fn get_custom_field_type_str(&self, field: &FieldType, lng: Language) -> Result<String,
        RustlyzerError> {
        match lng {
            _ => match field {
                FieldType::Custom(i) => {
                    match self.custom_fields.get(i).ok_or(RustlyzerError::CustomFieldNotInRecords)?
                        .variant {
                        CustomFieldVariant::Radio { .. } => Ok("ラジオボタン".to_string()),
                        CustomFieldVariant::Dropdown { .. } => Ok("プルダウン".to_string()),
                        CustomFieldVariant::MultiSelect { .. } => Ok("マルチセレクト".to_string()),
                        CustomFieldVariant::Text { .. } => Ok("テキスト".to_string()),
                        CustomFieldVariant::TextArea { .. } => Ok("テキストエリア".to_string()),
                        CustomFieldVariant::Html { .. } => Ok("HTML".to_string()),
                    }
                },
                _ => Err(RustlyzerError::WrongArgument)
            }
        }
    }

    pub fn get_custom_field_label(&self, field: &FieldType) -> Result<String, RustlyzerError> {
        match field {
            FieldType::Custom(i) => Ok(self
                .custom_fields
                .get(i)
                .ok_or(RustlyzerError::CustomFieldNotInRecords)?
                .label
                .to_owned()),
            _ => Err(RustlyzerError::WrongArgument),
        }
    }

    pub fn get_custom_field_variant(
        &self,
        field: &FieldType,
    ) -> Result<&CustomFieldVariant, RustlyzerError> {
        match field {
            FieldType::Custom(i) => Ok(&self
                .custom_fields
                .get(i)
                .ok_or(RustlyzerError::CustomFieldNotInRecords)?
                .variant),
            _ => Err(RustlyzerError::WrongArgument),
        }
    }

    pub fn is_custom_field_multiselect(&self, field: &FieldType) -> bool {
        match field {
            FieldType::Custom(i) => {
                if let Some(custom_field) = self.custom_fields.get(i) {
                    if let CustomFieldVariant::MultiSelect { .. } = custom_field.variant {
                        return true;
                    }
                }
                return false;
            }
            _ => false,
        }
    }

    pub fn get_custom_field_option_key(
        &self,
        field: &FieldType,
        value: &str
    ) -> Result<String, RustlyzerError> {
        if let FieldType::Custom(i) = field {
           return match &self.custom_fields.get(i).ok_or(RustlyzerError::CustomFieldNotInRecords)?
                .variant {
                CustomFieldVariant::MultiSelect { options } |
                CustomFieldVariant::Dropdown { options } |
                CustomFieldVariant::Radio { options } => {
                    for (k,v) in options.iter() {
                       if v == value {
                           return Ok(k.to_owned());
                       }
                    }
                    return Err(RustlyzerError::KeyNotInOptions);
                    // Ok(options.get(key).ok_or
                    // (RustlyzerError::KeyNotInOptions)?.to_owned())
                }
                _ => Err(RustlyzerError::WrongArgument)
            }
        }
        return Err(RustlyzerError::WrongArgument);
    }

    pub fn get_custom_field_option_value(
        &self,
        field: &FieldType,
        key: &str,
    ) -> Result<String, RustlyzerError> {
        if let FieldType::Custom(i) = field {
            // if let CustomFieldVariant::MultiSelect { options } = &self
            //     .custom_fields
            //     .get(i)
            //     .ok_or(RustlyzerError::CustomFieldNotInRecords)?
            //     .variant
            // {
            //     return Ok(options
            //         .get(key)
            //         .ok_or(RustlyzerError::KeyNotInOptions)?
            //         .to_owned());
            // }
            // return Err(RustlyzerError::WrongArgument);
            return match &self.custom_fields.get(i).ok_or(RustlyzerError::CustomFieldNotInRecords)?
                .variant {
                CustomFieldVariant::MultiSelect { options } |
                CustomFieldVariant::Dropdown { options } |
                CustomFieldVariant::Radio { options } => {
                    Ok(options.get(key).ok_or
                    (RustlyzerError::KeyNotInOptions)?.to_owned())
                }
                _ => Err(RustlyzerError::WrongArgument)
            }
        }
        return Err(RustlyzerError::WrongArgument);
    }

    pub fn get_custom_field_option_keys(
        &self,
        field: &FieldType,
    ) -> Result<Vec<String>, RustlyzerError> {
        match field {
            FieldType::Custom(i) => {
                let variant = &self
                    .custom_fields
                    .get(i)
                    .ok_or(RustlyzerError::CustomFieldNotInRecords)?
                    .variant;
                match variant {
                    CustomFieldVariant::MultiSelect { options } |
                    CustomFieldVariant::Dropdown { options } |
                    CustomFieldVariant::Radio { options } => Ok(options
                        .keys()
                        .map(|k| k.to_owned())
                        .collect::<Vec<String>>()),
                    _ => Err(RustlyzerError::WrongArgument),
                }
            }
            _ => Err(RustlyzerError::WrongArgument),
        }
    }

    pub fn get_custom_field_option_values(&self, field: &FieldType) -> Result<Vec<String>,
        RustlyzerError> {
                match field {
            FieldType::Custom(i) => {
                let variant = &self
                    .custom_fields
                    .get(i)
                    .ok_or(RustlyzerError::CustomFieldNotInRecords)?
                    .variant;
                match variant {
                    CustomFieldVariant::MultiSelect { options } |
                    CustomFieldVariant::Dropdown { options } |
                    CustomFieldVariant::Radio { options } => Ok(options
                        .values()
                        .map(|k| k.to_owned())
                        .collect::<Vec<String>>()),
                    _ => Err(RustlyzerError::WrongArgument),
                }
            }
            _ => Err(RustlyzerError::WrongArgument),
        }
    }

    // pub fn get_custom_field
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct MetaInput {
    #[serde(rename = "Pages")]
    pub pages: Vec<Page>,
}

/// Page struct containing the metadata for the custom fields.
/// Metadata is contained in elements: Vec<CustomField>
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Page {
    #[serde(rename = "Elements")]
    pub elements: Vec<CustomField>,
}

/// CustomField variant containing the metadata for one custom field variant.
/// Can be of type: Dropdown, Radio, MultiSelect, Text, TextArea, Html
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "Type")]
pub enum CustomFieldVariant {
    /// Dropdown custom field.
    /// Value options are in options: Vec<FieldOption>
    #[serde(rename = "dropdown")]
    Dropdown {
        #[serde(
            rename = "Options",
            skip_serializing,
            deserialize_with = "options_as_map"
        )]
        options: IndexMap<String, String>,
    },
    /// Radio custom field.
    /// Value options are in options: Vec<FieldOption>
    #[serde(rename = "radio")]
    Radio {
        #[serde(
            rename = "Options",
            skip_serializing,
            deserialize_with = "options_as_map"
        )]
        options: IndexMap<String, String>,
    },
    /// MultiSelect custom field.
    /// Value options are in options: Vec<FieldOption>
    #[serde(rename = "checkbox")]
    MultiSelect {
        #[serde(
            rename = "Options",
            skip_serializing,
            deserialize_with = "options_as_map"
        )]
        options: IndexMap<String, String>,
    },
    /// Text custom field.
    #[serde(rename = "text")]
    Text,
    /// TextArea custom field.
    #[serde(rename = "textarea")]
    TextArea,
    /// Html custom field.
    #[serde(rename = "html")]
    Html {
        #[serde(rename = "Html")]
        html: String,
    },
}

fn options_as_map<'de, D>(de: D) -> Result<IndexMap<String, String>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let options = Vec::<CustomFieldOption>::deserialize(de)?;
    let options = options
        .into_iter()
        .map(|opt| (opt.value, opt.label))
        .collect::<Vec<(String, String)>>();
    Ok(IndexMap::from_iter(options.into_iter()))
}

/// CustomField containing the metadata for one custom field.
/// `key` - Key to identify field
/// `label` - Label for field
/// `required` - If field is required or not
/// `variant` - `CustomFieldVariant` for field.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CustomField {
    #[serde(rename = "QuestionKey")]
    pub key: String,
    #[serde(rename = "Label", default = "default_label")]
    pub label: String,
    #[serde(rename = "Required", default = "default_required")]
    pub required: bool,
    #[serde(flatten)]
    pub variant: CustomFieldVariant,
}

fn default_label() -> String {
    "".to_string()
}
fn default_required() -> bool {
    false
}

/// Option value set for aggregate fields.
/// Value is in value: String
/// Label is in label: String
#[derive(Serialize, Deserialize, Debug, Clone)]
struct CustomFieldOption {
    #[serde(rename = "Value")]
    value: String,
    #[serde(rename = "Label")]
    label: String,
}
