use super::data::enums::{PurchaseStatus, MaritalStatus, Gender, ChildrenRange, Job, AgeRange1070,
                     YearlyIncomeRange};
use super::data::enums::places::Prefecture;
use std::collections::{HashMap, hash_map::Entry};
use crate::helpers::EnumAttrs;
use crate::ds::field::FieldType::{YearlyIncome, Age};
use crate::ds::data::enums::PurchaseStatus::Purchased;
use crate::errors::RustlyzerError;
use std::hash::Hash;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone)]
pub struct DataSetConfig {
    pub lng: Language,
    pub created_year: u16,
    pub includes: Vec<IncludeCriteria>,
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
    ///     - Possible values: [("purchase-status", "evaluated")]
    pub fn new_with_includes(language: String, created_year: u16, ignores: Vec<(String, String)>) ->
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
        Ok(DataSetConfig { lng, created_year, includes: Self::includes_from_vec(ignores, lng)? })
    }

    pub fn new(language: String, created_year: u16) -> Result<Self, RustlyzerError> {
        Self::new_with_includes(language, created_year, Vec::<(String, String)>::new())
    }

    fn includes_from_vec(tups: Vec<(String, String)>, lng: Language) ->
                                                                    Result<Vec<IncludeCriteria>,
                                                                        RustlyzerError> {
       let mut includes = Vec::<IncludeCriteria>::new();
        for (cat, val) in tups.into_iter() {
            match cat {
                p if p == PurchaseStatus::id_name_of_enum() => {
                    includes.push(IncludeCriteria::PurchaseStatusInclude
                        (PurchaseStatus::from_display_name(val.as_str(), lng)?));
                },
                m if m == MaritalStatus::id_name_of_enum() => {
                    includes.push(IncludeCriteria::MaritalStatusInclude
                        (MaritalStatus::from_display_name(val.as_str(), lng)?));
                },
                g if g == Gender::id_name_of_enum() => {
                    includes.push(IncludeCriteria::GenderInclude
                        (Gender::from_display_name(val.as_str(), lng)?));
                },
                c if c == ChildrenRange::id_name_of_enum() => {
                    includes.push(IncludeCriteria::ChildrenRangeInclude
                        (ChildrenRange::from_display_name(val.as_str(), lng)?))
                },
                j if j == Job::id_name_of_enum() => {
                    includes.push(IncludeCriteria::JobInclude
                        (Job::from_display_name(val.as_str(), lng)?));
                },
                a if a == AgeRange1070::id_name_of_enum() => {
                    includes.push(IncludeCriteria::AgeRange1070Include
                        (AgeRange1070::from_display_name(val.as_str(), lng)?));
                },
                y if y == YearlyIncomeRange::id_name_of_enum() => {
                    includes.push(IncludeCriteria::YearlyIncomeRangeInclude
                        (YearlyIncomeRange::from_display_name(val.as_str(), lng)?));
                },
                pr if pr == Prefecture::id_name_of_enum() => {
                    includes.push(IncludeCriteria::PrefectureInclude
                        (Prefecture::from_display_name(val.as_str(), lng)?));
                },
                x => {
                    return Err(RustlyzerError::InvalidConfigItemError(x.to_string()));
                }
            }
        }
        Ok(includes)
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
pub enum IncludeCriteria {
    PurchaseStatusInclude(PurchaseStatus),
    MaritalStatusInclude(MaritalStatus),
    GenderInclude(Gender),
    ChildrenRangeInclude(ChildrenRange),
    JobInclude(Job),
    AgeRange1070Include(AgeRange1070),
    YearlyIncomeRangeInclude(YearlyIncomeRange),
    PrefectureInclude(Prefecture)
}

impl IncludeCriteria {
    pub(crate) fn map_from_vec(list: &Vec<Self>) -> HashMap<String, Vec<IncludeCriteria>> {
        let mut map = HashMap::<String, Vec<IncludeCriteria>>::new();
        for item in list.iter() {
            let mut entry = match map.entry(item.cat_as_str()) {
                Entry::Occupied(o) => o.into_mut(),
                Entry::Vacant(v) => v.insert(
                    Vec::<IncludeCriteria>::new()
                )
            };
            // let mut entry_op = map.get_mut(item.cat_as_str());
            // match entry_op {
            //     None => map.
            // }
            // let entry = entry_op.get_or_insert(Vec::<IncludeCriteria>::new());
            entry.push(item.to_owned());
        }
        map
    }

    pub(crate) fn cat_as_str(&self) -> String {
        let res =match self {
            IncludeCriteria::PurchaseStatusInclude(_) => "PurchasedStatusInclude",
            IncludeCriteria::MaritalStatusInclude(_) => "MaritalStatusInclude",
            IncludeCriteria::GenderInclude(_) => "GenderInclude",
            IncludeCriteria::ChildrenRangeInclude(_) => "ChildrenRangeInclude",
            IncludeCriteria::JobInclude(_) => "JobInclude",
            IncludeCriteria::AgeRange1070Include(_) => "AgeRange1070Include",
            IncludeCriteria::YearlyIncomeRangeInclude(_) => "YearlyIncomeRangeInclude",
            IncludeCriteria::PrefectureInclude(_) => "PrefectureInclude"
        };
        res.to_string()
    }
}

// Constants API
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all="camelCase")]
pub struct IncludeCriteriaItem {
    pub enum_name: String,
    pub variants: Vec<String>,
}

impl IncludeCriteriaItem {
    fn new(enum_name: String, variants: Vec<String>) -> Self {
        IncludeCriteriaItem {enum_name, variants }
    }
}
pub fn provide_include_criteria_constants() -> Vec<(String, HashMap<String, IncludeCriteriaItem>)>{
    const lngs: [Language;2] = [Language::En, Language::Ja];
    let mut available_include_criteria_items = Vec::<(String, HashMap<String,
        IncludeCriteriaItem>)>::new();
    // Purchase Status
    let mut en_map = HashMap::<String, IncludeCriteriaItem>::new();
    for &lng in lngs.iter() {
        en_map.insert(
            lng.as_string(Language::En),
            IncludeCriteriaItem::new(
                PurchaseStatus::display_name_of_enum(lng),
                PurchaseStatus::get_all_display_names(lng) 
            )
        );
    }
    available_include_criteria_items.push(
        ( PurchaseStatus::id_name_of_enum(), en_map )
    );
    // Marital Status
    let mut en_map = HashMap::<String, IncludeCriteriaItem>::new();
    for &lng in lngs.iter() {
        en_map.insert(
            lng.as_string(Language::En),
            IncludeCriteriaItem::new(
                MaritalStatus::display_name_of_enum(lng),
                MaritalStatus::get_all_display_names(lng)
            )
        );
    }
    available_include_criteria_items.push(
        (MaritalStatus::id_name_of_enum(), en_map)
    );
    // Gender 
    let mut en_map = HashMap::<String, IncludeCriteriaItem>::new();
    for &lng in lngs.iter() {
        en_map.insert(
            lng.as_string(Language::En),
            IncludeCriteriaItem::new(
                Gender::display_name_of_enum(lng),
                Gender::get_all_display_names(lng)
            )
        );
    }
    available_include_criteria_items.push(
        (Gender::id_name_of_enum(), en_map)
    );
    // Children Range
    let mut en_map = HashMap::<String, IncludeCriteriaItem>::new();
    for &lng in lngs.iter() {
        en_map.insert(
            lng.as_string(Language::En),
            IncludeCriteriaItem::new(
                ChildrenRange::display_name_of_enum(lng),
                ChildrenRange::get_all_display_names(lng)
            )
        );
    }
    available_include_criteria_items.push(
        (ChildrenRange::id_name_of_enum(), en_map)
    );
    // Job
    let mut en_map = HashMap::<String, IncludeCriteriaItem>::new();
    for &lng in lngs.iter() {
        en_map.insert(
            lng.as_string(Language::En),
            IncludeCriteriaItem::new(
                Job::display_name_of_enum(lng),
                Job::get_all_display_names(lng)
            )
        );
    }
    available_include_criteria_items.push(
        (Job::id_name_of_enum(), en_map)
    );
    // Age Range 1070
    let mut en_map = HashMap::<String, IncludeCriteriaItem>::new();
    for &lng in lngs.iter() {
        en_map.insert(
            lng.as_string(Language::En),
            IncludeCriteriaItem::new(
                AgeRange1070::display_name_of_enum(lng),
                AgeRange1070::get_all_display_names(lng)
            )
        );
    }
    available_include_criteria_items.push(
        (AgeRange1070::id_name_of_enum(), en_map)
    );
    // Yearly Income
    let mut en_map = HashMap::<String, IncludeCriteriaItem>::new();
    for &lng in lngs.iter() {
        en_map.insert(
            lng.as_string(Language::En),
            IncludeCriteriaItem::new(
                YearlyIncomeRange::display_name_of_enum(lng),
                YearlyIncomeRange::get_all_display_names(lng)
            )
        );
    }
    available_include_criteria_items.push(
        (YearlyIncomeRange::id_name_of_enum(), en_map)
    );
    // Prefecture
    let mut en_map = HashMap::<String, IncludeCriteriaItem>::new();
    for &lng in lngs.iter() {
        en_map.insert(
            lng.as_string(Language::En),
            IncludeCriteriaItem::new(
                Prefecture::display_name_of_enum(lng),
                Prefecture::get_all_display_names(lng)
            )
        );
    }
    available_include_criteria_items.push(
        (Prefecture::id_name_of_enum(), en_map)
    );
    available_include_criteria_items
}

