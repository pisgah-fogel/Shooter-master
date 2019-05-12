//! Shooter-master game
//! Author djeck

extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;
extern crate rand;

use piston::window::WindowSettings;
use piston::event_loop::*;
use piston::input::*;
use glutin_window::GlutinWindow;
use opengl_graphics::{GlGraphics, OpenGL};

use std::collections::LinkedList;
use std::iter::FromIterator;

pub struct Player {
	gl : GlGraphics,
	width: u32,
	height: u32

}

pub struct Game {
	gl : GlGraphics,
	rows: u32,
	cols: u32,
	player: Player,
	square_width: u32,
	square_height: u32,
	score: u32
}

impl Game {
	fn render(&mut self, args: &RenderArgs) {
		use graphics;
		const GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];
		const WHITE: [f32; 4] = [1.0; 4];
		self.gl.draw(args.viewport(), |c, gl| {
			graphics::clear(GREEN, gl);
//			rectangle([1.0, 0.0, 0.0, 1.0], // red
//				[0.0, 0.0, 32.0, 32.0],
//				c.transform,
//				gl);
//				 Rectangle::new([0.5, 1.0, 0.0, 0.3])
//					.draw([50.0, 50.0, 100.0, 100.0], &draw_state, c.transform, g);
			});
		//self.player.render(args); // TODO remplace
		//self.element[i].render(&mut self.gl, args, self.square_width); // TODO
	}
	fn update(&mut self, args: &UpdateArgs) -> bool {
		true
	}
	fn pressed(&mut self, btn: &Button) {
		match btn {
			&Button::Keyboard(Key::Up) => println!("Key up"),
			&Button::Keyboard(Key::Down) => println!("Key down"),
			&Button::Keyboard(Key::Left) => println!("Key left"),
			&Button::Keyboard(Key::Right) => println!("Key right"),
			_ => println!("Other key")
			};
	}
}

fn main() {
	let opengl = OpenGL::V3_2; // OpenGL::V2_1

	const COLS: u32 = 30;
	const ROWS: u32 = 20;
	const SQUARE_WIDTH: u32 = 20;
	const SQUARE_HEIGHT: u32 = 20;

	let WIDTH = COLS * SQUARE_WIDTH;
	let HEIGHT = ROWS * SQUARE_HEIGHT;

	let mut window: GlutinWindow = WindowSettings::new("Shooter-master", [WIDTH, HEIGHT])
		.opengl(opengl)
		.exit_on_esc(true)
		.build()
		.unwrap();

	let mut game = Game {
		gl: GlGraphics::new(opengl),
		rows: ROWS,
		cols: COLS,
		square_width: SQUARE_WIDTH,
		square_height: SQUARE_HEIGHT,
		score: 0,
		player: Player {
			gl: GlGraphics::new(opengl),
			width: SQUARE_WIDTH,
			height: SQUARE_HEIGHT
			},
		};

	let mut events = Events::new(EventSettings::new()).ups(10);
	while let Some(e) = events.next(&mut window) {
		if let Some(r) = e.render_args() {
			game.render(&r);
		}

		if let Some(u) = e.update_args() {
			if !game.update(&u) {
				break;
			}
		}

		if let Some(k) = e.button_args() {
			if k.state == ButtonState::Press {
				game.pressed(&k.button);
			}
		}
	}
	
}
