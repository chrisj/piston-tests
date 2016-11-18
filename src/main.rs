extern crate time;

extern crate piston;
extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;

use piston::window::WindowSettings;
use piston::event_loop::*;
use piston::input::*;
use glutin_window::GlutinWindow as Window;
use opengl_graphics::{ GlGraphics, OpenGL };

static mut lastTime: f64 = 0.0;

pub struct App {
    mouse_x: f64,
    mouse_y: f64,
    gl: GlGraphics, // OpenGL drawing backend.
    position: f64,
    rotation: f64   // Rotation for the square.
}

impl App {
    fn render(&mut self, args: &RenderArgs) {

        // println!("pressed: {}", args.ext_dt);
        use graphics::*;

        const GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];
        const RED:   [f32; 4] = [1.0, 0.0, 0.0, 1.0];

        let square = rectangle::square(0.0, 0.0, 50.0);
        let rotation = self.rotation;
        let position = self.position;
        // println!("val {}", 500.0 * args.ext_dt);

        let (x, y) = ((position + 500.0 * args.ext_dt) % args.width as f64,
                      (args.height / 2) as f64);

        let cursor = rectangle::centered_square(self.mouse_x, self.mouse_y, 2.0);

        self.gl.draw(args.viewport(), |c, gl| {
            // Clear the screen.
            clear(GREEN, gl);

            rectangle(RED, cursor, c.transform, gl);

            let transform = c.transform.trans(x, y)
                                       .rot_rad(rotation)
                                       .trans(-25.0, -25.0);

            // Draw a box rotating around the middle of the screen.
            rectangle(RED, square, transform, gl);
        });
    }

    fn update(&mut self, args: &UpdateArgs) {
        // Rotate 2 radians per second.
        self.position += 500.0 * args.dt;
        // self.rotation += 10.0 * args.dt;
       
        // println!("dt: {}", args.dt);
    }
}

fn main() {
    // Change this to OpenGL::V2_1 if not working.
    let opengl = OpenGL::V3_2;

    // Create an Glutin window.
    let mut window: Window = WindowSettings::new(
            "spinning-square",
            [500, 500]
        )
        .opengl(opengl)
        .vsync(true)
        .fullscreen(false)
        .resizable(false)
        .decorated(false)
        .exit_on_esc(true)
        .build()
        .unwrap();

    // Create a new game and run it.
    let mut app = App {
        mouse_x: 0.0,
        mouse_y: 0.0,
        gl: GlGraphics::new(opengl),
        rotation: 0.0,
        position: 0.0,
    };

    let mut events = window.events().ups(120);
    while let Some(e) = events.next(&mut window) {
        if let Some(r) = e.render_args() {
            unsafe {
                let diff = time::precise_time_s() - lastTime;
                if diff > 0.017 {
                    println!("t {}", time::precise_time_s() - lastTime);
                }
                lastTime = time::precise_time_s();
            }
            app.render(&r);
        }

        if let Some(u) = e.update_args() {
            app.update(&u);
        }
        
        if let Some(pos) = e.mouse_cursor_args() {
            app.mouse_x = pos[0];
            app.mouse_y = pos[1];
        }

        if let Some(button) = e.press_args() {

            match button {
                Button::Keyboard(key) => {
                    println!("pressed: {:?}", key);
                }
                _ => {}
            }
        }
    }
}

