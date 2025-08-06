
#![allow(non_snake_case)]
use dioxus::prelude::*;

fn main() {
    //dioxus::logger::initialize_default(); //tracing::info!("initialized");
    #[cfg(not(feature = "desktop"))]  dioxus::launch(App);

    #[cfg(feature = "desktop")] { use dioxus::desktop::{self, WindowBuilder, LogicalSize};
    LaunchBuilder::desktop().with_cfg(desktop::Config::new()
        .with_window(WindowBuilder::new().with_title(env!("CARGO_PKG_NAME"))
            .with_inner_size(LogicalSize::new(1448, 996)))
        .with_custom_index(include_str!("../index.html").replace(r"{base_path}", "."))
        //.with_icon("assets/ptable.svg") // XXX: desktop::tao::window::Icon::from_rgba(...)?
    ).launch(App) };
}

fn App() -> Element {
let media_print = r"@media print {
  @page { size: A4 landscape; margin: 0; }  /* 设置页面的方向为横向并清除默认页边距 */
  body { width: 297mm; margin: auto; }      /* 设置主体样式以适应横向打印 */
  /* .non-printable { display: none; }         隐藏非打印元素 */
  select { appearance: none; background: transparent; }
}";

let dom_repair = r#"
  document.getElementById('spinner').style.display = 'none';

  const isChrome = /Chrome/.test(navigator.userAgent) && /Google Inc/.test(navigator.vendor);
  if (isChrome) { document.body.style = "zoom: 0.5; width: fit-content"; } else {
  //if (/^((?!chrome|android).)*safari/i.test(navigator.userAgent))
    document.body.style = //document.documentElement.style = // <html>
      "transform: scale(0.5); transform-origin: 0px 0px; width: 100vw; height: 100vh;";

    const style = document.createElement('style');
    style.innerHTML = `
select { appearance: none; background: transparent; }
select:not([size]) { /* https://flowbite.com/docs/forms/select/ */
  background-image: url("data:image/svg+xml;charset=utf-8,%3Csvg aria-hidden='true' xmlns='http://www.w3.org/2000/svg' fill='none' viewBox='0 0 10 6'%3E%3Cpath stroke='%236B7280' stroke-linecap='round' stroke-linejoin='round' stroke-width='1' d='m1 1 4 4 4-4'/%3E%3C/svg%3E");
  background-position: right .75rem center;
  background-repeat: no-repeat;
  background-size: .75em .75em;
  padding-right: 2.25rem;
  -webkit-print-color-adjust: exact;
  print-color-adjust: exact;
}`;
    document.head.appendChild(style);
  }
"#;

    use dioxus::document::{Link, Style/*, Title, Script, Meta*/};
    rsx! {  //Router::<Route> {}
        //Title { "Elements Periodic Table" } // ahead put in index.html and Dioxus.toml
        // XXX: asset!("...") does not work for desktop when web.app.base_path is set
        //Link { rel: "icon",       href: "assets/ptable.svg" }
        Link { rel: "stylesheet", href: "assets/tailwind.css" }
        //Stylesheet { href: asset!("assets/tailwind.css") }
        //Script { src: "https://unpkg.com/@tailwindcss/browser@4" }
        //Script { src: "https://cdn.tailwindcss.com" }
        Style { {media_print} } script { {dom_repair} }
        PeriodicTable {}
    }
}

macro_rules! tr { ($lang:expr, $id:expr) => { $lang.read().translate($id) } }

fn PeriodicTable() -> Element {
    #[derive(Clone, Copy)] #[repr(u8)] enum DiagType {
        Aufbau = 0, Binding, Bubble, Solar, Earth, /* Human, */ SModel, }
    let mut diag = use_signal(|| (DiagType::Aufbau, 0x01u8));
    fn diag_is_loaded(v: (DiagType, u8)) -> bool {   (0x01 << v.0 as u8) & v.1 != 0x00 }
    let load_diag = move |_|
        diag.with_mut(|v| v.1 |= 0x01 << v.0 as u8);

    use_context_provider(|| Signal::new(Coloring::Class));
    use_context_provider(|| Signal::new(Localization::new()));
    let mut coloring = use_context::<Signal<Coloring>>();
    let mut lang = use_context::<Signal<Localization>>();
    //use_context_provider(|| Signal::new(Selection { r#type: SelType::None, val: 0, }));
    //let mut group_sel = use_context::<Signal<Selection>>();   // move to ahead of rsx!

    let bg_lan = COLORING_CLASSES[ElemClass::Lanthanoids as usize].0;
    let bg_act = COLORING_CLASSES[ElemClass::Actinoids   as usize].0;
    let style_blk = "self-start text-center text-lg/6 rounded-xs";
    let style_grp = "self-end   text-center text-lg font-bold";
    let style_sel = "absolute top-2 text-2xl text-center
        print:hidden focus:outline-hidden text-red-600";
    let wm_vert = "writing-mode: vertical-lr;";

    rsx! { div { class: "grid grid-cols-[auto_repeat(18,1fr)_auto]
        grid-rows-[auto_repeat(7,1fr)_auto_1fr_1fr_auto] w-[181rem] p-6 gap-0.5",
        // box: [1448, 998] * 2 // scale-50 origin-top-left h-[128rem]
        //style: "transform: scale(0.5); transform-origin: 0 0;", // use js script in index.html
        //style: "zoom: 0.5;",  // malformed in Safari, which works well scaling on <html>

        p { class: "relative top-3 text-lg font-bold leading-none h-16",
            style: wm_vert,   {tr!(lang, "PERIOD")}
        }
        p { class: style_grp, {tr!(lang, "GROUP")} br{} "IA - 1" }
        p { class: "relative col-span-16 text-center",
            select { class: "{style_sel} left-0",  name: "lang-sel", //id: "lang-sel",
                onchange: move |evt| lang.write().set(evt.value()),
                option { value: "en-US", "en" } option { value: "zh-CN", "中文" }
            }
            a { class: "font-bold text-5xl", href: "https://github.com/mhfan/inperiod",
                {tr!(lang, "Periodic Table of the Elements")} // env!("CARGO_PKG_REPOSITORY")
            }
            select { class: "{style_sel} right-0", name: "diag-sel", //id: "diag-sel",
                onchange: move |evt| diag.with_mut(|v| v.0 = unsafe {
                    std::mem::transmute::<u8, DiagType>(evt.value().parse::<u8>().unwrap()) }),
                option { value: DiagType::Aufbau  as u8, {tr!(lang,  "Aufbau")} }
                option { value: DiagType::Binding as u8, {tr!(lang, "Binding")} }
                optgroup { label: tr!(lang, "Abundances"),
                    option { value: DiagType::Bubble as u8, {tr!(lang, "Bubble")} }
                    option { value: DiagType::Solar  as u8, {tr!(lang,  "Solar")} }
                    option { value: DiagType::Earth  as u8, {tr!(lang,  "Earth")} }
                    option { value: DiagType::Earth  as u8, {tr!(lang,  "Human")} }
                }
                option { value: DiagType::SModel  as u8, {tr!(lang,  "SModel")} }
            }
        }
        p { class: style_grp, "VIIIA - 18" }
        p { class: "relative -bottom-4 font-bold leading-none ml-2",
            style: wm_vert, {tr!(lang, "E-max")} br{} {tr!(lang, "E-shell")}
        }
        div { class: "grid row-span-7 px-1 items-center text-xl font-bold", // divide-y
            /* onmouseout:  move |_| group_sel.write().r#type = SelType::None,
            onmouseover: move |evt| {
                use {dioxus::web::WebEventExt, wasm_bindgen::JsCast};
                let ep = evt.as_web_event().target()
                    .and_then(|e| e.dyn_ref::<web_sys::HtmlElement>()).unwrap();
                let val = ep.inner_text().parse::<u8>();
                *group_sel.write() = Selection { r#type: SelType::Period, val };
            }, */  for i in 1..=7 { p { "{i}" } } // class: "content-center",
        }

        ElemTile { ordinal: 1 } p { class: style_grp, "IIA - 2" }
        div { class: "empty col-span-2" }   // XXX: can be moved one cell left on needed
        ElemTile { ordinal: 43, annot: Some(true) } div { class: "empty col-span-2" }
        div { class: "relative col-span-5", div { class: "absolute h-[150%] w-full flex",
            if !diag_is_loaded(*diag.read()) { div {
                class: "absolute self-center w-full flex justify-center", LoadSpinner {}
            } } {match diag.read().0 {  DiagType::Aufbau => rsx! { AufbauPrinciple {} },
                //DiagType::Aufbau => rsx! { img { src: "assets/Aufbau.svg" } },
                // https://www.nagwa.com/en/explainers/809139094679/

            // https://commons.wikimedia.org/wiki/File:Binding_energy_curve_-_common_isotopes.svg
                DiagType::Binding => rsx! { img { src: "assets/Binding_energy_isotopes.svg",
                    class: "h-[108%] relative -top-[4%]", onload: load_diag,
                } },
                // https://commons.wikimedia.org/wiki/File:Isotopic_Abundance_bubble_chart.png
                DiagType::Bubble => rsx! { img { src: "assets/Isotopic_Abundance_bubble.png",
                        class: "h-[130%] relative -top-[15%] -left-[10%]", onload: load_diag,
                    }
                    if diag_is_loaded(*diag.read_unchecked()) { div {
                        class: "self-center relative -left-8",
                        b { {tr!(lang, "The most abundant isotopes")} } ":"
                        ul { class: "list-disc",
                            li { {tr!(lang, "Relative abundance is proportional to the area.")} }
                            li { {tr!(lang, "Isotopes with equal numbers of protons and neutrons are unusually abundant.")} }
                            li { b { "¹H " } {tr!(lang, "(large blue circle) comprises 74% of the ordinary matter of the universe.")} }
                            li { a { href: "https://en.wikipedia.org/wiki/Nucleosynthesis",
                                {tr!(lang, "Color corresponds to nucleosynthetic process")} } ":"
                                div { class: "grid grid-cols-[max-content_auto] gap-1",
style { ".shadow-t {{
text-shadow: -1px -1px 0 black, 1px -1px 0 black, -1px 1px 0 black, 1px 1px 0 black;
}}" }
                                    b { class: "shadow-t text-yellow-300", {tr!(lang, "Yellow")} }
                                    span { "- " {tr!(lang, "Exploding massive stars")} }
                                    b { class: "shadow-t text-green-500",  {tr!(lang, "Green")} }
                                    span { "- " {tr!(lang, "Dying low-mass stars")} }
                                    b { class: "shadow-t text-blue-400",   {tr!(lang, "Blue")}  }
                                    span { "- " {tr!(lang, "Big Bang fusion")} }
                                }
                            }
                        }
                    } }
                },
                // https://commons.wikimedia.org/wiki/File:SolarSystemAbundances.svg
                DiagType::Solar => rsx! { img { src: "assets/SolarSystemAbundances.png",
                    class: "h-[118%] relative -top-[8%] -left-[10%]", onload: load_diag,
                } },
                // https://commons.wikimedia.org/wiki/File:Element_abundance_earth_ppm_chart.svg
                DiagType::Earth => rsx! {
                    img { src: "assets/Abundance_earth.svg", onload: load_diag }
                    img { src: "assets/Abundance_human.svg", class: "ml-16", }
            // https://commons.wikimedia.org/wiki/File:Element_abundance_human_body_ppm_chart.svg
                },
                DiagType::SModel => rsx! { img {    // https://en.wikipedia.org/wiki/Standard_Model
                    src: format!("assets/Standard_Model_{}.svg", lang.read().get()),
                    class: "h-[200%] relative -top-[4%]", onload: load_diag,
                } }, //_ => unreachable!(),
            } }
        } }
        div { class: "flex col-span-5",     // XXX: more information can be put here
            div { class: "grid grid-cols-5 {style_grp} w-full",
                for i in 13..=17 { p { {format!("{}A - {i}", ROMAN_NUM[i - 10])} } }
            }
        }

        ElemTile { ordinal: 2 }
        div { class: "row-span-7 grid grid-rows-[repeat(7,1fr)] ml-1 *:content-center
                font-mono text-nowrap text-right tracking-tighter divide-y",
            p { "2 K" }
            p { "8 L" br{}  "2 K" }
            p { "8 M" br{}  "8 L" br{}  "2 K" }
            p { "8 N" br{} "18 M" br{}  "8 L" br{}  "2 K" }
            p { "8 O" br{} "18 N" br{} "18 M" br{}  "8 L" br{}  "2 K" }
            p { "8 P" br{} "18 O" br{} "32 N" br{} "18 M" br{}  "8 L" br{} "2 K" }
            p { "8 Q" br{} "18 P" br{} "32 O" br{} "32 N" br{} "18 M" br{} "8 L" br{} "2 K" }
        }

        ElemTile { ordinal: 3 } ElemTile { ordinal: 4 }
        div { class: "relative flex flex-col col-span-10 row-span-2",
            div { class: "flex mx-auto mt-auto mb-2 text-lg/6",
                div { class: "flex flex-col",
                    b { class: "self-start mb-1 px-1 border-b border-black",
                        {tr!(lang, "Phase at STP")}
                    }
                    b { span { class: "text-liquid", {tr!(lang, "Liquid")} }
                        span { class: "mx-4 text-gas", {tr!(lang, "Gas")} }
                        span { {tr!(lang, "Solid")} }
                        span { class: "ml-4 text-synthetic", {tr!(lang, "Synthetic")} }
                    }
                    select { class: "mt-4 mb-1 self-start text-xl font-bold focus:outline-hidden",
                        onchange: move |evt| *coloring.write() = match evt.value().as_str() {
                            "0" => Coloring::Class, "1" => Coloring::Origin,
                            "2" => Coloring::Flame, _ => unreachable!(),
                        }, name: "coloring-sel",
                        option { value: "0", {tr!(lang, "Categories")} }
                        option { value: "1", {tr!(lang, "Cosmic origin")} }
                        option { value: "2", {tr!(lang, "Flame test")} }
                    }
                    div { class: "grid grid-cols-2 grid-rows-5 text-center w-[23rem] h-[15rem]",
                        {use ElemClass::*; match *coloring.read() {
                            Coloring::Class  => rsx! { for (item, cls) in [AlkaliMetals,
                                NobleGases, AlkalineEarthMetals, Halogens, TransitionMetals,
                                OtherNonmetals, PoorMetals, Metalloids, Lanthanoids, Actinoids]
                                .map(|x| (COLORING_CLASSES[x as usize], x)) { p {
                                    class: format!("content-center rounded-xs {}", item.0),
                                    {tr!(lang, item.1)} if matches!(cls, Lanthanoids) { "*" }
                                } }
                            },
                            Coloring::Origin => rsx! { for item in COLORING_ORIGINS { p {
                                style: format!("background-color: {};", item.0),
                                class: "content-center rounded-xs px-2", {tr!(lang, item.1)}
                            } } },
                            Coloring::Flame  => rsx! { img { src: "assets/flame-test.jpg",
                                class: "rounded-xs row-span-full col-span-full h-full",
                            } }
                        } }
                    }
                }
                div { visibility: if matches!(diag.read().0, DiagType::SModel) { "hidden"
                } else { "visible" }, class: "self-end ml-8", PhysConsts {} }
            }
            p { class: "absolute right-0 mt-1 text-nowrap", style: wm_vert,
                {tr!(lang, "metal - nonmetal divider")}
            }
            div { class: "grid grid-cols-10 {style_grp} w-full",
                for i in 3..=7 { p { {format!("{}B - {i}", ROMAN_NUM[i])} } }
                /* for i in 8..=10 { p {
                    class: "shadow-[0_2px] shadow-indigo-300", "VIIIB - {i}" }
                } } */     // border-b-2
                p { class: "col-span-3 shadow-[0_2px] shadow-indigo-300", "VIIIB - 8|9|10" }
                p { "IB - 11" }  p { "IIB - 12" }
            }
        }   for i in   5..= 56 { ElemTile { ordinal: i } }

        div { class: "{STYLE_TILE} text-2xl {bg_lan}", span { class: "self-end font-bold", "71" }
            p { class: "text-center m-auto", b { "57 ~ 70" } br{} {tr!(lang, "Lanthanoids")} }
        }   for i in  72..= 88 { ElemTile { ordinal: i } }
        div { class: "{STYLE_TILE} text-2xl {bg_act}", span { class: "self-end font-bold", "103" }
            p { class: "text-center m-auto", b { "89 ~ 102" } br{} {tr!(lang, "Actinoids")} }
        }   for i in 104..=118 { ElemTile { ordinal: i } }

        div { class: "empty row-span-4" }
        div { class: "{style_blk} col-span-2  bg-red-100",
            b { "s" } {tr!(lang, "-block")} " (" {tr!(lang, "plus")} b { " He" } ")"
        }
        div { class: "{style_blk} col-span-10 bg-blue-100 border-x mb-6",
            b { "d" } {tr!(lang, "-block")} " (" {tr!(lang, "exclude")}
            i { " Lan/Act " } {tr!(lang, "series")} ")"
        }
        div { class: "{style_blk} col-span-6  bg-green-100",
            b { "p" } {tr!(lang, "-block")} " (" {tr!(lang, "exclude")} i { " He" } ")"
        }   div { class: "empty" }

        div { class: "flex flex-col col-span-3 row-span-2 mr-2",
            p { class: "text-lg font-bold leading-none pb-1", {tr!(lang, "Notes")} ":" }
            ul { class: "list-disc",
                li { {tr!(lang, "Standard atomic mass")} " (A" sub { "r" } "°, "
                    i { {tr!(lang, "in Dalton or")} } " "
                    span { class: "text-xl leading-none", "𝓂" sub { "μ" } } "): "
                        {tr!(lang, "the weighted arithmetic mean of the relative isotopic masses of all isotopes of an element, weighted by their abundance on Earth")}
                }
                li { em { "[ ] " } {tr!(lang, "indicate mass number of most stable isotope")} }
                li { {tr!(lang, "Density units")} i { ": g/cm3 " }
                     {tr!(lang, "for solids and")} i { " g/L " }
                     {tr!(lang, "for liquid or")} br{} i { " kg/m3 " }
                     {tr!(lang, "for gases at 0° Celsius")}
                }
                li { em { "* " }
                    {tr!(lang, "mark means the electronegativity is in the bottom-right")}
                }
                li { {tr!(lang, "Here ")} {tr!(lang, "atomic radius")} {tr!(lang, " is ")}
                    i { {tr!(lang, "van der Waals radii")} }
                }
                li { {tr!(lang, "Rare earth metals")} {tr!(lang, " include")} ": "
                    i { {tr!(lang, "Lanthanoids")} " (La ~ Lu), Sc " {tr!(lang, "and")} " Y" }
                }
                //li { i { "§" } " indicates crystal structure is unusual" }
                // or may require explanation
            }
            p { class: "text-lg font-bold mt-auto", {tr!(lang, "References")} ":" }
            p { a { href: "https://iupac.org/what-we-do/periodic-table-of-elements/", "IUPAC" } ", "
                a { href: "https://www.nist.gov/pml/periodic-table-elements", "Nist.gov" } ", "
                a { href: "https://pubchem.ncbi.nlm.nih.gov/periodic-table/", "PubChem" } ", "
                a { href: "https://www.webelements.com/periodicity/contents/", "WebElements" } ", "
                a { href: "https://en.wikipedia.org/wiki/Periodic_table", "Wikipedia" } ", " br{}
                a { href: "https://www.vertex42.com/ExcelTemplates/periodic-table-of-elements.html",
                    "Vertex42" } ", "
                a { href: "https://github.com/lmmentel/mendeleev", "mendeleev" } ", etc."
            }
        }

        for i in 57..= 71 { ElemTile { ordinal: i } }
        p { class: "text-center text-lg font-bold", style: wm_vert, {tr!(lang, "Lanthanides")} }
        for i in 89..=103 { ElemTile { ordinal: i } }
        p { class: "text-center text-lg font-bold", style: wm_vert, {tr!(lang, "Actinides")} }
        p { class: "col-span-3 mt-2", {tr!(lang, "All rights reserved.")} " © 2024 "
            a { href: "https://github.com/mhfan", "M.H.Fan" } //" v" {env!("CARGO_PKG_VERSION")}
        }
        div { class: "{style_blk} col-span-14 bg-yellow-100",
            b { "f" } {tr!(lang, "-block")}
        }
        div { class: "{style_blk} bg-blue-100 border-l", p { class: "invisible", "d-block" } }
        div { class: "empty" }
    } }
}

use inperiod::{ChemElem, ElemClass, CosmicOrigin, l10n::Localization, UNICODE_SUPERS, ROMAN_NUM};
//#[derive(Clone, Copy)] enum SelType { None, Period, Group, Block, Class, }
//#[derive(Clone, Copy)] struct Selection { r#type: SelType, val: u8, }
enum Coloring { Class, Origin, Flame, }

static COLORING_CLASSES: [(&str, &str);    ElemClass::MAX as usize] = [ // strict aligned order
    ("bg-rose-200",     "Alkali metals"),
    ("bg-pink-100",     "Alkaline earth metals"),
    ("bg-slate-200",    "Transition metals"),
    ("bg-stone-300",    "Poor metals"),
    ("bg-cyan-200",     "Metalloids"),
    ("bg-lime-200",     "Other nonmetals"),
    ("bg-fuchsia-200",  "Halogens"),
    ("bg-violet-200",   "Noble gases"),
    ("bg-amber-100",    "Rare earth metals"), // "Lanthanoids"
    ("bg-orange-100",   "Actinoids"),
    ("bg-zinc-50",      "Unknown"),
];

static COLORING_ORIGINS: [(&str, &str); CosmicOrigin::MAX as usize] = [ // strict aligned order
    ("#e2e8f0", "Big Bang fusion"),
    ("#d6d3d1", "Cosmic ray fission"),
    ("#a5f3fc", "Dying low-mass stars"),
    ("#d9f99d", "Merging neutron stars"),
    ("#ddd6fe", "Exploding massive stars"),
    ("#fecdd3", "White dwarf supernovae"),
    ("#fce7f3", "Radioactive decay"),
    ("#fafafa", "No stable isotopes"),
];

static STYLE_TILE: &str = "flex flex-col rounded-xs p-1 border border-indigo-300";

#[component] fn ElemTile(ordinal: u8, annot: Option<bool>) -> Element {
    let elem = ChemElem::from(ordinal);
    let (name, (os_main, os_all)) = (elem.name(), elem.oxidation_states());
    #[allow(clippy::upper_case_acronyms)] enum Tooltips { None, ECFG, CS, OStates }
    let lang = use_context::<Signal<Localization>>();
    let mut tips = use_signal(|| Tooltips::None);

    let coloring = use_context::<Signal<Coloring>>();
    let (bg_color, bg_style) = match *coloring.read() {
        Coloring::Class  => (COLORING_CLASSES[elem.category() as usize].0, "".to_string()),
        Coloring::Origin => ("", format!("background: conic-gradient({});", {  let mut sum = 0;
            elem.cosmic_origin().iter().map(|&(co, ratio)| { let start = sum; sum += ratio;
                format!("{} {start}% {sum}%", COLORING_ORIGINS[co as usize].0)
            }).collect::<Vec<_>>().join(", ")
        })),
        Coloring::Flame  => ("", if let Some(color) = &elem.flame_color() {
            format!("background-color: #{:06x}80;", color.0)
        } else { "".to_string() }),
    };

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

    let tips_cmn = "absolute rounded-xs border border-orange-600 bg-white z-10";
    let metal_bound  = match ordinal {  // hidden peer-hover:block // group-hover::block
        1 => "shadow-black-b", 118 => "shadow-black-l", 4 => "shadow-[0_-2px_black]",
        21|39|71 => "border-rare",      // Rare earth metals indication
        2 => "shadow-[0_2px_#fca5a5]",  // indicate He is of s-block, shadow-red-300
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

    rsx! { div {    //shadow-border-1 shadow-indigo-300     // box: [152, 198]
        class: "relative {STYLE_TILE} hover:shadow-orange-600 hover:shadow-spread-2
            {bg_color} {metal_bound}", style: bg_style, if annot.is_some_and(|bl| bl) {
            a { class: "absolute bottom-full font-bold text-lg/6 text-amber-600 self-center",
                href: "https://ciaaw.org/radioactive-elements.htm", {tr!(lang, "radioactive")}
            }
            div { class: "absolute right-full mr-2 text-lg leading-tight text-nowrap text-right",
                //onmouseenter: |evt| evt.stop_propagation(), // XXX: not work for :hover
                p { "*" a { href: "https://ciaaw.org/atomic-weights.htm",
                    {tr!(lang, "atomic weight")}
                } }
                a { href: "https://www.nist.gov/pml/periodic-table-elements",
                    {tr!(lang, "1st ionization energy")} " (eV)"
                }
                p { class: "mt-3 mb-5", {tr!(lang, "symbol")} } p { {tr!(lang, "name")} }
                p { b    { class: "text-blue-700", {tr!(lang, "melting")} } "/"
                    span { class: "text-red-700",  {tr!(lang, "boiling")} }
                    {tr!(lang, " point")} " (℃)"
                }
                p { "*" {tr!(lang, "density")} }
                a { href: "https://en.wikipedia.org/wiki/Electron_configuration",
                    {tr!(lang, "electron configuration")}
                }
            }
            div { class: "absolute  left-full ml-2 text-lg leading-tight text-nowrap",
                p { {tr!(lang, "atomic number")} }
                p { class: "mb-1", {tr!(lang, "electron affinity")} }
                a { href: "https://en.wikipedia.org/wiki/Oxidation_state",
                    {tr!(lang, "main oxidation states")}
                }
                p { class: "my-1", {tr!(lang, "Chinese name with pinyin")} }
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
                p { class: "flex text-lg/6 font-bold", {elem.atomic_weight().to_string()}
                    if elem.is_radioactive() { span { class: "mx-auto", "☢️" } }
                }
                p { class: "flex text-base/5",
                    {elem.ionization_energy().map_or_else(|| "-".to_string(),
                        |ie| format!("{:.3}", ie.0).trim_end_matches('0')
                            .trim_end_matches('.').to_owned())}
                    span { class: "pl-2 mx-auto",
                        {elem.electron_affinity().map_or_else(|| " ".to_string(),
                            |ea| ea.to_string())}
                    }
                }
            }
            span { class: "ml-1 text-2xl font-bold", "{ordinal}" }
        }
        div { class: "flex",
            span { class: "ml-1 grow self-center text-5xl {color_symbol}", {elem.symbol()} }
            div  { class: "flex flex-col",
                p { class: "text-center leading-tight", {elem.name_py()} }
                a { href: format!("https://zh.wikipedia.org/wiki/{}", elem.name_ch()),
                    class: "text-right text-3xl", {elem.name_ch().to_string()}
                }
            }
            div { class: "relative ml-2 text-right",
                onmouseout:  move |_| tips.set(Tooltips::None),
                onmouseover: move |_| tips.set(Tooltips::OStates),
                div { class: format!("absolute font-bold {}", // group
                        if 7 < os_main.len() { "leading-none" } else { "leading-tight" }),
                    for os in  os_main.iter().rev() { pre {
                    {format!("{}{os}", match *os { x if 0 < x => "+", 0 => " ", _ => "" })}
                    } }
                    if matches!(*tips.read(), Tooltips::OStates) &&
                        os_main.len() < os_all.len() { div {    // group-hover::block
                        class: "{tips_cmn} left-full -top-4 ml-1.5 p-2 text-lg/5 font-normal",
                        for os in os_all.iter().rev() { pre { class:
                            if os_main.contains(os) { "font-extrabold" } else { "" },
                    {format!("{}{os}", match *os { x if 0 < x => "+", 0 => " ", _ => "" })}
                        } }
                    } }
                }   pre { class: "invisible", "  " }
            }
        }
        p { a { href: "https://en.wikipedia.org/wiki/{name}", class: "text-lg/6", {name} }
            span { class: "ml-2 font-bold", {match ordinal { 42|59 => "*".to_string(),
                _ => elem.en_pauling().map_or_else(|| "".to_string(), |en| en.to_string())
            } } }   // name is too long, move to bottom-right
        }
        p { class: "relative text-base/5",
            span { class: "text-blue-700 font-bold",
                {elem.melting_point().map_or_else(|| "-".to_string(),
                    |mp| format!("{}", (mp - 273.15 + 0.5) as i32))}
            } "/"
            span { class: "text-red-700",
                {elem.boiling_point().map_or_else(|| "-".to_string(),
                    |bp| format!("{}", (bp - 273.15 + 0.5) as i32))}
            }
            {elem.crystal_structure().map_or_else(
                || rsx! { span { class: "ml-2", "-" } }, |(cs, file)| rsx! {
                span { class: "px-2", onmouseout: move |_| tips.set(Tooltips::None),
                    onmouseover: move |_| tips.set(Tooltips::CS), {cs}
                }
                if matches!(*tips.read(), Tooltips::CS) { figure {
                    class: "{tips_cmn} w-[20rem] mx-1.5", // peer-hover:block
                    style: {match ordinal {
                        2 =>    "right: 100%; top: 0;",
                        _ => if ordinal == 71  || ordinal == 103 ||
                            matches!(elem.group(), 17..=19) { "right: 100%; bottom: 0;"
                        } else { "left: 100%; top: 0;" }
                    } },
                    figcaption { class: "text-center text-lg text-blue-600 font-bold",
                        {tr!(lang, file.replace(['-', '_'], " ").as_str())}
                    }   img { class: "w-full", src: "assets/crystal-s/{file}.svg" }
                } }
            })}
        }
        p { class: "text-base/5 *:ml-2",
            {elem.density().map_or_else(|| "-".to_string(), |den| format!("{den:.4}")
                .trim_end_matches('0').trim_end_matches('.').to_string())}
            span { class: "font-bold text-purple-700",
                {elem.atomic_radius().map_or_else(|| "-".to_string(), |cr| cr.to_string())}
            }
            span { class: "font-bold", {elem.ground_state().map_or_else(
                    || rsx! { "-" }, |(s1, s2, s3)| rsx! { if 1 < s2.len() {
                        sup  { {s1} } {s2.chars().next().unwrap().to_string()}
                        span { class: "relative -top-1", "°" }
                        sub  { class: "-left-[0.5em]", {s3} } // use relative 'em' here
                    } else { sup { {s1} } {s2}   sub { {s3} } } })
            } }
        }
        p { class: "flex",
            span { class: "text-nowrap font-bold text-lg/6", // peer
                onmouseout:  move |_| tips.set(Tooltips::None),
                onmouseover: move |_| tips.set(Tooltips::ECFG), {revised_ecfg}
            }
            if matches!(*tips.read(), Tooltips::ECFG) { figure {
                class: "{tips_cmn} w-[40rem] mx-0.5", // peer-hover:block
                style: {match  ordinal {
                    2|7..=10 => "right: 100%;    top: -0.2rem;",
                    57|89    =>  "left: 100%; bottom: -0.2rem;",
                    _ => if ordinal == 71  || ordinal == 103 || matches!(elem.group(), 15..=19) {
                                "right: 100%; bottom: -0.2rem;"
                    } else {     "left: 100%;    top: -0.2rem;" }
                } }, ShowEcfg { ordinal }
            } }

            if matches!(ordinal, 42|59) { span { class: "ml-auto font-bold",
                {elem.en_pauling().map(|en| en.to_string())}
            } }
        }
    } }
}

/// https://physics.nist.gov/cuu/Constants/index.html
fn PhysConsts() -> Element {
    let cmnts = "text-xl leading-none";
    let lang = use_context::<Signal<Localization>>();
    rsx! {
        p { class: "flex px-1",
            span { class: "font-bold", {tr!(lang, "Common physical constants")} }
            span { class: "ml-auto",   {tr!(lang, "Source: ")}
                a { href: "https://physics.nist.gov/constants", "physics.nist.gov" } " (2022)"
            }
        }
        div { class: "grid grid-cols-[repeat(2,auto)]
                border border-black divide-black divide-x",
            div { class: "grid grid-cols-[repeat(3,auto)] gap-x-3 px-2",
                p { {tr!(lang, "electron mass")} }
                span { class: "{cmnts} -top-2", "𝓂" sub { class: "text-lg leading-none", "𝑒" } }
                p { "9.109 383 7139(28) × 10⁻³¹ kg" }

                p { {tr!(lang, "atomic mass unit")} span { class: cmnts, " 𝓂" } "(¹²C)/12" }
                span { class: cmnts, "𝓂" sub { "μ" } }
                p { "1.660 539 068 92(52) × 10⁻²⁷ kg" }     // 𝜋ε

                p { {tr!(lang, "fine-structure const.")} span { class: cmnts, " 𝑒" } "²/4π𝜖₀ℏ𝑐" }
                span { class: cmnts, "𝛼" } p { "7.297 352 5643(11) × 10⁻³ (~1/137)" } // α

                p { {tr!(lang, "Newtonian const. of gravitation")} } span { "𝐺" }
                p { "6.674 30(15) × 10⁻¹¹ m³ kg⁻¹ s⁻²" }

                p { {tr!(lang, "Rydberg constant")} } span { "𝑅" sub { "∞" } }
                p { "10 973 731.568 157(12) [m⁻¹]" }

                p { {tr!(lang, "classical electron radius")}
                    span { class: "text-lg leading-none", " 𝛼²𝑎₀" }
                }
                span { class: "text-lg leading-none", "𝑟" sub { "𝑒" } }
                p { "2.817 940 3205(13) × 10⁻¹⁵ m" }

                p { {tr!(lang, "molar volume of ideal gas")} " 𝑅𝑇/𝑝" } span { "𝑉" sub { "m" } }
                p { "22.413 969 54... × 10⁻³ m³/mol" }

                p { {tr!(lang, "first radiation constant")} " 2π" span { class: cmnts, "ℎ" } "𝑐²" }
                span { i { class: "text-lg leading-none", "c" } "₁" }
                p { "3.741 771 852... × 10⁻¹⁶ [W m²]" }

                p { {tr!(lang, "second radiation constant")} span { class: cmnts, " ℎ" } "𝑐/𝑘" }
                span { i { class: "text-lg leading-none", "c" } "₂" }
                p { "1.438 776 877... × 10⁻² [m K]" }

                p { "¹³³Cs " {tr!(lang, "hyperfine transition freq.")} }
                span { "∆ν" sub { "Cs" } } p { "9 192 631 770 Hz" }
            }
            div { class: "grid grid-cols-[repeat(3,auto)] gap-x-3 px-2",
                p { {tr!(lang, "Avogadro constant")} } span { "𝑁" sub { "A" } }
                p { "6.022 140 76 × 10²³ mol⁻¹" }

                p { {tr!(lang, "Planck constant")} } span { class: "text-2xl leading-none", "ℎ" }
                p { "6.626 070 15 × 10⁻³⁴ J/Hz" }

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
                span { class: cmnts, "𝑒" } p { "1.602 176 634 × 10⁻¹⁹ C (J)" }

                p { {tr!(lang, "speed of light in vacuum")} }
                span { class: cmnts, "𝑐" } p { "299 792 458 m/s" }

                p { class: "col-span-3", "STP: 𝑇 = 273.15 K (0 ℃), 𝑝 = 101.325 kPa" }
            }
        }
    }
}

#[component] fn ShowEcfg(ordinal: u8) -> Element {
    let elem = ChemElem::from(ordinal);
    let lang = use_context::<Signal<Localization>>();
    let ecfg = elem.electron_configuration().expand();

    rsx! { svg { width: "640", height: "512", xmlns: "http://www.w3.org/2000/svg",
        "font-size": "small", "font-weight": "normal", //title { title }

        g { text { x: "200", y: "504", "font-weight": "bold",
                {tr!(lang, "Electron shell/orbital configuration")}
            }
            text { x: "560", y: "464", "font-size": "48", {elem.symbol()} }

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
            path { stroke: "green", d:
                "M88,44 h 88 m 0,64 h-88 m 0,64 h 88 m 0,64 h-88 m 0,64 h 88 m 0,64 h-88",
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

fn AufbauPrinciple() -> Element {
    let lang = use_context::<Signal<Localization>>();
    rsx! { svg { width: "500", height: "300", xmlns: "http://www.w3.org/2000/svg",
        "font-size": "small", "font-family": "sans", //title { {title} }

        text { x: "14", y: "20", fill: "blue", "font-size": "medium", "n" }
        text { transform: "translate(16,16)",
            tspan { x: "0", y:  "32", "1 K" tspan { dy: "-6", "font-size": "10", "2"  } }
            tspan { x: "0", y:  "64", "2 L" tspan { dy: "-6", "font-size": "10", "8"  } }
            tspan { x: "0", y:  "96", "3 M" tspan { dy: "-6", "font-size": "10", "18" } }
            tspan { x: "0", y: "128", "4 N" tspan { dy: "-6", "font-size": "10", "32" } }
            tspan { x: "0", y: "160", "5 O" tspan { dy: "-6", "font-size": "10", "32" } }
            tspan { x: "0", y: "192", "6 P" tspan { dy: "-6", "font-size": "10", "18" } }
            tspan { x: "0", y: "224", "7 Q" tspan { dy: "-6", "font-size": "10", "8"  } }
        }
        /* text { transform: "translate(22,274) rotate(-90)", fill: "blue",
            "letter-spacing": "-1", "n ="
        } */
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
        text { x: "92", y: "296", "font-weight": "bold",
            {tr!(lang, "Aufbau Principle")} " (" {tr!(lang, "Madelung rule")} ")"
        }   // https://www.nagwa.com/en/explainers/809139094679/
        text { x: "180", y: "52", fill: "blue", "font-size": "8",
            tspan { tspan { "font-size": "14", "n" } ": "
                {tr!(lang, "Principle quantum number")}
            }
            tspan { x: "180", dy: "16", tspan { "font-size": "14", "ℓ" } ": "
                {tr!(lang, "Azimuthal quantum number")}
            }
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
}

#[allow(unused)] fn LoadSpinner() -> Element { rsx! {
    svg { "viewBox": "0 0 100 100", xmlns: "http://www.w3.org/2000/svg",
        "aria-hidden": "true", fill: "none", "stroke-width": "5",
        width: "100", height: "100", //stroke: "blue", color: "gray",
        class: "text-gray-200 stroke-blue-600 animate-spin", // dark:text-gray-600
        circle { cx: "50", cy: "50", r: "45", stroke: "currentColor", }
        path { d: "M50,5a45,45 0 0 1 45,45", "stroke-linecap": "round", stroke: "currentStroke" }
    }
} }

#[allow(unused)] fn ElemDetail() -> Element { rsx! { // TODO: https://pt.ziziyi.com/
} }

/* for fullstack web renderer
#[derive(Debug, Clone, Routable, PartialEq)] #[rustfmt::skip] enum Route {
    #[layout(Navbar)] #[route("/")] Home {},
    #[route("/blog/:id")] Blog { id: i32 },
}

#[component] pub fn Hero() -> Element {
    rsx! { div { id: "hero",
        img { src: HEADER_SVG, id: "header" }
        div { id: "links",
            a { href: "https://dioxuslabs.com/learn/0.6/", "📚 Learn Dioxus" }
            a { href: "https://dioxuslabs.com/awesome", "🚀 Awesome Dioxus" }
            a { href: "https://github.com/dioxus-community/", "📡 Community Libraries" }
            a { href: "https://github.com/DioxusLabs/sdk", "⚙️ Dioxus Development Kit" }
            a { href: "https://marketplace.visualstudio.com/items?itemName=DioxusLabs.dioxus",
                "💫 VSCode Extension" }
            a { href: "https://discord.gg/XgGxMSkvUM", "👋 Community Discord" }
        }
    } }
}

#[component] fn Home() -> Element { rsx! { Hero {} Echo {} } }

#[component] pub fn Blog(id: i32) -> Element {
    rsx! { div { id: "blog",
        h1 { "This is blog #{id}!" } // Content
        p { "In blog #{id}, we show how the Dioxus router works and
            how URL parameters can be passed as props to our route components." }

        Link { to: Route::Blog { id: id - 1 }, "Previous" } span { " <---> " }
        Link { to: Route::Blog { id: id + 1 }, "Next" }     // Navigation links
    } }
}

/// Shared navbar component.
#[component] fn Navbar() -> Element { rsx! {
    div { id: "navbar",
        Link { to: Route::Home {}, "Home" }
        Link { to: Route::Blog { id: 1 }, "Blog" }
    }   Outlet::<Route> {}
} }

/// Echo component that demonstrates fullstack server functions.
#[component] fn Echo() -> Element {
    let mut response = use_signal(|| String::new());
    rsx! { div { id: "echo",
        h4 { "ServerFn Echo" }
        input { placeholder: "Type here to echo...",
            oninput:  move |event| async move {
                let data = echo_server(event.value()).await.unwrap();
                response.set(data);
            },
        }

        if !response().is_empty() { p { "Server echoed: " i { "{response}" } } }
    } }
}

/// Echo the user input on the server.
#[server(EchoServer)] async fn echo_server(input: String) -> Result<String, ServerFnError> {
    Ok(input)
} */

