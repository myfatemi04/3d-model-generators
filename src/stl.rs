use crate::vector::Vector;

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
	pub fn new(a: Vector, b: Vector, c: Vector) -> Triangle {
		let side_a = subtract(&b, &a);
		let side_b = subtract(&c, &b);
		let normal = cross_product(side_a, side_b).normalize();

		Triangle { a, b, c, normal }
	}

	pub fn normal(&self) -> Vector {
		self.normal
	}

	pub fn to_ascii(&self) -> String {
		let mut result = String::new();
		result.push_str(format!("facet normal {}\n", self.normal.to_ascii()).as_str());
		result.push_str("\touter loop\n");
		
		for val in &[self.a, self.b, self.c] {
			result.push_str(format!("\t\tvertex {}\n", val.to_ascii()).as_str());
		}

		result.push_str("\tendloop\n");
		result.push_str("endfacet\n");

		result
	}

	pub fn to_binary(&self) -> Vec<u8> {
		let mut result: Vec<u8> = vec!();
		
		for v in &[self.normal, self.a, self.b, self.c] {
			for b in &v.to_binary() {
				result.push(*b);
			}
		}

		// Attribute byte count
		result.push(0);
		result.push(0);
		
		result
	}
}

pub struct STL {
	pub name: String,
	pub triangles: Vec<Triangle>
}

impl STL {
	pub fn new(name: String) -> STL {
		STL {
			name,
			triangles: Vec::new()
		}
	}

	// For the full format for an STL file, look here:
	// https://en.wikipedia.org/wiki/STL_(file_format)
	pub fn to_ascii(&self) -> String {
		let mut result = String::new();
		result.push_str(format!("solid {}\n", self.name).as_str());

		for triangle in &self.triangles {
			result.push_str(format!("{}\n", &triangle.to_ascii()).as_str());
		}

		result.push_str(format!("endsolid {}\n", self.name).as_str());
		result
	}

	pub fn to_binary(&self) -> Vec<u8> {
		// 80-byte empty header
		let mut result: Vec<u8> = vec![0; 80];

		let triangle_count = self.triangles.len();
		let triangle_count_bytes = (triangle_count as u32).to_le_bytes();

		for b in &triangle_count_bytes {
			result.push(*b);
		}

		for triangle in &self.triangles {
			for b in triangle.to_binary() {
				result.push(b);
			}
		}

		result
	}
}
