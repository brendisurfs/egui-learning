use three_d::{prelude::*, Camera, Gm, OrbitControl, Window, WindowSettings};
fn main() {
    let window = Window::new(WindowSettings {
        title: "Shapes!".to_string(),
        max_size: Some((1280, 720)),
        ..Default::default()
    })
    .unwrap();

    let context = window.gl();

    let mut camera = Camera::new_perspective(
        window.viewport(),
        vec3(5.0, 2.0, 2.5),
        vec3(0.0, 0.0, -0.5),
        vec3(0.0, 1.0, 0.0),
        degrees(45.0),
        0.1,
        1000.0,
    );

    let mut control = OrbitControl::new(*camera.target(), 1.0, 100.0);

    // let mut sphere = Gm::new();
}
