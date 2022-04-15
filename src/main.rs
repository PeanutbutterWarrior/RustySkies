mod noise;
use noise::Point;
use std::io::Write;

fn main() {
    println!("Hello, world!");

    let mut noise_gen = noise::WorleyNoiseGen::new();

    let mut file_out = std::fs::File::create(r"C:\Users\james\Documents\out.txt").unwrap();

    for x in -200..=200 {
        for y in -200..=200 {
            write!(file_out, "{} {} {}\n", x, y, noise_gen.generate_noise(Point(x, y))).unwrap();
        }
    }
}
