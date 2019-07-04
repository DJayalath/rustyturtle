extern crate minifb;

use minifb::{Key, WindowOptions, Window, KeyRepeat}; // For window
use std::fs; // For file reading
use std::env; // For args

const WIDTH: usize = 1280;
const HEIGHT: usize = 720;

fn main() {

    // Pixel buffer
    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];
    let mut last_colour = Vec::new();

    // Create window
    let mut window = Window::new("rustyturtle",
                                 WIDTH,
                                 HEIGHT,
                                 WindowOptions::default()).unwrap_or_else(|e| {
        panic!("{}", e);
    });

    // Create turtle
    let mut turtle = Turtle::new((0, 0), Orientation::NORTH, true, 10);

    // Collect file name
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!("File not detected!");
    } else {
        
        let filename = &args[1];

        // Read file if available
        let contents = fs::read_to_string(filename)
            .expect("Something went wrong reading the file");

        // Run instructions from file
        for instruction in contents.lines() {

            let command: Vec<&str> = instruction.split(" ").collect();

            for _ in 0..command[1].parse::<u32>().unwrap() {

                // Run instruction
                if let Err(e) = process_instr(&command[0], &mut turtle) {
                    println!("Application error: {}", e);
                }

                // Clear turtle indicator pos
                if turtle.pen_down {
                    draw(&mut buffer, turtle.pos, 0x00FFFFFF, turtle.size, turtle.size);
                } else {
                    draw_last(&mut buffer, turtle.pos, &last_colour, turtle.size, turtle.size);
                };
            }

        }
    }

    while window.is_open() && !window.is_key_down(Key::Escape) {

        // Clear turtle indicator pos
        if turtle.pen_down {
            draw(&mut buffer, turtle.pos, 0x00FFFFFF, turtle.size, turtle.size);
        } else {
            draw_last(&mut buffer, turtle.pos, &last_colour, turtle.size, turtle.size);
        };

        if let Err(e) = process_input(&mut window, &mut turtle) {
            println!("Application error: {}", e);
        }

        if window.is_key_pressed(Key::Space, KeyRepeat::No) {
            turtle.pen_down = !turtle.pen_down;
        }

        last_colour = draw(&mut buffer, turtle.pos, 0x0000FF00, turtle.size, turtle.size);

        // We unwrap here as we want this code to exit if it fails. Real applications may want to handle this in a different way
        window.update_with_buffer(&buffer).unwrap();
    }
}

enum Orientation {
    NORTH,
    SOUTH,
    EAST,
    WEST,
}

struct Turtle {
    pos: (i32, i32),
    direction: Orientation,
    pen_down: bool,
    size: u32,
}

impl Turtle {
    fn new(pos: (i32, i32), direction: Orientation, pen_down: bool, size: u32) -> Turtle {
        Turtle { pos, direction, pen_down, size }
    }
}

fn draw(buffer: &mut Vec<u32>, pos: (i32, i32), colour: u32, w: u32, h: u32) -> Vec<u32> {
    
    let mut index = pos.1 * WIDTH as i32 + pos.0;
    let mut last_colour: Vec<u32> = Vec::new();

    for _ in 0..h {
        for j in 0..w { 
            last_colour.push(buffer[index as usize + j as usize]);
            buffer[index as usize + j as usize] = colour;
        }
        index += WIDTH as i32;
    }

    last_colour
}

fn draw_last(buffer: &mut Vec<u32>, pos: (i32, i32), last_colour: &Vec<u32>, w: u32, h: u32) {
        
    let mut index = pos.1 * WIDTH as i32 + pos.0;
    let mut colour_index = 0;

    for _ in 0..h {
        for j in 0..w { 
            buffer[index as usize + j as usize] = last_colour[colour_index];
            colour_index += 1;
        }
        index += WIDTH as i32;
    }
}

fn process_input(window: &mut minifb::Window, turtle: &mut Turtle) -> Result<(), &'static str> {

    if window.is_key_down(Key::W) || window.is_key_down(Key::Up) {
        if turtle.pos.1 - 1 < 0 {
            return Err("Turtle out of range!");
        }
        turtle.pos.1 -= 1;
        turtle.direction = Orientation::NORTH;
    } else if window.is_key_down(Key::S) || window.is_key_down(Key::Down) {
        if turtle.pos.1 + 1 + turtle.size as i32 >= HEIGHT as i32 {
            return Err("Turtle out of range!");
        }
        turtle.pos.1 += 1;
        turtle.direction = Orientation::SOUTH;
    }
    if window.is_key_down(Key::A) || window.is_key_down(Key::Left) {
        if turtle.pos.0 - 1 < 0 {
            return Err("Turtle out of range!");
        }
        turtle.pos.0 -= 1;
        turtle.direction = Orientation::WEST;
    } else if window.is_key_down(Key::D) || window.is_key_down(Key::Right) {
        if turtle.pos.0 + 1 + turtle.size as i32 >= WIDTH as i32 {
            return Err("Turtle out of range!");
        }
        turtle.pos.0 += 1;
        turtle.direction = Orientation::EAST;
    }

    Ok(())
}

fn process_instr(instr: &str, turtle: &mut Turtle) -> Result<(), &'static str> {

    if instr == "NORTH" {
        if turtle.pos.1 - 1 < 0 {
            return Err("Turtle out of range!");
        }
        turtle.pos.1 -= 1;
        turtle.direction = Orientation::NORTH;
    } else if instr == "SOUTH" {
        if turtle.pos.1 + 1 + turtle.size as i32 >= HEIGHT as i32 {
            return Err("Turtle out of range!");
        }
        turtle.pos.1 += 1;
        turtle.direction = Orientation::SOUTH;
    }
    if instr == "WEST" {
        if turtle.pos.0 - 1 < 0 {
            return Err("Turtle out of range!");
        }
        turtle.pos.0 -= 1;
        turtle.direction = Orientation::WEST;
    } else if instr == "EAST" {
        if turtle.pos.0 + 1 + turtle.size as i32 >= WIDTH as i32 {
            return Err("Turtle out of range!");
        }
        turtle.pos.0 += 1;
        turtle.direction = Orientation::EAST;
    }

    Ok(())
}