#[derive(Copy, Clone)]
pub struct Vector {
	pub x: f64,
	pub y: f64,
	pub z: f64
}

pub fn zero() -> Vector {
	Vector { x: 0.0, y: 0.0, z: 0.0 }
}

impl Vector {
	pub fn new(x: f64, y: f64, z: f64) -> Vector {
		Vector { x, y, z }
	}

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