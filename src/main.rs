mod vector;
mod gradient;

use gradient::make_gradient;

fn main() {

    make_gradient().unwrap();
    println!("Gradient Created!")

}
