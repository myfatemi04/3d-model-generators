pub mod stl;
pub mod menger_sponge;
pub mod vector;

use std::time::{SystemTime};

fn elapsed_since(start: SystemTime) -> u128 {
    SystemTime::now().duration_since(start).unwrap().as_millis()
}

fn main() {
    let start = SystemTime::now();

    let model = menger_sponge::generate_generic_sponge(2);
    println!("Generated model in {} ms", elapsed_since(start));

    let stl = model.to_stl();
    println!("Converted to STL in {} ms", elapsed_since(start));

    let text = stl.to_binary();
    println!("Serialized STL in {} ms", elapsed_since(start));

    match std::fs::write("model.stl", text) {
        Ok(_) => {
            println!("Successfully saved STL");
        },
        Err(err) => {
            println!("Error writing STL file: {}", err);
        }
    }
}
