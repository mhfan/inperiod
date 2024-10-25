/****************************************************************
 * $ID: lib.rs  	Mon 21 Oct 2024 16:38:01+0800               *
 *                                                              *
 * Maintainer: 范美辉 (MeiHui FAN) <mhfan@ustc.edu>              *
 * Copyright (c) 2024 M.H.Fan, All rights reserved.             *
 ****************************************************************/

//! module/crate level documentation
// src/lib.rs (default library entry point)

/*  https://gitlab.com/ygor.souza/mendeleev, https://github.com/lmmentel/mendeleev
    https://en.wikipedia.org/wiki/Category:Chemical_element_data_pages
    https://physics.nist.gov/PhysRefData/ASD/ionEnergy.html
    https://pubchem.ncbi.nlm.nih.gov/periodic-table/
    https://www.nist.gov/pml/periodic-table-elements
    https://github.com/baotlake/periodic-table-pro
    https://github.com/Bowserinator/Periodic-Table-JSON
    https://github.com/sandmor/periodic-table-on-an-enum

    https://commons.wikimedia.org/wiki/File:元素周期表.png
    https://commons.wikimedia.org/wiki/File:Periodic_table_large.svg
    https://www.futurity.org/periodic-table-new-elements-1087782-2/
    https://www.vertex42.com/Files/pdfs/2/periodic-table_color.pdf
    https://www.vertex42.com/ExcelTemplates/periodic-table-of-elements.html
    https://iupac.org/what-we-do/periodic-table-of-elements/, https://svs.gsfc.nasa.gov/13873/
    https://commons.wikimedia.org/wiki/File:Periodic_Table_-_Atomic_Properties_of_the_Elements.png
    https://commons.wikimedia.org/wiki/File:Periodic_Table_of_the_elements.jpg
    https://commons.wikimedia.org/wiki/File:Periodic_table_detailed_enwp.svg

    https://en.m.wikipedia.org/wiki/Abundance_of_the_chemical_elements
    https://en.wikipedia.org/wiki/Periodic_table
    https://elements.wlonk.com/index.htm, https://ptable.com */

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
        unsafe { std::mem::transmute::<u8, ChemElem>(val) }
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
    pub const fn name_ch(&self) -> char         { ELEM_CH[*self as usize] }
    pub const fn atomic_number(&self) -> u8 { *self as u8 }
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
        } else { assert!(false); }
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

    pub const fn group(&self) -> u8 {   // XXX: cache/save it for frequent usage?
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
               5|13|31|49|81|113 => 13, // Boron group
               6|14|32|50|82|114 => 14, // Carbon group
               7|15|33|51|83|115 => 15, // Pnictogens
               8|16|34|52|84|116 => 16, // Chalcogens
               9|17|35|53|85|117 => 17, // Halogens
            2|10|18|36|54|86|118 => 18, // Noble gases
            //57..=70|89..=102 => 0,    // Rare earth metals (Lanthanides and Actinides)
            _ => 0, // None?
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
            1|6|7|8|15|16|34 => ElemClass::OtherNonmetal("Other Nonmetals"),
            5|14|33|52|85 => ElemClass::Metalloid("Metalloids"), // semi-metals
            13|31|49|50|81..=84|114 => // XXX: 113..=117
                ElemClass::PoorMetal("Poor metals"),  // post-transition metals
            109..=118 if self.atomic_number() != 112 => ElemClass::Unknown("Unknown"),

            // Rare earth metals (Lanthanoids plus Sc and Y)
            57..=70  => ElemClass::Lanthanide("Lanthanoids"),   // Lanthanides (include Lu)
            89..=102 => ElemClass::Actinide("Actinoids"),       // Actinides (include Lr)
            _ => match self.group() {
                1  => ElemClass::Alkali("Alkali metals"),
                2  => ElemClass::Alkaline("Alkaline earth metals"),
                17 => ElemClass::Halogen("Halogens"),
                18 => ElemClass::NobleGas("Noble gases"),
                _  => ElemClass::Transition("Transition metals"),   // 3..=12 and (112)
            }
        }
    }

    pub const fn period(&self) -> u8 {
        match self.atomic_number() {
            1|2 => 1, 3..=10 => 2, 11..=18 => 3, 19..=36 => 4,
            37..=54 => 5, 55..=86 => 6, 87..=118 => 7, _ => 0, // None?
        }
    }

    pub const fn block(&self) -> u8 {
        match self.group() { 1|2 => b's', 3..=12 => b'd', 13..=18 => b'p', _ => b'f', }
    }

    pub const fn is_ratioactive(&self) -> bool { matches!(self.atomic_number(), 43|61|84..=118) }

    // TODO: parse PubChemElements_all.json? electron_configuration, electron_affinity,
    // atomic_mass/weight, atomic_radius, melting_point, boiling_point, density,

    /// XXX: https://en.wikipedia.org/wiki/Periodic_table_(crystal_structure)
    //  https://github.com/baotlake/periodic-table-pro/blob/37239360e6f5daa605b3fd947895ed2dfdce0cd7/packages/data/json/crystalStructure.json
    //  https://periodictable.com/Properties/A/CrystalStructure.html
    pub const fn crystal_structure(&self) -> Option<(&'static str, &'static str)> {
        Some(match self.atomic_number() {
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
    //  https://physics.nist.gov/cgi-bin/ASD/ie.pl?spectra=h-Og&submit=Retrieve+Data&units=1&format=2&order=0&at_num_out=on&sp_name_out=on&shells_out=on&level_out=on&e_out=0&unc_out=on
    pub const fn ground_state(&self) -> Option<(&'static str, &'static str, &'static str)> {
        Some( match self.atomic_number() {
            28 => ("3", "F", "4"),
            41 => ("6", "D", "1/2"),
            44 => ("5", "F", "5"),
            46|70|102 => ("1", "S", "0"),
            57|71|89  => ("2", "D", "3/2"),
            58 => ("1", "G°", "4"),
            59 => ("4", "I°", "9/2"),
            60 => ("5", "I",  "4"),
            61 => ("6", "H°", "5/2"),
            62|94 => ("7", "F",  "0"),
            63|95 => ("8", "S°", "7/2"),
            64|96 => ("9", "D°", "2"),
            65|97 => ("6", "H°", "15/2"),
            66|98 => ("5", "I",  "8"),
            67|99 => ("5", "I°", "15/2"),
            68|100 => ("3", "H",  "6"),
            69|101 => ("3", "F°", "7/2"),
            74 => ("5", "D", "0"),
            78 => ("3", "D", "3"),
            90 => ("3", "F", "2"),
            91 => ("4", "K", "11/2"),
            92 => ("5", "L°", "6"),
            93 => ("6", "L",  "11/2"),
            109..=118 => return None, // ("", "-", "")

            _ => match self.group() {
                1|11 => ("2", "S", "1/2"),
                2|12|18 => ("1", "S", "0"),
                3  => ("3", "D", "3/2"),
                4  => ("3", "F", "2"),
                5  => ("4", "F", "3/2"),
                6  => ("7", "S", "3"),
                7  => ("6", "S", "5/2"),
                8  => ("5", "D", "4"),
                9  => ("4", "F", "9/2"),
                13 => ("2", "P°", "1/2"),
                14 => ("3", "P",  "0"),
                15 => ("2", "S°", "3/2"),
                16 => ("3", "P",  "2"),
                17 => ("2", "P°", "3/2"),
                _ => return None,
            }
        })
    }

    /// https://en.wikipedia.org/wiki/Electronegativities_of_the_elements_(data_page)
    /*  pip install mendeleev
        from mendeleev.fetch import fetch_electronegativities;
        df = fetch_electronegativities(scales=["Pauling"]);
        print(df["Pauling"].to_string());
        df["Pauling"].to_csv("en_pauling.csv"); */
    pub const fn en_pauling(&self) -> Option<f32> {
        Some(match self.atomic_number() {       1  => 2.2,  // 2
            3  => 0.98, 4  => 1.57, 5  => 2.04, 6  => 2.55, 7  => 3.04, 8  => 3.44, 9  => 3.98, // 10
            11 => 0.93, 12 => 1.31, 13 => 1.61, 14 => 1.9,  15 => 2.19, 16 => 2.58, 17 => 3.16, // 18
            19 => 0.82, 20 => 1.0,  21 => 1.36, 22 => 1.54, 23 => 1.63, 24 => 1.66, 25 => 1.55,
            26 => 1.83, 27 => 1.88, 28 => 1.91, 29 => 1.9,  30 => 1.65, 31 => 1.81, 32 => 2.01,
            33 => 2.18, 34 => 2.55, 35 => 2.96, // 36
            37 => 0.82, 38 => 0.95, 39 => 1.22, 40 => 1.33, 41 => 1.6,  42 => 2.16, 43 => 2.1,
            44 => 2.2,  45 => 2.28, 46 => 2.2,  47 => 1.93, 48 => 1.69, 49 => 1.78, 50 => 1.96,
            51 => 2.05, 52 => 2.1,  53 => 2.66, 54 => 2.6,  55 => 0.79, 56 => 0.89, 57 => 1.1,
            58 => 1.12, 59 => 1.13, 60 => 1.14, // 61
            62 => 1.17, 64 => 1.2,              // 63, 65
            66 => 1.22, 67 => 1.23, 68 => 1.24, 69 => 1.25, // 70
            71 => 1.0, 72 => 1.3, 73 => 1.5, 74 => 1.7, 75 => 1.9, 76 => 2.2, 77 => 2.2, 78 => 2.2,
            79 => 2.4, 80 => 1.9, 81 => 1.8, 82 => 1.8, 83 => 1.9, 84 => 2.0, 85 => 2.2, // 86
            87 => 0.7, 88 => 0.9, 89 => 1.1, 90 => 1.3, 91 => 1.5, 92 => 1.7, 93 => 1.3, 94 => 1.3,
            _ => return None,   // 95..=118
        })
    }

    /** ```
        assert!(inperiod::ChemElem::iter().map(|x| x.oxidation_states()).all(
            |(main, all)| main.iter().all(|&x| all.contains(&x))));
    ```
    https://en.wikipedia.org/wiki/Oxidation_state */
    pub const fn oxidation_states(&self) -> (&'static [i8], &'static [i8]) {
        let main: &'static [i8] = match self.atomic_number() {
            1|85 => &[-1, 1], // 2
            3|11|19|37|47|55|87 => &[1],
            4|12|20|28..=30|36|38|48|56|88 => &[2],
            5|13|21|31|39|45|49|57|59..=62|64..=71|79|83|89|95..=103 => &[3],
            6|14 => &[-4, 4],
            7|15|33|51 => &[-3, 3, 5],
            8 =>  &[-2],
            9 =>  &[-1], // 10
            16 => &[-2, 2, 4, 6],
            17|53 => &[-1, 1, 3, 5, 7], // 18
            22|40|72|75|76|90|94|104 => &[4],
            23|41|73 => &[5],
            24 => &[3, 6],
            25 => &[2, 4, 7],
            26|27|63 =>  &[2, 3],
            32|50 => &[-4, 2, 4],
            34|52 => &[-2, 2, 4, 6],
            35 => &[-1, 1, 3, 5],
            42|74 => &[4, 6],
            43 => &[4, 7],
            44|58|77 => &[3, 4],
            46|78|82 => &[2, 4],
            54 => &[2, 4, 6],
            80 => &[1, 2],
            81 => &[1, 3],
            84 => &[-2, 2, 4], // 86
            91|93 => &[5],
            92 => &[6],
            _  => &[], // 105..=118
        };

        let all: &'static [i8] = match self.atomic_number() {
            1|19|37|55 => &[-1,    1], // 2
            3|87 =>    &[1],
            4|12 => &[0, 1, 2],
            5 =>      &[-5,             -1, 0, 1, 2, 3],
            6|14|32|50 => &[-4, -3, -2, -1, 0, 1, 2, 3, 4],
            7|15|33|51|83 =>  &[-3, -2, -1, 0, 1, 2, 3, 4, 5],
            8 =>              &[-2, -1, 0, 1, 2],
            9 =>            &[-1], // 10
            11 =>           &[-1, 0, 1],
            13 =>       &[-2, -1, 0, 1, 2, 3],
            16|34|52 => &[-2, -1, 0, 1, 2, 3, 4, 5, 6],
            17 =>           &[-1,    1, 2, 3, 4, 5, 6, 7], // 18
            20|36|38|56 =>         &[1, 2],
            21|39|57|62|64|67..=71 =>   &[0, 1, 2, 3],
            22|28 =>            &[-2, -1, 0, 1, 2, 3, 4],
            23|41|73 =>     &[-3,     -1, 0, 1, 2, 3, 4, 5],
            24|42|74 => &[-4,     -2, -1, 0, 1, 2, 3, 4, 5, 6],
            25|75 =>        &[-3,     -1, 0, 1, 2, 3, 4, 5, 6, 7],
            26 =>       &[-4,     -2, -1, 0, 1, 2, 3, 4, 5, 6, 7],
            27 =>           &[-3,     -1, 0, 1, 2, 3, 4, 5],
            29|72 =>            &[-2,     0, 1, 2, 3, 4],
            30 =>               &[-2,     0, 1, 2],
            31 => &[-5, -4, -3, -2, -1, 0, 1, 2, 3],
            35 =>     &[-1,    1, 2, 3, 4, 5,    7],
            40|58 =>         &[1, 2, 3, 4],
            43|45 =>  &[-3,     -1,    1, 2, 3, 4, 5, 6, 7],
            44 => &[-4,     -2,        1, 2, 3, 4, 5, 6, 7, 8],
            46 =>                    &[1, 2, 3, 4, 5],
            47 =>         &[-2, -1, 0, 1, 2, 3],
            48|80 =>      &[-2,        1, 2],
            49 => &[-5,     -2, -1, 0, 1, 2, 3],
            53 => &[-1,    1, 2, 3, 4, 5,    7],
            54 =>           &[2,    4,    6,    8],
            59 =>     &[0, 1, 2, 3, 4, 5],
            60 =>     &[0,    2, 3, 4],
            61|100..=102 => &[2, 3],
            63 =>     &[0,    2, 3],
            65|66 =>  &[0, 1, 2, 3, 4],
            76 => &[-4,     -2, -1, 0, 1, 2, 3, 4, 5, 6, 7, 8],
            77 =>     &[-3, -2, -1,    1, 2, 3, 4, 5, 6, 7, 8, 9],
            78 =>     &[-3, -2, -1, 0, 1, 2, 3, 4, 5, 6],
            79 =>     &[-3, -2, -1, 0, 1, 2, 3,    5],
            81 => &[-5,     -2, -1,    1, 2, 3],
            82 => &[-4,     -2, -1, 0, 1, 2, 3, 4],
            84 => &[-2, 2, 4, 5, 6],
            85 => &[-1, 1, 3, 5, 7],
            86 => &[2, 6],
            88 => &[2],
            89|103 => &[3],
            90 => &[-1, 1, 2, 3, 4],
            91|97|98 =>  &[2, 3, 4, 5],
            92 => &[-1, 1, 2, 3, 4, 5, 6],
            93|95 =>     &[2, 3, 4, 5, 6, 7],
            94 =>        &[2, 3, 4, 5, 6, 7, 8],
            96|106 =>       &[3, 4, 5, 6],
            99 =>        &[2, 3, 4],
            104 => &[3, 4],
            105 => &[3, 4, 5],
            107 => &[3, 4, 5, 7],
            108 => &[3, 4, 6, 8],
            109 => &[1, 3, 6],
            110 => &[2, 4, 6],
            111 => &[-1, 3, 5],
            112 => &[2, 4], // 113 ~ 115
            116 => &[-2, 4],
            117 => &[-1, 5],
            118 => &[-1, 1, 2, 4, 6],
            _   => &[],
        };

        //let extend = all.iter().filter_map(|&x|
        //    if main.contains(&x) { Some(x) } else { None }).collect::<Vec<_>>();

        (main, all)
    }

}

pub enum ElemClass {
    Alkali(&'static str),
    Alkaline(&'static str),
    Transition(&'static str),
    PoorMetal(&'static str),
    Metalloid(&'static str),
    OtherNonmetal(&'static str),
    Halogen(&'static str),
    NobleGas(&'static str),
    Unknown(&'static str),
    Lanthanide(&'static str),
    Actinide(&'static str),
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
    "qīng", "hài", "lǐ", "pī", "péng", "tàn", "dàn", "yǎng", "fú", "nǎi",
    "nà", "měi", "lǚ", "guī", "lín", "liú", "lǜ", "yà", "jiǎ", "gài",
    "kàng", "tài", "fán", "gè", "měng", "tiě", "gǔ", "niè", "tóng", "xīn",
    "jiā", "zhě", "shēn", "xī", "xiù", "kè", "rú", "sī", "yǐ", "gào",
    "ní", "mù", "dé", "liǎo", "lǎo", "bǎ", "yín", "gé", "yīn", "xī",
    "tī", "dì", "diǎn", "xiān", "sè", "bèi", "lán", "shì", "pǔ", "nǚ",
    "pǒ", "shān", "yǒu", "gá", "tè", "dī", "huǒ", "ěr", "diū", "yì",
    "lǔ", "hā", "tǎn", "wū", "lái", "é", "yī", "bó", "jīn", "gǒng",
    "tā", "qiān", "bì", "pō", "ài", "dōng", "fāng", "léi", "ā", "tǔ",
    "pú", "yóu", "ná", "bū", "méi", "jū", "péi", "kāi", "āi", "fèi",
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

