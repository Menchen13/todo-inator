// build.rs
fn main() {
    // This tells Rust: "Expect a config flag named 'fuzzing'"
    println!("cargo::rustc-check-cfg=cfg(fuzzing)");
    //slint ui interagtion
    slint_build::compile("ui/app.slint").unwrap();
}
