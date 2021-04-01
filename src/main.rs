pub mod stl;
pub mod menger_sponge;
pub mod vector;

fn main() {
    let sponge = menger_sponge::MengerSponge::new(vector::zero(), 10.0);
    let stl = sponge.to_stl();
    let text = stl.to_ascii();

    match std::fs::write("sponge.stl", text) {
        Ok(_) => {
            println!("Successfully created STL");
        },
        Err(err) => {
            println!("Error writing STL file: {}", err);
        }
    }
}
