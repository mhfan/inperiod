#![allow(non_snake_case)]

use dioxus::prelude::*;
use dioxus_logger::tracing;

// Urls are relative to your Cargo.toml file
//const _TAILWIND_URL: &str = manganis::mg!(file("assets/tailwind.css"));

fn main() {
    dioxus_logger::init(tracing::Level::INFO).expect("failed to init logger");
    //tracing::info!("starting app");

    #[cfg(not(feature = "desktop"))] launch(App);
    #[cfg(feature = "desktop")] LaunchBuilder::desktop().with_cfg(dioxus::desktop::Config::new()
        .with_window(dioxus::desktop::WindowBuilder::new().with_title(env!("CARGO_PKG_NAME")))
        .with_custom_head("<link rel='stylesheet' href='tailwind.css'/>".into())
        //.with_custom_head("<script  src='https://cdn.tailwindcss.com'/>".into())
        //.with_custom_index(r"<!DOCTYPE html><html>...</html>".into())
    ).launch(App);
}

fn App() -> Element {
    rsx! {  //Router::<Route> {}
        link { rel: "stylesheet", href: "main.css" }
        img { src: "header.svg", id: "header" }
        div { id: "links",
            a { target: "_blank", href: "https://dioxuslabs.com/learn/0.5/", "📚 Learn Dioxus" }
            a { target: "_blank", href: "https://dioxuslabs.com/awesome", "🚀 Awesome Dioxus" }
            a { target: "_blank", href: "https://github.com/dioxus-community/", "📡 Community Libraries" }
            a { target: "_blank", href: "https://github.com/DioxusLabs/dioxus-std", "⚙️ Dioxus Standard Library" }
            a { target: "_blank", href: "https://marketplace.visualstudio.com/items?itemName=DioxusLabs.dioxus", "💫 VSCode Extension" }
            a { target: "_blank", href: "https://discord.gg/XgGxMSkvUM", "👋 Community Discord" }
        }
    }
}

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

