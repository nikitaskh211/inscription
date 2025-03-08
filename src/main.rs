mod gui;
use gui::guiw::GUIWindow;

fn main() {
    let mut gui_window = GUIWindow::new(800, 600, "GLFW Window");

    while !gui_window.should_close() {
        gui_window.poll_events();
        gui_window.process_events();
        gui_window.swap_buffers();
    }
}
