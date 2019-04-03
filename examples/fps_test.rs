// This example uses piston_window crate.

use piston_window::{
    PistonWindow,
    WindowSettings,
    EventLoop,
    AdvancedWindow,
    clear, rectangle,
};

use fps_controller::FPSController;

fn main() {
    let mut window: PistonWindow = WindowSettings::new("fps_test", [300, 200])
        .exit_on_esc(true)
        .build().unwrap();
    window.set_ups(0);

    let mut fps_controller = FPSController::new(); // same as FPSController::from_fps(60)

    while let Some(e) = window.next() {
        fps_controller.tick();

        window.draw_2d(&e, |c, g| {
            clear([1.0; 4], g);
            rectangle(
                [0.0, 0.0, 1.0, 1.0],
                [30.0, 30.0, 50.0, 70.0],
                c.transform,
                g
            );
        });

        let fps = fps_controller.run(|fc|{
            println!("delay! frame_count: {}", fc);
        });
        let fps_trunc = fps.trunc();
        let fps_fract = (fps.fract() * 100.0).trunc();
        window.set_title(format!("fps: {:02}.{}", fps_trunc, fps_fract));
    }
}
