use crate::{
    chinese_date::days_of_chinese_month,
    language::{Language, ShortTranslate, StaticTranslate, Translate},
    ChineseDate,
};
use std::fmt::{Display, Formatter, Result as FmtResult};

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
#[non_exhaustive]
pub enum Festival {
    Chunjie,
    Yuanxiaojie,
    Duanwujie,
    Zhongqiujie,
    NorthernXiaonian,
    SouthernXiaonian,
    Chuxi,
}

impl Festival {
    pub fn from_chinese_date(date: ChineseDate) -> Option<Self> {
        use Festival::*;
        if date.leap() {
            return None;
        }
        match date.month() {
            1 => match date.day() {
                1 => Some(Chunjie),
                15 => Some(Yuanxiaojie),
                _ => None,
            },
            5 => (date.day() == 5).then_some(Duanwujie),
            8 => (date.day() == 15).then_some(Zhongqiujie),
            12 => match date.day() {
                23 => Some(NorthernXiaonian),
                24 => Some(SouthernXiaonian),
                other => (other
                    == days_of_chinese_month(date.chinese_year(), date.chinese_month()).unwrap())
                .then_some(Chuxi),
            },
            _ => None,
        }
    }
}

impl StaticTranslate for Festival {
    fn static_translate(&self, language: Language) -> &'static str {
        use Festival::*;
        use Language::*;
        match language {
            English => match self {
                Chunjie => "Chunjie",
                Yuanxiaojie => "Yuanxiaojie",
                Duanwujie => "Duanwujie",
                Zhongqiujie => "Zhongqiujie",
                NorthernXiaonian => "Northern Xiaonian",
                SouthernXiaonian => "Southern Xiaonian",
                Chuxi => "Chuxi",
            },
            ChineseSimplified => match self {
                Chunjie => "春节",
                Yuanxiaojie => "元宵节",
                Duanwujie => "端午节",
                Zhongqiujie => "中秋节",
                NorthernXiaonian => "北方小年",
                SouthernXiaonian => "南方小年",
                Chuxi => "除夕",
            },
            ChineseTraditional => match self {
                Chunjie => "春節",
                Yuanxiaojie => "元宵節",
                Duanwujie => "端午節",
                Zhongqiujie => "中秋節",
                NorthernXiaonian => "北方小年",
                SouthernXiaonian => "南方小年",
                Chuxi => "除夕",
            },
        }
    }
}

impl Translate for Festival {
    fn translate(&self, language: Language, f: &mut Formatter) -> FmtResult {
        self.static_translate(language).fmt(f)
    }
}

impl ShortTranslate for Festival {
    fn short_translate(&self, language: Language, f: &mut Formatter) -> FmtResult {
        let translation = self.static_translate(language);
        if language == Language::English {
            if translation.len() > 6 {
                write!(f, "{}.", &translation[..5])
            } else {
                translation.fmt(f)
            }
        } else {
            if translation.len() > 9 {
                write!(f, "{}..", &translation[..6])
            } else {
                translation.fmt(f)
            }
        }
    }
}