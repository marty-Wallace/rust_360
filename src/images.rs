
extern crate image;
extern crate rand;

use image::{DynamicImage, GenericImage, FilterType, Rgba};
use image::png::PNGEncoder;
use std::path::Path;
use std::cmp;
use rand::{Rng, StdRng};


pub fn itemset_delegater(itemset: &str, mut rng: &mut StdRng) -> Vec<u8> {
    match itemset {
        "circles" => {
            circle_set(&mut rng)
        },

        "squares" => {
            square_set(&mut rng)
        },

        "shapes" => {
            shapes_generate(&mut rng)
        },

        "abstract" => {
            abstract_generate(&mut rng)
        },

        "checkers" => {
            checkers_generate(&mut rng)
        },

        _ => unimplemented!()
    }
}

fn square_set(rng: &mut StdRng) -> Vec<u8> {
    let w = 729 / 3;
    let mut img = image::DynamicImage::new_rgba8(w, w);
    let colors = vec![[26, 188, 156, 255], [46, 204, 113, 255],
                          [155, 89, 182, 255], [22, 160, 133, 255],
                          [39, 174, 96, 255], [41, 128, 185, 255], [142, 68, 173, 255],
                          [241, 196, 15, 255], [230, 126, 34, 255], [231, 76, 60, 255],
                          [243, 156, 18, 255], [211, 84, 0, 255], [192, 57, 43, 255],
                          [127, 140, 141, 255],
    ];
    // [52, 152, 219, 255]
    let base_color = [44, 63, 80, 255];
    fill_square(&mut img, base_color, 0, 0,w);
    square(&mut img, &colors, rng,0, 0, w);
    image_to_vec(&img)

}

fn circle_set(rng: &mut StdRng) -> Vec<u8> {
    let w = 729 / 3;
    let mut img = image::DynamicImage::new_rgba8(w, w);
    // rgb(52, 73, 94)
    let colors = vec![
        [26, 188, 156, 255], [46, 204, 113, 255],
        [155, 89, 182, 255], [22, 160, 133, 255],
        [39, 174, 96, 255], [41, 128, 185, 255],
        [142, 68, 173, 255], [241, 196, 15, 255],
        [230, 126, 34, 255], [231, 76, 60, 255],
        [243, 156, 18, 255], [211, 84, 0, 255],
        [192, 57, 43, 255], [127, 140, 141, 255],
    ];

    let base_color = [44, 63, 80, 255];
    fill_square(&mut img, base_color, 0, 0,w);
    circle(&mut img, &colors, rng,0, 0, w);
    image_to_vec(&img)

}

fn fill_square(im: &mut DynamicImage, color: [u8; 4], x: u32, y: u32, length: u32) {
    for i in 0..length {
        for j in 0..length {
            im.put_pixel(j+x, i+y, Rgba(color));
        }
    }
}

fn fill_circle(im: &mut DynamicImage, color: [u8; 4], x: u32, y: u32, length: u32) {
    let r = length / 2;
    let cx = x + r;
    let cy = y + r;
    for i in 0..length {
        for j in 0..length {
            let px = j+x;
            let py = i+y;
            let dx = (cmp::max(px,cx) - cmp::min(px, cx)).pow(2);
            let dy = (cmp::max(py,cy) - cmp::min(py, cy)).pow(2);
            let d2 = dx + dy;
            if f64::from(d2).sqrt() < f64::from(r)   {
                im.put_pixel(j + x, i + y, Rgba(color));
            }
        }
    }
}

fn square(mut im: &mut DynamicImage, colors: &[[u8; 4]], rng: &mut StdRng, x: u32, y: u32, l: u32) {
    let color = rng.choose(colors)
        .expect("Failed to choose random color");

    let inner = l / 3;
    fill_square(&mut im, *color, x+inner, y+inner, inner);

    if inner > 3 {
        square(&mut im, colors, rng, x, y, inner);
        square(&mut im, colors, rng, x + inner, y, inner);
        square(&mut im, colors, rng, x + (inner*2), y, inner);
        square(&mut im, colors, rng, x, y + inner, inner);
        square(&mut im, colors, rng, x, y + (inner * 2), inner);
        square(&mut im, colors, rng, x + (inner * 2), y + inner, inner);
        square(&mut im, colors, rng, x + inner, y + (inner*2), inner);
        square(&mut im, colors, rng, x + (inner * 2), y + (inner *2), inner);
    }
}

fn circle(mut im: &mut DynamicImage, colors: &[[u8; 4]], rng: &mut StdRng, x: u32, y: u32, l: u32) {
    let color = rng.choose(colors)
        .expect("Failed to choose random color");

    let inner = l / 3;
    fill_circle(&mut im, *color, x+inner, y+inner, inner);

    if inner > 3 {
        circle(&mut im, colors, rng, x, y, inner);
        circle(&mut im, colors, rng, x + inner, y, inner);
        circle(&mut im, colors, rng, x + (inner*2), y, inner);
        circle(&mut im, colors, rng, x, y + inner, inner);
        circle(&mut im, colors, rng, x, y + (inner * 2), inner);
        circle(&mut im, colors, rng, x + (inner * 2), y + inner, inner);
        circle(&mut im, colors, rng, x + inner, y + (inner*2), inner);
        circle(&mut im, colors, rng, x + (inner * 2), y + (inner *2), inner);
        // circle(&mut im, colors, rng, x + inner, y + inner, inner);
    }
}

pub fn shapes_generate(rng: &mut StdRng) -> Vec<u8> {

    let shapes = vec!["DarkBlueRect.png",
                      "DarkGreenRect.png",
                      "DarkPurpleRect.png",
                      "LightBlueRect.png",
                      "LightGreenRect.png",
                      "LightPurple.png",
                      "MagentaRect.png",
                      "OrangeRect.png",
                      "PinkRec.png",
                      "RedRect.png",
                      "YellowRect.png"
    ];

    let eyes = vec!["BrownEye.png",
                    "DarkBlueEye.png",
                    "DarkGreenEye.png",
                    "DarkPurpleEye.png",
                    "LightBlueEye.png",
                    "LightGreenEye.png",
                    "LightPurpleEye.png",
                    "MagentaEye.png",
                    "OrangeEye.png",
                    "PinkEye.png",
                    "RedEye.png",
                    "YellowEye.png"
    ];

    let background_file = format!("res/Shapes/Rects/{}", shapes[rng.gen::<usize>() % shapes.len()]);
    println!("{}", background_file);
    let mut base_image  = image::open(&Path::new(&background_file)).unwrap();
    let i = rng.gen::<u32>() % 7 + 1;
    for _ in 0..i {
        let eye_file= format!("res/Shapes/Eyes/{}", eyes[rng.gen::<usize>() % eyes.len()]);
        println!("{}", eye_file);
        let im2 = image::open(&Path::new(&eye_file)).unwrap();

        let (x1, y1) = base_image.dimensions();
        let (x2, y2) = im2.dimensions();
        let scale = rng.gen::<u32>() % 4 + 1;

        let im2 = im2.resize(x2 / scale, y2 / scale, FilterType::Nearest);
        let (x2, y2) = im2.dimensions();


        let x3 = rng.gen::<u32>() % (x1-x2);
        let y3 = rng.gen::<u32>() % (y1-y2);
        for i in 0..y2 {
            for j in 0..x2 {
                base_image.blend_pixel(j+x3, i+y3, im2.get_pixel(j, i));
            }
        }
    }

    image_to_vec(&base_image)
}

pub fn checkers_generate(rng: &mut StdRng) -> Vec<u8>{

    let shapes = vec![
        "checker.png",
    ];

    let eyes = vec!["BHO.png",
                    "BLO.png",
                    "BMO.png",
                    "GHO.png",
                    "GLO.png",
                    "GMO.png",
                    "MHO.png",
                    "MLO.png",
                    "MMO.png",
                    "PHO.png",
                    "PLO.png",
                    "PMO.png",
                    "WHO.png",
                    "WLO.png",
                    "WMO.png",
                    "YHO.png",
                    "YLO.png",
                    "YMO.png",
    ];

    let background_file = format!("res/Checkers/{}", shapes[rng.gen::<usize>() % shapes.len()]);
    let base_image  = image::open(&Path::new(&background_file)).unwrap();
    let mut base_image = base_image.resize(400, 400, FilterType::CatmullRom);
    let i = rng.gen::<u32>() % 10 + 1;

    for j in 0..i {
        let eye_file= format!("res/Checkers/{}", eyes[rng.gen::<usize>() % eyes.len()]);
        println!("{}", eye_file);
        let im2 = image::open(&Path::new(&eye_file)).unwrap();

        let (x1, y1) = base_image.dimensions();
        let x1 = x1 - 10;
        let y1 = y1 - 10;

        let (x2, y2) = im2.dimensions();
        let scale = rng.gen::<u32>() % ((j/2)+2) + 1;

        let im2 = im2.resize(x2 / scale, y2 / scale, FilterType::Nearest);
        let (x2, y2) = im2.dimensions();

        let mut x3 = rng.gen::<u32>() % (x1-x2) + 5;
        let mut y3 = rng.gen::<u32>() % (y1-y2) + 5;
        for i in 0..y2 {
            for j in 0..x2 {
                base_image.blend_pixel(j + x3, i + y3, im2.get_pixel(j, i));
            }
        }
    }
    image_to_vec(&base_image)
}


pub fn abstract_generate(rng: &mut StdRng) -> Vec<u8>{

    let backgrounds = vec!["Blue.png", "DarkBlue.png", "DarkGrey.png", "Green.png", "LighterGrey.png", "Sunflower.png", "Turquoise.png"];
    let shapes = vec!["Rectangle.png", "RedDot.png", "Star.png"];

    let background_file = format!("res/abstract_itemset/backgrounds/{}", backgrounds[rng.gen::<usize>() % backgrounds.len()]);
    let mut base_image  = image::open(&Path::new(&background_file)).unwrap();
    let i = rng.gen::<u32>() % 5;
    for _ in 0..i {
        let shape_file = format!("res/abstract_itemset/shapes/{}", shapes[rng.gen::<usize>() % shapes.len()]);
        let im2 = image::open(&Path::new(&shape_file)).unwrap();
        for _ in 0..4 {
            if rng.gen::<u32>() % 4 > 0 {
                im2.rotate90();
            }
        }
        let (x1, y1) = base_image.dimensions();
        let (x2, y2) = im2.dimensions();
        let x3 = rng.gen::<u32>() % (x1-x2);
        let y3 = rng.gen::<u32>() % (y1-y2);
        for i in 0..y2 {
            for j in 0..x2 {
                base_image.blend_pixel(j+x3, i+y3, im2.get_pixel(j, i));
            }
        }
    }
    image_to_vec(&base_image)
}

#[inline]
fn image_to_vec(image: &DynamicImage) -> Vec<u8> {
    let mut x : Vec<u8> = vec![];
    PNGEncoder::new(&mut x)
        .encode(image.raw_pixels().as_slice(),
                image.width(),
                image.height(),
                image.color())
        .ok();
    x
}


#[cfg(test)]
mod tests {
    use super::*;
    use rand::{Rng, StdRng, SeedableRng};

    #[test]
    fn random_generate() {
        let mut init_rng = StdRng::new().expect("Failed to create rng");
        let mut bytes: [u8; 30] = [0_u8; 30];
        init_rng.fill_bytes(&mut bytes);
        let seed = bytes
            .iter()
            .map(|x| *x as usize)
            .collect::<Vec<_>>();
        let mut rng1 = StdRng::from_seed(&seed);
        let mut rng2 = StdRng::from_seed(&seed);
        //since itemset doesn't matter right now
        // assert_eq!(shapes_generate("itemset", &mut rng1), shapes_generate("itemset", &mut rng2));
    }

}
