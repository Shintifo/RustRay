use std::fs::File;
use std::io::Write;
use crate::vector::Vec3;

pub fn make_gradient() -> std::io::Result<()> {
    let mut file = File::create("gradient.ppm")?;

    let nx = 200;
    let ny = 100;

    writeln!(file, "P3\n{} {}\n255", nx, ny)?;

    for j in (0..=ny - 1).rev() {
        for i in 0..nx {
            let v = Vec3::new(i as f32 / nx as f32, j as f32 / ny as f32, 0.8);

            let ir = (255.0 * v[0]) as u8;
            let ig = (255.0 * v[1]) as u8;
            let ib = (255.0 * v[2]) as u8;
            writeln!(file, "{} {} {}", ir, ig, ib)?;
        }
    }

    Ok(())
}
