// const F021_LIBRARY: &str = "F021_API_CortexR4_BE_L2FMC.lib";
const F021_LIBRARY_STRIPPED: &str = "F021_API_CortexR4_BE_L2FMC_NDS.lib";
fn main() {
    println!(
        "cargo::rustc-link-arg={}/{}",
        env!("CARGO_MANIFEST_DIR"),
        F021_LIBRARY_STRIPPED
    );
}
