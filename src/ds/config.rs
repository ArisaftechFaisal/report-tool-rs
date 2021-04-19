use super::data::enums::{PurchaseStatus, MaritalStatus, Gender, ChildrenRange, Job, AgeRange1070,
                     YearlyIncomeRange};
use super::data::enums::places::Prefecture;
use std::collections::HashMap;
use crate::helpers::EnumAttrs;
use crate::ds::field::FieldType::{YearlyIncome, Age};
use crate::ds::data::enums::PurchaseStatus::Purchased;
use crate::errors::RustlyzerError;

#[derive(Debug, Clone)]
pub struct DataSetConfig {
    pub lng: Language,
    pub created_year: u16,
    pub ignores: Vec<IgnoreCriteria>,
}

impl DataSetConfig {
    /// Create config to be used for DataSet operations.
    ///
    /// ### Arguments
    ///
    /// - `language` - Desired language for data representation
    ///     - Possible values: "en", "ja"
    ///     - Incorrect values will result in "en"
    /// - `created_year` - Year the data was collected
    ///     - Input data cannot have birthdate after the created_year
    /// - `ignores` - Data to be ignored
    ///     - Formatted as a list of tuples
    ///     - Possible values: [("PurchaseStatus", "evaluated")]
    pub fn new_with_ignores(language: String, created_year: u16, ignores: Vec<(String, String)>) ->
                                                                                     Result<Self,
        RustlyzerError> {
        const LNG_JA: &'static str = "ja";
        const LNG_EN: &'static str = "en";

        let lng: Language;
        match language.as_ref() {
            LNG_JA => lng = Language::Ja,
            _ => lng = Language::En,
        }
        // let created_date = chrono::DateTime::parse_from_str(format!("{} Jan 01", year), "%Y %b %d").unwrap();
        Ok(DataSetConfig { lng, created_year, ignores: Self::ignores_from_vec(ignores, lng)? })
    }

    pub fn new(language: String, created_year: u16) -> Result<Self, RustlyzerError> {
        Self::new_with_ignores(language, created_year, Vec::<(String, String)>::new())
    }

    fn ignores_from_vec(tups: Vec<(String, String)>, lng: Language) ->
                                                                    Result<Vec<IgnoreCriteria>,
                                                                        RustlyzerError> {
       let mut ignores = Vec::<IgnoreCriteria>::new();
        for (cat, val) in tups.into_iter() {
            match cat {
                p if p == PurchaseStatus::display_name_of_enum(Language::En) => {
                    ignores.push(IgnoreCriteria::PurchaseStatus
                        (PurchaseStatus::from_display_name(val.as_str(), lng)?));
                },
                m if m == MaritalStatus::display_name_of_enum(Language::En) => {
                    ignores.push(IgnoreCriteria::MaritalStatus
                        (MaritalStatus::from_display_name(val.as_str(), lng)?));
                },
                g if g == Gender::display_name_of_enum(Language::En) => {
                    ignores.push(IgnoreCriteria::Gender
                        (Gender::from_display_name(val.as_str(), lng)?));
                },
                c if c == ChildrenRange::display_name_of_enum(Language::En) => {
                    ignores.push(IgnoreCriteria::ChildrenRange
                        (ChildrenRange::from_display_name(val.as_str(), lng)?))
                },
                j if j == Job::display_name_of_enum(Language::En) => {
                    ignores.push(IgnoreCriteria::Job
                        (Job::from_display_name(val.as_str(), lng)?));
                },
                a if a == AgeRange1070::display_name_of_enum(Language::En) => {
                    ignores.push(IgnoreCriteria::AgeRange1070
                        (AgeRange1070::from_display_name(val.as_str(), lng)?));
                },
                y if y == YearlyIncomeRange::display_name_of_enum(Language::En) => {
                    ignores.push(IgnoreCriteria::YearlyIncomeRange
                        (YearlyIncomeRange::from_display_name(val.as_str(), lng)?));
                },
                pr if pr == Prefecture::display_name_of_enum(Language::En) => {
                    ignores.push(IgnoreCriteria::Prefecture
                        (Prefecture::from_display_name(val.as_str(), lng)?));
                },
                x => {
                    return Err(RustlyzerError::InvalidConfigItemError(x.to_string()));
                }
            }
        }
        Ok(ignores)
    }
}

#[derive(Debug, Copy, Clone)]
pub enum Language {
    En,
    Ja,
}

impl EnumAttrs for Language {
    fn as_str(&self, lng: Language) -> &'static str {
        match lng {
            _ => match self {
                Language::En => "en",
                Language::Ja => "ja"
            }
        }
    }

    fn get_all() -> Vec<Self> {
        vec![Language::En, Language::Ja]
    }

    fn display_name_of_enum(lng: Language) -> String {
        let res = match lng {
           Language::En => "Language",
            Language::Ja => "言語"
        };
        res.to_string()
    }
}

#[derive(Debug, Copy, Clone)]
pub enum IgnoreCriteria {
    PurchaseStatus(PurchaseStatus),
    MaritalStatus(MaritalStatus),
    Gender(Gender),
    ChildrenRange(ChildrenRange),
    Job(Job),
    AgeRange1070(AgeRange1070),
    YearlyIncomeRange(YearlyIncomeRange),
    Prefecture(Prefecture)
}

// Constants API
pub struct IgnoreCriteriaItem {
    pub enum_name: String,
    pub variants: Vec<String>,
}

impl IgnoreCriteriaItem {
    fn new(enum_name: String, variants: Vec<String>) -> Self {
        IgnoreCriteriaItem {enum_name, variants }
    }
}
pub fn provide_ignore_criteria_constants() {
    const lngs: [Language;2] = [Language::En, Language::Ja];
    let mut available_ignore_criteria_items = HashMap::<String, HashMap<String,
        IgnoreCriteriaItem>>::new();
    // Purchase Status
    let mut en_map = HashMap::<String, IgnoreCriteriaItem>::new();
    for &lng in lngs.iter() {
        en_map.insert(
            lng.as_string(Language::En),
            IgnoreCriteriaItem::new(
                PurchaseStatus::display_name_of_enum(lng),
                PurchaseStatus::get_all_display_names(lng) 
            )
        );
    }
    available_ignore_criteria_items.insert(
        PurchaseStatus::display_name_of_enum(Language::En),
        en_map
    );
    // Marital Status
    let mut en_map = HashMap::<String, IgnoreCriteriaItem>::new();
    for &lng in lngs.iter() {
        en_map.insert(
            lng.as_string(Language::En),
            IgnoreCriteriaItem::new(
                MaritalStatus::display_name_of_enum(lng),
                MaritalStatus::get_all_display_names(lng)
            )
        );
    }
    available_ignore_criteria_items.insert(
        MaritalStatus::display_name_of_enum(Language::En),
        en_map
    );
    // Gender 
    let mut en_map = HashMap::<String, IgnoreCriteriaItem>::new();
    for &lng in lngs.iter() {
        en_map.insert(
            lng.as_string(Language::En),
            IgnoreCriteriaItem::new(
                Gender::display_name_of_enum(lng),
                Gender::get_all_display_names(lng)
            )
        );
    }
    available_ignore_criteria_items.insert(
        Gender::display_name_of_enum(Language::En),
        en_map
    );
    // Children Range
    let mut en_map = HashMap::<String, IgnoreCriteriaItem>::new();
    for &lng in lngs.iter() {
        en_map.insert(
            lng.as_string(Language::En),
            IgnoreCriteriaItem::new(
                ChildrenRange::display_name_of_enum(lng),
                ChildrenRange::get_all_display_names(lng)
            )
        );
    }
    available_ignore_criteria_items.insert(
        ChildrenRange::display_name_of_enum(Language::En),
        en_map
    );
    // Job
    let mut en_map = HashMap::<String, IgnoreCriteriaItem>::new();
    for &lng in lngs.iter() {
        en_map.insert(
            lng.as_string(Language::En),
            IgnoreCriteriaItem::new(
                Job::display_name_of_enum(lng),
                Job::get_all_display_names(lng)
            )
        );
    }
    available_ignore_criteria_items.insert(
        Job::display_name_of_enum(Language::En),
        en_map
    );
    // Age Range 1070
    let mut en_map = HashMap::<String, IgnoreCriteriaItem>::new();
    for &lng in lngs.iter() {
        en_map.insert(
            lng.as_string(Language::En),
            IgnoreCriteriaItem::new(
                AgeRange1070::display_name_of_enum(lng),
                AgeRange1070::get_all_display_names(lng)
            )
        );
    }
    available_ignore_criteria_items.insert(
        AgeRange1070::display_name_of_enum(Language::En),
        en_map
    );
    // Yearly Income
    let mut en_map = HashMap::<String, IgnoreCriteriaItem>::new();
    for &lng in lngs.iter() {
        en_map.insert(
            lng.as_string(Language::En),
            IgnoreCriteriaItem::new(
                YearlyIncomeRange::display_name_of_enum(lng),
                YearlyIncomeRange::get_all_display_names(lng)
            )
        );
    }
    available_ignore_criteria_items.insert(
        YearlyIncomeRange::display_name_of_enum(Language::En),
        en_map
    );
    // Prefecture
    let mut en_map = HashMap::<String, IgnoreCriteriaItem>::new();
    for &lng in lngs.iter() {
        en_map.insert(
            lng.as_string(Language::En),
            IgnoreCriteriaItem::new(
                Prefecture::display_name_of_enum(lng),
                Prefecture::get_all_display_names(lng)
            )
        );
    }
    available_ignore_criteria_items.insert(
        Prefecture::display_name_of_enum(Language::En),
        en_map
    );
}

