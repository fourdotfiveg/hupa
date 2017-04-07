
fn main() {
    if cfg!(feature = "text-json") == false {
        println!("There is no enabled feature to allow metadata files.");
        println!("Do not remove default features.");
        std::process::exit(1);
    }
}
