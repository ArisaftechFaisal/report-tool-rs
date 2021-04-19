use super::Language;
use core::fmt::Formatter;
use serde::{Deserialize, Serialize};
use std::fmt;
use crate::helpers::EnumAttrs;

#[derive(Debug, PartialOrd, PartialEq, Copy, Clone)]
pub enum Region {
    Hokkaido,
    Tohoku,
    Kanto,
    Chubu,
    Kansai,
    Chugoku,
    Shikoku,
    Kyushu,
}

#[derive(Debug, Serialize, Deserialize, Copy, Clone)]
pub enum Prefecture {
    #[serde(rename = "愛知県")]
    Aichi,
    #[serde(rename = "秋田県")]
    Akita,
    #[serde(rename = "青森県")]
    Aomori,
    #[serde(rename = "千葉県")]
    Chiba,
    #[serde(rename = "愛媛県")]
    Ehime,
    #[serde(rename = "福井県")]
    Fukui,
    #[serde(rename = "福岡県")]
    Fukuoka,
    #[serde(rename = "福島県")]
    Fukushima,
    #[serde(rename = "岐阜県")]
    Gifu,
    #[serde(rename = "群馬県")]
    Gunma,
    #[serde(rename = "広島県")]
    Hiroshima,
    #[serde(rename = "北海道")]
    Hokkaido,
    #[serde(rename = "兵庫県")]
    Hyogo,
    #[serde(rename = "茨城県")]
    Ibaraki,
    #[serde(rename = "石川県")]
    Ishikawa,
    #[serde(rename = "岩手県")]
    Iwate,
    #[serde(rename = "香川県")]
    Kagawa,
    #[serde(rename = "鹿児島県")]
    Kagoshima,
    #[serde(rename = "神奈川県")]
    Kanagawa,
    #[serde(rename = "高知県")]
    Kochi,
    #[serde(rename = "熊本県")]
    Kumamoto,
    #[serde(rename = "京都府")]
    Kyoto,
    #[serde(rename = "三重県")]
    Mie,
    #[serde(rename = "宮城県")]
    Miyagi,
    #[serde(rename = "宮崎県")]
    Miyazaki,
    #[serde(rename = "長野県")]
    Nagano,
    #[serde(rename = "長崎県")]
    Nagasaki,
    #[serde(rename = "奈良県")]
    Nara,
    #[serde(rename = "新潟県")]
    Nigata,
    #[serde(rename = "大分県")]
    Oita,
    #[serde(rename = "岡山県")]
    Okayama,
    #[serde(rename = "沖縄県")]
    Okinawa,
    #[serde(rename = "大阪府")]
    Osaka,
    #[serde(rename = "佐賀県")]
    Saga,
    #[serde(rename = "埼玉県")]
    Saitama,
    #[serde(rename = "滋賀県")]
    Shiga,
    #[serde(rename = "島根県")]
    Shimane,
    #[serde(rename = "静岡県")]
    Shizuoka,
    #[serde(rename = "栃木県")]
    Tochigi,
    #[serde(rename = "徳島県")]
    Tokushima,
    #[serde(rename = "東京都")]
    Tokyo,
    #[serde(rename = "鳥取県")]
    Tottori,
    #[serde(rename = "富山県")]
    Toyama,
    #[serde(rename = "和歌山県")]
    Wakayama,
    #[serde(rename = "山形県")]
    Yamagata,
    #[serde(rename = "山口県")]
    Yamaguchi,
    #[serde(rename = "山梨県")]
    Yamanashi,
}

impl EnumAttrs for Region {
    fn as_str(&self, lng: Language) -> &'static str {
        match lng {
            Language::Ja => match self {
                Region::Hokkaido => "北海道",
                Region::Tohoku => "東北",
                Region::Kanto => "関東",
                Region::Chubu => "中部",
                Region::Kansai => "関西",
                Region::Chugoku => "中国",
                Region::Shikoku => "四国",
                Region::Kyushu => "九州",
            },
            Language::En => match self {
                Region::Hokkaido => "Hokkaido",
                Region::Tohoku => "Tohoku",
                Region::Kanto => "Kanto",
                Region::Chubu => "Chubu",
                Region::Kansai => "Kansai",
                Region::Chugoku => "Chugoku",
                Region::Shikoku => "Shikoku",
                Region::Kyushu => "Kyushu",
            },
        }
    }

    fn get_all() -> Vec<Region> {
        vec![
            Region::Kanto,
            Region::Kansai,
            Region::Chubu,
            Region::Kyushu,
            Region::Chugoku,
            Region::Tohoku,
            Region::Hokkaido,
            Region::Shikoku,
        ]
    }

    fn display_name_of_enum(lng: Language) -> String {
        let res = match lng {
            Language::En => "Region",
            Language::Ja => "地域",
        };
        res.to_string()
    }
    // fn get_all_string(lng: Language) -> Vec<String> {
    //     Region::get_all().into_iter().map(|x| x.as_str(lng)).collect()
    // }
}

impl Region{
    pub fn from_prefecture(prefecture: &Prefecture) -> Self {
        match prefecture {
            Prefecture::Hokkaido => Region::Hokkaido,
            Prefecture::Akita
            | Prefecture::Aomori
            | Prefecture::Fukushima
            | Prefecture::Iwate
            | Prefecture::Miyagi
            | Prefecture::Yamagata => Region::Tohoku,
            Prefecture::Chiba
            | Prefecture::Gunma
            | Prefecture::Ibaraki
            | Prefecture::Kanagawa
            | Prefecture::Saitama
            | Prefecture::Tochigi
            | Prefecture::Tokyo => Region::Kanto,
            Prefecture::Aichi
            | Prefecture::Fukui
            | Prefecture::Gifu
            | Prefecture::Ishikawa
            | Prefecture::Nagano
            | Prefecture::Nigata
            | Prefecture::Shizuoka
            | Prefecture::Toyama
            | Prefecture::Yamanashi => Region::Chubu,
            Prefecture::Hyogo
            | Prefecture::Kyoto
            | Prefecture::Mie
            | Prefecture::Nara
            | Prefecture::Osaka
            | Prefecture::Shiga
            | Prefecture::Wakayama => Region::Kansai,
            Prefecture::Hiroshima
            | Prefecture::Okayama
            | Prefecture::Shimane
            | Prefecture::Tottori
            | Prefecture::Yamaguchi => Region::Chugoku,
            Prefecture::Ehime | Prefecture::Kagawa | Prefecture::Kochi | Prefecture::Tokushima => {
                Region::Shikoku
            }
            Prefecture::Fukuoka
            | Prefecture::Kagoshima
            | Prefecture::Kumamoto
            | Prefecture::Miyazaki
            | Prefecture::Nagasaki
            | Prefecture::Oita
            | Prefecture::Okinawa
            | Prefecture::Saga => Region::Kyushu,
        }
    }
}

impl EnumAttrs for Prefecture {
    fn as_str(&self, lng: Language) -> &'static str {
        match lng {
            Language::Ja => match self {
                Prefecture::Aichi => "愛知県",
                Prefecture::Akita => "秋田県",
                Prefecture::Aomori => "青森県",
                Prefecture::Chiba => "千葉県",
                Prefecture::Ehime => "愛媛県",
                Prefecture::Fukui => "福井県",
                Prefecture::Fukuoka => "福岡県",
                Prefecture::Fukushima => "福島県",
                Prefecture::Gifu => "岐阜県",
                Prefecture::Gunma => "群馬県",
                Prefecture::Hiroshima => "広島県",
                Prefecture::Hokkaido => "北海道",
                Prefecture::Hyogo => "兵庫県",
                Prefecture::Ibaraki => "茨城県",
                Prefecture::Ishikawa => "石川県",
                Prefecture::Iwate => "岩手県",
                Prefecture::Kagawa => "香川県",
                Prefecture::Kagoshima => "鹿児島県",
                Prefecture::Kanagawa => "神奈川県",
                Prefecture::Kochi => "高知県",
                Prefecture::Kumamoto => "熊本県",
                Prefecture::Kyoto => "京都府",
                Prefecture::Mie => "三重県",
                Prefecture::Miyagi => "宮城県",
                Prefecture::Miyazaki => "宮崎県",
                Prefecture::Nagano => "長野県",
                Prefecture::Nagasaki => "長崎県",
                Prefecture::Nara => "奈良県",
                Prefecture::Nigata => "新潟県",
                Prefecture::Oita => "大分県",
                Prefecture::Okayama => "岡山県",
                Prefecture::Okinawa => "沖縄県",
                Prefecture::Osaka => "大阪府",
                Prefecture::Saga => "佐賀県",
                Prefecture::Saitama => "埼玉県",
                Prefecture::Shiga => "滋賀県",
                Prefecture::Shimane => "島根県",
                Prefecture::Shizuoka => "静岡県",
                Prefecture::Tochigi => "栃木県",
                Prefecture::Tokushima => "徳島県",
                Prefecture::Tokyo => "東京都",
                Prefecture::Tottori => "鳥取県",
                Prefecture::Toyama => "富山県",
                Prefecture::Wakayama => "和歌山県",
                Prefecture::Yamagata => "山形県",
                Prefecture::Yamaguchi => "山口県",
                Prefecture::Yamanashi => "山梨県",
            },
            Language::En => match self {
                Prefecture::Aichi => "Aichi",
                Prefecture::Akita => "Akita",
                Prefecture::Aomori => "Aomori",
                Prefecture::Chiba => "Chiba",
                Prefecture::Ehime => "Ehime",
                Prefecture::Fukui => "",
                Prefecture::Fukuoka => "Fukuoka",
                Prefecture::Fukushima => "Fukushima",
                Prefecture::Gifu => "Gifu",
                Prefecture::Gunma => "Gunma",
                Prefecture::Hiroshima => "Hiroshima",
                Prefecture::Hokkaido => "Hokkaido",
                Prefecture::Hyogo => "Hyogo",
                Prefecture::Ibaraki => "Ibaraki",
                Prefecture::Ishikawa => "Ishikawa",
                Prefecture::Iwate => "Iwate",
                Prefecture::Kagawa => "Kagawa",
                Prefecture::Kagoshima => "Kagoshima",
                Prefecture::Kanagawa => "Kanagawa",
                Prefecture::Kochi => "Kochi",
                Prefecture::Kumamoto => "Kumamoto",
                Prefecture::Kyoto => "Kyoto",
                Prefecture::Mie => "Mie",
                Prefecture::Miyagi => "Miyagi",
                Prefecture::Miyazaki => "Miyazaki",
                Prefecture::Nagano => "Nagano",
                Prefecture::Nagasaki => "Nagasaki",
                Prefecture::Nara => "Nara",
                Prefecture::Nigata => "Nigata",
                Prefecture::Oita => "Oita",
                Prefecture::Okayama => "Okayama",
                Prefecture::Okinawa => "Okinawa",
                Prefecture::Osaka => "Osaka",
                Prefecture::Saga => "Saga",
                Prefecture::Saitama => "Saitama",
                Prefecture::Shiga => "Shiga",
                Prefecture::Shimane => "Shimane",
                Prefecture::Shizuoka => "Shizuoka",
                Prefecture::Tochigi => "Tochigi",
                Prefecture::Tokushima => "Tokushima",
                Prefecture::Tokyo => "Tokyo",
                Prefecture::Tottori => "Tottori",
                Prefecture::Toyama => "Toyama",
                Prefecture::Wakayama => "Wakayama",
                Prefecture::Yamagata => "Yamagata",
                Prefecture::Yamaguchi => "Yamaguchi",
                Prefecture::Yamanashi => "Yamanashi",
            },
        }
    }

    fn get_all() -> Vec<Self> {
        vec![
                Prefecture::Aichi,
                Prefecture::Akita,
                Prefecture::Aomori,
                Prefecture::Chiba,
                Prefecture::Ehime,
                Prefecture::Fukui,
                Prefecture::Fukuoka,
                Prefecture::Fukushima,
                Prefecture::Gifu,
                Prefecture::Gunma,
                Prefecture::Hiroshima,
                Prefecture::Hokkaido,
                Prefecture::Hyogo,
                Prefecture::Ibaraki,
                Prefecture::Ishikawa,
                Prefecture::Iwate,
                Prefecture::Kagawa,
                Prefecture::Kagoshima,
                Prefecture::Kanagawa,
                Prefecture::Kochi,
                Prefecture::Kumamoto,
                Prefecture::Kyoto,
                Prefecture::Mie,
                Prefecture::Miyagi,
                Prefecture::Miyazaki,
                Prefecture::Nagano,
                Prefecture::Nagasaki,
                Prefecture::Nara,
                Prefecture::Nigata,
                Prefecture::Oita,
                Prefecture::Okayama,
                Prefecture::Okinawa,
                Prefecture::Osaka,
                Prefecture::Saga,
                Prefecture::Saitama,
                Prefecture::Shiga,
                Prefecture::Shimane,
                Prefecture::Shizuoka,
                Prefecture::Tochigi,
                Prefecture::Tokushima,
                Prefecture::Tokyo,
                Prefecture::Tottori,
                Prefecture::Toyama,
                Prefecture::Wakayama,
                Prefecture::Yamagata,
                Prefecture::Yamaguchi,
                Prefecture::Yamanashi,
        ]
    }

    fn display_name_of_enum(lng: Language) -> String {
        let res = match lng {
            Language::En => "Prefecture",
            Language::Ja => "県"
        };
        res.to_string()
    }
}

impl fmt::Display for Prefecture {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_string(Language::En))
    }
}

impl fmt::Display for Region {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str(Language::En))
    }
}
