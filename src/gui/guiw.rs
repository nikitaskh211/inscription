extern crate glfw;

use glfw::{Context};

#[derive(Debug)]
pub struct GUIWindow {
    pub window: glfw::PWindow, // Public field
    pub events: glfw::GlfwReceiver<(f64, glfw::WindowEvent)>, // Public field
}

impl GUIWindow {
    pub fn new(width: u32, height: u32, title: &str) -> Self {
        let mut glfw = glfw::init(glfw::fail_on_errors).expect("Failed to initialize GLFW");

        let (mut window, events) = glfw
            .create_window(width, height, title, glfw::WindowMode::Windowed)
            .expect("Failed to create GLFW window");

        window.make_current();
        window.set_key_polling(true);

        GUIWindow { window, events }
    }

    pub fn poll_events(&mut self) {
        self.window.glfw.poll_events();
    }

    pub fn process_events(&mut self) {
        for (_, event) in glfw::flush_messages(&self.events) {
            println!("{:?}", event);
        }
    }

    pub fn should_close(&self) -> bool {
        self.window.should_close()
    }

    pub fn swap_buffers(&mut self) {
        self.window.swap_buffers();
    }
}
