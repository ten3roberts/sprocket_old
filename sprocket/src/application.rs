use crate::{event::Event, graphics};
use crate::{
    graphics::window::{Window, WindowMode},
    Time, Timer,
};
use graphics::vulkan::{renderer::Renderer, ResourceManager};
use log::{error, info};
use std::{
    sync::{mpsc, Arc},
    time,
};

pub struct Application {
    name: String,
    windows: Vec<Window>,
    event_receiver: mpsc::Receiver<Event>,
    event_sender: mpsc::Sender<Event>,
    renderer: Option<Renderer>,
    graphics_context: Option<graphics::GraphicsContext>,
    resource_manager: Option<Arc<ResourceManager>>,
    time: Time,
}

impl Application {
    /// Creates a new blank application with the given name
    pub fn new(name: &str) -> Application {
        let (event_sender, event_receiver) = mpsc::channel::<Event>();

        Window::init_glfw();
        Application {
            name: String::from(name),
            windows: Vec::new(),
            event_receiver,
            event_sender,
            graphics_context: None,
            resource_manager: None,
            renderer: None,
            time: Time::new(),
        }
    }

    pub fn init_graphics(&mut self) {
        self.graphics_context = match graphics::init(graphics::Api::Vulkan, &self.windows[0]) {
            Ok(context) => Some(context),
            Err(msg) => {
                error!("Failed to initialize graphics '{}'", msg);
                None
            }
        };

        // Create vulkan renderer if vulkan
        if let graphics::GraphicsContext::Vulkan(context) = self.graphics_context.as_ref().unwrap()
        {
            self.resource_manager = Some(Arc::new(ResourceManager::new(Arc::clone(context))));
            self.renderer = match Renderer::new(
                Arc::clone(context),
                &self.windows[0],
                Arc::clone(&self.resource_manager.as_ref().unwrap()),
            ) {
                Ok(renderer) => Some(renderer),
                Err(e) => {
                    error!("Failed to create renderer '{}'", e);
                    None
                }
            };
        } else {
        }
    }

    pub fn add_window(&mut self, title: &str, width: i32, height: i32, mode: WindowMode) {
        let window = Window::new(title, width, height, mode, self.event_sender.clone());
        self.windows.push(window);
    }

    pub fn run(&mut self) {
        let mut cleanup_timer = Timer::with_target(time::Duration::from_secs(2));
        let mut timer = Timer::with_target(time::Duration::from_secs(5));
        while !self.windows.is_empty() {
            if cleanup_timer.signaled() {
                self.resource_manager.as_ref().unwrap().cleanup(5); // Change to swapchain.image_count() in renderer system
                cleanup_timer.restart();
            }
            if timer.signaled() {
                info!(
                    "Frame: {}, elapsed: {}, delta: {}, fr: {}, us: {}",
                    self.time.framecount(),
                    self.time.elapsed_f32(),
                    self.time.delta_f32(),
                    self.time.framerate(),
                    self.time.delta_us(),
                );
                info!(
                    "Resources: {:?}",
                    self.resource_manager.as_ref().unwrap().info()
                );
                timer.restart();
            }
            // Process each window for events
            self.windows
                .iter()
                .for_each(|window| window.process_events());

            if let Some(renderer) = &mut self.renderer {
                renderer.draw_frame(&self.windows[0], &self.time);
            }

            // Receive and handle events
            while let Ok(event) = self.event_receiver.try_recv() {
                if let Event::MousePosition(_, _) = event {
                } else {
                    info!("Event: {:?}", event);
                }
            }
            self.windows.retain(|window| !window.should_close());
            self.time.update();
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }
}

impl Drop for Application {
    fn drop(&mut self) {
        self.resource_manager = None;
        Window::terminate_glfw();
    }
}
