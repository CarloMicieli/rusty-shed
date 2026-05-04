use rusty_shed_lib::generate_types;

fn main() {
    println!("Generating TypeScript bindings...");
    generate_types();
    println!("Done!");
}
