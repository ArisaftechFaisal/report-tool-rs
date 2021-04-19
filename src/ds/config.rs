use super::data::enums::{PurchaseStatus, MaritalStatus, Gender, ChildrenRange, Job, AgeRange1070,
                     YearlyIncomeRange};
use super::data::enums::places::Prefecture;

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
    pub fn new(language: String, created_year: u16) -> Self {
        const LNG_JA: &'static str = "ja";
        const LNG_EN: &'static str = "en";

        let lng: Language;
        match language.as_ref() {
            LNG_JA => lng = Language::Ja,
            _ => lng = Language::En,
        }
        // let created_date = chrono::DateTime::parse_from_str(format!("{} Jan 01", year), "%Y %b %d").unwrap();
        DataSetConfig { lng, created_year, ignores: Vec::<IgnoreCriteria>::new() }
    }

    fn ignores_from_vec(tups: Vec<(String, String)>) -> Vec<IgnoreCriteria> {
       let mut ignores = Vec::<IgnoreCriteria>::new();
        // tups.into_iter().for_each(move|(cat, val)|  {
        //   match cat.as_str() {
        //      "PurchaseStatus" =>
        //   }
        // });
        Vec::<IgnoreCriteria>::new()
    }
}

#[derive(Debug, Copy, Clone)]
pub enum Language {
    En,
    Ja,
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

// Constants
// const IgnoreCriteriaDict