use super::Language;
use crate::helpers::EnumAttrs;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub(crate) enum PurchaseStatus {
    #[serde(rename = "purchased")]
    Purchased,
    // #[serde(rename = "not purchased")]
    // NotPurchased,
    #[serde(rename = "rejected")]
    Rejected,
    #[serde(rename = "evaluated")]
    Evaluated
}

impl EnumAttrs<PurchaseStatus> for PurchaseStatus {
    fn as_str(&self, lng: Language) -> &'static str {
        match lng {
            _ => {
                match self {
                    PurchaseStatus::Purchased => "Purchased",
                    PurchaseStatus::Rejected => "Rejected",
                    PurchaseStatus::Evaluated => "Evaluated"
                }
            }
        }
    }

    fn get_all() -> Vec<PurchaseStatus> {
        vec![
            PurchaseStatus::Purchased,
            PurchaseStatus::Rejected,
            PurchaseStatus::Evaluated
        ]
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub(crate) enum Gender {
    #[serde(alias = "male", alias = "男")]
    Male,
    #[serde(rename = "female", alias = "女")]
    Female,
}

impl EnumAttrs<Gender> for Gender {
    fn as_str(&self, lng: Language) -> &'static str {
        match lng {
            Language::Ja => {
                match self {
                   Gender::Male =>  "男性",
                    Gender::Female => "女性"
                }
            }
            Language::En | _ => {
                match self {
                    Gender::Male => "Male",
                    Gender::Female => "Female"
                }
            }
        }
    }

    fn get_all() -> Vec<Gender> {
        vec![
            Gender::Female,
            Gender::Male
        ]
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub(crate) enum MaritalStatus {
    #[serde(alias = "single", alias = "未婚")]
    Single,
    #[serde(alias = "married", alias = "既婚")]
    Married,
}

impl EnumAttrs<MaritalStatus> for MaritalStatus {
    fn as_str(&self, lng: Language) -> &'static str {
        match lng {
            Language::Ja => {
                match self {
                    MaritalStatus::Married => "既婚",
                    MaritalStatus::Single => "未婚"
                }
            }
            Language::En | _ => {
                match self {
                    MaritalStatus::Married => "Married",
                    MaritalStatus::Single => "Single"
                }
            }
        }
    }

    fn get_all() -> Vec<MaritalStatus> {
        vec![
            MaritalStatus::Married,
            MaritalStatus::Single
        ]
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub(crate) enum Job {
    #[serde(rename="専業主婦（主夫）")]
    FullTimeHousewife,
    #[serde(rename="パート・アルバイト")]
    PartTime,
    #[serde(rename="会社員（事務系）")]
    EmployeeOffice,
    #[serde(rename="会社員（その他）")]
    EmployeeOthers,
    #[serde(rename="会社員（技術系）")]
    EmployeeTech,
    #[serde(rename="無職")]
    Unemployed,
    #[serde(rename="学生")]
    Student,
    #[serde(rename="自営業")]
    SelfEmployed,
    #[serde(rename="自由業")]
    Freelancer,
    #[serde(rename="公務員")]
    CivilServant,
    #[serde(rename="経営者・役員")]
    Entrepreneur,
    #[serde(rename="その他")]
    Others
}

impl EnumAttrs<Job> for Job {
    // fn get_all_string(lng: Language) -> Vec<String> {
    //     Job::get_all().into_iter().map(|x| x.as_str(lng).to_string()).collect()
    // }

    fn get_all() -> Vec<Job> {
       vec![
           Job::FullTimeHousewife,
           Job::PartTime,
           Job::EmployeeOffice,
           Job::EmployeeOthers,
           Job::EmployeeTech,
           Job::Unemployed,
           Job::Student,
           Job::SelfEmployed,
           Job::Freelancer,
           Job::CivilServant,
           Job::Entrepreneur,
           Job::Others,
       ]
    }

    fn as_str(&self, lng: Language) -> &'static str {
        match lng {
            _ => match self {
                Job::FullTimeHousewife => "専業主婦（主夫）",
                Job::PartTime => "パート・アルバイト",
                Job::EmployeeOffice => "会社員（事務系）",
                Job::EmployeeOthers => "会社員（その他）",
                Job::EmployeeTech => "会社員（技術系）",
                Job::Unemployed => "無職",
                Job::Student => "学生",
                Job::SelfEmployed => "自営業",
                Job::Freelancer => "自由業",
                Job::CivilServant => "公務員",
                Job::Entrepreneur => "経営者・役員",
                Job::Others => "その他",
            }
        }
    }
}

// Extra enums
pub enum AgeRange1060 {
    Under10s,
    Group20s,
    Group30s,
    Group40s,
    Group50s,
    Above60s
}

impl EnumAttrs<AgeRange1060> for AgeRange1060 {
    fn as_str(&self, lng: Language) -> &'static str {
        match lng {
            _ => {
                match self {
                    AgeRange1060::Under10s => "10代以下",
                    AgeRange1060::Group20s => "20代",
                    AgeRange1060::Group30s => "30代",
                    AgeRange1060::Group40s => "40代",
                    AgeRange1060::Group50s => "50代",
                    AgeRange1060::Above60s => "60代以上",
                }
            }
        }
    }

    fn get_all() -> Vec<AgeRange1060> {
        vec![
            AgeRange1060::Under10s,
            AgeRange1060::Group20s,
            AgeRange1060::Group30s,
            AgeRange1060::Group40s,
            AgeRange1060::Group50s,
            AgeRange1060::Above60s,
        ]
    }
}

pub enum AgeRange1070 {
    Under10s,
    Group20s,
    Group30s,
    Group40s,
    Group50s,
    Group60s,
    Above70s
}

impl EnumAttrs<AgeRange1070> for AgeRange1070 {
    fn as_str(&self, lng: Language) -> &'static str {
        match lng {
            _ => {
                match self {
                    AgeRange1070::Under10s => "10代以下",
                    AgeRange1070::Group20s => "20代",
                    AgeRange1070::Group30s => "30代",
                    AgeRange1070::Group40s => "40代",
                    AgeRange1070::Group50s => "50代",
                    AgeRange1070::Group60s => "60代",
                    AgeRange1070::Above70s => "70代以上",
                }
            }
        }
    }

    fn get_all() -> Vec<AgeRange1070> {
        vec![
            AgeRange1070::Under10s,
            AgeRange1070::Group20s,
            AgeRange1070::Group30s,
            AgeRange1070::Group40s,
            AgeRange1070::Group50s,
            AgeRange1070::Group60s,
            AgeRange1070::Above70s,
        ]
    }
}

pub(crate) enum ChildrenRange {
    Group0,
    Group1,
    Group2,
    Group3,
    Above4
}

impl EnumAttrs<ChildrenRange> for ChildrenRange {
    fn as_str(&self, lng: Language) -> &'static str {
        match lng {
            _ => match self {
                ChildrenRange::Group0 => "0人",
                ChildrenRange::Group1 => "1人",
                ChildrenRange::Group2 => "2人",
                ChildrenRange::Group3 => "3人",
                ChildrenRange::Above4 => "4人以上",
            }
        }
    }

    fn get_all() -> Vec<ChildrenRange> {
        vec![
            ChildrenRange::Group0,
            ChildrenRange::Group1,
            ChildrenRange::Group2,
            ChildrenRange::Group3,
            ChildrenRange::Above4,
        ]
    }
}

pub(crate) enum YearlyIncomeRange {
    Below1Mil,
    Group1To2Mil,
    Group2To3Mil,
    Group3To4Mil,
    Group4To5Mil,
    Group5To6Mil,
    Group6To7Mil,
    Group7To8Mil,
    Group8To9Mil,
    Group9To10Mil,
    Group10To12Mil,
    Group12To15Mil,
    Group15To20Mil,
    Above20Mil
}

impl EnumAttrs<YearlyIncomeRange> for YearlyIncomeRange {
    fn as_str(&self, lng: Language) -> &'static str {
        match lng {
            _ => match self {
                YearlyIncomeRange::Below1Mil => "100万円未満",
                YearlyIncomeRange::Group1To2Mil => "100～200万円未満",
                YearlyIncomeRange::Group2To3Mil => "200～300万円未満",
                YearlyIncomeRange::Group3To4Mil => "300～400万円未満",
                YearlyIncomeRange::Group4To5Mil => "400～500万円未満",
                YearlyIncomeRange::Group5To6Mil => "500～600万円未満",
                YearlyIncomeRange::Group6To7Mil => "600～700万円未満",
                YearlyIncomeRange::Group7To8Mil => "700～800万円未満",
                YearlyIncomeRange::Group8To9Mil => "800～900万円未満",
                YearlyIncomeRange::Group9To10Mil => "900～1000万円未満",
                YearlyIncomeRange::Group10To12Mil => "1000～1200万円未満",
                YearlyIncomeRange::Group12To15Mil => "1200～1500万円未満",
                YearlyIncomeRange::Group15To20Mil => "1500～2000万円未満",
                YearlyIncomeRange::Above20Mil => "2000万円以上",
            }
        }
    }

    fn get_all() -> Vec<YearlyIncomeRange> {
        vec![
            YearlyIncomeRange::Below1Mil,
            YearlyIncomeRange::Group1To2Mil,
            YearlyIncomeRange::Group2To3Mil,
            YearlyIncomeRange::Group3To4Mil,
            YearlyIncomeRange::Group4To5Mil,
            YearlyIncomeRange::Group5To6Mil,
            YearlyIncomeRange::Group6To7Mil,
            YearlyIncomeRange::Group7To8Mil,
            YearlyIncomeRange::Group8To9Mil,
            YearlyIncomeRange::Group9To10Mil,
            YearlyIncomeRange::Group10To12Mil,
            YearlyIncomeRange::Group12To15Mil,
            YearlyIncomeRange::Group15To20Mil,
            YearlyIncomeRange::Above20Mil
        ]
    }
}