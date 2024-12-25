use three_d::*;

pub fn animate() {
    use log::info;

    // Create a window (a canvas on web)
    let window = Window::new(WindowSettings {
        title: "a cool rubik's cube. sorry you can't see it".to_string(),
        ..Default::default()
    })
        .unwrap();

    // Get the graphics context from the window
    let context = window.gl();

    // Create a camera
    let mut camera = Camera::new_perspective(
        window.viewport(),
        // the camera is at 0/0/2
        vec3(0.0, 0.0, 2.0),
        // we're looking at 0/0/0
        vec3(0.0, 0.0, 0.0),
        // "up" is y
        vec3(0.0, 1.0, 0.0),
        // FOV
        degrees(45.0),
        0.1,
        10.0,
    );

    let cube_mesh = Mesh::new(&context, &CpuMesh::cube());

    // Construct a model, with some material, thereby transferring the mesh data to the GPU
    let mut cube = Gm::new(
        cube_mesh,
        PhysicalMaterial::new_transparent(
            &context,
            &CpuMaterial {
                albedo: Srgba {
                    r: 0,
                    g: 0,
                    b: 255,
                    a: 100,
                },
                ..Default::default()
            },
        ),
    );
    cube.set_transformation(Mat4::from_translation(vec3(0.0, 0.0, 1.3)) * Mat4::from_scale(0.1));


    // Add an animation to the triangle.
    cube.set_animation(|time| Mat4::from_angle_y(radians(time * 0.005)));

    // Start the main render loop
    window.render_loop(
        move |frame_input| // Begin a new frame with an updated frame input
            {
                // Ensure the viewport matches the current window viewport which changes if the window is resized
                camera.set_viewport(frame_input.viewport);

                // Update the animation of the triangle
                cube.animate(frame_input.accumulated_time as f32);
                // info!("accumulated_time: {0}", frame_input.accumulated_time);

                // Get the screen render target to be able to render something on the screen
                frame_input.screen()
                    // Clear the color and depth of the screen render target
                    .clear(ClearState::color_and_depth(0.8, 0.8, 0.8, 1.0, 1.0))
                    // Render the triangle with the color material which uses the per vertex colors defined at construction
                    .render(
                        &camera, &cube, &[]
                    );

                // Returns default frame output to end the frame
                FrameOutput::default()
            },
    );
}