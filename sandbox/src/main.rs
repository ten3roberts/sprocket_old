use log::{debug, error, info, trace, warn};
use sprocket::*;
fn main() {
    let mut application = Application::new("Sandbox");
    info!("Created application {}", application.name());

    logger::init(log::LevelFilter::Trace);

    application.add_window("Sandbox", 800, 600, WindowMode::Windowed);
    application.run();

    info!("Terminating application");

    // let mut window =
    //     Window::new("Sandbox", 800, 600, WindowMode::FullScreen).expect("Failed to create window");

    // loop {
    //     thread::sleep(Duration::from_millis(500));
    //     window.poll_events();
    // }
}
