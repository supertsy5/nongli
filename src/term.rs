pub enum SolarTerm {
    Xiaohan,
    Dahan,
    Lichun,
    Yushui,
    Jingzhe,
    Chunfen,
    Qingming,
    Guyu,
    Lixia,
    Xiaoman,
    Mangzhong,
    Xiazhi,
    Xiaoshu,
    Dashu,
    Liqiu,
    Chushu,
    Bailu,
    Qiufen,
    Hanlu,
    Shuangjiang,
    Lidong,
    Xiaoxue,
    Daxue,
    Dongzhi,
}

impl SolarTerm {
    pub fn as_ordinal(self) -> u8 {
        self as u8
    }
    pub fn from_ordinal(ordinal: u8) -> Option<Self> {
        use SolarTerm::*;
        match ordinal {
            0 => Some(Xiaohan),
            1 => Some(Dahan),
            2 => Some(Lichun),
            3 => Some(Yushui),
            4 => Some(Jingzhe),
            5 => Some(Chunfen),
            6 => Some(Qingming),
            7 => Some(Guyu),
            8 => Some(Lixia),
            9 => Some(Xiaoman),
            10 => Some(Mangzhong),
            11 => Some(Xiazhi),
            12 => Some(Xiaoshu),
            13 => Some(Dashu),
            14 => Some(Liqiu),
            15 => Some(Chushu),
            16 => Some(Bailu),
            17 => Some(Qiufen),
            18 => Some(Hanlu),
            19 => Some(Shuangjiang),
            20 => Some(Lidong),
            21 => Some(Xiaoxue),
            22 => Some(Daxue),
            23 => Some(Dongzhi),
            _ => None,
        }
    }
    pub fn is_midterm(self) -> bool {
        self.as_ordinal() % 2 > 0
    }
}