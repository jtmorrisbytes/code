pub fn main() {
    use glfw::fail_on_errors;
    let mut glfw = glfw::init(glfw::fail_on_errors!()).unwrap();
    glfw.window_hint(glfw::WindowHint::OpenGlDebugContext(true));
    glfw.window_hint(glfw::WindowHint::ContextVersion(3, 0));
    glfw.window_hint(glfw::WindowHint::OpenGlForwardCompat(true));
    let (mut window, events) = glfw
        .create_window(800, 600, "a game window", glfw::WindowMode::Windowed)
        .unwrap();
    window.make_current();
    window.set_key_polling(true);
    window.set_cursor_pos_polling(true);
    gl::load_with(|name| glfw.get_proc_address_raw(name));
    let mut game = Game::initialize();
    window.set_framebuffer_size_callback(|_window, width, height| {
        game::opengl::gl_viewport(0, 0, width, height);
    });

    while !window.should_close() {
        glfw.poll_events();
        for (_, event) in glfw::flush_messages(&events) {
            game.process_window_event(&event);
            match event {
                glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
                    window.set_should_close(true)
                }
                _ => {}
            }
        }
        game.update();
        game.render();
        window.swap_buffers();
    }
}
use game::Game;
use glfw::{Action, Context, Key};
