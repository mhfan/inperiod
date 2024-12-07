
#![allow(non_snake_case)]
use dioxus::prelude::*;
//use dioxus_logger::tracing;

// URLs are relative to your Cargo.toml file
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

use inperiod::{l10n::Localization, ChemElem, ElemClass::*, ROMAN_NUM};
macro_rules! tr { ($lang:expr, $id:expr) => { $lang.read().translate($id) } }
//#[derive(Clone, Copy)] enum SelType { None, Period, Group, Block, Class, }
//#[derive(Clone, Copy)] struct Selection { r#type: SelType, val: u8, }

fn PeriodicTable() -> Element {
    use_context_provider(|| Signal::new(Localization::new()));
    let mut lang = use_context::<Signal<Localization>>();
    //use_context_provider(|| Signal::new(Selection { r#type: SelType::None, val: 0, }));
    //let mut group_sel = use_context::<Signal<Selection>>();   // move to ahead of rsx!

    rsx! { div { class: "grid grid-cols-[auto_repeat(18,1fr)_auto] w-[181rem] p-6 gap-0.5 relative
        grid-rows-[auto_repeat(7,1fr)_auto_1fr_1fr_auto]", // scale-50 origin-top-left h-[128rem]
        //style: "transform: scale(0.5); transform-origin: 0 0;", // use js script in index.html
        //style: "zoom: 0.5;", // malformed in Safari, which works well scaling on <html>

        p { class: "font-bold relative -bottom-4", style: "writing-mode: vertical-rl;",
            {tr!(lang, "PERIOD")}
        }
        div { class: "self-end text-center font-bold",
            p { class: "leading-none", {tr!(lang, "GROUP")} }
            p { class: "text-lg/6", "IA - 1" }
        }
        p { class: "relative col-[span_16_/_span_16] text-center font-bold text-5xl",
            a { href: "https://github.com/mhfan/inperiod",
                {tr!(lang, "Periodic Table of the Elements")}
            }
            select { class: "absolute top-2 left-0 text-2xl text-center font-normal
                non-printable text-red-600", name: "lang-sel", //id: "lang-sel",
                onchange: move |e| lang.write().set_lang(e.value()),
                option { value: "en-US", "en" } option { value: "zh-CN", "中文" }
            }
        }
        p { class: "self-end text-center font-bold text-lg", "VIIIA - 18" }
        p { class: "font-bold leading-none relative -bottom-4 content-center ml-2",
            style: "writing-mode: vertical-rl;", {tr!(lang, "E-shell")} br{} {tr!(lang, "E-max")}
        }
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
            //div { class: "absolute h-[150%]", AufbauPrincipal {} }
            p { class: "absolute bottom-[-5rem] right-1",
                {tr!(lang, "metal - nonmetal dividing line")} }
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
            shadow-[0_2px] shadow-indigo-300", "VIIIB - {i}" }
        } } */
        p { class: "self-end text-center text-lg font-bold col-span-3
            shadow-[0_2px] shadow-indigo-300", "VIIIB - 8|9|10"
        }   // border-b-2
        p { class: "self-end text-center text-lg font-bold",  "IB - 11" }
        p { class: "self-end text-center text-lg font-bold", "IIB - 12" }

        div { class: "absolute flex w-full h-full pb-10", style: "grid-area: 3 / 4 / 5 / 14;",
            div { class: "flex self-end mx-auto",
                div { class: "text-lg/6",
                    div { class: "flex flex-col font-bold mb-4",
                        p { class: "self-start mb-1 px-1 border-b border-black",
                            {tr!(lang, "Phase at STP")}
                        }
                        p { span { class: "text-liquid", {tr!(lang, "Liquid")} }
                            span { class: "mx-4 text-gas", {tr!(lang, "Gas")} }
                            span { {tr!(lang, "Solid")} }
                            span { class: "ml-4 text-synthetic", {tr!(lang, "Synthetic")} }
                        }
                    }
                    p { class: "font-bold px-1", {tr!(lang, "Categories")} }
                    div { class: "grid grid-cols-2 grid-rows-5 text-center",
                        p { class: "content-center rounded-sm bg-alkali", "Alkali Metals 碱金属" }
                        p { class: "content-center rounded-sm bg-noble-gas px-1",
                            "Noble Gases 稀有气体"
                        }
                        p { class: "content-center rounded-sm bg-alkaline",
                            "Alkaline Earth Metals" br{} "碱土金属"
                        }
                        p { class: "content-center rounded-sm bg-halogen", "Halogens 卤素" }

                        p { class: "content-center rounded-sm bg-transition",
                            "Transition Metals" br{} "过渡金属"
                        }
                        p { class: "content-center rounded-sm bg-non-metal",
                            "Other nonmetals" br{} "其它非金属"
                        }   // 活泼非金属
                        p { class: "content-center rounded-sm bg-poor-metal", "Poor Metals 贫金属" }
                        p { class: "content-center rounded-sm bg-metalloid", "Metalloids 类金属" }

                        p { class: "content-center rounded-sm bg-lanthanide",
                            "Rare Earth Metals*" br{} "稀土金属"
                        }
                        p { class: "content-center rounded-sm bg-unknown", "Unknown 未知" }
                    }
                }
                div { class: "self-end ml-6 text-lg/6", PhysConsts {} }
            }
        }
        div { class: "absolute flex w-full h-full pb-8", style: "grid-area: 2 / 8 / 3 / 19;",
        }   // XXX: show legend for origin/abundance?

        for i in 13..=56 { ElemTile { ordinal: i, annote: false } }
        div { class: "flex flex-col text-2xl rounded-sm p-1 bg-lanthanide
            shadow-border-1 shadow-indigo-300", span { class: "self-end font-bold", "71" }
            p { class: "text-center m-auto", b { "57 ~ 70" } br{} "Lanthanoids" br{} "(镧系)" }
        }
        for i in 72..=88 { ElemTile { ordinal: i, annote: false } }
        div { class: "flex flex-col text-2xl rounded-sm p-1 bg-actinide
            shadow-border-1 shadow-indigo-300", span { class: "self-end font-bold", "103" }
            p { class: "text-center m-auto", b { "89 ~ 102" } br{} "Actinoids" br{} "(锕系)" }
        }
        for i in 104..=118 { ElemTile { ordinal: i, annote: false } }

        div { class: "empty row-span-4" }
        div { class: "text-center text-lg/6 rounded-sm col-span-2  bg-red-100 self-start",
            b { "s" } {tr!(lang, "-block")} " (" {tr!(lang, "plus")} b { " He" } ")"
        }
        div { class: "text-center text-lg/6 rounded-sm col-span-10 bg-blue-100 border-x mb-6",
            b { "d" } {tr!(lang, "-block")} " (" {tr!(lang, "exclude")}
            i { " Lan/Act " } {tr!(lang, "series")} ")"
        }
        div { class: "text-center text-lg/6 rounded-sm col-span-6  bg-green-100 self-start",
            b { "p" } {tr!(lang, "-block")} " (" {tr!(lang, "exclude")} i { " He" } ")"
        }   div { class: "empty" }

        div { class: "flex flex-col col-span-3 row-span-2 mr-2",
            p { class: "text-lg font-bold leading-none pb-1", {tr!(lang, "Notes:")} }
            ul { class: "list-disc",
                li { {tr!(lang, "Standard atomic mass (A")} sub { "r" }
                    i { {tr!(lang, "°, in Dalton or ")} }
                    span { class: "text-xl leading-none", "𝓂" sub { "μ" } }
                    {tr!(lang, ") is the weighted arithmetic mean of the relative isotopic masses of all isotopes of an element, weighted by their abundance on Earth")}
                }
                li { i { "[ ]" } {tr!(lang, " indicate mass number of most stable isotope")} }
                li { {tr!(lang, "Density units are ")} i { "g/cm3" } {tr!(lang, " for solids and ")}
                    i { "g/L" } {tr!(lang, " for liquid")} br{} {tr!(lang, " or ")} i { "kg/m3" }
                    {tr!(lang, " at 0° Celsius for gases")}
                }
                li { {tr!(lang, "* mark means the electronegativity is in the bottom-right")} }
                li { {tr!(lang, "Rare earth metals include: ")}
                    i { {tr!(lang, "Lanthanoids (La ~ Lu), Sc and Y")} }
                }
                li { {tr!(lang, "Atomic radius is ")} i { {tr!(lang, "van der Waals radii")} } }
                //li { i { "§" } " indicates crystal structure is unusual" }
                // or may require explanation
            }
            p { class: "text-lg font-bold mt-auto", {tr!(lang, "References:")} }
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
        p { class: "text-center text-lg font-bold rotate-180",
            style: "writing-mode: vertical-rl;", "Lanthanides"
        }
        for i in 89..=103 { ElemTile { ordinal: i, annote: false } }
        p { class: "text-center text-lg font-bold rotate-180",
            style: "writing-mode: vertical-rl;", "Actinides"
        }
        p { class: "col-span-3 mt-2", {tr!(lang, " All rights reserved.")} " © 2024 "
            a { href: "https://github.com/mhfan", "M.H.Fan" }
        }
        div { class: "self-start rounded-sm text-center text-lg/6
            col-[span_14_/_span_14] bg-yellow-100", b { "f" } {tr!(lang, "-block")}
        }
        div { class: "self-start rounded-sm border-l bg-blue-100",
            p { class: "invisible", "d-block" }
        }   div { class: "empty" }
    } }
}

#[component] fn ElemTile(ordinal: u8, annote: bool) -> Element {
    let elem = ChemElem::from(ordinal);
    let mut over_ecfg = use_signal(|| false);
    let lang = use_context::<Signal<Localization>>();
    let (name, (os_main, os_all)) = (elem.name(), elem.oxidation_states());

    let revised_ecfg = match ordinal {
        //format!("{prefix}{}", ecfg.rfind(' ').map_or("", |pos| &ecfg[pos..]))
        81..=86   => format!("[Hg] 6p{}", UNICODE_SUPERS[ordinal as usize -  80]),
        113..=118 => format!("[Cn] 7p{}", UNICODE_SUPERS[ordinal as usize - 112]),

        _ => {  let ecfg = elem.electron_configuration();
            if ordinal < 11 { ecfg.expand().map(|x|
                x.to_string()).collect::<Vec<_>>().join(" ")
            } else { ecfg.to_string() }
        }
    };

    let bg_color = match elem.category() {
        Unknown             => "bg-unknown",
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
    };

    let metal_bound = match ordinal {
        1 => "shadow-black-b", 118 => "shadow-black-l", 4 => "shadow-[0_-2px_black]",
        21|39|71 => "shadow-spread-2 shadow-amber-300", // rare earth metals indication
        2 => "shadow-[0_2px_#fca5a5]", // indicate He is of s-block, shadow-red-300
        5|14|33|52|85 => "shadow-black-bl", _ => "",
    };

    let color_symbol = match ordinal {
        1|2|7..=10|17|18|36|54|86 => "text-gas",
        43|61|96..=118 => "text-synthetic",
        35|80 => "text-liquid", _ => "",
    };

    /* let highlight = {
        let sel = use_context::<Signal<Selection>>()();
        if match sel.r#type {
            SelType::Period => sel.val == elem.period(),
            SelType::Group  => sel.val == elem.group(),
            SelType::Block  => sel.val == elem.block(),
            SelType::Class  => sel.val == elem.category(),
            _ => false,
        } { "outline-green-800 outline-2 outline" } else { "" }
    }; */

    use inperiod::{UNICODE_SUPERS, CosmicOrigin::{self, *}};
    let origin_color = |x: CosmicOrigin| match x {
        BigBangFusion => "#e2e8f0",

        CosmicRayFission => "#d6d3d1",
        RadioactiveDecay => "#fce7f3",

        DyingLowMassStars       => "#a5f3fc",
        ExplodingMassiveStars   => "#ddd6fe",
        WhiteDwarfSupernovae    => "#fecdd3",
        MergingNeutronStars     => "#d9f99d",

        HumanSynthesis => "#fafafa",
    };

    rsx! { div { //shadow-border-1 shadow-indigo-300    // size: 152x198
        class: "flex flex-col relative rounded-sm p-1 border border-indigo-300
            hover:shadow-orange-600 hover:shadow-spread-2 {bg_color} {metal_bound}",
        style: if false { format!("background: conic-gradient({});", {  let mut sum = 0;
            elem.cosmic_origin().iter().map(|&(co, ratio)| { let start = sum; sum += ratio;
                format!("{} {start}% {sum}%", origin_color(co.into()))
            }).collect::<Vec<_>>().join(", ")
        })}, if annote {
            a { class: "absolute font-bold text-lg/6 text-amber-600 self-center",
                href: "https://ciaaw.org/radioactive-elements.htm",
                style: "top: -1.5rem;", {tr!(lang, "radioactive")}
            }
            div { class: "absolute text-lg leading-tight text-nowrap text-right",
                onmouseover: |evt| evt.stop_propagation(), // XXX: not work for :hover
                style: "right: calc(100% + 0.4rem);",
                p { a { href: "https://ciaaw.org/atomic-weights.htm",
                    {tr!(lang, "*atomic weight")}
                } }
                a { href: "https://www.nist.gov/pml/periodic-table-elements",
                    {tr!(lang, "1st ionization energy")} " (eV)"
                }
                p { class: "mt-3 mb-5", {tr!(lang, "symbol")} } p { {tr!(lang, "name")} }
                p { span { class: "text-blue-700", {tr!(lang, "melting")} } "/"
                    span { class: "text-red-700",  {tr!(lang, "boiling")} }
                    {tr!(lang, " point")} " (℃)"
                }
                p { {tr!(lang, "*density")} }
                a { href: "https://en.wikipedia.org/wiki/Electron_configuration",
                    {tr!(lang, "electron configuration")}
                }
            }
            div { class: "absolute text-lg leading-tight text-nowrap",
                style: "left: calc(100% + 0.4rem);",
                p { class: "mt-1", {tr!(lang, "atomic number")} }
                p { {tr!(lang, "electron affinity")} }
                a { class: "my-1",
                    href: "https://en.wikipedia.org/wiki/Oxidation_state",
                    {tr!(lang, "main oxidation states")}
                }
                p { {tr!(lang, "Chinese name with pinyin")} }
                a { href: "https://en.wikipedia.org/wiki/Electronegativity",
                    {tr!(lang, "electronegativity")} " (pauling)*"
                }
                p { a { href: "https://en.wikipedia.org/wiki/Periodic_table_(crystal_structure)", {tr!(lang, "crystal structure")}
                } }
                a { href: "https://www.nist.gov/pml/periodic-table-elements",
                    {tr!(lang, "ground-state level")} // "term symbol"?
                }
                p { span { class: "text-purple-700 font-bold",
                    a { href: "https://en.wikipedia.org/wiki/Atomic_radius",
                        {tr!(lang, "atomic radius")}
                    }
                } " (pm)*" }
            }
        }

        div { class: "flex",
            div { class: "grow",
                p { class: "flex text-lg/6 font-bold", { elem.atomic_weight().to_string() }
                    if elem.is_radioactive() { span { class: "ml-1 text-center grow", "☢️" } }
                }
                p { class: "flex text-base/5",
                    { elem.ionization_energy().map_or_else(|| "-".to_string(),
                        |ie| format!("{:.3}", ie.0).trim_end_matches('0')
                            .trim_end_matches('.').to_owned()) }
                    span { class: "ml-2 text-center grow",
                        { elem.electron_affinity().map_or_else(|| " ".to_string(),
                            |ea| ea.to_string()) }
                    }
                }
            }
            span { class: "text-2xl font-bold ml-1", "{ordinal}" }
        }

        div { class: "flex-col grow",
            div { class: "flex",
                span { class: "self-center text-5xl grow ml-1 {color_symbol}", { elem.symbol() } }
                div { class: "flex flex-col mr-1",
                    p { class: "text-center leading-tight", { elem.name_py() } }
                    a { href: format!("https://zh.wikipedia.org/wiki/{}", elem.name_ch()),
                        class: "text-right text-3xl", { elem.name_ch().to_string() }
                    }
                }
                div { class: "text-right ml-1 relative",
                    div { class: "absolute w-full h-full group font-bold leading-tight",
                        for os in os_main.iter().rev() { pre { { format!("{}{os}",
                            match *os { x if 0 < x => "+", 0 => " ", _ => "" })
                        } } }
                        if os_main.len() < os_all.len() { div { class: "absolute hidden
                            left-full -top-2 ml-1.5 p-1 text-lg/5 font-normal rounded
                            border border-orange-600 bg-indigo-50 group-hover:block z-10",
                            for os in os_all.iter().rev() { pre { class:
                                if os_main.contains(os) { "font-extrabold" } else { "" },
                    { format!("{}{os}", match *os { x if 0 < x => "+", 0 => " ", _ => "" }) }
                            } }
                        } }
                    }   pre { class: "invisible", "  " }
                }
            }
            p { a { href: "https://en.wikipedia.org/wiki/{name}", class: "text-lg/6", { name } }
                span { class: "ml-2 font-bold", { match ordinal {
                    42|59 => "*".to_string(), // name is too long, move to bottom-right
                    _ => elem.en_pauling().map_or_else(|| "".to_string(), |en| en.to_string())
                } } }
            }

            p { class: "flex text-base/5 relative",
                span { class: "text-blue-700 font-bold",
                    { elem.melting_point().map_or_else(|| "-".to_string(),
                        |mp| format!("{}", (mp - 273.15 + 0.5) as i32)) }
                } "/"
                span { class: "text-red-700",
                    { elem.boiling_point().map_or_else(|| "-".to_string(),
                        |bp| format!("{}", (bp - 273.15 + 0.5) as i32)) }
                }
                { elem.crystal_structure().map_or_else(|| rsx! { span { class: "ml-2", "-" } },
                    |(cs, file)| rsx! {
                    span { class: "pl-2 grow peer", { cs.to_string() } }
                    figure { class: "absolute w-[10rem] max-w-none border rounded
                            border-orange-600 bg-white hidden peer-hover:block z-10",
                        style: { match ordinal {
                            89..=103|2|10|18|36|54|71|86|118 =>
                                "right: calc(100% + 0.4rem); bottom: 0px;",
                            _ => "left: calc(100% + 0.4rem);",
                        } }, //figcaption {} // TODO: title/desc
                        img { class: "w-full", src: "crystal-s/{file}", }
                    }
                }) }
            }
            p { class: "flex text-base/5",
                { elem.density().map_or_else(|| "-".to_string(), |den| format!("{den:.4}")
                    .trim_end_matches('0').trim_end_matches('.').to_string()) }
                span { class: "ml-2 font-bold text-purple-700",
                    { elem.atomic_radius().map_or_else(|| "-".to_string(), |cr| cr.to_string()) }
                }
                span { class: "ml-2 font-bold",
                    { elem.ground_state().map_or_else(|| rsx! { "-" },
                        |(s1, s2, s3)| rsx! { if 1 < s2.len() {
                            sup  { {s1} } { s2.chars().next().unwrap().to_string() }
                            span { style: "position: relative; top: -0.1em;", "°" }
                            sub  { style: "left: -0.6em;", {s3} }
                        } else { sup { {s1} } {s2} sub { {s3} } } }) }
                }
            }
            p { class: "flex mt-auto text-nowrap font-bold text-lg/6 group",
                onmouseout:  move |_| over_ecfg.set(false),
                onmouseover: move |_| over_ecfg.set(true), { revised_ecfg }
                if *over_ecfg.read() { figure {
                    class: "absolute w-[40rem] max-w-none border rounded
                        border-orange-600 bg-white group-hover:block z-10",
                    style: { match ordinal {
                        2|7..=10 => "right: calc(100% + 0.125rem);    top: -0.2em;",
                        57|89    =>  "left: calc(100% + 0.125rem); bottom: -0.2em;",
                        _ => if ordinal == 71  ||
                                ordinal == 103 || matches!(elem.group(), 15..=19) {
                                    "right: calc(100% + 0.125rem); bottom: -0.2em;"
                        } else {     "left: calc(100% + 0.125rem);    top: -0.2em;" }
                    } }, ShowEcfg { ordinal } //figcaption {}
                } }

                if matches!(ordinal, 42|59) { span { class: "ml-2 font-bold grow text-right",
                    { elem.en_pauling().map(|en| en.to_string()) }
                } }
            }
            // TODO: show origin/source and abundance according to selection
        }
    } }
}

#[component] fn ShowEcfg(ordinal: u8) -> Element {
    let elem = ChemElem::from(ordinal);
    let lang = use_context::<Signal<Localization>>();
    let ecfg = elem.electron_configuration().expand();

    rsx! { svg { width: "640", height: "512", xmlns: "http://www.w3.org/2000/svg",
        "font-size": "small", "font-weight": "normal", //title { title }

        g { text { x: "200", y: "500", {tr!(lang, "Electron shell/orbital configuration")} }
            text { x: "560", y: "464", "font-size": "48", { elem.symbol() } }

            path { stroke: "gray", "stroke-opacity": "0.3", d:
                    "M32,32 h 588 m 0,64 h-588 m 0,64 h 588 m 0,64 h-588
                     m 0,64 h 588 m 0,64 h-588 m 0,64 h 588 m 0,64 h-588 ",
            }
            text { x: "20", y: "400", transform: "rotate(-90 20,400)",
                {tr!(lang, "Energy increase (not to scale)")}
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
            text { y: "24", "letter-spacing": "-1",
                tspan { x: "120", "ℓ = 1" }
                tspan { x: "266", "ℓ = 2" }
                tspan { x: "474", "ℓ = 3" }
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

/// https://physics.nist.gov/cuu/Constants/index.html
fn PhysConsts() -> Element {
    let lang = use_context::<Signal<Localization>>();
    rsx! {
        p { class: "flex text-lg/6 justify-between px-1",
            span { class: "font-bold", {tr!(lang, "Common physical constants")} }
            span { {tr!(lang, "Source: ")}
                a { href: "https://physics.nist.gov/constants", "physics.nist.gov" } " (2022)"
            }
        }
        div { class: "grid grid-cols-[repeat(2,auto)]
            border-black border divide-black divide-x",
            div { class: "grid grid-cols-[repeat(3,auto)] gap-x-3 px-2",
                p { {tr!(lang, "electron mass")} }
                span { class: "text-xl leading-none -top-2",
                    "𝓂" sub { class: "text-lg leading-none", "𝑒" } }
                p { "9.109 383 7139(28) × 10⁻³¹ kg" }

                p { {tr!(lang, "atomic mass unit")}
                    span { class: "text-xl leading-none", " 𝓂" } "(¹²C)/12"
                }
                span { class: "text-xl leading-none", "𝓂" sub { "μ" } }
                p { "1.660 539 068 92(52) × 10⁻²⁷ kg" }

                p { {tr!(lang, "fine-structure const.")}
                    span { class: "text-xl leading-none", " 𝑒" } "²/4π𝜖₀ℏ𝑐" } // 𝜋ε
                span { class: "text-xl leading-none", "𝛼" } // α
                p { "7.297 352 5643(11) × 10⁻³ (~1/137)" }

                p { {tr!(lang, "Newtonian const. of gravitation")} } span { "𝐺" }
                p { "6.674 30(15) × 10⁻¹¹ m³ kg⁻¹ s⁻²" }

                p { {tr!(lang, "Rydberg constant")} } span { "𝑅" sub { "∞" } }
                p { "10 973 731.568 157(12) [m⁻¹]" }

                p { {tr!(lang, "classical electron radius")}
                    span { class: "text-lg leading-none", " 𝛼²𝑎₀" }
                }
                span { class: "text-lg leading-none", "𝑟" sub { "𝑒" } }
                p { "2.817 940 3205(13) × 10⁻¹⁵ m" }

                p { {tr!(lang, "molar volume of ideal gas")} " 𝑅𝑇/𝑝" }
                span { "𝑉" sub { "m" } } p { "22.413 969 54... × 10⁻³ m³/mol" }

                p { {tr!(lang, "first radiation constant")} " 2π"
                    span { class: "text-xl leading-none", "ℎ" } "𝑐²" }
                span { i { class: "text-lg leading-none", "c" } "₁" }
                p { "3.741 771 852... × 10⁻¹⁶ [W m²]" }

                p { {tr!(lang, "second radiation constant")}
                    span { class: "text-xl leading-none", " ℎ" } "𝑐/𝑘" }
                span { i { class: "text-lg leading-none", "c" } "₂" }
                p { "1.438 776 877... × 10⁻² [m K]" }

                p { "¹³³Cs " {tr!(lang, "hyperfine transition freq.")} }
                span { "∆ν" sub { "Cs" } } p { "9 192 631 770 Hz" }
            }
            div { class: "grid grid-cols-[repeat(3,auto)] gap-x-3 px-2",
                p { {tr!(lang, "Avogadro constant")} }
                span { "𝑁" sub { "A" } } p { "6.022 140 76 × 10²³ mol⁻¹" }

                p { {tr!(lang, "Planck constant")} }
                span { class: "text-2xl leading-none", "ℎ" } p { "6.626 070 15 × 10⁻³⁴ J/Hz" }
                p { class: "text-center", span { class: "text-2xl leading-none", "ℎ" } "/2π" }
                span { class: "text-lg leading-tight", "ℏ" } // ħ
                p { "1.054 571 817... × 10⁻³⁴ J s" }

                p { {tr!(lang, "Boltzmann constant")} }
                span { class: "text-xl leading-tight", "𝑘" }
                p { "1.380 649 00 × 10⁻²³ J/K" }

                p { {tr!(lang, "Faraday constant")} " 𝑁" sub { "A" } "𝑒" } span { "𝐹" }
                p { "96 485.332 12... C/mol" }

                p { {tr!(lang, "molar gas constant")} " 𝑁" sub { "A" } "𝑘" } span { "𝑅" }
                p { "8.314 462 618... J mol⁻¹ K⁻¹" }

                p { {tr!(lang, "elementary charge")} " (eV)" }
                span { class: "text-xl leading-none", "𝑒" } p { "1.602 176 634 × 10⁻¹⁹ C (J)" }

                p { {tr!(lang, "speed of light in vacuum")} }
                span { class: "text-xl leading-none", "𝑐" } p { "299 792 458 m/s" }

                p { class: "col-span-3", "STP: 𝑇 = 273.15 K (0 ℃), 𝑝 = 101.325 kPa" }
            }
        }
} }

/* fn AufbauPrincipal() -> Element {
    // XXX: embedded for translation, expected to work on Dioxus v0.6
    let lang = use_context::<Signal<Localization>>();
    rsx! { svg { width: "500", height: "300", xmlns: "http://www.w3.org/2000/svg",
        "font-size": "small", "font-family": "sans", //title { {title} }

        text { transform: "translate(16,16)",
            tspan { x: "0", y:  "32", "1 K" tspan { dy: "-6", "font-size": "10", "2"  } }
            tspan { x: "0", y:  "64", "2 L" tspan { dy: "-6", "font-size": "10", "8"  } }
            tspan { x: "0", y:  "96", "3 M" tspan { dy: "-6", "font-size": "10", "18" } }
            tspan { x: "0", y: "128", "4 N" tspan { dy: "-6", "font-size": "10", "32" } }
            tspan { x: "0", y: "160", "5 O" tspan { dy: "-6", "font-size": "10", "32" } }
            tspan { x: "0", y: "192", "6 P" tspan { dy: "-6", "font-size": "10", "18" } }
            tspan { x: "0", y: "224", "7 Q" tspan { dy: "-6", "font-size": "10", "8"  } }
        }
        text { transform: "translate(22,274) rotate(-90)", fill: "blue",
            "letter-spacing": "-1", "n ="
        }
        path { transform: "translate(16,27)", stroke: "gray", "stroke-width": "0.2",
            d: "M0,32 h152 M0,64 h216 M0,96 h280 M0,128h280 M0,160h280 M0,192h216 M0,224h152"
        }
        text { transform: "translate(2,18)", "letter-spacing": "-1",
            tspan { x:  "64", "ℓ = 0" }
            tspan { x: "128", "ℓ = 1" }
            tspan { x: "192", "ℓ = 2" }
            tspan { x: "256", "ℓ = 3" }
            tspan { x: "320", opacity: "0.2", "ℓ = 4" }
            tspan { x: "384", opacity: "0.2", "ℓ = 5" }
            tspan { x: "448", opacity: "0.2", "ℓ = 6" }
        }
        text { x: "92", y: "296", {tr!(lang, "Aufbau Principle")}
            " (" {tr!(lang, "Madelung rule")} ")" }
        text { x: "180", y: "52", fill: "blue", "font-size": "8",
            tspan { "n: " {tr!(lang, "Principle quantum number")} }
            tspan { x: "180", dy: "16", "ℓ: " {tr!(lang, "Azimuthal quantum number")} }
        }

        defs {
            ellipse { rx: "16", ry: "12", stroke: "black", "stroke-width": "0.5", id: "orbit" }
            marker { "refX": "8", "refY": "8", "viewBox": "0 0 16 16", fill: "red", id: "arrow",
                "markerWidth": "10", "markerHeight": "10", orient: "auto-start-reverse",
                path { d: "M0,2 L16,8 L0,14 L6,8 z" } // A marker to be used as an arrowhead
            }
            path { transform: "translate(48,51)  rotate(-26.5)", d: "M8,8 h48",  id: "orbit_s" }
            path { transform: "translate(48,115) rotate(-26.5)", d: "M8,8 h120", id: "orbit_p" }
            path { transform: "translate(48,179) rotate(-26.5)", d: "M8,8 h192", id: "orbit_d" }
            path { transform: "translate(48,243) rotate(-26.5)", d: "M8,8 h264", id: "orbit_f" }
        }
        g { "marker-start": "url(#arrow)", stroke: "red",
            r#use { href: "#orbit_s" } r#use { href: "#orbit_s", y: "32" }
            r#use { href: "#orbit_p" } r#use { href: "#orbit_p", y: "32" }
            r#use { href: "#orbit_d" } r#use { href: "#orbit_d", y: "32" }
            r#use { href: "#orbit_f" } //r#use { href: "#orbit_f", y: "32" }
            r#use { href: "#orbit_d", y: "64", x: "64" }
        }
        g { transform: "translate( 80,11)", fill: "pink",
            r#use { href: "#orbit", y:  "32" }
            r#use { href: "#orbit", y:  "64" }
            r#use { href: "#orbit", y:  "96" }
            r#use { href: "#orbit", y: "128" }
            r#use { href: "#orbit", y: "160" }
            r#use { href: "#orbit", y: "192" }
            r#use { href: "#orbit", y: "224" }
            r#use { href: "#orbit", y: "256", opacity: "0.2" }
            g { transform: "translate(-8,5)", fill: "black",
                text { y:  "32", "1s" }
                text { y:  "64", "2s" }
                text { y:  "96", "3s" }
                text { y: "128", "4s" }
                text { y: "160", "5s" }
                text { y: "192", "6s" }
                text { y: "224", "7s" }
                text { y: "256", "s" tspan { dy: "-6", "font-size": "10", "2" } }
            }
        }
        g { transform: "translate(144,11)", fill: "lightgreen",
            r#use { href: "#orbit", y:  "64" }
            r#use { href: "#orbit", y:  "96" }
            r#use { href: "#orbit", y: "128" }
            r#use { href: "#orbit", y: "160" }
            r#use { href: "#orbit", y: "192" }
            r#use { href: "#orbit", y: "224" }
            g { transform: "translate(-8,5)", fill: "black",
                text { y:  "64", "2p" }
                text { y:  "96", "3p" }
                text { y: "128", "4p" }
                text { y: "160", "5p" }
                text { y: "192", "6p" }
                text { y: "224", "7p" }
                text { y: "256", "p" tspan { dy: "-6", "font-size": "10", "6" } }
            }
        }
        g { transform: "translate(208,11)", fill: "lightblue",
            r#use { href: "#orbit", y:  "96" }
            r#use { href: "#orbit", y: "128" }
            r#use { href: "#orbit", y: "160" }
            r#use { href: "#orbit", y: "192" }
            r#use { href: "#orbit", y: "224", opacity: "0.2", }
            g { transform: "translate(-8,5)", fill: "black",
                text { y:  "96", "3d" }
                text { y: "128", "4d" }
                text { y: "160", "5d" }
                text { y: "192", "6d" }
                text { y: "224", opacity: "0.2", "7d" }
                text { y: "256", "d" tspan { dy: "-6", "font-size": "10", "10" } }
            }
        }
        g { transform: "translate(272,11)", fill: "orange",
            r#use { href: "#orbit", y: "128" }
            r#use { href: "#orbit", y: "160" }
            r#use { href: "#orbit", y: "192", opacity: "0.2", }
            r#use { href: "#orbit", y: "224", opacity: "0.2" }
            g { transform: "translate(-8,5)", fill: "black",
                text { y: "128", "4f" }
                text { y: "160", "5f" }
                text { y: "192", opacity: "0.2", "6f" }
                text { y: "224", opacity: "0.2", "7f" }
                text { y: "256", "f" tspan { dy: "-6", "font-size": "10", "14" } }
            }
        }
        g { transform: "translate(336,11)", fill: "gray", opacity: "0.2",
            r#use { href: "#orbit", "y": "160" }
            r#use { href: "#orbit", "y": "192" }
            r#use { href: "#orbit", "y": "224" }
            g { transform: "translate(-8,5)", fill: "black",
                text { y: "160", "5g" }
                text { y: "192", "6g" }
                text { y: "224", "7g" }
                text { y: "256", "g" tspan { dy: "-6", "font-size": "10", "18" } }
            }
        }
        g { transform: "translate(400,11)", fill: "gray", opacity: "0.2",
            r#use { href: "#orbit", y: "192" }
            r#use { href: "#orbit", y: "224" }
            g { transform: "translate(-8,5)", fill: "black",
                text { y: "192", "6h" }
                text { y: "224", "7h" }
                text { y: "256", "h" tspan { dy: "-6", "font-size": "10", "22" } }
            }
        }
        g { transform: "translate(464,11)", fill: "gray", opacity: "0.2",
            r#use { href: "#orbit", y: "224" }
            g { transform: "translate(-8,5)", fill: "black",
                text { y: "224", "7i" }
                text { y: "256", "i" tspan { dy: "-6", "font-size": "10", "26" } }
            }
        }
    } }
} */

#[allow(unused)] fn ElemDetail() -> Element { rsx! { // TODO: https://pt.ziziyi.com/
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

