mod graphics;
mod math;
mod scenes;
mod ui;

use glfw::Context;
use std::cell::RefCell;
use std::sync::mpsc::Receiver;
use std::rc::Rc;

pub struct Game {
    window: Rc<RefCell<Window>>,
    scene_manager: SceneManager,
}

impl Game {
    pub fn new() -> Game {
        let window = Rc::new(RefCell::new(Window::new(1280, 720, "Wars")));

        let scene_manager = SceneManager::new(Box::new(scenes::MenuScene::new(window.clone())));

        window.borrow_mut().show();

        return Game {
            window,
            scene_manager,
        };
    }

    pub fn is_running(&self) -> bool {
        return self.window.borrow().is_open();
    }

    pub fn update(&mut self) {
        unsafe { gl::Clear(gl::COLOR_BUFFER_BIT) };
        
        self.scene_manager.update_active();
        self.scene_manager.render_active();

        self.window.borrow_mut().update();
    }
}

pub struct Window {
    glfw: glfw::Glfw,
    window: glfw::Window,
    _events: Receiver<(f64, glfw::WindowEvent)>,
}

extern "system" fn opengl_debug_callback(
    _source: u32,
    _type: u32,
    _id: u32,
    _severity: u32,
    _length: i32,
    message: *const i8,
    _: *mut std::ffi::c_void,
) {
    let message = unsafe {
        std::ffi::CStr::from_ptr(message)
    }.to_bytes().to_vec();
    
    println!("[OPENGL]: {}", String::from_utf8(message).unwrap());
}

impl Window {
    fn new(width: u32, height: u32, title: &str) -> Window {
        let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).expect("Failed to initialize GLFW!");

        glfw.window_hint(glfw::WindowHint::ContextVersion(4, 3));
        glfw.window_hint(glfw::WindowHint::OpenGlProfile(
            glfw::OpenGlProfileHint::Core,
        ));
        glfw.window_hint(glfw::WindowHint::Visible(false));
        glfw.window_hint(glfw::WindowHint::Resizable(false));

        glfw.window_hint(glfw::WindowHint::OpenGlDebugContext(true));

        let (mut window, events) = glfw
            .create_window(width, height, title, glfw::WindowMode::Windowed)
            .expect("Failed to create the Window!");

        window.make_current();
        window.set_key_polling(true);

        gl::load_with(|procname| glfw.get_proc_address_raw(procname));

        unsafe {
            gl::Enable(gl::DEBUG_OUTPUT);
            gl::Enable(gl::DEBUG_OUTPUT_SYNCHRONOUS);

            gl::DebugMessageCallback(Some(opengl_debug_callback), std::ptr::null());
        }

        glfw.with_primary_monitor(|_, m| {
            let video_mode = m.unwrap().get_video_mode().unwrap();

            let new_x: u32 = (video_mode.width - width) / 2;
            let new_y: u32 = (video_mode.height - height) / 2;

            window.set_pos(new_x.try_into().unwrap(), new_y.try_into().unwrap());
        });

        return Window {
            glfw,
            window,
            _events: events,
        };
    }

    fn show(&mut self) {
        self.window.show();
    }

    fn is_open(&self) -> bool {
        return !(self.window.should_close());
    }

    fn update(&mut self) {
        self.window.swap_buffers();
        self.glfw.poll_events();
    }

    pub fn _is_key_down(&self, key: glfw::Key) -> bool {
        let action = self.window.get_key(key);
        return action == glfw::Action::Press || action == glfw::Action::Repeat;
    }

    pub fn _is_mouse_button_down(&self, button: glfw::MouseButton) -> bool {
        let action = self.window.get_mouse_button(button);
        return action == glfw::Action::Press || action == glfw::Action::Repeat;
    }
    
    pub fn get_mouse_position(&self) -> (f64, f64) {
        self.window.get_cursor_pos()
    }
}

struct SceneManager {
    active_scene: Box<dyn Scene>,
}

impl SceneManager {
    fn new(initial_scene: Box<dyn Scene>) -> SceneManager {
        return SceneManager {
            active_scene: initial_scene,
        };
    }

    fn _set_active(&mut self, new_active_scene: Box<dyn Scene>) {
        self.active_scene = new_active_scene;
    }

    fn update_active(&mut self) {
        self.active_scene.update();
    }

    fn render_active(&mut self) {
        self.active_scene.render();
    }
}

pub trait Scene {
    fn update(&mut self);

    fn render(&mut self);
}
