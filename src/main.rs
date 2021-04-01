pub mod stl;
pub mod menger_sponge;
pub mod vector;

fn main() {
    let model = menger_sponge::generate_generic_sponge(1);
    let stl = model.to_stl();
    let text = stl.to_ascii();

    match std::fs::write("model.stl", text) {
        Ok(_) => {
            println!("Successfully created STL");
        },
        Err(err) => {
            println!("Error writing STL file: {}", err);
        }
    }
}
