use slt::Color;

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub enum Element {
    // Powders
    Sand,
    Dust,
    Coal,
    Gunpowder,
    Snow,
    Salt,
    Sawdust,
    Ash,
    Rust,
    // Liquids
    Water,
    Oil,
    Lava,
    Acid,
    Mercury,
    SaltWater,
    Alcohol,
    Cryo,
    // Gases
    Fire,
    Steam,
    Smoke,
    Hydrogen,
    Plasma,
    Oxygen,
    Methane,
    // Solids
    Stone,
    Metal,
    Wood,
    Ice,
    Glass,
    Brick,
    Wire,
    Heater,
    Cooler,
    Plant,
    Fuse,
    Battery,
    Insulator,
    // Special
    Wall,
    Cloner,
    Void,
    Spark,
    Pscn,
    Nscn,
    Swch,
    Fan,
    Grav,
    Bhol,
    Whol,
    Phot,
    Neut,
    Uran,
    Bomb,
    Dest,
    Embr,
    Gol,
    Aray,
    Dmnd,
    Gold,
    Iron,
    Brmt,
    Wax,
    Soap,
    Gel,
    Desl,
    Mrcr,
    GelL,
    Cflm,
    Bizrg,
    WarpG,
    Dray,
    Cray,
    Dtec,
    Tsns,
    Psns,
    Pcln,
    Pbcn,
    Hswc,
    Pump,
    Gpmp,
    Pipe,
    PortalIn,
    PortalOut,
    WifiA,
    WifiB,
    Sing,
    Prot,
    Ttan,
    Ite,
    Stne2,
    Ite3,
    Yest,
    Frzz,
    Bcol,
    Isoz,
    Pqrt,
    Qrtz,
    Clst,
    Fsep,
    Ignt,
    Deut,
    Iszs,
    Pste,
    Mwax,
    Vxgl,
    Glow,
    Soap2,
    Rbdm,
    Lrbd,
    Plsm2,
    Fog,
    Smke2,
    Boyl,
    Gas2,
    Nble,
    Tung,
    QrtzS,
    Ptnm,
    Nkel,
    Zinc,
    Copr,
    Slcn,
    Rbr,
    Stkm,
    Thrm,
    Nitr,
    Shld,
    Almn,
    Insl2,
    Ntct,
    Ptct,
    Plut,
    Amtr,
    Elec,
    Posi,
    Frme,
    Stor,
    Pvod,
    Cbnw,
    Ldtc,
    Filt,
    Invs,
    Conv,
    Bray,
    Dlay,
    Thor,
    Trit,
    Cesm,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Behavior {
    Powder,
    Liquid,
    Gas,
    Solid,
}

pub const CATEGORY_NAMES: [&str; 10] = [
    "Powder",
    "Liquid",
    "Gas",
    "Solid",
    "Electronics",
    "Energy",
    "Special",
    "Sensor",
    "Transport",
    "Radioactive",
];
pub const NUM_CATEGORIES: usize = 10;

pub fn elements_in_category(cat: usize) -> &'static [Element] {
    match cat {
        0 => &[
            Element::Sand,
            Element::Dust,
            Element::Coal,
            Element::Gunpowder,
            Element::Snow,
            Element::Salt,
            Element::Sawdust,
            Element::Ash,
            Element::Rust,
            Element::Brmt,
            Element::Wax,
            Element::Soap,
            Element::Gel,
            Element::Ite,
            Element::Stne2,
            Element::Ite3,
            Element::Yest,
            Element::Frzz,
            Element::Bcol,
            Element::Isoz,
            Element::Pqrt,
            Element::Qrtz,
            Element::Clst,
            Element::Fsep,
            Element::Ignt,
        ],
        1 => &[
            Element::Water,
            Element::Oil,
            Element::Lava,
            Element::Acid,
            Element::Mercury,
            Element::SaltWater,
            Element::Alcohol,
            Element::Cryo,
            Element::Desl,
            Element::Mrcr,
            Element::GelL,
            Element::Deut,
            Element::Iszs,
            Element::Pste,
            Element::Mwax,
            Element::Vxgl,
            Element::Glow,
            Element::Soap2,
            Element::Rbdm,
            Element::Lrbd,
            Element::Cbnw,
        ],
        2 => &[
            Element::Fire,
            Element::Steam,
            Element::Smoke,
            Element::Hydrogen,
            Element::Plasma,
            Element::Oxygen,
            Element::Methane,
            Element::Cflm,
            Element::Bizrg,
            Element::WarpG,
            Element::Plsm2,
            Element::Fog,
            Element::Smke2,
            Element::Boyl,
            Element::Gas2,
            Element::Nble,
            Element::Trit,
        ],
        3 => &[
            Element::Stone,
            Element::Metal,
            Element::Wood,
            Element::Ice,
            Element::Glass,
            Element::Brick,
            Element::Dmnd,
            Element::Gold,
            Element::Iron,
            Element::Ttan,
            Element::Heater,
            Element::Cooler,
            Element::Plant,
            Element::Fuse,
            Element::Gol,
            Element::Tung,
            Element::QrtzS,
            Element::Ptnm,
            Element::Nkel,
            Element::Zinc,
            Element::Copr,
            Element::Slcn,
            Element::Rbr,
            Element::Stkm,
            Element::Thrm,
            Element::Nitr,
            Element::Shld,
            Element::Almn,
            Element::Insl2,
            Element::Ntct,
            Element::Ptct,
            Element::Frme,
            Element::Invs,
            Element::Cesm,
        ],
        4 => &[
            Element::Pscn,
            Element::Nscn,
            Element::Swch,
            Element::Aray,
            Element::Dray,
            Element::Cray,
            Element::Pcln,
            Element::Pbcn,
            Element::Hswc,
            Element::Battery,
            Element::Wire,
            Element::Insulator,
            Element::Ldtc,
            Element::Dlay,
        ],
        5 => &[
            Element::Phot,
            Element::Neut,
            Element::Uran,
            Element::Bomb,
            Element::Dest,
            Element::Embr,
            Element::Spark,
            Element::Prot,
            Element::Elec,
            Element::Posi,
            Element::Bray,
        ],
        6 => &[
            Element::Wall,
            Element::Cloner,
            Element::Void,
            Element::Fan,
            Element::Grav,
            Element::Bhol,
            Element::Whol,
            Element::Sing,
            Element::Stor,
            Element::Pvod,
            Element::Filt,
            Element::Conv,
        ],
        7 => &[Element::Dtec, Element::Tsns, Element::Psns, Element::Ldtc],
        8 => &[
            Element::Pump,
            Element::Gpmp,
            Element::Pipe,
            Element::PortalIn,
            Element::PortalOut,
            Element::WifiA,
            Element::WifiB,
        ],
        9 => &[
            Element::Uran,
            Element::Isoz,
            Element::Iszs,
            Element::Deut,
            Element::Plut,
            Element::Amtr,
            Element::Elec,
            Element::Posi,
            Element::Thor,
            Element::Trit,
        ],
        _ => &[],
    }
}

impl Element {
    pub fn id(self) -> u16 {
        self as u16
    }

    pub fn from_id(id: u16) -> Option<Self> {
        match id {
            0 => Some(Element::Sand),
            1 => Some(Element::Dust),
            2 => Some(Element::Coal),
            3 => Some(Element::Gunpowder),
            4 => Some(Element::Snow),
            5 => Some(Element::Salt),
            6 => Some(Element::Sawdust),
            7 => Some(Element::Ash),
            8 => Some(Element::Rust),
            9 => Some(Element::Water),
            10 => Some(Element::Oil),
            11 => Some(Element::Lava),
            12 => Some(Element::Acid),
            13 => Some(Element::Mercury),
            14 => Some(Element::SaltWater),
            15 => Some(Element::Alcohol),
            16 => Some(Element::Cryo),
            17 => Some(Element::Fire),
            18 => Some(Element::Steam),
            19 => Some(Element::Smoke),
            20 => Some(Element::Hydrogen),
            21 => Some(Element::Plasma),
            22 => Some(Element::Oxygen),
            23 => Some(Element::Methane),
            24 => Some(Element::Stone),
            25 => Some(Element::Metal),
            26 => Some(Element::Wood),
            27 => Some(Element::Ice),
            28 => Some(Element::Glass),
            29 => Some(Element::Brick),
            30 => Some(Element::Wire),
            31 => Some(Element::Heater),
            32 => Some(Element::Cooler),
            33 => Some(Element::Plant),
            34 => Some(Element::Fuse),
            35 => Some(Element::Battery),
            36 => Some(Element::Insulator),
            37 => Some(Element::Wall),
            38 => Some(Element::Cloner),
            39 => Some(Element::Void),
            40 => Some(Element::Spark),
            41 => Some(Element::Pscn),
            42 => Some(Element::Nscn),
            43 => Some(Element::Swch),
            44 => Some(Element::Fan),
            45 => Some(Element::Grav),
            46 => Some(Element::Bhol),
            47 => Some(Element::Whol),
            48 => Some(Element::Phot),
            49 => Some(Element::Neut),
            50 => Some(Element::Uran),
            51 => Some(Element::Bomb),
            52 => Some(Element::Dest),
            53 => Some(Element::Embr),
            54 => Some(Element::Gol),
            55 => Some(Element::Aray),
            56 => Some(Element::Dmnd),
            57 => Some(Element::Gold),
            58 => Some(Element::Iron),
            59 => Some(Element::Brmt),
            60 => Some(Element::Wax),
            61 => Some(Element::Soap),
            62 => Some(Element::Gel),
            63 => Some(Element::Desl),
            64 => Some(Element::Mrcr),
            65 => Some(Element::GelL),
            66 => Some(Element::Cflm),
            67 => Some(Element::Bizrg),
            68 => Some(Element::WarpG),
            69 => Some(Element::Dray),
            70 => Some(Element::Cray),
            71 => Some(Element::Dtec),
            72 => Some(Element::Tsns),
            73 => Some(Element::Psns),
            74 => Some(Element::Pcln),
            75 => Some(Element::Pbcn),
            76 => Some(Element::Hswc),
            77 => Some(Element::Pump),
            78 => Some(Element::Gpmp),
            79 => Some(Element::Pipe),
            80 => Some(Element::PortalIn),
            81 => Some(Element::PortalOut),
            82 => Some(Element::WifiA),
            83 => Some(Element::WifiB),
            84 => Some(Element::Sing),
            85 => Some(Element::Prot),
            86 => Some(Element::Ttan),
            87 => Some(Element::Ite),
            88 => Some(Element::Stne2),
            89 => Some(Element::Ite3),
            90 => Some(Element::Yest),
            91 => Some(Element::Frzz),
            92 => Some(Element::Bcol),
            93 => Some(Element::Isoz),
            94 => Some(Element::Pqrt),
            95 => Some(Element::Qrtz),
            96 => Some(Element::Clst),
            97 => Some(Element::Fsep),
            98 => Some(Element::Ignt),
            99 => Some(Element::Deut),
            100 => Some(Element::Iszs),
            101 => Some(Element::Pste),
            102 => Some(Element::Mwax),
            103 => Some(Element::Vxgl),
            104 => Some(Element::Glow),
            105 => Some(Element::Soap2),
            106 => Some(Element::Rbdm),
            107 => Some(Element::Lrbd),
            108 => Some(Element::Plsm2),
            109 => Some(Element::Fog),
            110 => Some(Element::Smke2),
            111 => Some(Element::Boyl),
            112 => Some(Element::Gas2),
            113 => Some(Element::Nble),
            114 => Some(Element::Tung),
            115 => Some(Element::QrtzS),
            116 => Some(Element::Ptnm),
            117 => Some(Element::Nkel),
            118 => Some(Element::Zinc),
            119 => Some(Element::Copr),
            120 => Some(Element::Slcn),
            121 => Some(Element::Rbr),
            122 => Some(Element::Stkm),
            123 => Some(Element::Thrm),
            124 => Some(Element::Nitr),
            125 => Some(Element::Shld),
            126 => Some(Element::Almn),
            127 => Some(Element::Insl2),
            128 => Some(Element::Ntct),
            129 => Some(Element::Ptct),
            130 => Some(Element::Plut),
            131 => Some(Element::Amtr),
            132 => Some(Element::Elec),
            133 => Some(Element::Posi),
            134 => Some(Element::Frme),
            135 => Some(Element::Stor),
            136 => Some(Element::Pvod),
            137 => Some(Element::Cbnw),
            138 => Some(Element::Ldtc),
            139 => Some(Element::Filt),
            140 => Some(Element::Invs),
            141 => Some(Element::Conv),
            142 => Some(Element::Bray),
            143 => Some(Element::Dlay),
            144 => Some(Element::Thor),
            145 => Some(Element::Trit),
            146 => Some(Element::Cesm),
            _ => None,
        }
    }

    pub fn name(&self) -> &'static str {
        match self {
            Element::Sand => "Sand",
            Element::Dust => "Dust",
            Element::Coal => "Coal",
            Element::Gunpowder => "Gunpwdr",
            Element::Snow => "Snow",
            Element::Salt => "Salt",
            Element::Sawdust => "Sawdust",
            Element::Ash => "Ash",
            Element::Rust => "Rust",
            Element::Water => "Water",
            Element::Oil => "Oil",
            Element::Lava => "Lava",
            Element::Acid => "Acid",
            Element::Mercury => "Mercury",
            Element::SaltWater => "SaltWtr",
            Element::Alcohol => "Alcohol",
            Element::Cryo => "Cryo",
            Element::Fire => "Fire",
            Element::Steam => "Steam",
            Element::Smoke => "Smoke",
            Element::Hydrogen => "H\u{2082}",
            Element::Plasma => "Plasma",
            Element::Oxygen => "O\u{2082}",
            Element::Methane => "CH\u{2084}",
            Element::Stone => "Stone",
            Element::Metal => "Metal",
            Element::Wood => "Wood",
            Element::Ice => "Ice",
            Element::Glass => "Glass",
            Element::Brick => "Brick",
            Element::Wire => "Wire",
            Element::Heater => "Heater",
            Element::Cooler => "Cooler",
            Element::Plant => "Plant",
            Element::Fuse => "Fuse",
            Element::Battery => "Battery",
            Element::Insulator => "Insltr",
            Element::Wall => "Wall",
            Element::Cloner => "Clone",
            Element::Void => "Void",
            Element::Spark => "Spark",
            Element::Pscn => "PSCN",
            Element::Nscn => "NSCN",
            Element::Swch => "SWCH",
            Element::Fan => "Fan",
            Element::Grav => "Grav",
            Element::Bhol => "BHol",
            Element::Whol => "WHol",
            Element::Phot => "Phot",
            Element::Neut => "Neut",
            Element::Uran => "Uran",
            Element::Bomb => "Bomb",
            Element::Dest => "Dest",
            Element::Embr => "Embr",
            Element::Gol => "GOL",
            Element::Aray => "Aray",
            Element::Dmnd => "Dmnd",
            Element::Gold => "Gold",
            Element::Iron => "Iron",
            Element::Brmt => "Brmt",
            Element::Wax => "Wax",
            Element::Soap => "Soap",
            Element::Gel => "Gel",
            Element::Desl => "Desl",
            Element::Mrcr => "Mrcr",
            Element::GelL => "Gel_L",
            Element::Cflm => "Cflm",
            Element::Bizrg => "Bizrg",
            Element::WarpG => "Warp_G",
            Element::Dray => "Dray",
            Element::Cray => "Cray",
            Element::Dtec => "Dtec",
            Element::Tsns => "Tsns",
            Element::Psns => "Psns",
            Element::Pcln => "Pcln",
            Element::Pbcn => "Pbcn",
            Element::Hswc => "Hswc",
            Element::Pump => "Pump",
            Element::Gpmp => "Gpmp",
            Element::Pipe => "Pipe",
            Element::PortalIn => "PortalIn",
            Element::PortalOut => "PortalOut",
            Element::WifiA => "WifiA",
            Element::WifiB => "WifiB",
            Element::Sing => "Sing",
            Element::Prot => "Prot",
            Element::Ttan => "Ttan",
            Element::Ite => "Ite",
            Element::Stne2 => "Stne2",
            Element::Ite3 => "Ite3",
            Element::Yest => "Yest",
            Element::Frzz => "Frzz",
            Element::Bcol => "Bcol",
            Element::Isoz => "Isoz",
            Element::Pqrt => "Pqrt",
            Element::Qrtz => "Qrtz",
            Element::Clst => "Clst",
            Element::Fsep => "Fsep",
            Element::Ignt => "Ignt",
            Element::Deut => "Deut",
            Element::Iszs => "Iszs",
            Element::Pste => "Pste",
            Element::Mwax => "Mwax",
            Element::Vxgl => "Vxgl",
            Element::Glow => "Glow",
            Element::Soap2 => "Soap2",
            Element::Rbdm => "Rbdm",
            Element::Lrbd => "Lrbd",
            Element::Plsm2 => "Plsm2",
            Element::Fog => "Fog",
            Element::Smke2 => "Smke2",
            Element::Boyl => "Boyl",
            Element::Gas2 => "Gas2",
            Element::Nble => "Nble",
            Element::Tung => "Tung",
            Element::QrtzS => "Qrtz_S",
            Element::Ptnm => "Ptnm",
            Element::Nkel => "Nkel",
            Element::Zinc => "Zinc",
            Element::Copr => "Copr",
            Element::Slcn => "Slcn",
            Element::Rbr => "Rbr",
            Element::Stkm => "Stkm",
            Element::Thrm => "Thrm",
            Element::Nitr => "Nitr",
            Element::Shld => "Shld",
            Element::Almn => "Almn",
            Element::Insl2 => "Insl2",
            Element::Ntct => "Ntct",
            Element::Ptct => "Ptct",
            Element::Plut => "Plut",
            Element::Amtr => "Amtr",
            Element::Elec => "Elec",
            Element::Posi => "Posi",
            Element::Frme => "Frme",
            Element::Stor => "Stor",
            Element::Pvod => "Pvod",
            Element::Cbnw => "Cbnw",
            Element::Ldtc => "Ldtc",
            Element::Filt => "Filt",
            Element::Invs => "Invs",
            Element::Conv => "Conv",
            Element::Bray => "Bray",
            Element::Dlay => "Dlay",
            Element::Thor => "Thor",
            Element::Trit => "Trit",
            Element::Cesm => "Cesm",
        }
    }

    pub fn color(&self) -> Color {
        match self {
            Element::Sand => Color::Rgb(210, 180, 120),
            Element::Dust => Color::Rgb(190, 180, 160),
            Element::Coal => Color::Rgb(55, 45, 40),
            Element::Gunpowder => Color::Rgb(100, 90, 65),
            Element::Snow => Color::Rgb(240, 245, 255),
            Element::Salt => Color::Rgb(220, 220, 210),
            Element::Sawdust => Color::Rgb(170, 140, 90),
            Element::Ash => Color::Rgb(120, 120, 120),
            Element::Rust => Color::Rgb(170, 90, 45),
            Element::Water => Color::Rgb(30, 100, 210),
            Element::Oil => Color::Rgb(85, 60, 20),
            Element::Lava => Color::Rgb(255, 60, 10),
            Element::Acid => Color::Rgb(130, 255, 30),
            Element::Mercury => Color::Rgb(150, 160, 175),
            Element::SaltWater => Color::Rgb(40, 120, 200),
            Element::Alcohol => Color::Rgb(160, 180, 220),
            Element::Cryo => Color::Rgb(60, 190, 255),
            Element::Fire => Color::Rgb(255, 160, 40),
            Element::Steam => Color::Rgb(200, 200, 230),
            Element::Smoke => Color::Rgb(90, 90, 90),
            Element::Hydrogen => Color::Rgb(150, 200, 255),
            Element::Plasma => Color::Rgb(200, 120, 255),
            Element::Oxygen => Color::Rgb(120, 170, 255),
            Element::Methane => Color::Rgb(170, 220, 170),
            Element::Stone => Color::Rgb(140, 135, 125),
            Element::Metal => Color::Rgb(175, 180, 195),
            Element::Wood => Color::Rgb(130, 90, 40),
            Element::Ice => Color::Rgb(170, 220, 250),
            Element::Glass => Color::Rgb(200, 220, 230),
            Element::Brick => Color::Rgb(160, 80, 50),
            Element::Wire => Color::Rgb(245, 210, 60),
            Element::Heater => Color::Rgb(220, 90, 40),
            Element::Cooler => Color::Rgb(90, 180, 255),
            Element::Plant => Color::Rgb(70, 170, 70),
            Element::Fuse => Color::Rgb(210, 170, 90),
            Element::Battery => Color::Rgb(180, 150, 80),
            Element::Insulator => Color::Rgb(130, 80, 170),
            Element::Wall => Color::Rgb(110, 110, 110),
            Element::Cloner => Color::Rgb(200, 200, 220),
            Element::Void => Color::Rgb(50, 10, 60),
            Element::Spark => Color::Rgb(255, 255, 180),
            Element::Pscn => Color::Rgb(250, 210, 130),
            Element::Nscn => Color::Rgb(130, 190, 255),
            Element::Swch => Color::Rgb(190, 190, 120),
            Element::Fan => Color::Rgb(95, 120, 150),
            Element::Grav => Color::Rgb(140, 90, 210),
            Element::Bhol => Color::Rgb(16, 16, 20),
            Element::Whol => Color::Rgb(240, 240, 255),
            Element::Phot => Color::Rgb(255, 255, 120),
            Element::Neut => Color::Rgb(180, 255, 170),
            Element::Uran => Color::Rgb(110, 230, 80),
            Element::Bomb => Color::Rgb(150, 60, 30),
            Element::Dest => Color::Rgb(255, 90, 190),
            Element::Embr => Color::Rgb(255, 130, 50),
            Element::Gol => Color::Rgb(170, 200, 110),
            Element::Aray => Color::Rgb(220, 150, 255),
            Element::Dmnd => Color::Rgb(170, 250, 255),
            Element::Gold => Color::Rgb(240, 195, 45),
            Element::Iron => Color::Rgb(135, 145, 155),
            Element::Brmt => Color::Rgb(145, 125, 110),
            Element::Wax => Color::Rgb(245, 225, 140),
            Element::Soap => Color::Rgb(190, 230, 250),
            Element::Gel => Color::Rgb(95, 205, 170),
            Element::Desl => Color::Rgb(120, 90, 35),
            Element::Mrcr => Color::Rgb(120, 170, 110),
            Element::GelL => Color::Rgb(70, 185, 155),
            Element::Cflm => Color::Rgb(120, 210, 255),
            Element::Bizrg => Color::Rgb(230, 110, 240),
            Element::WarpG => Color::Rgb(150, 120, 255),
            Element::Dray => Color::Rgb(220, 170, 250),
            Element::Cray => Color::Rgb(255, 170, 230),
            Element::Dtec => Color::Rgb(250, 220, 140),
            Element::Tsns => Color::Rgb(255, 120, 120),
            Element::Psns => Color::Rgb(120, 190, 255),
            Element::Pcln => Color::Rgb(210, 210, 250),
            Element::Pbcn => Color::Rgb(195, 180, 235),
            Element::Hswc => Color::Rgb(210, 140, 90),
            Element::Pump => Color::Rgb(120, 160, 205),
            Element::Gpmp => Color::Rgb(155, 110, 220),
            Element::Pipe => Color::Rgb(115, 150, 170),
            Element::PortalIn => Color::Rgb(70, 115, 255),
            Element::PortalOut => Color::Rgb(255, 110, 120),
            Element::WifiA => Color::Rgb(120, 230, 190),
            Element::WifiB => Color::Rgb(120, 190, 230),
            Element::Sing => Color::Rgb(35, 30, 50),
            Element::Prot => Color::Rgb(255, 220, 155),
            Element::Ttan => Color::Rgb(150, 165, 185),
            Element::Ite => Color::Rgb(168, 152, 136),
            Element::Stne2 => Color::Rgb(146, 140, 132),
            Element::Ite3 => Color::Rgb(130, 124, 118),
            Element::Yest => Color::Rgb(235, 220, 146),
            Element::Frzz => Color::Rgb(150, 240, 255),
            Element::Bcol => Color::Rgb(72, 62, 56),
            Element::Isoz => Color::Rgb(90, 255, 115),
            Element::Pqrt => Color::Rgb(235, 210, 255),
            Element::Qrtz => Color::Rgb(220, 238, 250),
            Element::Clst => Color::Rgb(115, 105, 98),
            Element::Fsep => Color::Rgb(255, 190, 95),
            Element::Ignt => Color::Rgb(255, 116, 62),
            Element::Deut => Color::Rgb(92, 154, 255),
            Element::Iszs => Color::Rgb(72, 228, 132),
            Element::Pste => Color::Rgb(206, 188, 150),
            Element::Mwax => Color::Rgb(255, 206, 120),
            Element::Vxgl => Color::Rgb(116, 210, 225),
            Element::Glow => Color::Rgb(80, 255, 206),
            Element::Soap2 => Color::Rgb(214, 248, 255),
            Element::Rbdm => Color::Rgb(176, 126, 216),
            Element::Lrbd => Color::Rgb(210, 154, 248),
            Element::Plsm2 => Color::Rgb(190, 96, 255),
            Element::Fog => Color::Rgb(186, 196, 206),
            Element::Smke2 => Color::Rgb(124, 90, 150),
            Element::Boyl => Color::Rgb(196, 176, 126),
            Element::Gas2 => Color::Rgb(168, 210, 242),
            Element::Nble => Color::Rgb(178, 200, 255),
            Element::Tung => Color::Rgb(110, 120, 136),
            Element::QrtzS => Color::Rgb(198, 226, 244),
            Element::Ptnm => Color::Rgb(198, 198, 214),
            Element::Nkel => Color::Rgb(150, 170, 166),
            Element::Zinc => Color::Rgb(168, 186, 202),
            Element::Copr => Color::Rgb(198, 120, 76),
            Element::Slcn => Color::Rgb(108, 122, 140),
            Element::Rbr => Color::Rgb(74, 58, 66),
            Element::Stkm => Color::Rgb(255, 228, 168),
            Element::Thrm => Color::Rgb(214, 94, 58),
            Element::Nitr => Color::Rgb(228, 216, 164),
            Element::Shld => Color::Rgb(80, 180, 255),
            Element::Almn => Color::Rgb(182, 194, 212),
            Element::Insl2 => Color::Rgb(112, 88, 174),
            Element::Ntct => Color::Rgb(250, 180, 120),
            Element::Ptct => Color::Rgb(120, 188, 250),
            Element::Plut => Color::Rgb(84, 238, 98),
            Element::Amtr => Color::Rgb(255, 86, 226),
            Element::Elec => Color::Rgb(255, 255, 206),
            Element::Posi => Color::Rgb(255, 170, 245),
            Element::Frme => Color::Rgb(146, 146, 158),
            Element::Stor => Color::Rgb(170, 158, 216),
            Element::Pvod => Color::Rgb(78, 26, 98),
            Element::Cbnw => Color::Rgb(118, 176, 255),
            Element::Ldtc => Color::Rgb(255, 205, 136),
            Element::Filt => Color::Rgb(162, 110, 255),
            Element::Invs => Color::Rgb(36, 36, 44),
            Element::Conv => Color::Rgb(255, 154, 196),
            Element::Bray => Color::Rgb(255, 96, 156),
            Element::Dlay => Color::Rgb(206, 166, 102),
            Element::Thor => Color::Rgb(104, 224, 112),
            Element::Trit => Color::Rgb(138, 214, 255),
            Element::Cesm => Color::Rgb(224, 186, 126),
        }
    }

    pub fn behavior(&self) -> Behavior {
        match self {
            Element::Sand
            | Element::Dust
            | Element::Coal
            | Element::Gunpowder
            | Element::Snow
            | Element::Salt
            | Element::Sawdust
            | Element::Ash
            | Element::Rust => Behavior::Powder,

            Element::Water
            | Element::Oil
            | Element::Lava
            | Element::Acid
            | Element::Mercury
            | Element::SaltWater
            | Element::Alcohol
            | Element::Cryo => Behavior::Liquid,

            Element::Fire
            | Element::Steam
            | Element::Smoke
            | Element::Hydrogen
            | Element::Plasma
            | Element::Oxygen
            | Element::Methane
            | Element::Phot
            | Element::Neut
            | Element::Embr => Behavior::Gas,

            Element::Stone
            | Element::Metal
            | Element::Wood
            | Element::Ice
            | Element::Glass
            | Element::Brick
            | Element::Wire
            | Element::Heater
            | Element::Cooler
            | Element::Plant
            | Element::Fuse
            | Element::Battery
            | Element::Insulator
            | Element::Wall
            | Element::Cloner
            | Element::Void
            | Element::Spark
            | Element::Pscn
            | Element::Nscn
            | Element::Swch
            | Element::Fan
            | Element::Grav
            | Element::Bhol
            | Element::Whol
            | Element::Uran
            | Element::Bomb
            | Element::Dest
            | Element::Gol
            | Element::Aray => Behavior::Solid,
            Element::Brmt | Element::Wax | Element::Soap | Element::Gel => Behavior::Powder,
            Element::Ite
            | Element::Stne2
            | Element::Ite3
            | Element::Yest
            | Element::Frzz
            | Element::Bcol
            | Element::Isoz
            | Element::Pqrt
            | Element::Qrtz
            | Element::Clst
            | Element::Fsep
            | Element::Ignt => Behavior::Powder,
            Element::Desl | Element::Mrcr | Element::GelL => Behavior::Liquid,
            Element::Deut
            | Element::Iszs
            | Element::Pste
            | Element::Mwax
            | Element::Vxgl
            | Element::Glow
            | Element::Soap2
            | Element::Rbdm
            | Element::Lrbd
            | Element::Cbnw => Behavior::Liquid,
            Element::Cflm | Element::Bizrg | Element::WarpG | Element::Prot => Behavior::Gas,
            Element::Plsm2
            | Element::Fog
            | Element::Smke2
            | Element::Boyl
            | Element::Gas2
            | Element::Nble
            | Element::Elec
            | Element::Posi
            | Element::Bray
            | Element::Trit => Behavior::Gas,
            Element::Dmnd
            | Element::Gold
            | Element::Iron
            | Element::Dray
            | Element::Cray
            | Element::Dtec
            | Element::Tsns
            | Element::Psns
            | Element::Pcln
            | Element::Pbcn
            | Element::Hswc
            | Element::Pump
            | Element::Gpmp
            | Element::Pipe
            | Element::PortalIn
            | Element::PortalOut
            | Element::WifiA
            | Element::WifiB
            | Element::Sing
            | Element::Ttan
            | Element::Tung
            | Element::QrtzS
            | Element::Ptnm
            | Element::Nkel
            | Element::Zinc
            | Element::Copr
            | Element::Slcn
            | Element::Rbr
            | Element::Stkm
            | Element::Thrm
            | Element::Nitr
            | Element::Shld
            | Element::Almn
            | Element::Insl2
            | Element::Ntct
            | Element::Ptct
            | Element::Plut
            | Element::Amtr
            | Element::Frme
            | Element::Stor
            | Element::Pvod
            | Element::Ldtc
            | Element::Filt
            | Element::Invs
            | Element::Conv
            | Element::Dlay
            | Element::Thor
            | Element::Cesm => Behavior::Solid,
        }
    }

    pub fn density(&self) -> f32 {
        match self {
            Element::Plasma => 0.02,
            Element::Fire => 0.05,
            Element::Hydrogen => 0.08,
            Element::Steam => 0.1,
            Element::Smoke => 0.15,
            Element::Oxygen => 0.18,
            Element::Snow => 0.4,
            Element::Methane => 0.45,
            Element::Phot => 0.03,
            Element::Neut => 0.04,
            Element::Embr => 0.06,
            Element::Dust => 0.5,
            Element::Sawdust => 0.52,
            Element::Ash => 0.58,
            Element::Oil => 0.8,
            Element::Alcohol => 0.79,
            Element::Wood => 0.9,
            Element::Plant => 0.88,
            Element::Ice => 0.92,
            Element::Cryo => 0.94,
            Element::Water => 1.0,
            Element::SaltWater => 1.04,
            Element::Gunpowder => 1.0,
            Element::Salt => 1.1,
            Element::Acid => 1.1,
            Element::Rust => 1.35,
            Element::Coal => 1.2,
            Element::Sand => 1.5,
            Element::Glass => 2.2,
            Element::Stone => 2.5,
            Element::Lava => 2.5,
            Element::Brick => 2.8,
            Element::Metal => 3.0,
            Element::Mercury => 5.0,
            Element::Wire => 6.0,
            Element::Heater | Element::Cooler => 7.0,
            Element::Battery => 7.4,
            Element::Insulator => 7.2,
            Element::Fuse => 1.1,
            Element::Spark => 0.2,
            Element::Pscn => 7.1,
            Element::Nscn => 7.1,
            Element::Swch => 6.9,
            Element::Fan => 7.8,
            Element::Grav => 8.2,
            Element::Bhol => 999.0,
            Element::Whol => 999.0,
            Element::Uran => 8.4,
            Element::Bomb => 2.0,
            Element::Dest => 8.8,
            Element::Gol => 1.05,
            Element::Aray => 7.6,
            Element::Dmnd => 999.0,
            Element::Gold => 9.2,
            Element::Iron => 8.0,
            Element::Brmt => 3.1,
            Element::Wax => 0.95,
            Element::Soap => 1.08,
            Element::Gel => 1.12,
            Element::Desl => 0.83,
            Element::Mrcr => 1.06,
            Element::GelL => 1.05,
            Element::Cflm => 0.04,
            Element::Bizrg => 0.16,
            Element::WarpG => 0.09,
            Element::Dray => 7.5,
            Element::Cray => 7.5,
            Element::Dtec => 7.2,
            Element::Tsns => 7.2,
            Element::Psns => 7.2,
            Element::Pcln => 999.0,
            Element::Pbcn => 999.0,
            Element::Hswc => 7.0,
            Element::Pump => 7.3,
            Element::Gpmp => 8.0,
            Element::Pipe => 8.3,
            Element::PortalIn => 999.0,
            Element::PortalOut => 999.0,
            Element::WifiA => 7.1,
            Element::WifiB => 7.1,
            Element::Sing => 999.0,
            Element::Prot => 0.03,
            Element::Ttan => 999.0,
            Element::Ite => 1.7,
            Element::Stne2 => 1.75,
            Element::Ite3 => 1.8,
            Element::Yest => 0.62,
            Element::Frzz => 1.2,
            Element::Bcol => 1.25,
            Element::Isoz => 1.55,
            Element::Pqrt => 1.9,
            Element::Qrtz => 2.0,
            Element::Clst => 1.82,
            Element::Fsep => 1.12,
            Element::Ignt => 1.08,
            Element::Deut => 1.11,
            Element::Iszs => 1.16,
            Element::Pste => 1.26,
            Element::Mwax => 0.9,
            Element::Vxgl => 1.9,
            Element::Glow => 1.03,
            Element::Soap2 => 0.45,
            Element::Rbdm => 1.45,
            Element::Lrbd => 1.35,
            Element::Plsm2 => 0.02,
            Element::Fog => 0.17,
            Element::Smke2 => 0.16,
            Element::Boyl => 0.2,
            Element::Gas2 => 0.21,
            Element::Nble => 0.24,
            Element::Tung => 12.6,
            Element::QrtzS => 2.35,
            Element::Ptnm => 11.9,
            Element::Nkel => 8.7,
            Element::Zinc => 7.1,
            Element::Copr => 8.9,
            Element::Slcn => 2.4,
            Element::Rbr => 1.15,
            Element::Stkm => 1.0,
            Element::Thrm => 2.4,
            Element::Nitr => 1.6,
            Element::Shld => 999.0,
            Element::Almn => 6.8,
            Element::Insl2 => 7.3,
            Element::Ntct => 7.0,
            Element::Ptct => 7.0,
            Element::Plut => 8.9,
            Element::Amtr => 0.01,
            Element::Elec => 0.02,
            Element::Posi => 0.02,
            Element::Frme => 8.1,
            Element::Stor => 8.4,
            Element::Pvod => 999.0,
            Element::Cbnw => 1.01,
            Element::Ldtc => 7.3,
            Element::Filt => 7.0,
            Element::Invs => 999.0,
            Element::Conv => 8.2,
            Element::Bray => 0.03,
            Element::Dlay => 7.0,
            Element::Thor => 8.3,
            Element::Trit => 0.07,
            Element::Cesm => 5.5,
            Element::Wall | Element::Cloner | Element::Void => 999.0,
        }
    }

    pub fn flammable(&self) -> bool {
        matches!(
            self,
            Element::Dust
                | Element::Coal
                | Element::Gunpowder
                | Element::Oil
                | Element::Wood
                | Element::Hydrogen
                | Element::Sawdust
                | Element::Alcohol
                | Element::Methane
                | Element::Plant
                | Element::Fuse
                | Element::Bomb
                | Element::Embr
                | Element::Wax
                | Element::Desl
                | Element::Bcol
                | Element::Yest
                | Element::Ignt
                | Element::Pste
                | Element::Mwax
                | Element::Rbdm
                | Element::Lrbd
                | Element::Smke2
                | Element::Thrm
                | Element::Nitr
        )
    }

    pub fn lifetime(&self) -> Option<u16> {
        match self {
            Element::Plasma => Some(25),
            Element::Fire => Some(50),
            Element::Smoke => Some(180),
            Element::Steam => Some(350),
            Element::Spark => Some(6),
            Element::Phot => Some(40),
            Element::Neut => Some(50),
            Element::Embr => Some(35),
            Element::Methane => Some(420),
            Element::Oxygen => Some(500),
            Element::Cflm => Some(90),
            Element::Bizrg => Some(260),
            Element::WarpG => Some(180),
            Element::Prot => Some(45),
            Element::Sing => Some(120),
            Element::Plsm2 => Some(30),
            Element::Fog => Some(320),
            Element::Smke2 => Some(220),
            Element::Boyl => Some(420),
            Element::Gas2 => Some(520),
            Element::Nble => Some(560),
            Element::Elec => Some(36),
            Element::Posi => Some(36),
            Element::Bray => Some(42),
            Element::Amtr => Some(30),
            Element::Trit => Some(560),
            _ => None,
        }
    }

    pub fn dispersion(&self) -> i32 {
        match self {
            Element::Water => 5,
            Element::SaltWater => 4,
            Element::Mercury => 5,
            Element::Oil => 4,
            Element::Alcohol => 5,
            Element::Acid => 4,
            Element::Lava => 2,
            Element::Cryo => 4,
            Element::Hydrogen => 4,
            Element::Oxygen => 4,
            Element::Methane => 5,
            Element::Steam => 3,
            Element::Plasma => 3,
            Element::Smoke => 2,
            Element::Fire => 1,
            Element::Phot => 5,
            Element::Neut => 5,
            Element::Embr => 2,
            Element::Desl => 3,
            Element::Mrcr => 4,
            Element::GelL => 2,
            Element::Cflm => 2,
            Element::Bizrg => 5,
            Element::WarpG => 6,
            Element::Prot => 6,
            Element::Deut => 4,
            Element::Iszs => 4,
            Element::Pste => 1,
            Element::Mwax => 3,
            Element::Vxgl => 2,
            Element::Glow => 5,
            Element::Soap2 => 6,
            Element::Rbdm => 4,
            Element::Lrbd => 4,
            Element::Cbnw => 5,
            Element::Plsm2 => 4,
            Element::Fog => 3,
            Element::Smke2 => 2,
            Element::Boyl => 5,
            Element::Gas2 => 5,
            Element::Nble => 4,
            Element::Elec => 6,
            Element::Posi => 6,
            Element::Bray => 7,
            Element::Trit => 5,
            _ => 0,
        }
    }

    pub fn is_conductor(&self) -> bool {
        matches!(
            self,
            Element::Wire
                | Element::Metal
                | Element::Mercury
                | Element::SaltWater
                | Element::Battery
                | Element::Pscn
                | Element::Nscn
                | Element::Swch
                | Element::Aray
                | Element::Gold
                | Element::Iron
                | Element::Dray
                | Element::Cray
                | Element::Dtec
                | Element::Tsns
                | Element::Psns
                | Element::Pcln
                | Element::Pbcn
                | Element::Hswc
                | Element::Pump
                | Element::Gpmp
                | Element::WifiA
                | Element::WifiB
                | Element::Ptnm
                | Element::Nkel
                | Element::Zinc
                | Element::Copr
                | Element::Almn
                | Element::Ntct
                | Element::Ptct
                | Element::Ldtc
                | Element::Filt
                | Element::Stor
                | Element::Conv
                | Element::Dlay
                | Element::Cesm
        )
    }

    pub fn is_insulator(&self) -> bool {
        matches!(
            self,
            Element::Insulator
                | Element::Glass
                | Element::Wall
                | Element::Brick
                | Element::Fan
                | Element::Dmnd
                | Element::Ttan
                | Element::QrtzS
                | Element::Rbr
                | Element::Shld
                | Element::Insl2
                | Element::Invs
        )
    }

    pub fn ignite_temp(&self) -> Option<f32> {
        match self {
            Element::Sawdust | Element::Plant => Some(0.42),
            Element::Fuse => Some(0.38),
            Element::Wood | Element::Coal => Some(0.55),
            Element::Oil | Element::Alcohol => Some(0.48),
            Element::Methane | Element::Hydrogen | Element::Gunpowder => Some(0.40),
            Element::Bomb => Some(0.35),
            Element::Embr => Some(0.2),
            Element::Wax => Some(0.32),
            Element::Desl => Some(0.44),
            Element::Bcol => Some(0.52),
            Element::Yest => Some(0.36),
            Element::Ignt => Some(0.26),
            Element::Pste => Some(0.44),
            Element::Mwax => Some(0.30),
            Element::Rbdm | Element::Lrbd => Some(0.28),
            Element::Thrm => Some(0.24),
            Element::Nitr => Some(0.20),
            _ => None,
        }
    }

    pub fn thermal_phase(&self, heat: f32) -> Option<Element> {
        match self {
            Element::Water if heat < 0.08 => Some(Element::Ice),
            Element::Water if heat > 0.70 => Some(Element::Steam),
            Element::SaltWater if heat < 0.05 => Some(Element::Ice),
            Element::SaltWater if heat > 0.75 => Some(Element::Steam),
            Element::Ice if heat > 0.30 => Some(Element::Water),
            Element::Snow if heat > 0.28 => Some(Element::Water),
            Element::Steam if heat < 0.18 => Some(Element::Water),
            Element::Lava if heat < 0.38 => Some(Element::Stone),
            Element::Stone if heat > 0.95 => Some(Element::Lava),
            Element::Methane if heat > 0.50 => Some(Element::Fire),
            Element::Hydrogen if heat > 0.45 => Some(Element::Plasma),
            Element::Embr if heat < 0.08 => Some(Element::Smoke),
            Element::Wax if heat > 0.33 => Some(Element::Desl),
            Element::Desl if heat < 0.20 => Some(Element::Wax),
            Element::Gel if heat > 0.24 => Some(Element::GelL),
            Element::GelL if heat < 0.16 => Some(Element::Gel),
            Element::Mwax if heat < 0.22 => Some(Element::Wax),
            Element::Wax if heat > 0.36 => Some(Element::Mwax),
            Element::Qrtz if heat > 0.60 => Some(Element::QrtzS),
            Element::QrtzS if heat > 0.96 => Some(Element::Vxgl),
            Element::Vxgl if heat < 0.58 => Some(Element::QrtzS),
            Element::Rbdm if heat < 0.25 => Some(Element::Lrbd),
            Element::Lrbd if heat > 0.42 => Some(Element::Rbdm),
            Element::Soap2 if heat < 0.10 => Some(Element::Soap),
            Element::Soap if heat > 0.34 => Some(Element::Soap2),
            Element::Cbnw if heat > 0.66 => Some(Element::Steam),
            _ => None,
        }
    }

    pub fn is_special(&self) -> bool {
        matches!(
            self,
            Element::Wall
                | Element::Cloner
                | Element::Void
                | Element::Spark
                | Element::Fan
                | Element::Grav
                | Element::Bhol
                | Element::Whol
                | Element::Dest
                | Element::PortalIn
                | Element::PortalOut
                | Element::Sing
                | Element::Pipe
                | Element::Shld
                | Element::Frme
                | Element::Stor
                | Element::Pvod
                | Element::Invs
                | Element::Conv
                | Element::Filt
                | Element::Dlay
        )
    }
}
