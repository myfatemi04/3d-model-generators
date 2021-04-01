use crate::vector::Vector;

#[derive(Clone)]
pub struct MengerSponge {
	components: Vec<MengerSpongeComponent>,
	max_corner: Vector,
	width: f64
}

impl MengerSponge {
	pub fn init(max_corner: Vector, width: f64) -> MengerSponge {
		let mut components: Vec<MengerSpongeComponent> = vec!();
		for _ in 0..27 {
			components.push(MengerSpongeComponent::Solid);
		}

		// Components are numbered from left to right, back to front, top to bottom.
		components[4] = MengerSpongeComponent::Empty;
		components[10] = MengerSpongeComponent::Empty;
		components[12] = MengerSpongeComponent::Empty;
		components[13] = MengerSpongeComponent::Empty;
		components[14] = MengerSpongeComponent::Empty;
		components[16] = MengerSpongeComponent::Empty;
		components[22] = MengerSpongeComponent::Empty;

		MengerSponge {
			components,
			max_corner,
			width
		}
	}

	pub fn spongeify() {

	}
}

#[derive(Clone)]
pub enum MengerSpongeComponent {
	Empty,
	Solid,
	Sponge (Box<MengerSponge>)
}

