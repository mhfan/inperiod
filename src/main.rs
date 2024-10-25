#![allow(non_snake_case)]

use dioxus::prelude::*;
use dioxus_logger::tracing;

// Urls are relative to your Cargo.toml file
//const _TAILWIND_CSS: &str = manganis::mg!(file("assets/tailwind.css"));

fn main() {
    dioxus_logger::init(tracing::Level::INFO).expect("failed to init logger");
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
        //    href: "https://unpkg.com/tailwindcss@^3.14/dist/tailwind.min.css" }
        //link { rel: "stylesheet",
        //    href: "https://cdn.jsdelivr.net/npm/tw-elements/dist/css/index.min.css" }

        div { class: "flex items-center justify-center w-fit",
            style: "transform: scale(0.8); transform-origin: 0 0;", PeriodicTable {}
        }
    }
}

#[allow(unused)] const ROMAN_NUM: [&str; 11] = [
    "", "I", "II", "III", "IV", "V", "VI", "VII", "VIII", "IX", "X" ];

#[component] fn PeriodicTable() -> Element { rsx! {
    div { class: "grid m-2 mr-4",
        style: "grid: 'lperiod title lshell' 'period main shell' '. block .' '. lac .';",

        div { class: "grid grid-cols-18 text-center place-content-end", style: "grid-area: title;",
            div { class: "self-end",
                p { class: "leading-none text-sm font-bold", "GROUP" } p { "IA - 1" } }
            p { class: "col-span-16 text-5xl font-bold", "Periodic Table of the Elements" }
            p { class: "self-end", "VIIIA - 0" }
        }
        p { style: "grid-area: lperiod; writing-mode: vertical-rl; transform: rotate(180deg);",
            class: "text-sm font-bold leading-tight relative -bottom-4", "PERIOD" }
        div { class: "grid mx-1 items-center", style: "grid-area: period;",
            { (1..=7).map(|i| rsx! { p { "{i}" } }) } }
        div { class: "grid grid-cols-18 grid-rows-7 relative",
            style: "grid-area: main;", ChemElem { ordinal: 1 }

            p { class: "text-center self-end", "IIA - 2" } div { class: "col-span-10" }
            { (13..=17).map(|i| rsx! { p { class: "text-center self-end",
                { format!("{}A - {i}", ROMAN_NUM[i - 10]) } } }) }

            ChemElem { ordinal: 2 } ChemElem { ordinal: 3 }
            ChemElem { ordinal: 4 } div { class: "col-span-10" }
            {  (5..=12).map(|i| rsx! { ChemElem { ordinal: i } }) }

            {   (3..=7).map(|i| rsx! { p { class: "text-center self-end",
                { format!("{}B - {i}", ROMAN_NUM[i]) } } }) }
            p { class: "text-center self-end col-span-3 border-b-2", "VIIIB - 8|9|10" }
            p { class: "text-center self-end", "IB - 11" }
            p { class: "text-center self-end", "IIB - 12" }

            { (13..=56).map(|i| rsx! { ChemElem { ordinal: i } }) }
            div { class: "text-center text-2xl content-center rounded-sm p-1
                shadow-border-1 shadow-indigo-200",
                p { span { class: "font-bold", "Lan" } " Series" } p { "(镧系)" } }
            { (72..=88).map(|i| rsx! { ChemElem { ordinal: i } }) }
            div { class: "text-center text-2xl content-center rounded-sm p-1
                shadow-border-1 shadow-indigo-200",
                p { span { class: "font-bold", "Act" } " Series" } p { "(锕系)" } }
            { (104..=118).map(|i| rsx! { ChemElem { ordinal: i } }) }

            div { class: "absolute flex w-full h-full pb-6",
                style: "grid-area: 1 / 3 / 4 / 13;", id: "legend", // XXX:
                div { class: "ml-[10%]", ChemElem { ordinal: 26 } }
            }
            //div { class: "flex", style: "grid-area: 3 / 3 / 4 / 13;", }
            div { class: "flex absolute w-full h-full pb-6",
                style: "grid-area: 1 / 13 / 2 / 18;", // XXX:
            }
        }
        div { class: "grid grid-cols-18 text-center divide-x mb-2", style: "grid-area: block;",
            div { class: "col-span-2",  span { class: "font-bold", "s" }
                "-block (plus " span { class: "font-bold", "He" } ")" }
            div { class: "col-span-10", span { class: "font-bold", "d" }
                "-block (exclude " span { class: "italic", "Lan/Act" } " series)" }
            div { class: "col-span-6",  span { class: "font-bold", "p" }
                "-block (exclude " span { class: "italic", "He" } ")" }
        }

        div { class: "grid grid-cols-18", style: "grid-area: lac;",
            div { class: "col-span-3 row-span-3 flex", id: "notes", // XXX:
                p { class: "self-end", "© 2024 "
                    a { href: "https://github.com/mhfan", "M.H.Fan" } }
            }

            { (57..= 71).map(|i| rsx! { ChemElem { ordinal: i } }) }
            { (89..=103).map(|i| rsx! { ChemElem { ordinal: i } }) }

            div { class: "col-[span_14_/_span_14] text-center",
                span { class: "font-bold", "f" } "-block" }
            div { class: "border-l", } // XXX:
        }

        p { style: "grid-area: lshell;  writing-mode: vertical-rl;",
            class: "text-sm font-bold leading-none relative -bottom-4 content-center ml-2",
            "e-shell" br{} "e-max" }
        div { class: "grid grid-rows-7 font-mono text-sm text-nowrap text-right
            tracking-tighter leading-tight divide-y", style: "grid-area: shell;",
            div { class: "content-center pl-2", p {  "2 K" } }
            div { class: "content-center pl-2", p {  "8 L" } p {  "2 K" } }
            div { class: "content-center pl-2", p {  "8 M" } p {  "8 L" } p {  "2 K" } }
            div { class: "content-center pl-2",
                      p {  "8 N" } p { "18 M" } p {  "8 L" } p {  "2 K" } }
            div { class: "content-center pl-2", p {  "8 O" } p { "18 N" }
                      p { "18 M" } p {  "8 L" } p {  "2 K" } }
            div { class: "content-center pl-2", p {  "8 P" } p { "18 O" }
                      p { "32 N" } p { "18 M" } p {  "8 L" } p {  "2 K" } }
            div { class: "content-center pl-2", p {  "8 Q" } p { "18 P" }
                      p { "32 O" } p { "32 N" } p { "18 M" } p {  "8 L" } p {  "2 K" }
            }
        }
    }
} }

#[component] fn ChemElem(ordinal: u8) -> Element { rsx! {
    div { class: "rounded-sm p-1 px-1.5 shadow-border-1 shadow-indigo-200 bg-indigo-50",
        div { class: "flex",
            div { class: "grow",
                p { class: "flex", id: "mass", "55.845"
                    span { class: "ml-2 text-center grow", id: "radioactive", "☢\u{fe0f}" } }
                p { class: "text-sm flex", id: "ionisation", "762.5"
                    span { class: "ml-2 text-center grow", id: "electroneg", "1.83" } }
            }
            span { class: "text-xl font-bold ml-2 flex-shrink", id: "ordinal", "{ordinal}" }
        }
        div { class: "flex",
            div { class: "flex-col flex",
                div { class: "flex",
                    span { class: "text-6xl grow", id: "symbol", "Fe" }
                    div { class: "items-center",
                        p { class: "text-center leading-tight", id: "pinyin", "tiě" }
                        p { class: "text-3xl text-right", id: "name_ch", "铁" }
                    }
                }
                p { class: "leading-tight", id: "name", "Iron" }

                p { class: "text-sm flex",
                    span { class: "text-blue-700", id: "melting", "1538" } "/"
                    span { class: "text-red-700",  id: "boiling", "2861" }
                    span { class: "ml-2 text-center grow", id: "crystal_s", "BCC" }
                }   // TODO: abundance, origin
                p { class: "text-sm flex", id: "density", "7.874"
                    span { class: "ml-2 font-bold text-purple-700", id: "radius", "126" }
                    span { class: "ml-2 text-center grow", id: "ground_s",
                        sup { "5" } "D" sub { "4" } }
                }

                p { class: "mt-auto", id: "config_e", "[Ar] 3d⁶ 4s²" }
            }
            div { class: "text-right ml-2 font-mono relative -top-3 self-center leading-none",
                id: "oxidation", p { "+6" } p { "+5" } p { "+4" }
                p { class: "font-extrabold", "+3" }
                p { class: "font-extrabold", "+2" }
                p { "+1" } p { "-1" } p { "-2" }
            }
        }
    }
    //  https://en.wikipedia.org/wiki/Unicode_subscripts_and_superscripts
    //  Unicode superscript numbers: ⁰ ¹ ² ³ ⁴ ⁵ ⁶ ⁷ ⁸ ⁹ ⁺ ⁻ ⁼ ⁽ ⁾
    //  Unicode   subscript numbers: ₀ ₁ ₂ ₃ ₄ ₅ ₆ ₇ ₈ ₉ ₊ ₋ ₌ ₍ ₎
} }

#[component] fn ElemDetail() -> Element { rsx! { // TODO: https://pt.ziziyi.com/
} }

//  TODO: https://physics.nist.gov/cuu/Constants/index.html
#[component] fn PhysConsts() -> Element { rsx! {
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

