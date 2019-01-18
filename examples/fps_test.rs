extern crate piston_window;
use piston_window::*;

extern crate fps_controller;
use fps_controller::FPSController;

fn main() {
    let mut window: PistonWindow = WindowSettings::new("fps_test", [300, 200])
        .exit_on_esc(true)
        .build().unwrap();
    window.set_ups(0);

    let mut fps_controller = FPSController::from_fps(100);

    while let Some(e) = window.next() {
        fps_controller.tick();

        let fps = fps_controller.run(||{
            println!("aaa");
        });
        window.set_title(format!("{}", fps));

        window.draw_2d(&e, |c, g| {
            clear([1.0; 4], g);
            rectangle(
                [0.0, 0.0, 1.0, 1.0],
                [30.0, 30.0, 50.0, 70.0],
                c.transform,
                g
            );
        });
    }
}
