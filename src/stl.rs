#[derive(Copy, Clone)]
pub struct Vector {
	x: f64,
	y: f64,
	z: f64
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
}

pub struct Triangle {
	a: Vector,
	b: Vector,
	c: Vector,
	normal: Vector
}

fn cross_product(a: Vector, b: Vector) -> Vector {
	let x = a.y * b.z - a.z * b.y;
	let y = a.z * b.x - a.x * b.z;
	let z = a.x * b.y - a.y * b.x;

	Vector { x, y, z }
}

fn subtract(a: &Vector, b: &Vector) -> Vector {
	Vector {
		x: a.x - b.x,
		y: a.y - b.y,
		z: a.z - b.z
	}
}

impl Triangle {
	pub fn init(a: Vector, b: Vector, c: Vector) -> Triangle {
		let side_a = subtract(&b, &a);
		let side_b = subtract(&b, &c);
		let normal = cross_product(side_a, side_b);

		Triangle {
			a,
			b,
			c,
			normal: normal.normalize()
		}
	}

	pub fn normal(&self) -> Vector {
		return self.normal;
	}
}
