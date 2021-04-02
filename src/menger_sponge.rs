use crate::stl::{STL, Triangle};
use crate::vector::Vector;

#[derive(Clone)]
pub struct MengerSponge {
	components: Vec<MengerSpongeComponent>,
	min_corner: Vector,
	width: f64
}

impl MengerSponge {
	pub fn new(min_corner: Vector, width: f64) -> MengerSponge {
		let mut components: Vec<MengerSpongeComponent> = Vec::with_capacity(27);
		for _ in 0..27 {
			components.push(MengerSpongeComponent::Solid);
		}

		// Components are numbered left to right, back to front, bottom to top, starting from the left, back, bottom corner.
		components[4] = MengerSpongeComponent::Empty;
		components[10] = MengerSpongeComponent::Empty;
		components[12] = MengerSpongeComponent::Empty;
		components[13] = MengerSpongeComponent::Empty;
		components[14] = MengerSpongeComponent::Empty;
		components[16] = MengerSpongeComponent::Empty;
		components[22] = MengerSpongeComponent::Empty;

		MengerSponge {
			components,
			min_corner,
			width
		}
	}

	pub fn spongeify(&mut self) {
		// For each solid component, create a new sponge.
		for i in 0..self.components.len() {
			match &mut self.components[i] {
				MengerSpongeComponent::Empty => {},
				MengerSpongeComponent::Solid => {
					// left to right...
					let x_relative = i % 3;
					// back to front... (i % 9 isolates the "slice" of the cube we look at)
					let z_relative = (i % 9) / 3;
					// bottom to top
					let y_relative = i / 9;

					// We're using a right-hand coordinate system, so +Z goes towards the camera,
					// +Y goes up, and +X goes to the left.

					let component_size = self.width / 3.0;

					let x_absolute = self.min_corner.x + (x_relative as f64) * component_size;
					let y_absolute = self.min_corner.y + (y_relative as f64) * component_size;
					let z_absolute = self.min_corner.z + (z_relative as f64) * component_size;

					let min_corner = Vector {
						x: x_absolute,
						y: y_absolute,
						z: z_absolute
					};

					let sponge = MengerSponge::new(min_corner, component_size);
					let boxed_sponge = Box::new(sponge);
					let component = MengerSpongeComponent::Sponge(boxed_sponge);

					self.components[i] = component;
				},
				MengerSpongeComponent::Sponge (container) => {
					(*container).spongeify();
				}
			}
		}
	}

	pub fn to_stl(&self) -> STL {
		let mut stl = STL::new(String::from("my_sponge"));

		self.add_triangles(&mut stl.triangles);

		stl
	}

	// Helper method. Adds the corresponding triangles to a vector that is passed by reference.
	fn add_triangles(&self, triangles: &mut Vec<Triangle>) {
		for i in 0..self.components.len() {
			match &self.components[i] {
				MengerSpongeComponent::Empty => {},
				MengerSpongeComponent::Solid => {
					// left to right...
					let x_relative = i % 3;
					// back to front... (i % 9 isolates the "slice" of the cube we look at)
					let z_relative = (i % 9) / 3;
					// top to bottom
					let y_relative = i / 9;

					// We're using a right-hand coordinate system, so +Z goes towards the camera,
					// +Y goes up, and +X goes to the left.

					let component_size = self.width / 3.0;

					let x_absolute = self.min_corner.x + (x_relative as f64) * component_size;
					let y_absolute = self.min_corner.y + (y_relative as f64) * component_size;
					let z_absolute = self.min_corner.z + (z_relative as f64) * component_size;

					// Convert the solid cube section to a group of triangles

					// Find the corners
					let corners: [Vector; 8] = [
						// Top slice
						Vector::new(x_absolute, y_absolute, z_absolute),
						Vector::new(x_absolute + component_size, y_absolute, z_absolute),
						Vector::new(x_absolute, y_absolute, z_absolute + component_size),
						Vector::new(x_absolute + component_size, y_absolute, z_absolute + component_size),

						Vector::new(x_absolute, y_absolute + component_size, z_absolute),
						Vector::new(x_absolute + component_size, y_absolute + component_size, z_absolute),
						Vector::new(x_absolute, y_absolute + component_size, z_absolute + component_size),
						Vector::new(x_absolute + component_size, y_absolute + component_size, z_absolute + component_size),
					];

					// Connect groups of three corners
					// Each triangle's vertices are clockwise
					let corner_groups: [(u8, u8, u8); 12] = [
						// Bottom face
						(0, 1, 2),
						(1, 3, 2),
						
						// Top face
						(4, 5, 6),
						(5, 7, 6),
						
						// Front face
						(6, 7, 2),
						(7, 3, 2),
						
						// Back face
						(0, 1, 4),
						(1, 5, 4),
						
						// Left face
						(4, 6, 0),
						(6, 2, 0),
						
						// Right face
						(5, 7, 1),
						(7, 3, 1)
					];
					
					for (a, b, c) in corner_groups.iter() {
						triangles.push(Triangle::new(corners[*a as usize], corners[*b as usize], corners[*c as usize]));
					}
				},
				MengerSpongeComponent::Sponge(boxed_sponge) => {
					let sponge = boxed_sponge.as_ref();
					sponge.add_triangles(triangles);
				}
			}
		}
	}
}

pub fn generate_generic_sponge(iterations: usize) -> MengerSponge {
	let mut sponge = MengerSponge::new(crate::vector::zero(), 10.0);
	for _ in 0..iterations {
		sponge.spongeify();
	}
	sponge
}

#[derive(Clone)]
pub enum MengerSpongeComponent {
	Empty,
	Solid,
	Sponge (Box<MengerSponge>)
}

