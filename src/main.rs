mod vector;
mod gradient;

use vector::{Vec3};

fn main() {
    let v1: Vec3 = Vec3::new(1.0, 2.0, 3.0);

    let unit = v1.unit_vector();

    println!("{:?}", unit);
    println!("{:?}", unit.length());
}
