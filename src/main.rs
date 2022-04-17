mod noise;
use noise::Point;
use std::io::Write;

const WIDTH: i64 = 400;
const HEIGHT: i64 = 400;

fn main() {
    let mut noise_gen = noise::WorleyNoiseGen::new();

    let mut file_out = match std::fs::File::create(r"output/out.txt") {
        Ok(file) => file,
        Err(_) => std::fs::File::create("out.txt").unwrap(),
    };

    let mut output = Vec::with_capacity((WIDTH * HEIGHT) as usize);

    for x in 0..=WIDTH {
        for y in 0..=HEIGHT {
            output.push((x, y, noise_gen.generate_noise(Point(x, y))));
        }
    }
    println!("Finished generating");

    for (x, y, noise) in output.into_iter() {
        file_out.write_all(format!("{} {} {}\n", x, y, noise).as_bytes()).unwrap();
    }
    println!("Finished writing")
}
