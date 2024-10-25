#![allow(non_snake_case)]

use dioxus::prelude::*;
//use dioxus_logger::tracing;

// Urls are relative to your Cargo.toml file
//const _TAILWIND_CSS: &str = manganis::mg!(file("assets/tailwind.css"));

fn main() {
    //dioxus_logger::init(tracing::Level::INFO).expect("failed to init logger");
    //tracing::info!("starting app");

    #[cfg(not(feature = "desktop"))] launch(App);
    #[cfg(feature = "desktop")] LaunchBuilder::desktop().with_cfg(dioxus::desktop::Config::new()
        .with_window(dioxus::desktop::WindowBuilder::new().with_title(env!("CARGO_PKG_NAME")))
        .with_custom_head("<link rel='stylesheet' href='tailwind.css'/>".into())
        //.with_custom_head("<script  src='https://cdn.tailwindcss.com'/>".into())
        .with_custom_head("<link rel='icon' href='ptable.png'/>".into())
        //.with_custom_index(r"<!DOCTYPE html><html>...</html>".into())
        //.with_icon("assets/ptable.png".into())    // FIXME:
    ).launch(App);
}

fn App() -> Element {
    rsx! {  //Router::<Route> {}
        //#[cfg(not(profile = "release"))]
        //script { src: "https://cdn.tailwindcss.com" }
        //link { rel: "stylesheet",
        //    href: "https://unpkg.com/tailwindcss@^2.0/dist/tailwind.min.css" }
        PeriodicTable {}
    }
}

#[component] fn PeriodicTable() -> Element { rsx! {     // w-fit scale-75 origin-top-left
    div { class: "grid grid-cols-[auto_repeat(18,1fr)_auto]
        grid-rows-[auto_repeat(7,1fr)_auto_1fr_1fr_auto] w-full p-6 gap-0.5 relative",

        p { class: "font-bold relative -bottom-4 rotate-180",
            style: "writing-mode: vertical-rl;", "PERIOD" }
        div { class: "self-end text-center font-bold",
            p { class: "leading-none", "GROUP" } p { class: "text-lg/6", "IA - 1" } }
        p { class: "col-[span_16_/_span_16] text-center font-bold text-5xl",
            a { href: "https://github.com/mhfan/inperiod", "Periodic Table" } " of the Elements" }
        p { class: "self-end text-center font-bold text-lg", "VIIIA - 18" }
        p { class: "font-bold leading-none relative -bottom-4 content-center ml-2",
            style: "writing-mode: vertical-rl;", "E-shell" br{} "E-max" }

        div { class: "grid row-span-7 mx-1 gap-0.5 items-center text-lg font-bold", // divide-y
            { (1..=7).map(|i| rsx! { p { "{i}" } }) } }
        ElemTile { ordinal: 1, annote: false }
        p { class: "self-end text-center text-lg font-bold", "IIA - 2" } div { class: "empty" }
        ElemTile { ordinal: 43, annote: true } div { class: "empty col-span-8" }
        { (13..=17).map(|i| rsx! { p { class: "self-end text-center text-lg font-bold",
            { format!("{}A - {i}", ROMAN_NUM[i - 10]) } } }) } ElemTile { ordinal: 2, annote: false }

        div { class: "grid grid-rows-[repeat(7,1fr)] row-span-7 gap-0.5
            font-mono text-nowrap text-right tracking-tighter divide-y",
            div { class: "content-center pl-1", p {  "2 K" } }
            div { class: "content-center pl-1", p {  "8 L" } p {  "2 K" } }
            div { class: "content-center pl-1", p {  "8 M" } p {  "8 L" } p {  "2 K" } }
            div { class: "content-center pl-1",
                      p {  "8 N" } p { "18 M" } p {  "8 L" } p {  "2 K" } }
            div { class: "content-center pl-1", p {  "8 O" } p { "18 N" }
                      p { "18 M" } p {  "8 L" } p {  "2 K" } }
            div { class: "content-center pl-1", p {  "8 P" } p { "18 O" }
                      p { "32 N" } p { "18 M" } p {  "8 L" } p {  "2 K" } }
            div { class: "content-center pl-1", p {  "8 Q" } p { "18 P" }
                      p { "32 O" } p { "32 N" } p { "18 M" } p {  "8 L" } p {  "2 K" }
            }
        }

        ElemTile { ordinal: 3, annote: false }
        ElemTile { ordinal: 4, annote: false } div { class: "empty col-span-10" }
        {  (5..=12).map(|i| rsx! { ElemTile { ordinal: i, annote: false } }) }

        {   (3..=7).map(|i| rsx! { p { class: "self-end text-center text-lg font-bold",
            { format!("{}B - {i}", ROMAN_NUM[i]) } } }) }
        p { class: "self-end text-center text-lg font-bold
            col-span-3 shadow-[0_2px] shadow-indigo-300", "VIIIB - 8|9|10" } // border-b-2
        p { class: "self-end text-center text-lg font-bold",  "IB - 11" }
        p { class: "self-end text-center text-lg font-bold", "IIB - 12" }

        div { class: "absolute flex w-full h-full pb-10", style: "grid-area: 3 / 4 / 5 / 14;",
            div { class: "flex self-end mx-auto",
                div { class: "text-lg/6",
                    div { class: "flex flex-col font-bold mb-4",
                        p { class: "self-start mb-1 px-1 border-b border-black", "Phase at STP" }
                        p { span { class: "text-liquid", "Liquid" }
                            span { class: "mx-4 text-gas", "Gas" } span { "Solid" }
                            span { class: "ml-4 text-synthetic", "Synthetic" }
                        }
                    }

                    p { class: "font-bold px-1", "Categories"}
                    div { class: "grid grid-cols-2 grid-rows-5 text-center",
                        p { class: "content-center rounded-sm bg-alkali", "Alkali Metals 碱金属" }
                        p { class: "content-center rounded-sm bg-noble-gas px-1",
                            "Noble Gases 稀有气体" }

                        p { class: "content-center rounded-sm bg-alkaline",
                            "Alkaline Earth Metals" br{} "碱土金属" }
                        p { class: "content-center rounded-sm bg-halogen", "Halogens 卤素" }

                        p { class: "content-center rounded-sm bg-transition",
                            "Transition Metals" br{} "过渡金属" }
                        p { class: "content-center rounded-sm bg-non-metal",
                            "Other nonmetals" br{} "其它非金属" }

                        p { class: "content-center rounded-sm bg-poor-metal", "Poor Metals 贫金属" }
                        p { class: "content-center rounded-sm bg-metalloid", "Metalloids 类金属" }

                        p { class: "content-center rounded-sm bg-lanthanide",
                            "* Rare Earth Metals" br{} "稀土金属" }
                        p { class: "content-center rounded-sm bg-unknown", "Unknown 未知" }
                    }
                }
                div { class: "ml-6 self-end text-lg/6", PhysConsts {} }
            }
        }
        div { class: "absolute flex w-full h-full pb-8", style: "grid-area: 2 / 8 / 3 / 19;",
        }   // XXX:

        { (13..=56).map(|i| rsx! { ElemTile { ordinal: i, annote: false } }) }
        div { class: "flex flex-col text-2xl rounded-sm p-1
            shadow-border-1 shadow-indigo-300 bg-lanthanide",
            span { class: "font-bold self-end", "71" }
            p { class: "text-center m-auto",
                b { "57 ~ 70" } br{} "Lanthanoids" br{} "(镧系)" }
        }
        { (72..=88).map(|i| rsx! { ElemTile { ordinal: i, annote: false } }) }
        div { class: "flex flex-col text-2xl rounded-sm p-1
            shadow-border-1 shadow-indigo-300 bg-actinide",
            span { class: "font-bold self-end", "103" }
            p { class: "text-center m-auto", b { "89 ~ 102" } br{} "Actinoids" br{} "(锕系)" }
        }
        { (104..=118).map(|i| rsx! { ElemTile { ordinal: i, annote: false } }) }

        div { class: "empty row-span-4" }
        div { class: "text-center text-lg/6 rounded-sm col-span-2  bg-red-100 self-start",
            b { "s" } "-block (plus " b { "He" } ")" }
        div { class: "text-center text-lg/6 rounded-sm col-span-10 bg-blue-100 border-x mb-6",
            b { "d" } "-block (exclude " i { "Lan/Act" } " series)" }
        div { class: "text-center text-lg/6 rounded-sm col-span-6  bg-green-100 self-start",
            b { "p" } "-block (exclude " i { "He" } ")" }
        div { class: "empty" }

        div { class: "flex flex-col col-span-3 row-span-2",
            p { class: "text-lg font-bold leading-none pb-1", "Notes:" }
            ul { class: "list-disc",
                li { i { "[ ]" } " indicate mass number of most stable isotope" }
                li { "Density units are " i { "g/cm3" } " for solids and " i { "g/L" }
                    " for liquid" br{} " or " i { "kg/m3" } " at 0° Celsius for gases" }
                li { "Electron configuration based on "
                    a { href: "https://iupac.org/what-we-do/periodic-table-of-elements/",
                        "IUPAC" } " guidelines" }
                //li { "Common oxidation states highlight in " b { "bold" } }
                li { "* mark means electronegativity is in the bottom-right" }
                li { "Rare earth metals include: Lanthanoids (La ~ Lu), Sc and Y" }
                li { "Atomic radius is van der Waals radii" }
                //li { i { "§" } " indicates crystal structure is unusual" }
                // or may require explanation
            }
            p { class: "text-lg font-bold mt-auto", "References:" }
            p { a { href: "https://www.nist.gov/pml/periodic-table-elements", "Nist.gov" } ", "
                a { href: "https://pubchem.ncbi.nlm.nih.gov/periodic-table/", "PubChem" } ", "
                a { href: "https://en.wikipedia.org/wiki/Category:Chemical_element_data_pages",
                    "Wikipedia" } ", "
                a { href: "https://www.vertex42.com/ExcelTemplates/periodic-table-of-elements.html",
                    "Vertex42" } ", and "
                a { href: "https://github.com/lmmentel/mendeleev", "mendeleev" }
            }
        }

        { (57..= 71).map(|i| rsx! { ElemTile { ordinal: i, annote: false } }) }
        p { style: "writing-mode: vertical-rl;",
            class: "text-center text-lg font-bold rotate-180", "Lanthanides" }
        { (89..=103).map(|i| rsx! { ElemTile { ordinal: i, annote: false } }) }
        p { style: "writing-mode: vertical-rl;",
            class: "text-center text-lg font-bold rotate-180", "Actinides" }

        div { class: "col-span-3", p { class: "mt-2", "© 2024 "
            a { href: "https://github.com/mhfan", "M.H.Fan" } ", All rights reserved." } }
        div { class: "text-center text-lg/6 self-start rounded-sm
            col-[span_14_/_span_14] bg-yellow-100", b { "f" } "-block" }
        div { class: "self-start rounded-sm border-l bg-blue-100",
            p { class: "invisible", "d-block" } } div { class: "empty" }
    }
} }

const ROMAN_NUM: [&str; 11] = [
    "", "I", "II", "III", "IV", "V", "VI", "VII", "VIII", "IX", "X", ];
/// https://en.wikipedia.org/wiki/Unicode_subscripts_and_superscripts
const UNICODE_SUPERS: [char; 16] = [ //&str = r"⁰¹²³⁴⁵⁶⁷⁸⁹⁺⁻⁼⁽⁾ᐟ";
    '⁰', '¹', '²', '³', '⁴', '⁵', '⁶', '⁷', '⁸', '⁹', '⁺', '⁻', '⁼', '⁽', '⁾', 'ᐟ', ];
//const UNICODE_SUBS: [char; 16] = [ //&str = r"₀₁₂₃₄₅₆₇₈₉₊₋₌₍₎⸝";
//    '₀', '₁', '₂', '₃', '₄', '₅', '₆', '₇', '₈', '₉', '₊', '₋', '₌', '₍', '₎', 'ₐ', ];

#[component] fn ElemTile(ordinal: u8, annote: bool) -> Element {
    let chem = inperiod::ChemElem::from(ordinal);
    let elem = mendeleev::ALL_ELEMENTS[ordinal as usize - 1];
    //let os_main = elem.oxidation_states(mendeleev::OxidationStateCategory::Main);
    let (name, (os_main, os_all)) = (chem.name(), chem.oxidation_states());

    use mendeleev::AtomicWeight;
    fn fmt_mass(mass: &AtomicWeight, maxp: usize) -> String {
        #[inline] fn trim_fmt_mass(val: &f64, maxp: usize) -> String {
            format!("{val:.prec$}", prec = maxp)
                .trim_end_matches('0').trim_end_matches('.').to_string()
        }
        match mass {
            AtomicWeight::Interval { conventional, .. } => trim_fmt_mass(conventional, maxp),
            AtomicWeight::Uncertainty { weight, .. } => trim_fmt_mass(weight, maxp),
            AtomicWeight::MassNumber { number } => format!("[{}]", number), // XXX:
        }
    }

    let simplify_ecfg = || match ordinal {
        //format!("{}{}", prefix, ecfg.rfind(' ').map_or("", |pos| &ecfg[pos..]))
        81..=86   => format!("[Hg] 6p{}", UNICODE_SUPERS[ordinal as usize -  80]),
        113..=118 => format!("[Cn] 7p{}", UNICODE_SUPERS[ordinal as usize - 112]),
        _ => elem.electronic_configuration().to_string()
    };

    use inperiod::ElemClass::*;
    let get_bg_color = || match chem.category() {   // chem.category()
        OtherNonmetal(_) => "bg-non-metal",
        Metalloid(_)     => "bg-metalloid",
        PoorMetal(_)     => "bg-poor-metal",
        Lanthanide(_)    => "bg-lanthanide",
        Actinide(_)      => "bg-actinide",
        Unknown(_)       => "bg-unknown",
        Alkali(_)        => "bg-alkali",
        Alkaline(_)      => "bg-alkaline",
        Halogen(_)       => "bg-halogen",
        NobleGas(_)      => "bg-noble-gas",
        Transition(_)    => "bg-transition"
    };

    let draw_metal_bound = || match ordinal {
        1 => "shadow-black-b", 118 => "shadow-black-l",
        2 => "shadow-[0_2px_#fca5a5]", // indicate He is of s-block, shadow-red-300
        5|14|33|52|85 => "shadow-black-bl", _ => "",
    };

    let color_for_symbol = || match ordinal {
        1|2|7..=10|17|18|36|54|86 => "text-gas",
        43|61|96..=118 => "text-synthetic",
        35|80 => "text-liquid", _ => "",
    };

    rsx! { //shadow-border-1 shadow-indigo-300 // 152x198
        div { class: format_args!("flex flex-col rounded-sm p-1 border border-indigo-300
            hover:shadow-orange-600 hover:shadow-spread-2 relative {} {}",
            get_bg_color(), draw_metal_bound()), if annote {
                p { class: "absolute font-bold text-lg/6 text-amber-600",
                    style: "top: -1.5rem; right: 1rem;", "ratioactive" }
                div { class: "absolute text-lg leading-tight text-nowrap text-right",
                    style: "right: calc(100% + 0.4rem);",
                    p { "*atomic weight/mass" }
                    a { href: "https://physics.nist.gov/cgi-bin/ASD/ie.pl?spectra=h-Og&submit=Retrieve+Data&units=1&format=0&order=0&at_num_out=on&sp_name_out=on&shells_out=on&level_out=on&e_out=0&unc_out=on", "1st ionization energy (eV)" }
                    p { class: "mt-3 mb-5", "symbol" } p { "name" }
                    p { span { class: "text-blue-700", "melting" } "/"
                        span { class: "text-red-700",  "boiling" } " point (°C)" }
                    p { "*density" } p { "*electron configuration" }
                }
                div { class: "absolute text-lg leading-tight text-nowrap",
                    style: "left: calc(100% + 0.4rem);",
                    p { class: "mt-1", "atomic number" }
                    p { "electron affinity" }
                    p { class: "my-1", "main "
                a { href: "https://en.wikipedia.org/wiki/Oxidation_state", "oxidation states" } }
                    p { "Chinese name with pinyin" }
                    a { href: "https://en.wikipedia.org/wiki/Electronegativities_of_the_elements_(data_page)", "electronegativity (pauling)*" }
                    p { a { href: "https://en.wikipedia.org/wiki/Periodic_table_(crystal_structure)", "crystal structure" } }
                    a { href: "https://www.nist.gov/system/files/documents/2024/06/25/NIST_periodictable_August24-print.pdf", "ground-state level" }
                    p { span { class: "text-purple-700 font-bold", "atomic radius" } " (pm)*" }
                }
            }

            div { class: "flex",
                div { class: "grow",
                    p { class: "flex text-lg/6 font-bold",
                        { fmt_mass(&elem.atomic_weight(), match ordinal { 90..=92 => 3, _ => 4 }) }
                        if chem.is_ratioactive() { span { class: "ml-1 text-center grow", "☢" } }
                    }
                    p { class: "flex text-base/5", {
                        elem.ionization_energy().map_or_else(|| "-".to_string(),
                            |ie| format!("{}", ie.0)) }
                        span { class: "ml-2 text-center grow", {
                            elem.electron_affinity().map_or_else(|| " ".to_string(),
                                |ea| format!("{}", ea.0))
                        } }
                    }
                }
                span { class: "text-2xl font-bold ml-1", "{ordinal}" }
            }
            div { class: "flex-col grow",
                div { class: "flex",
                    span { class: format_args!("text-5xl self-center grow ml-1 {}",
                        color_for_symbol()), { chem.symbol() } }
                    div { class: "flex flex-col mr-1",
                        p { class: "text-center leading-tight", { chem.name_py() } }
                        a { href: format_args!("https://zh.wikipedia.org/wiki/{}", chem.name_ch()),
                            class: "text-right text-3xl", { chem.name_ch().to_string() } }
                    }
                    div { class: "text-right ml-1 relative",
                        div { class: "absolute w-full h-full group font-bold leading-tight", {
                            os_main.iter().rev().map(|os| rsx! { pre { { format!("{}{os}",
                                match *os { x if 0 < x => "+", 0 => " ", _ => "" })
                            } } }) }
                            if os_main.len() < os_all.len() { div { class:
                                "absolute left-full -top-2 ml-1.5 p-1 text-lg/5 border rounded
                                border-orange-600 bg-indigo-50 group-hover:block hidden z-10", {
                                os_all.iter().rev().map(|os| rsx! { pre { class:
                                    if os_main.contains(os) { "font-extrabold" } else { "" },
                        { format!("{}{os}", match *os { x if 0 < x => "+", 0 => " ", _ => "" }) }
                                } }) }
                            } }
                        }   pre { class: "invisible", "  " }
                    }
                }
                p { a { href: format_args!("https://en.wikipedia.org/wiki/{name}"),
                        class: "text-lg/6", { name } }
                    span { class: "ml-2 font-bold", { match ordinal {
                        42|59 => "*".to_string(), // name is too long
                        _ => chem.en_pauling().map_or_else(|| "".to_string(),
                            |en| format!("{}", en))
                    } } }
                }

                p { class: "flex text-base/5 relative",
                    span { class: "text-blue-700 font-bold", {
                        elem.melting_point().map_or_else(|| "-".to_string(),
                            |mp| format!("{}", (mp.0 - 273.15 + 0.5) as i32))
                    } } "/"
                    span { class: "text-red-700",  {
                        elem.boiling_point().map_or_else(|| "-".to_string(),
                            |bp| format!("{}", (bp.0 - 273.15 + 0.5) as i32))
                    } }
                    { chem.crystal_structure().map_or_else(|| rsx! { span { class: "ml-2", "-" } },
                        |(cs, file)| rsx! {
                        span { class: "pl-2 grow peer", { cs.to_string() } }
                        div { class: "absolute w-[10vw] max-w-none border rounded
                                border-orange-600 bg-indigo-50 hidden peer-hover:block z-10",
                            style: format_args!("{}", match ordinal {
                                89..=103|2|10|18|36|54|71|86|118 =>
                                    "right: calc(100% + 0.4rem); bottom: 0px;",
                                _ => "left: calc(100% + 0.4rem);", }),
                            p { class: "text-center text-xl font-bold mb-2", // XXX:
                            }
                            img { class: "w-full", src: format_args!("crystal-s/{file}"), }
                        } }
                    ) }
                }
                p { class: "flex text-base/5", {
                    elem.density().map_or_else(|| "-".to_string(), |den| format!("{:.4}", den.0)
                        .trim_end_matches('0').trim_end_matches('.').to_string()) }
                    span { class: "ml-2 font-bold text-purple-700", {
                        elem.atomic_radius().map_or_else(|| "-".to_string(),
                            |cr| format!("{}  ", cr.0)) // XXX: covalent/metallic/ironic radius?
                    } }
                    span { class: "ml-2 font-bold", {
                        chem.ground_state().map_or_else(|| rsx! { "-" },
                            |(s1, s2, s3)| if 1 < s2.len() { rsx! {
                                sup  { {s1} } { s2.chars().next().unwrap().to_string() }
                                span { style: "position: relative; top: -0.1em;", "°" }
                                sub  { style: "left: -0.6em;", {s3} }
                            } } else { rsx! { sup { {s1} } {s2} sub { {s3} } } })
                    } }
                }
                p { class: "flex mt-auto text-nowrap font-bold text-lg/6", { simplify_ecfg() }
                    if matches!(ordinal, 42|59) {
                        span { class: "ml-2 font-bold grow text-right", {
                            chem.en_pauling().map(|en| format!("{}", en)) // XXX:
                        } }
                    }
                }
                // TODO: abundance, origin
            }
        }
    }
}

#[component] fn ElemDetail() -> Element { rsx! { // TODO: https://pt.ziziyi.com/
} }

//  https://physics.nist.gov/cuu/Constants/index.html
#[component] fn PhysConsts() -> Element { rsx! {
        p { class: "flex text-lg/6 justify-between px-1",
            span { class: "font-bold", "Common physical constants" }
            span { "Source: " a { href: "https://physics.nist.gov/constants",
                "physics.nist.gov" } " (2022)"
            }
        }
        div { class: "grid grid-cols-[repeat(2,auto)]
            border-black border divide-black divide-x",
            div { class: "grid grid-cols-[repeat(3,auto)] gap-x-3 px-2",
                p { "electron mass" }
                span { class: "text-xl leading-none -top-2",
                    "𝓂" sub { class: "text-lg leading-none", "𝑒" } }
                p { "9.109 383 7139(28) × 10⁻³¹ kg" }

                p { "atomic mass unit " span { class: "text-xl leading-none", "𝓂" } "(¹²C)/12" }
                span { class: "text-xl leading-none", "𝓂" sub { "μ" } }
                p { "1.660 539 068 92(52) × 10⁻²⁷ kg" }

                p { "fine-structure const. "
                    span { class: "text-xl leading-none", "𝑒" } "²/4π𝜖₀ℏ𝑐" } // 𝜋ε
                span { class: "text-xl leading-none", "𝛼" } // α
                p { "7.297 352 5643(11) × 10⁻³ (~1/137)" }

                p { "Newtonian const. of gravitation" } span { "𝐺" }
                p { "6.674 30(15) × 10⁻¹¹ m³ kg⁻¹ s⁻²" }

                p { "Rydberg constant" } span { "𝑅" sub { "∞" } }
                p { "10 973 731.568 157(12) [m⁻¹]" }

                p { "classical electron radius " span { class: "text-lg leading-none", "𝛼²𝑎₀" } }
                span { class: "text-lg leading-none", "𝑟" sub { "𝑒" } }
                p { "2.817 940 3205(13) × 10⁻¹⁵ m" }

                p { "molar  volume of ideal gas 𝑅𝑇/𝑝" } span { "𝑉" sub { "m" } }
                p { "22.413 969 54... × 10⁻³ m³/mol" }

                p { "first radiation constant 2π"
                    span { class: "text-xl leading-none", "ℎ" } "𝑐²" }
                span { i { class: "text-lg leading-none", "c" } "₁" }
                p { "3.741 771 852... × 10⁻¹⁶ [W m²]" }

                p { "second radiation constant "
                    span { class: "text-xl leading-none", "ℎ" } "𝑐/𝑘" }
                span { i { class: "text-lg leading-none", "c" } "₂" }
                p { "1.438 776 877... × 10⁻² [m K]" }

                p { "¹³³Cs hyperfine transition freq." } span { "∆ν" sub { "Cs" } }
                p { "9 192 631 770 Hz" }
            }
            div { class: "grid grid-cols-[repeat(3,auto)] gap-x-3 px-2",
                p { "Avogadro constant" } span { "𝑁" sub { "A" } }
                p { "6.022 140 76 × 10²³ mol⁻¹" }

                p { "Planck constant" } span { class: "text-2xl leading-none", "ℎ" }
                p { "6.626 070 15 × 10⁻³⁴ J/Hz" }
                p { class: "text-center", span { class: "text-2xl leading-none", "ℎ" } "/2π" }
                span { class: "text-lg leading-tight", "ℏ" } // ħ
                p { "1.054 571 817... × 10⁻³⁴ J s" }

                p { "Boltzmann constant" } span { class: "text-xl leading-tight", "𝑘" }
                p { "1.380 649 00 × 10⁻²³ J/K" }

                p { "Faraday constant 𝑁" sub { "A" } "𝑒" } span { "𝐹" }
                p { "96 485.332 12... C/mol" }

                p { "molar gas constant 𝑁" sub { "A" } "𝑘" } span { "𝑅" }
                p { "8.314 462 618... J mol⁻¹ K⁻¹" }

                p { "elementary charge (eV)" } span { class: "text-xl leading-none", "𝑒" }
                p { "1.602 176 634 × 10⁻¹⁹ C (J)" }

                p { "speed of light in vacuum" } span { class: "text-xl leading-none", "𝑐" }
                p { "299 792 458 m/s" }

                p { class: "col-span-3", "STP: 𝑇 = 273.15 K, 𝑝 = 101.325 kPa" }
            }
        }
} }

/* for fullstack web renderer
#[derive(Clone, Routable, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
enum Route {
    #[route("/")] Home {},
    #[route("/blog/:id")] Blog { id: i32 },
}

#[component] fn Blog(id: i32) -> Element {
    rsx! {
        Link { to: Route::Home {}, "Go to counter" }
        "Blog post {id}"
    }
}

#[component] fn Home() -> Element {
    let mut count = use_signal(|| 0);
    let mut text = use_signal(|| String::from("..."));

    rsx! {
        Link { to: Route::Blog { id: count() }, "Go to blog" }
        div {
            h1 { "High-Five counter: {count}" }
            button { onclick: move |_| count += 1, "Up high!" }
            button { onclick: move |_| count -= 1, "Down low!" }
            #[cfg(feature = "server")] button {
                onclick: move |_| async move {
                    if let Ok(data) = get_server_data().await {
                        tracing::info!("Client received: {}", data);
                        text.set(data.clone());
                        post_server_data(data).await.unwrap();
                    }
                },
                "Get Server Data"
            }
            p { "Server data: {text}"}
        }
    }
}

#[cfg(feature = "server")] #[server(PostServerData)]
async fn post_server_data(data: String) -> Result<(), ServerFnError> {
    tracing::info!("Server received: {}", data);
    Ok(())
}

#[cfg(feature = "server")] #[server(GetServerData)]
async fn get_server_data() -> Result<String, ServerFnError> {
    Ok("Hello from the server!".to_string())
} */

