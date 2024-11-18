
// https://doc.rust-lang.org/stable/cargo/reference/build-scripts.html

use std::process;
fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("cargo:rerun-if-changed=build.rs");    // XXX: prevent re-run indead
    let (twcfg, twcss) = ("tailwind.config.js", "tailwind_base.css");
    println!("cargo:rerun-if-changed={twcfg}");
    println!("cargo:rerun-if-changed={twcss}");
    println!("cargo:rerun-if-changed=src");

    if std::path::Path::new(twcss).exists() { return Ok(()) }
/*  npm install -D tailwindcss; npm install tw-elements
    sh -c "[ ! -d node_modules ] && npm i; npm run build_css"

    process::Command::new("npm").args(["run", "build_css"]).status()?;

    npx tailwindcss init  # generate a mininum tailwind.config.js
    npx tailwindcss -m -i tailwind_base.css -o assets/tailwind.css #-c tailwind.config.js #-w */

    let minify = if let Ok("release") =
        std::env::var("PROFILE").as_deref() { "-m" } else { "" };
    process::Command::new("npx").args(["tailwindcss", minify, //"-c", twcfg,
        "-i", twcss, "-o", "assets/tailwind.css"]).status()?;
    Ok(())
}

