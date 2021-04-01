use crate::vector::Vector;

#[derive(Clone)]
pub struct MengerSponge {
	components: Vec<MengerSpongeComponent>,
	start_corner: Vector,
	width: f64
}

impl MengerSponge {
	pub fn new(start_corner: Vector, width: f64) -> MengerSponge {
		let mut components: Vec<MengerSpongeComponent> = vec!();
		for _ in 0..27 {
			components.push(MengerSpongeComponent::Solid);
		}

		// Components are numbered left to right, back to front, top to bottom, starting from the left, back, top corner.
		components[4] = MengerSpongeComponent::Empty;
		components[10] = MengerSpongeComponent::Empty;
		components[12] = MengerSpongeComponent::Empty;
		components[13] = MengerSpongeComponent::Empty;
		components[14] = MengerSpongeComponent::Empty;
		components[16] = MengerSpongeComponent::Empty;
		components[22] = MengerSpongeComponent::Empty;

		MengerSponge {
			components,
			start_corner,
			width
		}
	}

	pub fn spongeify(&mut self) {
		// For each solid component, create a new sponge.
		for i in 0..self.components.len() {
			// left to right...
			let x_relative = i % 3;
			// back to front... (i % 9 isolates the "slice" of the cube we look at)
			let z_relative = (i % 9) / 3;
			// top to bottom
			let y_relative = -((i as i8) / 9);

			// We're using a right-hand coordinate system, so +Z goes towards the camera,
			// +Y goes up, and +X goes to the left.

			let component_size = self.width / 3.0;

			let x_absolute = self.start_corner.x + (x_relative as f64) * component_size;
			let y_absolute = self.start_corner.y + (y_relative as f64) * component_size;
			let z_absolute = self.start_corner.z + (z_relative as f64) * component_size;

			let start_corner = Vector {
				x: x_absolute,
				y: y_absolute,
				z: z_absolute
			};

			self.components[i] = MengerSpongeComponent::Sponge(Box::new(MengerSponge::new(start_corner, component_size)));
		}
	}
}

#[derive(Clone)]
pub enum MengerSpongeComponent {
	Empty,
	Solid,
	Sponge (Box<MengerSponge>)
}

