use image::{io::Reader as ImageReader, GenericImageView, ImageBuffer};
use std::{env, process};

const MAX_SIZE: u32 = 45;
const DARK_TO_LIGHT: [u8; 70] = [
    b'$', b'@', b'B', b'%', b'8', b'&', b'W', b'M', b'#', b'*', b'o', b'a', b'h', b'k', b'b', b'd',
    b'p', b'q', b'w', b'm', b'Z', b'O', b'0', b'Q', b'L', b'C', b'J', b'U', b'Y', b'X', b'z', b'c',
    b'v', b'u', b'n', b'x', b'r', b'j', b'f', b't', b'/', b'\\', b'|', b'(', b')', b'1', b'{',
    b'}', b'[', b']', b'?', b'-', b'_', b'+', b'~', b'<', b'>', b'i', b'!', b'l', b'I', b';', b':',
    b',', b'"', b'^', b'`', b'\'', b'.', b' ',
];
const RAMP_LENGTH: usize = 70;
// const DARK_TO_LIGHT_2: [u8; 92] = [
//     b' ', b'`', b'.', b'-', b'\'', b':', b'_', b',', b'^', b'=', b';', b'>', b'<', b'+', b'!',
//     b'r', b'c', b'*', b'/', b'z', b'?', b's', b'L', b'T', b'v', b')', b'J', b'7', b'(', b'|', b'F',
//     b'i', b'{', b'C', b'}', b'f', b'I', b'3', b'1', b't', b'l', b'u', b'[', b'n', b'e', b'o', b'Z',
//     b'5', b'Y', b'x', b'j', b'y', b'a', b']', b'2', b'E', b'S', b'w', b'q', b'k', b'P', b'6', b'h',
//     b'9', b'd', b'4', b'V', b'p', b'O', b'G', b'b', b'U', b'A', b'K', b'X', b'H', b'm', b'8', b'R',
//     b'D', b'#', b'$', b'B', b'g', b'0', b'M', b'N', b'W', b'Q', b'%', b'&', b'@',
// ];
// const RAMP_LENGTH_2: usize = 92;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Incorrect number of arguments!");
        process::exit(1);
    }

    let img = ImageReader::open(&args[1]).unwrap().decode().unwrap();

    let (width, height) = img.dimensions();
    let mut width_larger = true;
    let scale_factor = if width > height {
        width / MAX_SIZE
    } else {
        width_larger = false;
        height / MAX_SIZE
    };
    let adjusted_img = img
        .blur(if scale_factor < 50 {
            scale_factor
        } else {
            50
        } as f32)
        .grayscale()
        .to_luma16();

    let new_width = if width_larger {
        MAX_SIZE
    } else {
        width / scale_factor
    };
    let new_height = if !width_larger {
        MAX_SIZE
    } else {
        height / scale_factor
    };
    let smaller_img = ImageBuffer::from_fn(2 * new_width, new_height, |x, y| {
        adjusted_img
            .get_pixel(x * scale_factor / 2, y * scale_factor)
            .to_owned()
    });

    for (x, y, pixel) in smaller_img.enumerate_pixels() {
        let ascii_idx = (pixel.0[0] as f64 / 65535.0) * RAMP_LENGTH as f64;
        let ascii_character = DARK_TO_LIGHT[ascii_idx as usize];

        if x == 0 && y != 0 {
            println!();
        }
        print!("{}", ascii_character as char);
    }
    println!();
}
