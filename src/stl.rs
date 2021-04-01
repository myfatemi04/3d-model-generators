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

	pub fn to_ascii(&self) -> String {
		format!("{} {} {}", self.x, self.y, self.z)
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

	pub fn to_ascii(&self) -> String {
		let mut result = String::new();
		result.push_str(format!("facet normal {}\n", self.normal.to_ascii()));
		result.push_str("\touter loop\n");
		
		for val in &[self.a, self.b, self.c] {
			result.push_str(format!("\t\tvertex {}\n", val.to_ascii()));
		}

		result.push_str("\tendloop\n");
		result.push_str("endfacet\n");

		result
	}
}

pub struct STL {
	name: String,
	triangles: Vec<Triangle>
}

impl STL {
	pub fn init(name: String) -> STL {
		STL {
			name,
			triangles: Vec::new()
		}
	}

	// For the full format for an STL file, look here:
	// https://en.wikipedia.org/wiki/STL_(file_format)
	pub fn to_ascii(&self) -> String {
		let mut result = String::new();
		result.push_str(format!("solid {}", self.name));

		for triangle in &self.triangles {
			result.push_str(format!("{}\n", &triangle.to_ascii()));
		}

		result.push_str(format!("endsolid {}", self.name));
		result
	}
}
