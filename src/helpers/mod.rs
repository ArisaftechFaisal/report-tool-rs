use crate::ds::Language;
use std::fmt::Debug;
use crate::errors::RustlyzerError;

pub trait CustomHelpers<T> {
    fn below_half(&self, exp: u8) -> bool;
    fn above_zero(&self, exp: u8) -> bool;
}

pub trait Round<T>: CustomHelpers<T> {
    fn ceil(&self, sfig: u8) -> T;
    fn floor(&self, sfig: u8) -> T;
}

impl Round<u64> for u64 {
    fn ceil(&self, sfig: u8) -> u64 {
        let exp = self.to_string().len() as u8 - sfig;
        let unit = 10u64.pow(exp as u32);
        let mut res: u64 = (*self / unit) * unit;
        if self.above_zero(exp) {
            res += unit;
        }
        res
    }

    fn floor(&self, sfig: u8) -> u64 {
        let exp = self.to_string().len() as u8 - sfig;
        let unit = 10u64.pow(exp as u32);
        (*self / unit) * unit
    }
}

impl CustomHelpers<u64> for u64 {
    fn below_half(&self, exp: u8) -> bool {
        ((*self % 10u64.pow(exp as u32)) as f64 / 10u64.pow(exp as u32) as f64) < 0.5
    }

    fn above_zero(&self, exp: u8) -> bool {
        *self % 10u64.pow(exp as u32) > 0
    }
}

pub trait ExtractFromStr<T> {
    fn extract_from_str(cont: &str) -> Option<T>;
}

impl ExtractFromStr<usize> for usize {
    fn extract_from_str(cont: &str) -> Option<usize> {
        let mut string = cont.to_owned();
        string.retain(|c: char| c.is_numeric());
        if let Ok(res) = string.parse::<usize>() {
            Some(res)
        } else {
            None
        }
    }
}

pub trait EnumAttrs<T: EnumAttrs<T> + Sized>: Clone
{
    fn as_str(&self, lng: Language) -> &'static str;
    fn get_all() -> Vec<Self>;
    fn get_all_str(lng: Language) -> Vec<&'static str> {
        Self::get_all().into_iter().map(|x| x.as_str(lng)).collect()
    }
    fn as_string(&self, lng: Language) -> String {
        self.as_str(lng).to_string()
    }
    fn get_all_string(lng: Language) -> Vec<String> {
        Self::get_all().into_iter().map(|x| x.as_string(lng)).collect()
    }

    fn display_name_of_enum(lng: Language) -> String;
    fn display_name(&self, lng: Language) -> String {
        self.as_string(lng)
    }

    fn get_all_display_names(lng: Language) -> Vec<String> {
        let all: Vec<Self> = Self::get_all();
        all.into_iter().map(|this| this.display_name(lng)).collect::<Vec<String>>()
    }
    fn from_display_name(name: &str, lng: Language) -> Result<Self, RustlyzerError> {
       let all = Self::get_all();
        for variant in all.iter() {
            if variant.display_name(lng).as_str() == name {
                return Ok(variant.to_owned());
            }
        }
        Err(RustlyzerError::InvalidConfigError {
            config_item: Self::display_name_of_enum(lng),
            val: name.to_string(),
            expected_values: Self::get_all_display_names(lng)
        })
    }
}
