/****************************************************************
 * $ID: lib.rs  	Mon 21 Oct 2024 16:38:01+0800               *
 *                                                              *
 * Maintainer: 范美辉 (MeiHui FAN) <mhfan@ustc.edu>              *
 * Copyright (c) 2024 M.H.Fan, All rights reserved.             *
 ****************************************************************/

//! module/crate level documentation
// src/lib.rs (default library entry point)

use dioxus::prelude::*;

/*  https://pubchem.ncbi.nlm.nih.gov/periodic-table/
    https://github.com/Bowserinator/Periodic-Table-JSON
    https://commons.wikimedia.org/wiki/File:元素周期表.png
    https://commons.wikimedia.org/wiki/File:Periodic_table_large.svg
    https://www.futurity.org/periodic-table-new-elements-1087782-2/
    https://www.vertex42.com/Files/pdfs/2/periodic-table_color.pdf
    https://www.vertex42.com/ExcelTemplates/periodic-table-of-elements.html
    https://iupac.org/what-we-do/periodic-table-of-elements/, https://svs.gsfc.nasa.gov/13873/
    https://commons.wikimedia.org/wiki/File:Periodic_Table_-_Atomic_Properties_of_the_Elements.png
    https://commons.wikimedia.org/wiki/File:Periodic_Table_of_the_elements.jpg
    https://commons.wikimedia.org/wiki/File:Periodic_table_detailed_enwp.svg
    https://elements.wlonk.com/index.htm
    https://en.m.wikipedia.org/wiki/Abundance_of_the_chemical_elements
    https://en.wikipedia.org/wiki/Periodic_table, https://ptable.com */
#[derive(PartialEq, Clone, Props)] // Owned props must implement `PartialEq`!
#[allow(unused)] pub struct ElemProps {
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
    name_ch: char,  //pinying: String, // XXX: https://github.com/mozillazg/rust-pinyin
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
}

