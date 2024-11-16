
#![allow(non_snake_case)]
use dioxus::prelude::*;
//use dioxus_logger::tracing;
//use dioxus_sdk::{i18n::*, translate}; // TODO:

// Urls are relative to your Cargo.toml file
//const _TAILWIND_CSS: &str = manganis::mg!(file("assets/tailwind.css"));

fn main() {
    //dioxus_logger::init(tracing::Level::INFO).expect("failed to init logger");
    //tracing::info!("starting app");

    #[cfg(not(feature = "desktop"))] launch(App);
    #[cfg(feature = "desktop")] LaunchBuilder::desktop().with_cfg(dioxus::desktop::Config::new()
        .with_window(dioxus::desktop::WindowBuilder::new().with_title(env!("CARGO_PKG_NAME")))
        //.with_custom_head("<script  src='https://cdn.tailwindcss.com'/>".into())
        .with_custom_head("<link rel='stylesheet' href='tailwind.css'/>".into())
        .with_custom_index(include_str!("../index.html").replace(r"{base_path}", ".")
            .replace(r"{style_include}{script_include}", "").into())
        //.with_icon("assets/ptable.svg".into())    // FIXME:
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

//#[derive(Clone, Copy)] struct Selection { r#type: SelType, val: u8, }
//#[derive(Clone, Copy)] enum SelType { None, Period, Group, Block, Class, }

#[component] fn PeriodicTable() -> Element { rsx! {     // scale-50 origin-top-left h-[128rem]
    //use_context_provider(|| Signal::new(Selection { r#type: SelType::None, val: 0, }));
    //let mut group_sel = use_context::<Signal<Selection>>();   // move to ahead of rsx!
    div { class: "grid grid-cols-[auto_repeat(18,1fr)_auto] w-[181rem]
        grid-rows-[auto_repeat(7,1fr)_auto_1fr_1fr_auto] p-6 gap-0.5 relative",
        //style: "transform: scale(0.5); transform-origin: 0 0;",
        //style: "zoom: 0.5;", // FIXME: malformed in Safari, which works well scaling on <html>

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
            for i in 1..=7 { p {
                /*onmouseout:  move |_|  group_sel.write().r#type = SelType::None,
                onmouseover: move |_| *group_sel.write() = Selection {
                    r#type: SelType::Period, val: i, }, */"{i}"
            } }
        }
        ElemTile { ordinal: 1,  annote: false }
        p { class: "self-end text-center text-lg font-bold", "IIA - 2" } div { class: "empty" }
        ElemTile { ordinal: 43, annote: true } div { class: "empty col-span-2" }
        div { class: "relative col-span-6",
            // https://www.nagwa.com/en/explainers/809139094679/
            img { class: "absolute h-[150%]", src: "aufbau.svg" }
        }

        for i in 13..=17 { p { class: "self-end text-center text-lg font-bold",
            { format!("{}A - {i}", ROMAN_NUM[i - 10]) }
        } } ElemTile { ordinal: 2, annote: false }

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
        for i in 5..=12 { ElemTile { ordinal: i, annote: false } }

        for i in 3..=7  { p { class: "self-end text-center text-lg font-bold",
            { format!("{}B - {i}", ROMAN_NUM[i]) }
        } }
        /* for i in 8..=10 { p { class: "self-end text-center text-lg font-bold
            shadow-[0_2px] shadow-indigo-300", { format!("VIIIB - {i}") }
        } } */
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
                            "Rare Earth Metals*" br{} "稀土金属" }
                        p { class: "content-center rounded-sm bg-unknown", "Unknown 未知" }
                    }
                }
                div { class: "self-end ml-6 text-lg/6", PhysConsts {} }
            }
        }
        div { class: "absolute flex w-full h-full pb-8", style: "grid-area: 2 / 8 / 3 / 19;",
        }   // XXX:

        for i in 13..=56 { ElemTile { ordinal: i, annote: false } }
        div { class: "flex flex-col text-2xl rounded-sm p-1
            shadow-border-1 shadow-indigo-300 bg-lanthanide",
            span { class: "self-end font-bold", "71" }
            p { class: "text-center m-auto",
                b { "57 ~ 70" } br{} "Lanthanoids" br{} "(镧系)" }
        }
        for i in 72..=88 { ElemTile { ordinal: i, annote: false } }
        div { class: "flex flex-col text-2xl rounded-sm p-1
            shadow-border-1 shadow-indigo-300 bg-actinide",
            span { class: "self-end font-bold", "103" }
            p { class: "text-center m-auto", b { "89 ~ 102" } br{} "Actinoids" br{} "(锕系)" }
        }
        for i in 104..=118 { ElemTile { ordinal: i, annote: false } }

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
                li { "Standard relative atomic mass/weight based on " i { "¹²C" } } // Ar°(E)
                li { i { "[ ]" } " indicate mass number of most stable isotope" }
                li { "Density units are " i { "g/cm3" } " for solids and " i { "g/L" }
                    " for liquid" br{} " or " i { "kg/m3" } " at 0° Celsius for gases" }
                //li { "Common oxidation states highlight in " b { "bold" } }
                li { "* mark means electronegativity is in the bottom-right" }
                li { "Rare earth metals include: " i { "Lanthanoids (La ~ Lu), Sc and Y" } }
                li { "Atomic radius is " i { "van der Waals radii" } }
                //li { i { "§" } " indicates crystal structure is unusual" }
                // or may require explanation
            }
            p { class: "text-lg font-bold mt-auto", "References:" }
            p { a { href: "https://iupac.org/what-we-do/periodic-table-of-elements/", "IUPAC" } ", "
                a { href: "https://www.nist.gov/pml/periodic-table-elements", "Nist.gov" } ", "
                a { href: "https://pubchem.ncbi.nlm.nih.gov/periodic-table/", "PubChem" } ", "
                a { href: "https://www.webelements.com/periodicity/contents/", "WebElements" } ", "
                a { href: "https://en.wikipedia.org/wiki/Category:Chemical_element_data_pages",
                    "Wikipedia" } ", " br{}
                a { href: "https://www.vertex42.com/ExcelTemplates/periodic-table-of-elements.html",
                    "Vertex42" } ", and "
                a { href: "https://github.com/lmmentel/mendeleev", "mendeleev" }
            }
        }

        for i in 57..= 71 { ElemTile { ordinal: i, annote: false } }
        p { style: "writing-mode: vertical-rl;",
            class: "text-center text-lg font-bold rotate-180", "Lanthanides" }
        for i in 89..=103 { ElemTile { ordinal: i, annote: false } }
        p { style: "writing-mode: vertical-rl;",
            class: "text-center text-lg font-bold rotate-180", "Actinides" }

        div { class: "col-span-3", p { class: "mt-2", "© 2024 "
            a { href: "https://github.com/mhfan", "M.H.Fan" } ", All rights reserved." } }
        div { class: "self-start rounded-sm text-center text-lg/6
            col-[span_14_/_span_14] bg-yellow-100", b { "f" } "-block" }
        div { class: "self-start rounded-sm border-l bg-blue-100",
            p { class: "invisible", "d-block" } } div { class: "empty" }
    }
} }

use inperiod::{ChemElem, ElemClass::*, ROMAN_NUM, UNICODE_SUPERS};
#[component] fn ElemTile(ordinal: u8, annote: bool) -> Element {
    let elem = ChemElem::from(ordinal);
    let mut over_ecfg = use_signal(|| false);
    let (name, (os_main, os_all)) = (elem.name(), elem.oxidation_states());

    let simplify_long_ecfg = || match ordinal {
        //format!("{}{}", prefix, ecfg.rfind(' ').map_or("", |pos| &ecfg[pos..]))
        81..=86   => format!("[Hg] 6p{}", UNICODE_SUPERS[ordinal as usize -  80]),
        113..=118 => format!("[Cn] 7p{}", UNICODE_SUPERS[ordinal as usize - 112]),
        _ => elem.electron_configuration().to_string()
    };

    let get_bg_color = || match elem.category() {
        AlkaliMetals        => "bg-alkali",
        AlkalineEarthMetals => "bg-alkaline",
        TransitionMetals    => "bg-transition",
        PoorMetals          => "bg-poor-metal",
        Metalloids          => "bg-metalloid",
        OtherNonmetals      => "bg-non-metal",
        Halogens            => "bg-halogen",
        NobleGases          => "bg-noble-gas",
        Lanthanoids         => "bg-lanthanide",
        Actinoids           => "bg-actinide",
        Unknown             => "bg-unknown",
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

    /* let need_highlight = move || {
        let sel = use_context::<Signal<Selection>>()();
        match sel.r#type {
            SelType::Period => sel.val == elem.period(),
            SelType::Group  => sel.val == elem.group(),
            SelType::Block  => sel.val == elem.block(),
            SelType::Class  => sel.val == elem.category(),
            _ => false,
        }
    }; */

    rsx! { //shadow-border-1 shadow-indigo-300 // size: 152x198
        div { class: format!("flex flex-col rounded-sm p-1 border border-indigo-300
            hover:shadow-orange-600 hover:shadow-spread-2 relative {} {}",
            //if need_highlight() { "outline-green-800 outline-2 outline" } else { "" },
            get_bg_color(), draw_metal_bound()), if annote { div { //class: "pointer-events-none",
                //onmouseover: move |evt| evt.stop_propagation(), // XXX: not work for :hover
                a { class: "absolute font-bold text-lg/6 text-amber-600",
                    href: "https://ciaaw.org/radioactive-elements.htm",
                    style: "top: -1.5rem; right: 1rem;", "ratioactive" }
                div { class: "absolute text-lg leading-tight text-nowrap text-right",
                    style: "right: calc(100% + 0.4rem);",
                    p { a { href: "https://ciaaw.org/atomic-weights.htm",
                        "*standard atomic weight/mass" } }
                    a { href: "https://www.nist.gov/pml/periodic-table-elements",
                        "1st ionization energy (eV)" }
                    p { class: "mt-3 mb-5", "symbol" } p { "name" }
                    p { span { class: "text-blue-700", "melting" } "/"
                        span { class: "text-red-700",  "boiling" } " point (°C)" }
                    p { "*density" } a {
                        href: "https://en.wikipedia.org/wiki/Electron_configuration",
                        "electron configuration" }
                }
                div { class: "absolute text-lg leading-tight text-nowrap",
                    style: "left: calc(100% + 0.4rem);",
                    p { class: "mt-1", "atomic number" }
                    p { "electron affinity" }
                    p { class: "my-1", "main "
                a { href: "https://en.wikipedia.org/wiki/Oxidation_state", "oxidation states" } }
                    p { "Chinese name with pinyin" }
                    a { href: "https://en.wikipedia.org/wiki/Electronegativity",
                        "electronegativity (pauling)*" }
                    p { a { href: "https://en.wikipedia.org/wiki/Periodic_table_(crystal_structure)", "crystal structure" } }
                    a { href: "https://www.nist.gov/pml/periodic-table-elements",
                        "ground-state level" }
                    p { span { class: "text-purple-700 font-bold",
                        a { href: "https://en.wikipedia.org/wiki/Atomic_radius",
                            "atomic radius" } } " (pm)*" }
                }
            } }

            div { class: "flex",
                div { class: "grow",
                    p { class: "flex text-lg/6 font-bold", { elem.atomic_weight().to_string() }
                        if elem.is_ratioactive() { span { class: "ml-1 text-center grow", "☢️" } }
                    }
                    p { class: "flex text-base/5", {
                        elem.ionization_energy().map_or_else(|| "-".to_string(),
                            |ie| format!("{:.3}", ie.0).trim_end_matches('0')
                                .trim_end_matches('.').to_owned()) }
                        span { class: "ml-2 text-center grow", {
                            elem.electron_affinity().map_or_else(|| " ".to_string(),
                                |ea| ea.to_string())
                        } }
                    }
                }
                span { class: "text-2xl font-bold ml-1", "{ordinal}" }
            }
            div { class: "flex-col grow",
                div { class: "flex",
                    span { class: format!("self-center text-5xl grow ml-1 {}",
                        color_for_symbol()), { elem.symbol() } }
                    div { class: "flex flex-col mr-1",
                        p { class: "text-center leading-tight", { elem.name_py() } }
                        a { href: format!("https://zh.wikipedia.org/wiki/{}", elem.name_ch()),
                            class: "text-right text-3xl", { elem.name_ch().to_string() } }
                    }
                    div { class: "text-right ml-1 relative",
                        div { class: "absolute w-full h-full group font-bold leading-tight",
                            for os in os_main.iter().rev() { pre { { format!("{}{os}",
                                match *os { x if 0 < x => "+", 0 => " ", _ => "" })
                            } } }
                            if os_main.len() < os_all.len() { div { class:
                                "absolute hidden left-full -top-2 ml-1.5 p-1 text-lg/5 rounded
                                border border-orange-600 bg-indigo-50 group-hover:block z-10",
                                for os in os_all.iter().rev() { pre { class:
                                    if os_main.contains(os) { "font-extrabold" } else { "" },
                        { format!("{}{os}", match *os { x if 0 < x => "+", 0 => " ", _ => "" }) }
                                } }
                            } }
                        }   pre { class: "invisible", "  " }
                    }
                }
                p { a { href: format!("https://en.wikipedia.org/wiki/{name}"),
                        class: "text-lg/6", { name } }
                    span { class: "ml-2 font-bold", { match ordinal {
                        42|59 => "*".to_string(), // name is too long
                        _ => elem.en_pauling().map_or_else(|| "".to_string(), |en| en.to_string())
                    } } }
                }

                p { class: "flex text-base/5 relative",
                    span { class: "text-blue-700 font-bold", {
                        elem.melting_point().map_or_else(|| "-".to_string(),
                            |mp| format!("{}", (mp - 273.15 + 0.5) as i32))
                    } } "/"
                    span { class: "text-red-700",  {
                        elem.boiling_point().map_or_else(|| "-".to_string(),
                            |bp| format!("{}", (bp - 273.15 + 0.5) as i32))
                    } }
                    { elem.crystal_structure().map_or_else(|| rsx! { span { class: "ml-2", "-" } },
                        |(cs, file)| rsx! {
                        span { class: "pl-2 grow peer", { cs.to_string() } }
                        figure { class: "absolute w-[10rem] max-w-none border rounded
                                border-orange-600 bg-white hidden peer-hover:block z-10",
                            style: format!("{}", match ordinal {
                                89..=103|2|10|18|36|54|71|86|118 =>
                                    "right: calc(100% + 0.4rem); bottom: 0px;",
                                _ => "left: calc(100% + 0.4rem);", }),
                            //figcaption {} // XXX: title/desc
                            img { class: "w-full", src: format!("crystal-s/{file}"), }
                        } }
                    ) }
                }
                p { class: "flex text-base/5", {
                    elem.density().map_or_else(|| "-".to_string(), |den| format!("{:.4}", den)
                        .trim_end_matches('0').trim_end_matches('.').to_string()) }
                    span { class: "ml-2 font-bold text-purple-700", {
                        elem.atomic_radius().map_or_else(|| "-".to_string(), |cr| cr.to_string())
                    } }
                    span { class: "ml-2 font-bold", {
                        elem.ground_state().map_or_else(|| rsx! { "-" },
                            |(s1, s2, s3)| rsx! { if 1 < s2.len() {
                                sup  { {s1} } { s2.chars().next().unwrap().to_string() }
                                span { style: "position: relative; top: -0.1em;", "°" }
                                sub  { style: "left: -0.6em;", {s3} }
                            } else { sup { {s1} } {s2} sub { {s3} } } })
                    } }
                }
                p { class: "flex mt-auto text-nowrap font-bold text-lg/6 group",
                    onmouseout:  move |_| over_ecfg.set(false),
                    onmouseover: move |_| over_ecfg.set(true), { simplify_long_ecfg() }
                    if *over_ecfg.read() { figure {
                        class: "absolute w-[40rem] max-w-none border rounded
                            border-orange-600 bg-white group-hover:block z-10",
                        style: format!("{}", match ordinal {
                            2|7..=10 => "right: calc(100% + 0.125rem);    top: -0.2em;",
                            57|89    =>  "left: calc(100% + 0.125rem); bottom: -0.2em;",
                            _ => if ordinal == 71  ||
                                    ordinal == 103 || matches!(elem.group(), 15..=19) {
                                        "right: calc(100% + 0.125rem); bottom: -0.2em;"
                            } else {     "left: calc(100% + 0.125rem);    top: -0.2em;" }
                        }), ShowEcfg { ordinal } //figcaption {}
                    } }

                    if matches!(ordinal, 42|59) {
                        span { class: "ml-2 font-bold grow text-right", {
                            elem.en_pauling().map(|en| en.to_string())
                        } }
                    }
                }
                // TODO: show origin/source and abundance according to selection
            }
        }
    }
}

#[component] fn ShowEcfg(ordinal: u8) -> Element {
    let elem = ChemElem::from(ordinal);
    let ecfg = elem.electron_configuration().expand();

    rsx! { svg { width: "640", height: "512", xmlns: "http://www.w3.org/2000/svg",
        "font-size": "small", "font-weight": "normal",
        title { "Electron shell/orbital configuration" }

        g { text { x: "200", y: "500", "Electron shell/orbital configuration" }
            text { x: "560", y: "464", "font-size": "48", { elem.symbol() } }

            path { stroke: "gray", "stroke-opacity": "0.3", d:
                    "M32,32 h 588 m 0,64 h-588 m 0,64 h 588 m 0,64 h-588
                     m 0,64 h 588 m 0,64 h-588 m 0,64 h 588 m 0,64 h-588 ",
            }
            text { x: "20", y: "400", transform: "rotate(-90 20,400)",
                "Energy increase (not to scale)"
            }
            path { "stroke-linecap": "round", "stroke-linejoin": "round", fill: "gray",
                "stroke-width": "2", stroke: "gray", d: "M15,96 l-4,12 h8 Z v88",
            }
        }
        g { fill: "black",
            text { transform: "translate(68,0)",
                tspan { x: "0",  y: "24", "8s" }
                tspan { x: "0", dy: "64", "7s" }
                tspan { x: "0", dy: "64", "6s" }
                tspan { x: "0", dy: "64", "5s" }
                tspan { x: "0", dy: "64", "4s" }
                tspan { x: "0", dy: "64", "3s" }
                tspan { x: "0", dy: "64", "2s" }
                tspan { x: "0", dy: "64", "1s" }
            }
            text { transform: "translate(180,0)",
                tspan { x: "0",  y: "48", "7p" }
                tspan { x: "0", dy: "64", "6p" }
                tspan { x: "0", dy: "64", "5p" }
                tspan { x: "0", dy: "64", "4p" }
                tspan { x: "0", dy: "64", "3p" }
                tspan { x: "0", dy: "64", "2p" }
            }
            text { transform: "translate(358,0)",
                tspan { x: "0",  y: "62", "6d" }
                tspan { x: "0", dy: "64", "5d" }
                tspan { x: "0", dy: "64", "4d" }
                tspan { x: "0", dy: "64", "3d" }
            }
            text { x: "598", y:  "76", "5f" }
            text { x: "598", y: "140", "4f" }
            text { y: "24", "letter-spacing": "-2",
                tspan { x: "110", "ℓ = 1" }
                tspan { x: "256", "ℓ = 2" }
                tspan { x: "464", "ℓ = 3" }
            }
        }
        g { stroke: "gray", "stroke-dasharray": "4 4", "stroke-opacity": "0.5",
            path { d: "M84,80  L180,48" }
            path { d: "M84,144 L180,112 m16,-4 L358,62" }
            path { d: "M84,208 L180,176 m16,-4 L358,126 m16,-4 L598,76" }
            path { d: "M84,272 L180,240 m16,-4 L358,190 m16,-4 L598,140" }
            path { d: "M84,336 L180,306 m16,-4 L358,254" }
            path { d: "M84,400 L180,370" }
        }
        g { "stroke-width": "4", "stroke-linecap": "round", "stroke-dasharray": "24 8",
            path { stroke: "red", d:
                    "M40,20 h 24 m 0,64 h-24 m 0,64 h 24 m 0,64 h-24
                     m 0,64 h 24 m 0,64 h-24 m 0,64 h 24 m 0,64 h-24",
            }
            path { stroke: "green",
                d: "M88,44 h 88 m 0,64 h-88 m 0,64 h 88 m 0,64 h-88 m 0,64 h 88 m 0,64 h-88",
            }
            path { stroke: "blue", d: "M202,58 h 152 m 0,64 h-152 m  0,64 h 152 m 0,64 h-152" }
            path { stroke: "orange", "stroke-dasharray": "24 8", d: "M378,72 h 216 m 0,64 h-216" }
        }

        g { "font-size": "24", for shell in ecfg {
            if shell.orbital == b's' { text {
                transform: format!("translate(38,{})",  541 - 64 * shell.level as i32),
                tspan { x: "0", dx: "0 -16", if shell.ecount == 2 { "⭡⭣" } else { "⭡" } }
            } } else if shell.orbital == b'p' { text {
                transform: format!("translate(86,{})",  501 - 64 * shell.level as i32),
                for i in 0..3 { if i < shell.ecount {
                    tspan { x: "0", dx: format!("{} -16", i * 32),
                        if shell.ecount < 4 + i { "⭡" } else { "⭡⭣" }
                    }
                } }
            } } else if shell.orbital == b'd' { text {
                transform: format!("translate(200,{})", 451 - 64 * shell.level as i32),
                for i in 0..5 { if i < shell.ecount {
                    tspan { x: "0", dx: format!("{} -16", i * 32),
                        if shell.ecount < 6 + i { "⭡" } else { "⭡⭣" }
                    }
                } }
            } } else if shell.orbital == b'f' { text {
                transform: format!("translate(376,{})", 401 - 64 * shell.level as usize),
                for i in 0..7 { if i < shell.ecount {
                    tspan { x: "0", dx: format!("{} -16", i * 32),
                        if shell.ecount < 8 + i { "⭡" } else { "⭡⭣" }
                    }
                } }
            } }
        } }
    } }
}

#[component] fn ElemDetail() -> Element { rsx! { // TODO: https://pt.ziziyi.com/
} }

/// https://physics.nist.gov/cuu/Constants/index.html
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

