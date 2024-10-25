
// https://doc.rust-lang.org/stable/cargo/reference/build-scripts.html

fn main() {
    println!("cargo:rerun-if-changed=build.rs");    // XXX: prevent re-run indead
    let (twcfg, twcss) = ("tailwind.config.js", "tailwind_base.css");
    println!("cargo:rerun-if-changed={twcfg}");
    println!("cargo:rerun-if-changed={twcss}");
    println!("cargo:rerun-if-changed=src");

/*  npm install -D tailwindcss; npm install tw-elements
    sh -c "[ ! -d node_modules ] && npm i; npm run build_css"

    std::process::Command::new("npm").args(["run", "build_css"]).status().unwrap();

    npx tailwindcss init  # generate a mininum tailwind.config.js
    npx tailwindcss -m -i tailwind_base.css -o dist/tailwind.css #-c tailwind.config.js #-w */

    let minify = if let Ok("release") =
        std::env::var("PROFILE").as_deref() { "-m" } else { return; "" };
    std::process::Command::new("npx").args(["tailwindcss", minify, //"-c", twcfg,
        "-i", twcss, "-o", "dist/tailwind.css"]).status().unwrap();
}

