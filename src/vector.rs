#[derive(Copy, Clone)]
pub struct Vector {
	pub x: f64,
	pub y: f64,
	pub z: f64
}

impl Vector {
	pub fn magnitude(&self) -> f64 {
		return f64::powf(f64::powi(self.x, 2) + f64::powi(self.y, 2) + f64::powi(self.z, 2), 0.5);
	}

	pub fn normalize(&self) -> Vector {
		let magnitude = self.magnitude();
		return Vector {
			x: self.x / magnitude,
			y: self.y / magnitude,
			z: self.z / magnitude
		};
	}

	pub fn to_ascii(&self) -> String {
		format!("{} {} {}", self.x, self.y, self.z)
	}
}