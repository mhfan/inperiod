/****************************************************************
 * $ID: lib.rs  	Mon 21 Oct 2024 16:38:01+0800               *
 *                                                              *
 * Maintainer: 范美辉 (MeiHui FAN) <mhfan@ustc.edu>              *
 * Copyright (c) 2024 M.H.Fan, All rights reserved.             *
 ****************************************************************/

//! module/crate level documentation
// src/lib.rs (default library entry point)

/*  https://github.com/lmmentel/mendeleev
    https://www.webelements.com/periodicity/contents/
    https://en.wikipedia.org/wiki/Category:Chemical_element_data_pages
    https://physics.nist.gov/PhysRefData/ASD/ionEnergy.html
    https://pubchem.ncbi.nlm.nih.gov/periodic-table/
    https://www.nist.gov/pml/periodic-table-elements
    https://ciaaw.org/atomic-weights.htm
    https://github.com/baotlake/periodic-table-pro
    https://github.com/Bowserinator/Periodic-Table-JSON
    https://periodictable.com/Properties/A/HumanAbundance.html

    https://commons.wikimedia.org/wiki/File:元素周期表.png
    https://commons.wikimedia.org/wiki/File:Periodic_table_large.svg
    https://www.futurity.org/periodic-table-new-elements-1087782-2/
    https://www.vertex42.com/Files/pdfs/2/periodic-table_color.pdf
    https://www.vertex42.com/ExcelTemplates/periodic-table-of-elements.html
    https://commons.wikimedia.org/wiki/File:Nucleosynthesis_periodic_table.svg
    https://iupac.org/what-we-do/periodic-table-of-elements/, https://svs.gsfc.nasa.gov/13873/
    https://commons.wikimedia.org/wiki/File:Periodic_Table_-_Atomic_Properties_of_the_Elements.png
    https://commons.wikimedia.org/wiki/File:Periodic_Table_of_the_elements.jpg
    https://commons.wikimedia.org/wiki/File:Periodic_table_detailed_enwp.svg
    https://elements.wlonk.com/index.htm, https://ptable.com
    https://en.wikipedia.org/wiki/Periodic_table */

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)] pub enum ChemElem { // #[non_exhaustive]
    /** Hydrogen      */ H = 1,
    /** Helium        */ He,
    /** Lithium       */ Li,
    /** Beryllium     */ Be,
    /** Boron         */ B,
    /** Carbon        */ C,
    /** Nitrogen      */ N,
    /** Oxygen        */ O,
    /** Fluorine      */ F,
    /** Neon          */ Ne,
    /** Sodium        */ Na,
    /** Magnesium     */ Mg,
    /** Aluminum      */ Al,
    /** Silicon       */ Si,
    /** Phosphorus    */ P,
    /** Sulfur        */ S,
    /** Chlorine      */ Cl,
    /** Argon         */ Ar,
    /** Potassium     */ K,
    /** Calcium       */ Ca,
    /** Scandium      */ Sc,
    /** Titanium      */ Ti,
    /** Vanadium      */ V,
    /** Chromium      */ Cr,
    /** Manganese     */ Mn,
    /** Iron          */ Fe,
    /** Cobalt        */ Co,
    /** Nickel        */ Ni,
    /** Copper        */ Cu,
    /** Zinc          */ Zn,
    /** Gallium       */ Ga,
    /** Germanium     */ Ge,
    /** Arsenic       */ As,
    /** Selenium      */ Se,
    /** Bromine       */ Br,
    /** Krypton       */ Kr,
    /** Rubidium      */ Rb,
    /** Strontium     */ Sr,
    /** Yttrium       */ Y,
    /** Zirconium     */ Zr,
    /** Niobium       */ Nb,
    /** Molybdenum    */ Mo,
    /** Technetium    */ Tc,
    /** Ruthenium     */ Ru,
    /** Rhodium       */ Rh,
    /** Palladium     */ Pd,
    /** Silver        */ Ag,
    /** Cadmium       */ Cd,
    /** Indium        */ In,
    /** Tin           */ Sn,
    /** Antimony      */ Sb,
    /** Tellurium     */ Te,
    /** Iodine        */ I,
    /** Xenon         */ Xe,
    /** Cesium        */ Cs,
    /** Barium        */ Ba,
    /** Lanthanum     */ La,
    /** Cerium        */ Ce,
    /** Praseodymium  */ Pr,
    /** Neodymium     */ Nd,
    /** Promethium    */ Pm,
    /** Samarium      */ Sm,
    /** Europium      */ Eu,
    /** Gadolinium    */ Gd,
    /** Terbium       */ Tb,
    /** Dysprosium    */ Dy,
    /** Holmium       */ Ho,
    /** Erbium        */ Er,
    /** Thulium       */ Tm,
    /** Ytterbium     */ Yb,
    /** Lutetium      */ Lu,
    /** Hafnium       */ Hf,
    /** Tantalum      */ Ta,
    /** Tungsten      */ W,
    /** Rhenium       */ Re,
    /** Osmium        */ Os,
    /** Iridium       */ Ir,
    /** Platinum      */ Pt,
    /** Gold          */ Au,
    /** Mercury       */ Hg,
    /** Thallium      */ Tl,
    /** Lead          */ Pb,
    /** Bismuth       */ Bi,
    /** Polonium      */ Po,
    /** Astatine      */ At,
    /** Radon         */ Rn,
    /** Francium      */ Fr,
    /** Radium        */ Ra,
    /** Actinium      */ Ac,
    /** Thorium       */ Th,
    /** Protactinium  */ Pa,
    /** Uranium       */ U,
    /** Neptunium     */ Np,
    /** Plutonium     */ Pu,
    /** Americium     */ Am,
    /** Curium        */ Cm,
    /** Berkelium     */ Bk,
    /** Californium   */ Cf,
    /** Einsteinium   */ Es,
    /** Fermium       */ Fm,
    /** Mendelevium   */ Md,
    /** Nobelium      */ No,
    /** Lawrencium    */ Lr,
    /** Rutherfordium */ Rf,
    /** Dubnium       */ Db,
    /** Seaborgium    */ Sg,
    /** Bohrium       */ Bh,
    /** Hassium       */ Hs,
    /** Meitnerium    */ Mt,
    /** Darmstadtium" */ Ds,
    /** Roentgenium   */ Rg,
    /** Copernicium   */ Cn,
    /** Nihonium      */ Nh,
    /** Flerovium     */ Fl,
    /** Moscovium     */ Mc,
    /** Livermorium   */ Lv,
    /** Tennessine    */ Ts,
    /** Oganesson     */ Og,
    /** Count + 1     */ MAX,
}

impl From<u8> for ChemElem {
    fn from(val: u8) -> Self {
        assert!(0 < val && val < ChemElem::MAX as u8, "Invalid element/atomic number!");
        unsafe { std::mem::transmute(val) }
    }
}

pub struct ElemIter(u8);
impl ElemIter { const fn new() -> Self { ElemIter(0) } }
impl Iterator for ElemIter {      type Item = ChemElem;
    fn next(&mut self) -> Option<Self::Item> { self.0 += 1;
        if self.0 < ChemElem::MAX as u8 { Some(self.0.into()) } else { None }
    }
}

impl ChemElem {
    pub const fn name   (&self) -> &'static str { ELEM_NAME  [*self as usize] }
    pub const fn symbol (&self) -> &'static str { ELEM_SYMBOL[*self as usize] }
    pub const fn name_py(&self) -> &'static str { ELEM_PY[*self as usize] }
    pub const fn name_ch(&self) -> char         { ELEM_CH[*self as usize] } // XXX: &str or String?
    pub const fn atomic_number(&self) -> u8 { *self as _ }
    pub const fn iter() -> ElemIter { ElemIter::new() }
    pub fn list() -> Vec<ChemElem> { ChemElem::iter().collect() }

    /** ```
        use inperiod::ChemElem;

        assert_eq!(ChemElem::H.name(), "Hydrogen");
        assert_eq!(ChemElem::from_str("x"), None);
        assert_eq!(ChemElem::from_str("H"), Some(ChemElem::H));
        assert_eq!(ChemElem::iter().count(), ChemElem::MAX as usize - 1);

        if let Some(elem) = ChemElem::from_str("Iron") {
            assert_eq!(elem.atomic_number(), 26);
            assert_eq!(elem.name(), "Iron");
            assert_eq!(elem.symbol(), "Fe");
            assert_eq!(elem.name_ch(), '铁');
            assert_eq!(elem.name_py(), "tiě");
        } else { unreachable!() }
    ``` */
    #[allow(clippy::should_implement_trait)]
    pub fn from_str<S: AsRef<str>>(s: S) -> Option<Self> {
        let s = s.as_ref().trim();
        let len = s.len();

        if  len < 3 {
            if len == 1 {   let ch = s.chars().next()?;
                if let Some(x) = ELEM_CH.iter().position(|&x| x == ch) {
                    return Some((x as u8).into());
                }
            }
            ELEM_SYMBOL.iter().position(|&x| x == s).map(|x| (x as u8).into())
        } else {
            ELEM_NAME  .iter().position(|&x| x == s).map(|x| (x as u8).into())
        }
    }

    /// https://en.wikipedia.org/wiki/Group_(periodic_table)#List_of_group_names
    pub const fn group(&self) -> u8 {   // XXX: cache/save it for frequent access?
        match self.atomic_number() {
            1| 3|11|19|37|55|87  => 1,  // Alkali metals
               4|12|20|38|56|88  => 2,  // Alkaline earth metals
                    21|39|71|103 => 3,  // Transition metals (group 3~12)
                    22|40|72|104 => 4,
                    23|41|73|105 => 5,
                    24|42|74|106 => 6,
                    25|43|75|107 => 7,
                    26|44|76|108 => 8,
                    27|45|77|109 => 9,
                    28|46|78|110 => 10,
                    29|47|79|111 => 11, // Coinage metals
                    30|48|80|112 => 12,
               5|13|31|49|81|113 => 13, // Triels
               6|14|32|50|82|114 => 14, // Tetrels
               7|15|33|51|83|115 => 15, // Pnictogens
               8|16|34|52|84|116 => 16, // Chalcogens
               9|17|35|53|85|117 => 17, // Halogens
            2|10|18|36|54|86|118 => 18, // Noble gases
                57..=70|89..=102 => 19, // f-group/block (Lanthanides and Actinides)
            _ => unreachable!(),
        }
    }

    pub const fn group_to_name(group: u8) -> &'static str {
        assert!(0 < group && group < 19);
        const GROUP_SYMBOL: [&str; 19] = [ "", // placeholder
            "IA", "IIA", "IIIB", "IVB", "VB", "VIB", "VIIB", "VIIIB", "VIIIB", "VIIIB",
            "IB", "IIB", "IIIA", "IVA", "VA", "VIA", "VIIA", "VIIIA", ];
        GROUP_SYMBOL[group as usize]
    }

    /// https://en.wikipedia.org/wiki/Periodic_table#Classification_of_elements
    pub const fn category(&self) -> ElemClass {
        match self.atomic_number() {
            1|6|7|8|15|16|34 => ElemClass::OtherNonmetals,
            5|14|32|33|51|52 => ElemClass::Metalloids,  // semi-metals
            13|31|49|50|81..=84|113..117 => ElemClass::PoorMetals,  // post-transition metals
            //109..=118 if self.atomic_number() != 112 => ElemClass::Unknown("Unknown"),

            // Rare earth metals  (Lanthanoids plus Sc and Y)
            57..=70  => ElemClass::Lanthanoids,     // Lanthanides (include Lu)
            89..=102 => ElemClass::Actinoids,       // Actinides   (include Lr)
            _ => match self.group() {
                1  => ElemClass::AlkaliMetals,
                2  => ElemClass::AlkalineEarthMetals,
                17 => ElemClass::Halogens,
                18 => ElemClass::NobleGases,
                _  => ElemClass::TransitionMetals,  // 3..=12
            }
        }
    }

    pub const fn period(&self) -> u8 {
        match self.atomic_number() {
            1|2 => 1, 3..=10 => 2, 11..=18 => 3, 19..=36 => 4,
            37..=54 => 5, 55..=86 => 6, 87..=118 => 7, _ => unreachable!(),
        }
    }

    pub const fn block(&self) -> u8 {
        match self.group() { 1|2 => b's', 3..=12 => b'd', 13..=18 => b'p', _ => b'f', }
    }

    /// https://ciaaw.org/radioactive-elements.htm
    pub const fn is_ratioactive(&self) -> bool { matches!(self.atomic_number(), 43|61|84..=118) }

    /// XXX: https://en.wikipedia.org/wiki/Periodic_table_(crystal_structure)
    //  https://github.com/baotlake/periodic-table-pro/blob/37239360e6f5daa605b3fd947895ed2dfdce0cd7/packages/data/json/crystalStructure.json
    //  https://environmentalchemistry.com/yogi/periodic/crystal.html
    //  https://periodictable.com/Properties/A/CrystalStructure.html
    pub const fn crystal_structure(&self) -> Option<(&'static str, &'static str)> {
        /*return Some(match self.atomic_number() {
            1|6|7|12|30|34|43|44|48|52|71|75|76 => ("hex", "Hexagonal.svg"), // HCP
            _ => return None, // ("-", "")
        });

        #[allow(unreachable_code)] */Some(match self.atomic_number() {
            1|6|7|57..=61|95..=98 => ("hex", "Hexagonal.svg"),       // 六方晶系, 双六方密堆积 (DHCP)
            2|4|12|21|22|27|30|39|40|43|44|48|64..=69|71|72|75|76|81|
                103|104|107|108|112|113 => ("HCP", "Hexagonal_close_packed.svg"), // 六方密堆积
            3|11|15|19|23..=26 |37|41|42|55|56|63| 73 |
                74| 87 |88|105|106|110|111 => ("BCC", "Cubic-body-centered.svg"), // 体心立方晶系
            5|33|51|62|80|83| 34|52 => ("rhom", "Rhombohedral.svg"), // 三方晶系 (菱方)
            8|9| 84 => ("cubic", "Cubic.svg"),  // 立方晶系 (等轴)
            10|13|18|20|28|29|36|38|45..=47|54|70|77..=79| 82 |85|86|89|
                90|99..=102|109|114| 118  => ("FCC", "Cubic-face-centered.svg"),  // 面心立方晶系
            14|32 => ("DC",  "Diamond_cubic_crystal_structure.svg"), // 金刚石 (钻石) 结构
            16|17|31|35|92|93 => ("BCO", "Orthorhombic.svg"),        // 正交晶系 (斜方)
            53 => ("FCO",  "Face-centered_orthorhombic.svg"),        // 面心正交晶系
            49| 50 |91 => ("tetra", "Body-centered_tetragonal.svg"), // 体心四方
            94 => ("mono", "Monoclinic.svg"),   // 单斜晶系
            _ => return None, // ("-", "")
        })
    }

    /// https://www.nist.gov/pml/periodic-table-elements
    /// https://physics.nist.gov/PhysRefData/ASD/ionEnergy.html
    pub const fn ground_state(&self) -> Option<(&'static str, &'static str, &'static str)> {
        self.ground_level()
    }

    /// https://en.wikipedia.org/wiki/Electronegativity     // XXX: other EN scaling?
    /// https://en.wikipedia.org/wiki/Electronegativities_of_the_elements_(data_page)
    pub const fn en_pauling(&self) -> Option<f32> { self.electronegativity() }

    // TODO: abundance, origin
}

pub mod ciaaw;
pub mod ostates;
pub mod pubchem;    //include!(concat!(env!("OUT_DIR"), "/pubchem.rs"));
pub mod nist_asd;
//pub mod en_pauling;
pub mod ground_level;

pub const ROMAN_NUM: [&str; 11] = [ "",
    "I", "II", "III", "IV", "V", "VI", "VII", "VIII", "IX", "X", ];
/// https://en.wikipedia.org/wiki/Unicode_subscripts_and_superscripts
pub const UNICODE_SUPERS: [char; 16] = [ //&str = r"⁰¹²³⁴⁵⁶⁷⁸⁹⁺⁻⁼⁽⁾ᐟ";
    '⁰', '¹', '²', '³', '⁴', '⁵', '⁶', '⁷', '⁸', '⁹', '⁺', '⁻', '⁼', '⁽', '⁾', 'ᐟ', ];
//const UNICODE_SUBS: [char; 16] = [ //&str = r"₀₁₂₃₄₅₆₇₈₉₊₋₌₍₎⸝";
//    '₀', '₁', '₂', '₃', '₄', '₅', '₆', '₇', '₈', '₉', '₊', '₋', '₌', '₍', '₎', 'ₐ', ];

#[derive(PartialEq, Debug)] pub enum AtomicWeight {
    //Interval(core::ops::RangeInclusive<f64>), // conversional?
    //Uncertainty { value: f64, uncertainty: f64 },
    Abridged { value: f32, uncertainty: f32 },
    MassNumber(u32),
}

/** ```
    use inperiod::AtomicWeight;
    assert!(" 1.0080 ".parse::<AtomicWeight>() ==
        Ok(AtomicWeight::Abridged { value: 1.008, uncertainty: 0. }));
    assert_eq!("1.0080 (2)".parse::<AtomicWeight>(),
        Ok(AtomicWeight::Abridged { value: 1.008, uncertainty: 0.0002 }));
    assert_eq!("1.0080(12)".parse::<AtomicWeight>().unwrap().to_string(), "1.0080(12)");
    assert_eq!("39.95 ± 0.16".parse::<AtomicWeight>().unwrap().to_string(), "39.95(16)");
    assert_eq!("[294]".parse::<AtomicWeight>(), Ok(AtomicWeight::MassNumber(294)));
    assert_eq!("1.0080(2)".parse::<AtomicWeight>().unwrap(),
        "1.0080 ±  0.0002".parse::<AtomicWeight>().unwrap());
    assert!("9.109 383 7139(28)".parse::<AtomicWeight>().is_ok());
``` */
impl std::str::FromStr for AtomicWeight {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.trim();

        if s.starts_with('[') && s.ends_with(']') {
            let s = s[1..s.len()-1].trim();
            /* if let Some((start, end)) = s.split_once(',') {
                let start = start.trim_end().parse().map_err(|_| "Invalid value")?;
                let end   = end.trim_start().parse().map_err(|_| "Invalid value")?;
                Ok(AtomicWeight::Interval(core::ops::RangeInclusive::new(start, end)))
            } else { */
                Ok(AtomicWeight::MassNumber(s.parse().map_err(|_| "Invalid value")?))
            //}
        } else if let Some((value_part, uncertainty_part)) = s.split_once('±') {
            let value = value_part.trim_end().parse().map_err(|_| "Invalid value")?;
            let uncertainty = uncertainty_part.trim_start()
                .parse().map_err(|_| "Invalid uncertainty")?;
            Ok(AtomicWeight::Abridged { value, uncertainty })
        } else if let Some((value_part, rest)) = s.split_once('(') {
            let value_part = value_part.replace(' ', "");
            let value = value_part.parse().map_err(|_| "Invalid value")?;
            let uncertainty_str = rest.trim_end_matches(')').trim();

            let scale = if let Some(pos) = value_part.find('.') {
                10f32.powi((value_part.len() - pos - 1) as i32)
            } else { 1. };

            let uncertainty = uncertainty_str.parse::<u8>().map_err(|_|
                "Invalid uncertainty")? as f32 / scale;
            Ok(AtomicWeight::Abridged { value, uncertainty })
        } else {
            let value = s.parse().map_err(|_| "Invalid value")?;
            Ok(AtomicWeight::Abridged { value, uncertainty: 0. })
        }
    }
}

use core::fmt;
impl fmt::Display for AtomicWeight {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AtomicWeight::MassNumber(number) => write!(f, "[{}]", number),
            AtomicWeight::Abridged { value, uncertainty } => {
                if *uncertainty == 0. { return write!(f, "{value}") }
                //return write!(f, "{value} ± {uncertainty}")?;

                if *uncertainty < 1. {
                    let mut prec = (-uncertainty.log10()).ceil() as i32;
                    let mut digit =  uncertainty * 10f32.powi(prec);
                    while f32::EPSILON * 10. < digit.fract() &&
                        digit.fract() < 1. - f32::EPSILON * 10. {  prec += 1; digit *= 10.; }
                    write!(f, "{value:.prec$}({})", digit.round(), prec = prec as usize)
                } else { write!(f, "{value}({uncertainty})") }
            }
        }
    }
}

/* fn parse_ecfg(ecfg: &str) -> ElectronCFG {
    let (base, rest) = ecfg.trim_start().find(']').map_or((None, ecfg),
        |pos| (ChemElem::from_str(&ecfg[1..pos]), &ecfg[pos+1..]));

    let valence = rest.split_ascii_whitespace().filter_map(|part| {
        let mut chars = part.chars();
        let level = chars.next()?.to_digit(10)? as u8;
        let shell = chars.next()? as u8;
        Some(Subshell { level, orbital, ecount: part[2..].parse::<u8>().ok()? })
    }).collect();

    ElectronCFG { base, valence }
} */

/** ```
    use inperiod::ChemElem;
    assert_eq!(ChemElem::H .electron_configuration().to_string(), "1s");
    assert_eq!(ChemElem::He.electron_configuration().to_string(), "1s²");
    let ecfg = ChemElem::Og.electron_configuration();
    assert_eq!(ecfg.to_string(), "[Rn] 5f¹⁴ 6d¹⁰ 7s² 7p⁶");
    assert_eq!(ecfg.expand().map(|x| x.to_string()).collect::<Vec<_>>().join(" "),
        "1s² 2s² 2p⁶ 3s² 3p⁶ 3d¹⁰ 4s² 4p⁶ 4d¹⁰ 5s² 5p⁶ 4f¹⁴ 5d¹⁰ 6s² 6p⁶ 5f¹⁴ 6d¹⁰ 7s² 7p⁶");
    // https://www.webelements.com/oganesson/atoms.html
``` */
impl fmt::Display for ElectronCFG {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut first = true;
        if let Some(base) = self.base {     first = false;
            write!(f, "[{}]", base.symbol())?;
        }
        for subshell in self.valence {
            if !first { write!(f, " ")?; }
            write!(f, "{subshell}")?;
        }   Ok(())
    }
}

impl fmt::Display for Subshell {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}", self.level, self.orbital as char)?;
        if 1 < self.ecount {
            let mut ecount = self.ecount as usize;
            if 9 <  ecount {     // XXX: max two digits
                write!(f, "{}", UNICODE_SUPERS[ecount / 10])?;
                ecount %= 10;
            }   write!(f, "{}", UNICODE_SUPERS[ecount])?;
        }   Ok(())
    }
}

/// shell: K/L/M/N/O/P/Q/R (n: 1 ~ 8), capacity: 2 * n^2
///
/// orbital: s/p/d/f/g/h/i (l: 0 ~ 6), capacity: 4 * (l + 1) - 2
pub struct ElectronCFG {
    /// The noble gas of the preceding period, if any
    base: Option<ChemElem>,
    /// The subshells in the valence shell
    valence: &'static [Subshell],
}

impl ElectronCFG {
    pub fn expand(&self) -> Box<dyn Iterator<Item = &'static Subshell>> {
        if let Some(elem) = self.base {
            Box::new(elem.electron_configuration().expand().chain(self.valence.iter()))
        } else { Box::new(self.valence.iter()) }
    }
}

/// A subshell (s, p, d, or f) in the electronic configuration
pub struct Subshell {
    /// The shell's principal quantum number, energy level
    pub level: u8,
    /// The label type of orbital/subshell, based on its azimuthal quantum number
    pub orbital: u8, // OrbitalType,
    /// The number of electrons in this oribital
    pub ecount: u8,
}

impl From<(u8, u8, u8)> for Subshell {
    fn from(val: (u8, u8, u8)) -> Self { Self { level: val.0, orbital: val.1, ecount: val.2 } }
}

/// Electron subshell type, based on the azimuthal quantum number ℓ
#[repr(u8)] pub enum OrbitalType {
    /** ℓ = 0, historical name "Sharp" */       S = b's',
    /** ℓ = 1, historical name "Principal" */   P = b'p',
    /** ℓ = 2, historical name "Diffuse" */     D = b'd',
    /** ℓ = 3, historical name "Fundamental" */ F = b'f',
    /** ℓ = 4, no historical name */            G = b'g',
}

#[macro_export] macro_rules! ecfg {
    ($base:ident, $($valence:expr),+ $(,)?) => {
        ElectronCFG { base: Some(ChemElem::$base), valence: &[$($valence),+] }
    };
    ($($valence:expr),* $(,)?) => { ElectronCFG { base: None, valence: &[$($valence),*] } };
}

#[macro_export] macro_rules! ssc {
    ($l:expr, $t:literal, $n:expr) => { Subshell { level: $l, orbital: $t, ecount: $n, } };
}

#[repr(u8)] #[derive(Debug)] pub enum ElemClass {
    Unknown = 0,
    AlkaliMetals,
    AlkalineEarthMetals,
    TransitionMetals,
    PoorMetals,
    Metalloids,
    OtherNonmetals,
    Halogens,
    NobleGases,
    Lanthanoids,
    Actinoids,
}

const ELEM_SYMBOL: [&str; ChemElem::MAX as usize] = [ "", // placeholder
     "H", "He", "Li", "Be",  "B",  "C",  "N",  "O",  "F", "Ne",
    "Na", "Mg", "Al", "Si",  "P",  "S", "Cl", "Ar",  "K", "Ca",
    "Sc", "Ti",  "V", "Cr", "Mn", "Fe", "Co", "Ni", "Cu", "Zn",
    "Ga", "Ge", "As", "Se", "Br", "Kr", "Rb", "Sr",  "Y", "Zr",
    "Nb", "Mo", "Tc", "Ru", "Rh", "Pd", "Ag", "Cd", "In", "Sn",
    "Sb", "Te",  "I", "Xe", "Cs", "Ba", "La", "Ce", "Pr", "Nd",
    "Pm", "Sm", "Eu", "Gd", "Tb", "Dy", "Ho", "Er", "Tm", "Yb",
    "Lu", "Hf", "Ta",  "W", "Re", "Os", "Ir", "Pt", "Au", "Hg",
    "Tl", "Pb", "Bi", "Po", "At", "Rn", "Fr", "Ra", "Ac", "Th",
    "Pa",  "U", "Np", "Pu", "Am", "Cm", "Bk", "Cf", "Es", "Fm",
    "Md", "No", "Lr", "Rf", "Db", "Sg", "Bh", "Hs", "Mt", "Ds",
    "Rg", "Cn", "Nh", "Fl", "Mc", "Lv", "Ts", "Og",
];

const ELEM_NAME:   [&str; ChemElem::MAX as usize] = [ "", // placeholder
    "Hydrogen", "Helium", "Lithium", "Beryllium", "Boron",
    "Carbon", "Nitrogen", "Oxygen", "Fluorine", "Neon",
    "Sodium", "Magnesium", "Aluminum", "Silicon", "Phosphorus",
    "Sulfur", "Chlorine", "Argon", "Potassium", "Calcium",
    "Scandium", "Titanium", "Vanadium", "Chromium", "Manganese",
    "Iron", "Cobalt", "Nickel", "Copper", "Zinc",
    "Gallium", "Germanium", "Arsenic", "Selenium", "Bromine",
    "Krypton", "Rubidium", "Strontium", "Yttrium", "Zirconium",
    "Niobium", "Molybdenum", "Technetium", "Ruthenium", "Rhodium",
    "Palladium", "Silver", "Cadmium", "Indium", "Tin",
    "Antimony", "Tellurium", "Iodine", "Xenon", "Cesium",
    "Barium", "Lanthanum", "Cerium", "Praseodymium", "Neodymium",
    "Promethium", "Samarium", "Europium", "Gadolinium", "Terbium",
    "Dysprosium", "Holmium", "Erbium", "Thulium", "Ytterbium",
    "Lutetium", "Hafnium", "Tantalum", "Tungsten", "Rhenium",
    "Osmium", "Iridium", "Platinum", "Gold", "Mercury",
    "Thallium", "Lead", "Bismuth", "Polonium", "Astatine",
    "Radon", "Francium", "Radium", "Actinium", "Thorium",
    "Protactinium", "Uranium", "Neptunium", "Plutonium", "Americium",
    "Curium", "Berkelium", "Californium", "Einsteinium", "Fermium",
    "Mendelevium", "Nobelium", "Lawrencium", "Rutherfordium", "Dubnium",
    "Seaborgium", "Bohrium", "Hassium", "Meitnerium", "Darmstadtium",
    "Roentgenium", "Copernicium", "Nihonium", "Flerovium", "Moscovium",
    "Livermorium", "Tennessine", "Oganesson",
];

const ELEM_CH: [char; ChemElem::MAX as usize] = [ ' ', // placeholder
    '氢', '氦', '锂', '铍', '硼', '碳', '氮', '氧', '氟', '氖',
    '钠', '镁', '铝', '硅', '磷', '硫', '氯', '氩', '钾', '钙',
    '钪', '钛', '钒', '铬', '锰', '铁', '钴', '镍', '铜', '锌',
    '镓', '锗', '砷', '硒', '溴', '氪', '铷', '锶', '钇', '锆',
    '铌', '钼', '锝', '钌', '铑', '钯', '银', '镉', '铟', '锡',
    '锑', '碲', '碘', '氙', '铯', '钡', '镧', '铈', '镨', '钕',
    '钷', '钐', '铕', '钆', '铽', '镝', '钬', '铒', '铥', '镱',
    '镥', '铪', '钽', '钨', '铼', '锇', '铱', '铂', '金', '汞',
    '铊', '铅', '铋', '钋', '砹', '氡', '钫', '镭', '锕', '钍',
    '镤', '铀', '镎', '钚', '镅', '锔', '锫', '锎', '锿', '镄',
    '钔', '锘', '铹', '𬬻', '𬭊', '𬭳', '𬭛', '𬭶', '鿏', '𫟼',
    '𬬭', '鿔', '鿭', '𫓧', '镆', '𫟷', '鿬', '鿫',
];

/// https://github.com/mozillazg/python-pinyin
const ELEM_PY: [&str; ChemElem::MAX as usize] = [ "", // placeholder
    "qīng", "hài", "lǐ", "pí", "péng", "tàn", "dàn", "yǎng", "fú", "nǎi",
    "nà", "měi", "lǚ", "guī", "lín", "liú", "lǜ", "yà", "jiǎ", "gài",
    "kàng", "tài", "fán", "gè", "měng", "tiě", "gǔ", "niè", "tóng", "xīn",
    "jiā", "zhě", "shēn", "xī", "xiù", "kè", "rú", "sī", "yǐ", "gào",
    "ní", "mù", "dé", "liǎo", "lǎo", "bǎ", "yín", "gé", "yīn", "xī",
    "tī", "dì", "diǎn", "xiān", "sè", "bèi", "lán", "shì", "pǔ", "nǚ",
    "pǒ", "shān", "yǒu", "gá", "tè", "dī", "huǒ", "ěr", "diū", "yì",
    "lǔ", "hā", "tǎn", "wū", "lái", "é", "yī", "bó", "jīn", "gǒng",
    "tā", "qiān", "bì", "pō", "ài", "dōng", "fāng", "léi", "ā", "tǔ",
    "pú", "yóu", "ná", "bù", "méi", "jū", "péi", "kāi", "āi", "fèi",
    "mén", "nuò", "láo", "lú", "dù", "xǐ", "bō", "hēi", "mài", "dá",
    "lún", "gē", "nǐ", "fū", "mò", "lì", "tián", "ào",
];

//#[derive(PartialEq, Clone, Props)] // Owned props must implement `PartialEq`!
/* #[allow(unused)] pub struct ElemProps {
     group: u8,  // max: 18 (column)
    period: u8,  // max:  7 (row)

    block: u8,   // s, f, d, p
    //  metals (alkali, alkali-earth, lanthanoids, actinoids, transition, poor/other),
    //  metalloids/semi-metals and nonmetals (other, halogens, noble-gases)
    class: u8,

    ordinal: u8, // max: 118
    mass: f32,   // weight
    radioactive: bool,

    symbol: [u8; 2],
    name_ch: char,  //pinying: String, // https://github.com/mozillazg/rust-pinyin
    name: [u8; 24], //String,

    //  shell capacity: 4 * n^2, subshell capacity: 4 * (l + 1) - 2
    //  orbital: s/p/d/f/g/h/i (l: 0 ~ 6)
     config_e: [u8;  8], // enengy level
    oxidation: [i8; 10], // valence

    ionisation: f32, electroneg: f32, e_affinity: f32,
    density: f32, ground_s: String, radius: f32, m_v: bool, // metallic/covalent
    crystal_s: u8, melting: f32, boiling: f32, phase: u8,

    //discoverer: String, year: u16, //origin: u8,
    //  The Big-Bang, Dying low-mass stars, Exploding massive stars, Cosmic ray fission,
    //  White dwarf supernovae, Merging neutron stars, Radioactive decay, synthetic/human-made
    //abundance: f32,  // universe/galaxy, solar, crust, ocean, human body
} */

#[cfg(test)] mod tests {    use super::*;
    #[test] fn electron() {
        fn electron_count(elem: ChemElem) -> u8 {
            let ecfg = elem.electron_configuration();
            ecfg.valence.iter().map(|s| s.ecount).sum::<u8>()
                + ecfg.base.map_or(0, electron_count)
        }
        for elem   in ChemElem::iter() {
            assert_eq!(elem.atomic_number(), electron_count(elem));
        }
    }
}

