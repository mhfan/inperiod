/****************************************************************
 * $ID: l10n.rs     Tue 03 Dec 2024 10:05:21+0800               *
 *                                                              *
 * Maintainer: 范美辉 (MeiHui FAN) <mhfan@ustc.edu>              *
 * Copyright (c) 2024 M.H.Fan, All rights reserved.             *
 ****************************************************************/

use std::collections::HashMap;

pub struct Localization {
    data: Vec<HashMap<&'static str, &'static str>>,
    lidx: Option<u8>,
}

macro_rules! hashmap_init {
    ($($key:expr => $value:expr),* $(,)?) => { {
        let mut map = HashMap::new();
        $(map.insert($key, $value);)*
        map
    } }
}

impl Localization {
    pub fn translate<'a, 'b>(&self, id: &'a str) -> &'b str where 'a:'b {
        if let Some(lidx) = self.lidx {
            self.data[lidx as usize].get(id).copied().unwrap_or(id)
        } else { id }
    }

    //pub fn get(&self) -> &str { match self.lidx { Some(0) => "zh-CN", _ => "en-US" } }

    pub fn set<T: AsRef<str>>(&mut self, lang: T) {
        self.lidx = if "zh-CN" == lang.as_ref() { Some(0) } else { None };
    }

    #[allow(clippy::new_without_default)] pub fn new() -> Self {
#[allow(non_snake_case)] let zh_CN = hashmap_init!(
    "Periodic Table of the Elements" => "化　学　元　素　周　期　表",

    "Aufbau" => "构造",
    "Binding" => "结合能",
    "Abundances" => "丰度分布",
    "Bubble" => "气泡图",
    "Solar" => "太阳系",
    "Earth" => "地球",
    "Human" => "人体",

    "The most abundant isotopes" => "最丰富的同位素",
    "Relative abundance is proportional to the area." => "面积比例对应相对丰度",
    "Isotopes with equal numbers of protons and neutrons are unusually abundant." =>
        "有相同数量的质子和中子的元素通常是最丰富的",
    "(large blue circle) comprises 74% of the ordinary matter of the universe." =>
        "(大蓝色圆) 占据了宇宙中所有物质质量的 74%",
    "Color corresponds to nucleosynthetic process" => "不同颜色对应于核合成过程",
    "Yellow" => "黄色",
    "Green"  => "绿色",
    "Blue"   => "蓝色",

    "PERIOD" => "周 期",
    "-block" => " 区",
    "GROUP"  => "族",
    "plus"   => "加",
    "exclude" => "去除",
    "series"  => "系列",
    "E-max"   => "满电子数",
    "E-shell" => "电子壳层",
    "metal - nonmetal divider" => "金属 - 非金属 分隔线",

    "Phase at STP" => "物态 (STP)",
    "Liquid" => "液体",
    "Gas"    => "气体",
    "Solid"  => "固体",
    "Synthetic"  => "人工合成",

    "Categories" => "类别/分组",
    "Alkali metals" => "碱金属",
    "Alkaline earth metals" => "碱土金属",
    "Rare earth metals" => "稀土金属",
    "Transition metals" => "过渡金属",
    "Other nonmetals" => "其它非金属",
    "Noble gases" => "稀有气体",
    "Poor metals" => "贫金属",
    "Metalloids"  => "类金属",
    "Lanthanoids" => "镧系金属",
    "Lanthanides" => "镧系元素",
    "Actinides" => "锕系元素",
    "Actinoids" => "锕系金属",
    "Halogens" => "卤素",
    "Unknown"  => "未知",

    "Flame test" => "焰色反应",
    "Cosmic origin" => "宇宙起源",
    "Big Bang fusion" => "大爆炸聚变",
    "Cosmic ray fission" => "宇宙射线裂变",
    "Merging neutron stars" => "中子星合并",
    "Dying low-mass stars"  => "小质量恒星死亡",
    "Exploding massive stars" => "大质量恒星爆发",
    "White dwarf supernovae"  => "白矮星超新星",
    "Human synthesis" => "人工合成",
    "Radioactive decay"  => "放射性衰变",
    "No stable isotopes" => "无稳定同位素",

    "Common physical constants" => "常用物理常数",
    "Source: " => "来源: ",
    "electron mass" => "电子质量",
    "Rydberg constant" => "里德伯常量",
    "atomic mass unit" => "原子质量单位",
    "fine-structure const." => "精细结构常数",
    "Newtonian const. of gravitation" => "万有引力常数",
    "molar volume of ideal gas" => "理想气体的摩尔体积",
    "classical electron radius" => "经典电子半径",
    "first radiation constant"  => "第一辐射常量",
    "second radiation constant" => "第二辐射常量",
    "hyperfine transition freq." => "超精细转换频率",

    "Avogadro constant"  => "阿伏伽德罗常数",
    "Boltzmann constant" => "玻尔兹曼常数",
    "Planck constant"  => "普朗克常数",
    "Faraday constant" => "法拉第常数",
    "molar gas constant" => "摩尔气体常数",
    "elementary charge"  => "基本电荷",
    "speed of light in vacuum" => "真空光速",

    "Notes" => "说明",
    "in Dalton or" => "道尔顿 或",
    "Standard atomic mass" => "标准相对原子量",
    "the weighted arithmetic mean of the relative isotopic masses of all isotopes of an element, weighted by their abundance on Earth" =>
        "一个元素所有同位素相对原子量的加权算术平均值, 以其在地球上的丰度分布为权重",
    "indicate mass number of most stable isotope" => "表示放射性元素最稳定同位素的相对原子量",
    "Density units"  => "密度单位",
    "for solids and" => "- 固体,",
    "for liquid or"  => "- 液体,",
    "for gases at 0° Celsius" => "- 气体 (0 ℃)",
    "mark means the electronegativity is in the bottom-right" => "表示电负性显示在右下角",
    "van der Waals radii" => "范德华半径",

    " include" => "包括",
    "Here " => "这里的",
    " is "  => "是指",
    " point" => "",
    "and" => "和",

    "References" => "参考",
    "All rights reserved." => "版权所有",

    "radioactive" => "放射性",
    "atomic weight" => "原子量",
    "1st ionization energy" => "一级电离能",
    "symbol"  => "符号",
    "name"    => "名称",
    "melting" => "熔点",
    "boiling" => "沸点",
    "density" => "密度",
    "atomic number" => "原子序数",
    "electron affinity" => "电子亲和性",
    "electron configuration" => "电子排布",
    "main oxidation states"  => "常见/主要氧化态",
    "Chinese name with pinyin" => "中文名称和拼音",
    "crystal structure"  => "晶体结构",
    "ground-state level" => "基态级别",
    "electronegativity"  => "电负性",
    "atomic radius" => "原子半径",

    "Electron shell/orbital configuration" => "电子的壳层/轨道配置排布",
    "Energy increase (not to scale)" => "能 量 增 加 (不成比例)",
    "Principle quantum number" => "主量子数",
    "Azimuthal quantum number" => "角量子数",
    "Aufbau Principle" => "构造原理",
    "Madelung rule" => "马德兰规则",

    "Hexagonal" => "六方晶系", //双六方密堆积 (DHCP)
    "Hexagonal close packed" => "六方密堆积",
    "Cubic body centered" => "体心立方晶系",
    "Cubic face centered" => "面心立方晶系",
    "Diamond cubic crystal structure" => "金刚石 (钻石) 结构",
    "Face centered orthorhombic" => "面心正交晶系",
    "Body centered tetragonal" => "体心四方",
    "Rhombohedral" => "三方晶系 (菱方)",
    "Orthorhombic" => "正交晶系 (斜方)",
    "Cubic" => "立方晶系 (等轴)",
    "Monoclinic" => "单斜晶系",
);

        Self { data: vec![zh_CN], lidx: None }
    }
}

