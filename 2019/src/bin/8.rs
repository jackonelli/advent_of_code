use std::fs::File;
use std::io::prelude::*;

const WIDTH: usize = 25;
const HEIGHT: usize = 6;

#[derive(Clone)]
struct Layer {
    digit_count: [u32; 3],
    image: [u8; (WIDTH * HEIGHT)],
}
impl Layer {
    fn new() -> Self {
        Layer {
            digit_count: [0; 3],
            image: [3; WIDTH * HEIGHT],
        }
    }

    fn pixel_value_count(&self, value: usize) -> u32 {
        self.digit_count[value]
    }

    fn update_pixel(&mut self, pos: usize, pixel: u8) {
        if self.image[pos] < 2 {
        } else {
            self.image[pos] = pixel;
        }
    }

    fn print(&self) {
        let mut line = String::from("");
        for (pos, pixel) in self.image.iter().enumerate() {
            match pixel {
                0 => line.push(' '),
                1 => line.push('*'),
                2 => line.push(' '),
                _ => panic!("Wrong pixel val"),
            };
            if (pos + 1) % WIDTH == 0 {
                println!("{}", line);
                line = String::from("");
            }
        }
    }
}

fn parse_image(image_str: &str) -> Vec<Layer> {
    let mut image = Vec::new();
    let mut layer = Layer::new();
    for (digit_counter, digit) in image_str.chars().enumerate() {
        let pixel: usize = digit.to_digit(10).expect("Could not parse char") as usize;
        layer.digit_count[pixel] += 1;
        let pixel_number = (digit_counter + 1) % (WIDTH * HEIGHT);
        if pixel_number == 0 {
            image.push(layer.clone());
            layer = Layer::new();
        }
        layer.image[pixel_number] = pixel as u8;
    }
    image
}

fn collapse_image(image: Vec<Layer>) -> Layer {
    let mut collapsed_image = image[0].clone();
    for layer in image {
        for (pos, pixel) in layer.image.iter().enumerate() {
            collapsed_image.update_pixel(pos, *pixel);
        }
    }
    collapsed_image
}

fn main() {
    let file = "input/8/input";
    let mut file = File::open(file).expect("Opening file error");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Read to string error");
    let image = contents.trim();
    let image = parse_image(&image);
    let max_layer = image
        .iter()
        .min_by_key(|x| x.pixel_value_count(0))
        .expect("No max");
    println!(
        "{}",
        max_layer.pixel_value_count(1) * max_layer.pixel_value_count(2)
    );
    let collapse_imaged = collapse_image(image);
    collapse_imaged.print();
}
