extern crate neon_build;

fn main() {
    println!("dimcher...");
    neon_build::setup(); // must be called in build.rs

    // add project-specific build logic here...
}
