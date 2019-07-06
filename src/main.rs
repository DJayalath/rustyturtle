extern crate minifb;

use minifb::{Key, WindowOptions, Window, KeyRepeat}; // For window
use std::env; // For args
use std::process;
use rustyturtle::{Orientation, Turtle, PixelBuffer, DrawMode};

const WIDTH: usize = 1280;
const HEIGHT: usize = 720;

fn main() {

    // Create turtle
    let mut turtle = Turtle::new((0, 0), Orientation::NORTH, true, 10, 0x00FFFFFF);

    // Construct pixel buffer
    let mut pixel_buffer = PixelBuffer::new(vec![0; WIDTH * HEIGHT], vec![0; (turtle.size * turtle.size) as usize]);

    // Create window
    let mut window = Window::new("rustyturtle", WIDTH, HEIGHT, WindowOptions::default()).unwrap_or_else(|e| {
        panic!("CRITICAL ERROR - {}", e);
    });

    // Collect file name
    let args: Vec<String> = env::args().collect();

    // Handle case where file not given in args
    if args.len() != 2 {
        println!("Note: No draw file specified");
    } else {

        let filename = &args[1];

        if let Err(e) = rustyturtle::process_file(filename.clone(), &mut turtle, &mut pixel_buffer) {
            println!("ERROR - {}", e);
            process::exit(1);
        }
    }

    while window.is_open() && !window.is_key_down(Key::Escape) {

        // Clear turtle indicator pos
        if turtle.pen_down {
            pixel_buffer.draw(&turtle, DrawMode::NORMAL);
        } else {
            pixel_buffer.draw(&turtle, DrawMode::PASSIVE);
        };

        if let Err(e) = rustyturtle::process_input(&mut window, &mut turtle) {
            println!("ERROR - {}", e);
        }

        if window.is_key_pressed(Key::Space, KeyRepeat::No) {
            turtle.pen_down = !turtle.pen_down;
        }

        pixel_buffer.draw(&turtle, DrawMode::ACTIVE);

        // We unwrap here as we want this code to exit if it fails. Real applications may want to handle this in a different way
        window.update_with_buffer(&pixel_buffer.full).unwrap();
    }
}