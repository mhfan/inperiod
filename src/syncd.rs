/****************************************************************
 * $ID: syncd.rs  	Thu 07 Nov 2024 13:50:42+0800               *
 *                                                              *
 * Maintainer: 范美辉 (MeiHui FAN) <mhfan@ustc.edu>              *
 * Copyright (c) 2024 M.H.Fan, All rights reserved.             *
 ****************************************************************/

use std::{error::Error, fs::{self, File}, io::Write, path::PathBuf};
//use reqwest::blocking::get as reqwest_get; // XXX:
use scraper::{Html, Selector};
use inperiod::ChemElem;

/// cargo r --bin syncd -F syncdep
#[tokio::main] async fn main() -> Result<(), Box<dyn Error>> {
    //let mut rt = tokio::runtime::Runtime::new()?;   rt.block_on(async { ... })
    //for elem in ChemElem::iter() { println!("{}", elem.name_ch()); } return Ok(());

    let res = tokio::join!(parse_nist_asd(),
        //parse_electronegativity(), // use the data from pubchem instead
        parse_oxstates(), parse_pubchem(), parse_ciaaw());
    res.0?; res.1?; res.2?; res.3?; //res.4?;
    //parse_origin()?;
    Ok(())
}

const  USE_STMT: &[u8] = b"\nuse super::ChemElem::{self, *};\n";
const IMPL_STMT: &[u8] = b"impl ChemElem { // Auto-generated by syncd.rs, DO NOT EDIT.\n";

/// https://en.wikipedia.org/wiki/Electronegativity
#[allow(unused)] async fn parse_electronegativity() -> Result<(), Box<dyn Error>> {
    println!("load/parse electronegativity ...");
    let document = Html::parse_document(&reqwest::get(
        "https://en.wikipedia.org/wiki/Electronegativities_of_the_elements_(data_page)"
    ).await?.text().await?);
    let Some(table) = document.select(
        &Selector::parse("table.wikitable.sortable")?).next()
    else { return Err("Electronegativity table not found".into()) };

    let mut file = File::create(PathBuf::from("src").join("en_pauling.rs"))?;
    //let mut file = std::io::stdout();
    file.write_all(USE_STMT)?;  file.write_all(IMPL_STMT)?;
    file.write_all(b"    pub const fn en_pauling(&self) -> Option<f32> {\n")?;
    file.write_all(b"        Some(match *self {\n")?;

    let td_selector = Selector::parse("td")?;
    for row in table.select(&Selector::parse("tr")?).skip(1) {
        let cells = row.select(&td_selector).collect::<Vec<_>>();
        let en = cells[3].text().next().unwrap();
        if  en == "no data" { continue }
        let an = cells[1].text().next().unwrap();
        file.write_fmt(format_args!("            {an:2} => {en},\n"))?;
    }

    file.write_all(b"            _  => return None\n        })\n    }\n}\n\n")?;
    file.flush()?;  Ok(())
}

/// https://en.wikipedia.org/wiki/Oxidation_state
async fn parse_oxstates() -> Result<(), Box<dyn Error>> {
    println!("load/parse oxidation states ...");
    let document = Html::parse_document(&reqwest::get(
        "https://en.wikipedia.org/wiki/Template:List_of_oxidation_states_of_the_elements"
    ).await?.text().await?);
    let Some(table) = document.select(
        &Selector::parse("table.wikitable.sortable")?).next()
    else { return Err("Oxidation states table not found".into()) };

    let mut file = File::create(PathBuf::from("src").join("ostates.rs"))?;
    file.write_all(USE_STMT)?;  file.write_all(IMPL_STMT)?;
    file.write_all(b"    pub const fn oxidation_states(&self) -> (&[i8], &[i8]) {\n")?;
    file.write_all(b"        let all: &[i8] = match *self {\n")?;

    let td_selector = Selector::parse("td")?;
    let mut os_all = vec![[None; 15]; ChemElem::MAX as usize];
    for row in table.select(&Selector::parse("tr")?).skip(4) {
        let cells = row.select(&td_selector).collect::<Vec<_>>();

        let an = cells[0].text().next().unwrap().trim().parse::<u8>()?;
        let states = &mut os_all[an as usize];
        assert!(18 < cells.len());

        for (i, cell) in cells[3..18].iter().enumerate() {
            if !cell.text().collect::<String>().replace('−', "-").trim()
                .parse::<i8>().is_ok_and(|x| x == i as i8 - 5) { continue }
            states[i] = Some(cell.descendent_elements()
                .any(|x| x.value().name() == "b")); // "span"
        }

        if states.iter().all(|&x| x.is_none()) { file.write_all(b"\n")?; continue }
        file.write_fmt(format_args!("            {:2} => &[",
            //format!("{:?}", ChemElem::from(an))))?;
            cells[2].text().next().unwrap().trim_end()))?;

        for (i, &x) in states.iter().enumerate() {
            if x.is_none() { file.write_all(b"   ")?; } else {
                file.write_fmt(format_args!("{:2},", i as i8 - 5 ))?; }
        }   file.write_all(b"],\n")?;
    }   file.write_all(b"            _  => &[]\n        };\n")?;

    file.write_all(b"\n        let main: &[i8] = match *self {\n")?;
    for (an, states) in os_all.iter().enumerate().skip(1) {
        if states.iter().all(|&x| x.is_none_or(|x| !x)) { continue }
        file.write_fmt(format_args!("            {:2} => &[",
            format!("{:?}", ChemElem::from(an as u8))))?;

        for (i, &state) in states.iter().enumerate() {
            if !state.unwrap_or(false) { continue }
            file.write_fmt(format_args!("{:2},", i as i8 - 5 ))?;
        }   file.write_all(b"],\n")?;
    }   file.write_all(b"            _  => &[]\n        };\n")?;

    file.write_all(b"\n        (main, all)\n    }\n}\n\n")?;
    file.flush()?;  Ok(())
}

/// https://www.nist.gov/pml/periodic-table-elements
/// https://physics.nist.gov/PhysRefData/ASD/ionEnergy.html
async fn parse_nist_asd() -> Result<(), Box<dyn Error>> {
    println!("load/parse NIST/ASD ...");
    let path = PathBuf::from("nist_asd.csv");
    let content = if path.exists() { fs::read_to_string(path)? } else {
        reqwest::get(r"https://physics.nist.gov/cgi-bin/ASD/ie.pl?spectra=h-Og&submit=Retrieve+Data&units=1&format=2&order=0&at_num_out=on&sp_name_out=on&shells_out=on&level_out=on&e_out=0&unc_out=on").await?.text().await?
    };

    /* for line in content.lines() {
        let fields = line.split(',').collect::<Vec<_>>();
        if  fields.get(1).is_none_or(|&x| !x.ends_with(" I\"\"\"")) { continue }
        let fields = fields.iter().map(|&x|
            x.trim_start_matches("\"=\"\"").trim_end_matches("\"\"\"")).collect::<Vec<_>>();
        println!("{}", fields.join(", "));
    } */

    let mut file = File::create(PathBuf::from("src").join("nist_asd.rs"))?;
    file.write_all(USE_STMT)?;  file.write_all(IMPL_STMT)?;
    file.write_all(b"    pub const fn ionization_energy(&self) -> Option<(f64, f64)> {\n")?;
    file.write_all(b"        Some(match *self {\n")?;

    let mut fil2 = File::create(PathBuf::from("src").join("ground_level.rs"))?;
    fil2.write_all(USE_STMT)?;  fil2.write_all(IMPL_STMT)?;
    fil2.write_all(b"    pub const fn ground_level(&self) -> Option<(&str, &str, &str)> {\n")?;
    fil2.write_all(b"        Some(match *self {\n")?;

    let mut reader = csv::ReaderBuilder::new()
        .flexible(true).from_reader(content.as_bytes());
    for result in reader.records() {
        let record = result?;   // TODO: extract and store data of all levels?
        if  record.get(1).is_none_or(|x| !x.ends_with(" I\"")) { continue }

        let fields = record.iter().map(|x|
            x.trim_start_matches("=\"").trim_end_matches("\"")).collect::<Vec<_>>();
        let (an, state, uncert) = (&fields[1][0..2], fields[3], fields[7]);
        //println!("{}", fields.join(", "));

        file.write_fmt(format_args!("            {an} => ({}, {}),\n",
            fields[5], if uncert.is_empty() { "0." } else { uncert }))?;

        if let Some((s1, s2)) = state[1..].split_once(['<', '0']) {
            let s2 = if s2.is_empty() { "0" } else { s2.trim_end_matches('>') };
            if  an == "Pb" && !state.starts_with(char::is_numeric) {    // XXX:
                fil2.write_all(b"            Pb => (\"3\", \"P\" , \"0\"),\n")?;
                 eprintln!("{an}: {state:?} ?");    continue
            }
            fil2.write_fmt(format_args!("            {an} => ({:?}, {:?}{}, {s2:?}),\n",
                &state[..1], s1.replace('*', "°"), if s1.len() == 1 { " " } else { "" }))?;
        } else { eprintln!("{an}: {state:?} ?") }   // 106 ~ 108
    }

    // XXX: show predication for 106~118? https://en.wikipedia.org/wiki/Term_symbol
    //fil2.write_all(b"            Sg => (\"5\", \"D\" , \"0\"),\n")?;
    //fil2.write_all(b"            Bh => (\"6\", \"S\" , \"5/2\"),\n")?;
    //fil2.write_all(b"            Hs => (\"5\", \"D\" , \"4\"),\n")?;

    file.write_all(b"            _  => return None\n        })\n    }\n}\n\n")?;
    fil2.write_all(b"            _  => return None\n        })\n    }\n}\n\n")?;
    fil2.flush()?;  file.flush()?;  Ok(())
}

/// XXX: https://ciaaw.org/atomic-weights.htm
async fn parse_ciaaw() -> Result<(), Box<dyn Error>> {
    println!("load/parse CIAAW ...");
    let tr_selector = Selector::parse("tr")?;
    let td_selector = Selector::parse("td")?;
    let mut am_all = vec![""; ChemElem::MAX as usize];

    fn half_life_lt(a: &str, b: &str) -> bool {
        fn parse_hl_number(s: &str) -> f32 {
            s.trim_start_matches(|c: char| !c.is_ascii_digit())
                .split_once(['(', ' ']).and_then(|(x, _)|
                    x.trim_start().parse::<f32>().ok()).unwrap_or(0.)
        }

        let Some((an, au)) = a.trim_end().rsplit_once(' ') else { return false };
        let Some((bn, bu)) = b.trim_end().rsplit_once(' ') else { return false };
        if au == bu { parse_hl_number(an) < parse_hl_number(bn) } else {
            const HLUNITS: [&str; 8] = ["ms", "s", "min", "h", "d", "a", "Ka", "Ma"];
            HLUNITS.iter().position(|&x| x == au).unwrap_or(0) <
            HLUNITS.iter().position(|&x| x == bu).unwrap_or(0)
        }
    }

    let document = Html::parse_document(&reqwest::get(
        "https://ciaaw.org/radioactive-elements.htm").await?.text().await?);
    if let Some(table) = document.select(&Selector::parse("tbody")?).next() {
        let (mut half_life, mut atomic_number) = ("", 0usize);
        for row in table.select(&tr_selector) {
            let cells = row.select(&td_selector).collect::<Vec<_>>();
            if  cells.len() < 5 {   if cells.len() < 2 { continue }
                if half_life_lt(half_life,  cells[1].text().next().unwrap_or("")) {
                    am_all[atomic_number] = cells[0].text().next().unwrap();
                    //println!("{}: {} *", atomic_number, am_all[atomic_number]);
                }   continue
            }

            atomic_number = cells[0].text().next().unwrap().trim().parse()?;
            am_all[atomic_number] = cells[3].text().next().unwrap();
            half_life = cells[4].text().next().unwrap_or("");
        }
    }

    let mut file = File::create(PathBuf::from("src").join("ciaaw.rs"))?;
    file.write_all(b"\nuse super::{ChemElem, AtomicWeight::{self, *}};\n")?;
    file.write_all(IMPL_STMT)?;

    file.write_all(b"    pub const fn atomic_weight(&self) -> &AtomicWeight {")?;
    file.write_all(b" &MASS[self.atomic_number() as usize] }\n}\n\n")?;
    file.write_all(b"const MASS: [AtomicWeight; ChemElem::MAX as usize] = [ MassNumber(0),\n")?;

    let document = Html::parse_document(&reqwest::get( // XXX:
        "https://ciaaw.org/abridged-atomic-weights.htm").await?.text().await?);
    if let Some(table) = document.select(&Selector::parse("table")?).next() {
        for row in table.select(&tr_selector) {
            let cells = row.select(&td_selector).collect::<Vec<_>>();
            if  cells.len() < 4 { continue }

            let atomic_weight = cells[3].text().next().unwrap().trim();
            if let Some((value, uncerntainty)) = atomic_weight.split_once('±') {
                file.write_fmt(format_args!("    Abridged {{ value: {:6}, uncertainty: {} }},\n",
                    value.trim_end(), uncerntainty.trim_start()))?;
            } else {
                let an: usize = cells[0].text().next().unwrap().trim().parse()?;
                file.write_fmt(format_args!("    MassNumber({}),\n", am_all[an].trim()))?;
            }
        }
    }

    file.write_all(b"];\n\n")?;     file.flush()?;  Ok(())
}

/// https://pubchem.ncbi.nlm.nih.gov/periodic-table/
#[allow(non_snake_case)] async fn parse_pubchem() -> Result<(), Box<dyn Error>> {
    use serde::Deserialize;
    #[derive(Deserialize)] struct AllElem { Table: Table, }
    #[derive(Deserialize)] struct Table { Columns: Columns, Row: Vec<Row>, }
    #[derive(Deserialize)] struct Columns { Column: Vec<String>, }
    #[derive(Deserialize)] struct Row { Cell: Vec<String>, }

    println!("load/parse PubChem data ...");
    let path = PathBuf::from("PubChemElements_all.json");
    let content = if path.exists() { fs::read_to_string(path)? } else { reqwest::get(
        "https://pubchem.ncbi.nlm.nih.gov/rest/pug/periodictable/JSON").await?.text().await?
    };  let elem_all: AllElem = serde_json::from_str(&content)?;

    //let path = PathBuf::from(env::var("OUT_DIR")?).join("pubchem.rs"))?;
    let mut file = File::create(PathBuf::from("src").join("pubchem.rs"))?;
    file.write_all(b"\nuse super::{ChemElem::{self, *}, ElectronCFG, Subshell, ecfg, ssc};\n")?;
    file.write_all(IMPL_STMT)?;

    let column = &elem_all.Table.Columns.Column;
    let Some(an_pos) = column.iter().position(|x| x == "Symbol")
    else { return Err("AtomicNumber not found".into()) };

    /* if let Some(pos) = column.iter().position(|x| x == "AtomicMass") {
        file.write_all(b"    pub const fn atomic_mass(&self) -> f64 {")?;
        file.write_all(b" Self::MASS[self.atomic_number() as usize] }\n\n")?;

        file.write_all(b"const MASS: [f64; ChemElem::MAX as usize] = [ 0.,\n")?;
        for row in elem_all.Table.Row.iter() {  let s = &row.Cell[pos];
            file.write_fmt(format_args!("    {s}{},\n", if s.contains('.') { "" } else { "." }))?;
        }   file.write_all(b"];\n\n")?;
    } */

    fn camel_to_snake(camel_case: &str) -> String {
        let mut snake_case = String::with_capacity(camel_case.len() * 2);
        for c in camel_case.chars().peekable() {
            if c.is_ascii_uppercase() {
                if !snake_case.is_empty() { snake_case.push('_'); }
                snake_case.push(c.to_ascii_lowercase());
            } else { snake_case.push(c); }
        }       snake_case
    }

    let mut extract_data = |name: &str| -> Result<(), Box<dyn Error>> {
        let Some(pos) = column.iter().position(|x| x == name)
        else { return Err(format!("{name} not found").into()) };
        file.write_fmt(format_args!("    pub const fn {}(&self) -> Option<f32> {{\n",
            camel_to_snake(name)))?;
        file.write_all(b"        Some(match *self {\n")?;

        for row in elem_all.Table.Row.iter() {
            let s = &row.Cell[pos];     if s.is_empty() { continue }
            file.write_fmt(format_args!("            {:2} => {s}{},\n", row.Cell[an_pos],
                if s.contains('.') { "" } else { "." }))?;
        }
        file.write_all(b"            _  => return None\n        })\n    }\n\n")?;  Ok(())
    };

    extract_data("AtomicRadius")?;  // u16?
    //extract_data("IonizationEnergy")?;    // use NIST/ASD instead
    extract_data("ElectronAffinity")?;
    extract_data("Electronegativity")?;
    extract_data("MeltingPoint")?;
    extract_data("BoilingPoint")?;
    extract_data("Density")?;

    // https://en.wikipedia.org/wiki/Electron_configurations_of_the_elements_(data_page)
    if let Some(pos) = column.iter().position(|x| x == "ElectronConfiguration") {
        file.write_all(b"    pub const fn electron_configuration(&self) -> &ElectronCFG {")?;
        file.write_all(b"\n        &Self::ECFG[self.atomic_number() as usize]\n    }\n\n")?;
        file.write_all(b"const ECFG: [ElectronCFG; ChemElem::MAX as usize] = [ ecfg!(),\n")?;

        for row in elem_all.Table.Row.iter() {
            let ecfg = row.Cell[pos].as_str();
            let (base, rest) = ecfg.trim_start().find(']')
                .map_or(("", ecfg), |pos| (&ecfg[1..pos], &ecfg[pos+1..]));

            file.write_all(b"    ecfg![")?;
            if !base.is_empty() { file.write_fmt(format_args!("{base}, "))?; }

            let mut coll = rest.split_ascii_whitespace().collect::<Vec<_>>();
            coll.sort_by(|&a, &b| a.as_bytes()[0].cmp(&b.as_bytes()[0]));

            let mut first = true;
            for part in coll {
                if part.starts_with('(') { continue }
                if !first { file.write_all(b", ")?; } first = false;
                let p3 = if 2 < part.len() { &part[2..] } else { "1" };
                file.write_fmt(format_args!("ssc!({}, b'{}', {:>2})",
                    &part[..1], &part[1..2], p3))?;
            }   file.write_all(b"],\n")?;
        }       file.write_all(b"];\n\n")?;
    }

    file.write_all(b"}\n\n")?;  file.flush()?;  Ok(())
}

/// https://royalsocietypublishing.org/doi/10.1098/rsta.2019.0301
#[allow(unused)] fn parse_origin() -> Result<(), Box<dyn Error>> {
    println!("load/parse cosmic origin ...");
    let mut reader = csv::ReaderBuilder::new().flexible(true)
        .has_headers(false).delimiter(b'|').from_reader(ORIGIN_DATA.as_bytes());
    let (x_max, mut atomic) = (185f32, 1u8);
    let  x_max_square = x_max * x_max;

    let mut file = File::create(PathBuf::from("src").join("origin.rs"))?;
    file.write_all(b"\nuse super::{ChemElem, CosmicOrigin as CO};\n")?;
    file.write_all(IMPL_STMT)?;

    file.write_all(b"    pub const fn cosmic_origin(&self) -> &[(CO, u8)] {")?;
    file.write_all(b" ORIGIN[self.atomic_number() as usize] }\n}\n\n")?;
    file.write_all(b"const ORIGIN: [&[(CO, u8)]; ChemElem::MAX as usize] = [ &[],\n")?;

    for result in reader.records() {
        let record = result?;
        if 5 < record.len() { continue }

        let codes = record.get(4).unwrap().trim().as_bytes();
        let (len, mut coll) = (codes.len(), vec![]);
        if  len == 1 { coll.push((match atomic {    // https://svs.gsfc.nasa.gov/13873/
                43|61|84..=89|91|93 => 'r', _ => codes[0] as char }, 100.));
        } else {
            let x: f32 = record.get(3).unwrap().trim().parse()?;
            let frac =   if 2. * x < x_max { 2. * x * x } else {
                4. * x * x_max - 2. * x * x - x_max_square
            } / x_max_square;

            if len == 2 {
                if 0.99 < frac {    coll.push((codes[0] as char, 100.)); } else {
                    let  first = (codes[0] as char, (frac * 100.).round());
                    let second = (codes[1] as char, ((1. - frac) * 100.).round());
                    if 0.5 < frac { coll.push(first); coll.push(second); } else {
                                    coll.push(second); coll.push(first);
                    }
                }
            } else {    assert!(len == 3);
                let w = if atomic == 2 { 0.45 } else { 0.37 };
                coll.push((codes[2] as char, ((1. - frac) * 100.).round()));
                coll.push((codes[1] as char, (frac * (1. - w) * 100.).round()));
                coll.push((codes[0] as char, (frac * w * 100.).round()));
            }
        }

        file.write_all(b"    &[")?;
        file.write_all(coll.iter().map(|&(c, p)|
            format!("(CO::from_u8(b'{}'), {:2})", c, p))
            .collect::<Vec<_>>().join(", ").as_bytes())?;
        file.write_all(b"],\n")?;   atomic += 1;
    }

    for _ in atomic..=118 { file.write_all(b"    &[(CO::from_u8(b'z'), 100)],\n")?; }

/// from comments in https://commons.wikimedia.org/wiki/File:Nucleosynthesis_periodic_table.svg
const ORIGIN_DATA: &str = r#"
H | 1  |1  |0  |b
He|18  |1  |40 |ygb
Li| 1  |2  |79 |jby
Be| 2  |2  |0  |j
B |13  |2  |0  |j
C |14  |2  |65 |gy
N |15  |2  |65 |gy
O |16  |2  |0  |g
F |17  |2  |0  |g
Ne|18  |2  |0  |g
Na| 1  |3  |0  |g
Mg| 2  |3  |173|gc
Al|13  |3  |177|gc
Si|14  |3  |114|gc
P |15  |3  |163|gc
S |16  |3  |100|gc
Cl|17  |3  |133|gc
Ar|18  |3  |98 |gc
K | 1  |4  |130|gc
Ca| 2  |4  |93 |gc
Sc| 3  |4  |130|gc
Ti| 4  |4  |77 |gc
V | 5  |4  |65 |gc
Cr| 6  |4  |63 |gc
Mn| 7  |4  |53 |gc
Fe| 8  |4  |78 |gc
Co| 9  |4  |75 |gc
Ni|10  |4  |72 |gc
Cu|11  |4  |85 |gc
Zn|12  |4  |89 |gc
Ga|13  |4  |0  |g
Ge|14  |4  |0  |g
As|15  |4  |0  |g
Se|16  |4  |0  |g
Br|17  |4  |0  |g
Kr|18  |4  |0  |g
Rb| 1  |5  |0  |g
Sr| 2  |5  |47 |gy
Y | 3  |5  |37 |gy
Zr| 4  |5  |53 |gy
Nb| 5  |5  |48 |oy
Mo| 6  |5  |80 |oy
Tc| 7  |5  |0  |z
Ru| 8  |5  |111|oy
Rh| 9  |5  |137|oy
Pd|10  |5  |97 |oy
Ag|11  |5  |128|oy
Cd|12  |5  |91 |oy
In|13  |5  |107|oy
Sn|14  |5  |73 |oy
Sb|15  |5  |119|oy
Te|16  |5  |86 |oy
I |17  |5  |157|oy
Xe|18  |5  |131|oy
Cs| 1  |6  |134|oy
Ba| 2  |6  |56 |oy
La| 4  |7.5|81 |oy
Ce| 5  |7.5|64 |oy
Pr| 6  |7.5|94 |oy
Nd| 7  |7.5|83 |oy
Pm| 8  |7.5|0  |z
Sm| 9  |7.5|114|oy
Eu|10  |7.5|155|oy
Gd|11  |7.5|135|oy
Tb|12  |7.5|151|oy
Dy|13  |7.5|136|oy
Ho|14  |7.5|150|oy
Er|15  |7.5|132|oy
Tm|16  |7.5|139|oy
Yb|17  |7.5|111|oy
Lu|18  |7.5|127|oy
Hf| 4  |6  |88 |oy
Ta| 5  |6  |101|oy
W | 6  |6  |86 |oy
Re| 7  |6  |147|oy
Os| 8  |6  |144|oy
Ir| 9  |6  |169|oy
Pt|10  |6  |155|oy
Au|11  |6  |153|oy
Hg|12  |6  |81 |oy
Tl|13  |6  |63 |oy
Pb|14  |6  |95 |oy
Bi|15  |6  |176|oy
Po|16  |6  |0  |o
At|17  |6  |0  |o
Rn|18  |6  |0  |o
Fr| 1  |7  |0  |o
Ra| 2  |7  |0  |o
Ac| 4  |8.5|0  |o
Th| 5  |8.5|0  |o
Pa| 6  |8.5|0  |o
U | 7  |8.5|0  |o
Np| 8  |8.5|0  |o
Pu| 9  |8.5|0  |o
Am|10  |8.5|0  |z
Cm|11  |8.5|0  |z
Bk|12  |8.5|0  |z
Cf|13  |8.5|0  |z
Es|14  |8.5|0  |z
Fm|15  |8.5|0  |z
Md|16  |8.5|0  |z
No|17  |8.5|0  |z
Lr|18  |8.5|0  |z
  |3.25|0.9|0  |b|Big~Bang~fusion
  |3.25|2.4|0  |j|Cosmic~ray~fission
  | 5.8|0.9|0  |y|Dying~low-mass~stars
  | 5.8|2.4|0  |o|Merging~neutron~stars
  | 8.7|0.9|0  |g|Exploding~massive~stars
  | 8.7|2.4|0  |c|Exploding~white~dwarfs
  |11.7|0.9|0  |z|Human synthesis~No stable isotopes
"#;

    file.write_all(b"];\n\n")?;     file.flush()?;  Ok(())
}

