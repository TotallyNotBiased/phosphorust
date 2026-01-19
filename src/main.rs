use std::error::Error;
use std::num::NonZeroU32;
use std::rc::Rc;
use winit::application::ApplicationHandler;
use winit::event::WindowEvent;
use winit::event_loop::{self, ActiveEventLoop, EventLoop};
use winit::window::{Window, WindowId};

use winit::platform::x11::EventLoopBuilderExtX11;

struct App {
    window: Option<Rc<Window>>,
    surface: Option<softbuffer::Surface<Rc<Window>, Rc<Window>>>,
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let window_attributes = Window::default_attributes().with_title("phosphorust");
        let window = Rc::new(event_loop.create_window(window_attributes).unwrap());
        self.window = Some(window.clone());

        let context = softbuffer::Context::new(window.clone()).unwrap();
        let surface = softbuffer::Surface::new(&context, window.clone()).unwrap();
        self.surface = Some(surface);
        
        println!("Window created and surface attached.");
        
        window.request_redraw();
    }
    
    fn window_event(&mut self, event_loop: &ActiveEventLoop, id: WindowId, event: WindowEvent) {
        match event {
            WindowEvent::CloseRequested => {
                println!("The close button was pressed; stopping.");
                event_loop.exit();
            },
            WindowEvent::RedrawRequested => {
                if let (Some(window), Some(surface)) = (&self.window, &mut self.surface) {
                    let (width, height) = {
                        let size = window.inner_size();
                        (size.width, size.height)
                    };
                    
                    surface
                        .resize(
                            NonZeroU32::new(width).unwrap(),
                            NonZeroU32::new(height).unwrap(),
                        )
                        .unwrap();

                    let mut buffer = surface.buffer_mut().unwrap();

                    buffer.fill(0x6495ED); 

                    buffer.present().unwrap();
                }
            }
            _ => (),
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    env_logger::init();

    let event_loop = EventLoop::builder().with_x11().build()?;

    event_loop.set_control_flow(event_loop::ControlFlow::Poll);

    let mut app = App { window: None, surface: None, };
    event_loop.run_app(&mut app)?;

    Ok(())
}
