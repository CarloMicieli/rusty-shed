use rusty_shed_lib::generate_types;

fn main() -> Result<(), String> {
    println!("Generating TypeScript bindings...");
    generate_types()?;
    println!("Done!");
    Ok(())
}
