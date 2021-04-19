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

impl EnumAttrs<Region> for Region {
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

impl Prefecture {
    pub fn as_string(&self, lng: Language) -> String {
        match lng {
            Language::Ja => match self {
                Prefecture::Aichi => "愛知県".to_owned(),
                Prefecture::Akita => "秋田県".to_owned(),
                Prefecture::Aomori => "青森県".to_owned(),
                Prefecture::Chiba => "千葉県".to_owned(),
                Prefecture::Ehime => "愛媛県".to_owned(),
                Prefecture::Fukui => "福井県".to_owned(),
                Prefecture::Fukuoka => "福岡県".to_owned(),
                Prefecture::Fukushima => "福島県".to_owned(),
                Prefecture::Gifu => "岐阜県".to_owned(),
                Prefecture::Gunma => "群馬県".to_owned(),
                Prefecture::Hiroshima => "広島県".to_owned(),
                Prefecture::Hokkaido => "北海道".to_owned(),
                Prefecture::Hyogo => "兵庫県".to_owned(),
                Prefecture::Ibaraki => "茨城県".to_owned(),
                Prefecture::Ishikawa => "石川県".to_owned(),
                Prefecture::Iwate => "岩手県".to_owned(),
                Prefecture::Kagawa => "香川県".to_owned(),
                Prefecture::Kagoshima => "鹿児島県".to_owned(),
                Prefecture::Kanagawa => "神奈川県".to_owned(),
                Prefecture::Kochi => "高知県".to_owned(),
                Prefecture::Kumamoto => "熊本県".to_owned(),
                Prefecture::Kyoto => "京都府".to_owned(),
                Prefecture::Mie => "三重県".to_owned(),
                Prefecture::Miyagi => "宮城県".to_owned(),
                Prefecture::Miyazaki => "宮崎県".to_owned(),
                Prefecture::Nagano => "長野県".to_owned(),
                Prefecture::Nagasaki => "長崎県".to_owned(),
                Prefecture::Nara => "奈良県".to_owned(),
                Prefecture::Nigata => "新潟県".to_owned(),
                Prefecture::Oita => "大分県".to_owned(),
                Prefecture::Okayama => "岡山県".to_owned(),
                Prefecture::Okinawa => "沖縄県".to_owned(),
                Prefecture::Osaka => "大阪府".to_owned(),
                Prefecture::Saga => "佐賀県".to_owned(),
                Prefecture::Saitama => "埼玉県".to_owned(),
                Prefecture::Shiga => "滋賀県".to_owned(),
                Prefecture::Shimane => "島根県".to_owned(),
                Prefecture::Shizuoka => "静岡県".to_owned(),
                Prefecture::Tochigi => "栃木県".to_owned(),
                Prefecture::Tokushima => "徳島県".to_owned(),
                Prefecture::Tokyo => "東京都".to_owned(),
                Prefecture::Tottori => "鳥取県".to_owned(),
                Prefecture::Toyama => "富山県".to_owned(),
                Prefecture::Wakayama => "和歌山県".to_owned(),
                Prefecture::Yamagata => "山形県".to_owned(),
                Prefecture::Yamaguchi => "山口県".to_owned(),
                Prefecture::Yamanashi => "山梨県".to_owned(),
            },
            Language::En => match self {
                Prefecture::Aichi => "Aichi".to_owned(),
                Prefecture::Akita => "Akita".to_owned(),
                Prefecture::Aomori => "Aomori".to_owned(),
                Prefecture::Chiba => "Chiba".to_owned(),
                Prefecture::Ehime => "Ehime".to_owned(),
                Prefecture::Fukui => "".to_owned(),
                Prefecture::Fukuoka => "Fukuoka".to_owned(),
                Prefecture::Fukushima => "Fukushima".to_owned(),
                Prefecture::Gifu => "Gifu".to_owned(),
                Prefecture::Gunma => "Gunma".to_owned(),
                Prefecture::Hiroshima => "Hiroshima".to_owned(),
                Prefecture::Hokkaido => "Hokkaido".to_owned(),
                Prefecture::Hyogo => "Hyogo".to_owned(),
                Prefecture::Ibaraki => "Ibaraki".to_owned(),
                Prefecture::Ishikawa => "Ishikawa".to_owned(),
                Prefecture::Iwate => "Iwate".to_owned(),
                Prefecture::Kagawa => "Kagawa".to_owned(),
                Prefecture::Kagoshima => "Kagoshima".to_owned(),
                Prefecture::Kanagawa => "Kanagawa".to_owned(),
                Prefecture::Kochi => "Kochi".to_owned(),
                Prefecture::Kumamoto => "Kumamoto".to_owned(),
                Prefecture::Kyoto => "Kyoto".to_owned(),
                Prefecture::Mie => "Mie".to_owned(),
                Prefecture::Miyagi => "Miyagi".to_owned(),
                Prefecture::Miyazaki => "Miyazaki".to_owned(),
                Prefecture::Nagano => "Nagano".to_owned(),
                Prefecture::Nagasaki => "Nagasaki".to_owned(),
                Prefecture::Nara => "Nara".to_owned(),
                Prefecture::Nigata => "Nigata".to_owned(),
                Prefecture::Oita => "Oita".to_owned(),
                Prefecture::Okayama => "Okayama".to_owned(),
                Prefecture::Okinawa => "Okinawa".to_owned(),
                Prefecture::Osaka => "Osaka".to_owned(),
                Prefecture::Saga => "Saga".to_owned(),
                Prefecture::Saitama => "Saitama".to_owned(),
                Prefecture::Shiga => "Shiga".to_owned(),
                Prefecture::Shimane => "Shimane".to_owned(),
                Prefecture::Shizuoka => "Shizuoka".to_owned(),
                Prefecture::Tochigi => "Tochigi".to_owned(),
                Prefecture::Tokushima => "Tokushima".to_owned(),
                Prefecture::Tokyo => "Tokyo".to_owned(),
                Prefecture::Tottori => "Tottori".to_owned(),
                Prefecture::Toyama => "Toyama".to_owned(),
                Prefecture::Wakayama => "Wakayama".to_owned(),
                Prefecture::Yamagata => "Yamagata".to_owned(),
                Prefecture::Yamaguchi => "Yamaguchi".to_owned(),
                Prefecture::Yamanashi => "Yamanashi".to_owned(),
            },
        }
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
