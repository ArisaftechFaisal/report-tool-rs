#[derive(Debug, Clone, Copy)]
pub struct DataSetConfig {
    pub lng: Language,
    pub created_year: u16,
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