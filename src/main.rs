mod logger;
mod rndr;
mod vk_debug;
mod vk_init_core;
mod vk_init_rndr;
mod vk_models;
mod vk_utils;
mod mesh;
mod memory_guard;

use std::sync::atomic::{AtomicBool, AtomicU32, Ordering};
use winit::raw_window_handle::{HasDisplayHandle, HasWindowHandle};
use winit::window::{Window, WindowAttributes};
use winit::{
    application::ApplicationHandler,
    event::{ElementState, KeyEvent, WindowEvent},
    event_loop::{ActiveEventLoop, ControlFlow, EventLoop},
    keyboard::{Key, NamedKey},
    window::WindowId,
};

use crate::logger::logger_init;
use crate::mesh::Mesh;
use crate::rndr::Rndr;

const APP_NAME: &str = "Ash Triangle";

struct AppWindow {
    pub width: AtomicU32,
    pub height: AtomicU32,
    pub is_resized: AtomicBool,
}

static APP_WINDOW: AppWindow = AppWindow {
    width: AtomicU32::new(1920),
    height: AtomicU32::new(1080),
    is_resized: AtomicBool::new(false),
};

struct App {
    window: Option<Window>,
    rndr: Option<Rndr>,
    mesh: Option<Mesh>,
}

impl App {
    fn new() -> Self {
        Self {
            window: None,
            rndr: None,
            mesh: None,
        }
    }

    fn window(&self) -> &Window {
        self.window.as_ref().expect("window not created yet")
    }

    fn shutdown(&mut self) {
        if let (Some(mut mesh), Some(rndr)) = (self.mesh.take(), self.rndr.take()) {
            mesh.destroy(&rndr.device, &rndr.allocator);
            rndr.destroy();
        }
    }
}

impl Drop for App {
    fn drop(&mut self) {
        self.shutdown();
    }
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if self.window.is_none() {
            let window_width = APP_WINDOW.width.load(Ordering::Relaxed);
            let window_height = APP_WINDOW.height.load(Ordering::Relaxed);
            
            match event_loop.create_window(WindowAttributes::default().with_title(APP_NAME).with_inner_size(
                winit::dpi::LogicalSize::new(f64::from(window_width), f64::from(window_height)),
            )) {
                Ok(t) => self.window = Some(t),
                Err(e) => {
                    eprintln!("Failed to create window: {e}");
                    event_loop.exit();
                }
            }
        }

        if self.rndr.is_none() {
            let window_handle = self.window().window_handle().unwrap().as_raw();
            let display_handle = self.window().display_handle().unwrap().as_raw();

            let rndr = Rndr::new(window_handle, display_handle);

            let mesh = Mesh::new(
                &rndr.device,
                &rndr.allocator,
                &rndr.queue,
                &rndr.command_pool,
            );

            self.mesh = Some(mesh);
            self.rndr = Some(rndr);
        }
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _window_id: WindowId, event: WindowEvent) {
        match event {
            WindowEvent::CloseRequested | WindowEvent::KeyboardInput {
                event:
                    KeyEvent {
                        state: ElementState::Pressed,
                        logical_key: Key::Named(NamedKey::Escape),
                        ..
                    },
                ..
            } => {
                self.shutdown();
                event_loop.exit();
            },
            WindowEvent::Resized(size) => {
                APP_WINDOW.width.store(size.width, Ordering::Relaxed);
                APP_WINDOW.height.store(size.height, Ordering::Relaxed);
                APP_WINDOW.is_resized.store(true, Ordering::Relaxed);
            },
            _ => {},
        }
    }

    fn about_to_wait(&mut self, _event_loop: &ActiveEventLoop) {
        if let (Some(rndr), Some(mesh)) = (&mut self.rndr, &self.mesh) {
            rndr.render_frame(mesh);
        }
    }
}

fn main() {
    logger_init();

    let event_loop = EventLoop::new().unwrap();
    event_loop.set_control_flow(ControlFlow::Poll);

    let mut app = App::new();
    event_loop.run_app(&mut app).unwrap();
}
