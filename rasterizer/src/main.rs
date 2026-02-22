pub mod paint;

use cg_common::math::{Point2D, ShadedVertex2};
use cg_common::canvas::Canvas;
use winit::dpi::LogicalSize;


use std::error::Error;
use std::num::NonZeroU32;
use std::rc::Rc;
use winit::application::ApplicationHandler;
use winit::event::WindowEvent;
use winit::event_loop::{self, ActiveEventLoop, EventLoop};
use winit::window::{Window, WindowId};

// comment out for wayland and change event_loop declaration in main()
use winit::platform::x11::EventLoopBuilderExtX11;

use crate::paint::{draw_filled_triangle, draw_wireframe_triangle, draw_shaded_triangle};

struct App {
    window: Option<Rc<Window>>,
    surface: Option<softbuffer::Surface<Rc<Window>, Rc<Window>>>,
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let window_attributes = Window::default_attributes()
            .with_title("phosphorust rasterizer")
            .with_inner_size(LogicalSize::new(600.0, 600.0));
        let window = Rc::new(event_loop.create_window(window_attributes).unwrap());
        self.window = Some(window.clone());

        let context = softbuffer::Context::new(window.clone()).unwrap();
        let surface = softbuffer::Surface::new(&context, window.clone()).unwrap();
        self.surface = Some(surface);
        
        println!("Window created and surface attached.");
        
        window.request_redraw();
    }
    
    fn window_event(&mut self, event_loop: &ActiveEventLoop, _id: WindowId, event: WindowEvent) {
        match event {
            WindowEvent::CloseRequested => {
                println!("CloseRequested event. Window closed.");
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
                    buffer.fill(0);
                    
                    { // lifetime bullshit 
                        let mut canvas = Canvas {
                            width: buffer.width().get(),
                            height: buffer.height().get(),
                            buffer: &mut buffer,
                        };
                        // let p0 = Point2D { x: 0.0, y: 0.0 };
                        // canvas.put_pixel(p0, 0x6495ED);

                        let p1 = Point2D {x: -200.0, y: -250.0};
                        let p2 = Point2D {x: 200.0, y: 50.0};
                        let p3 = Point2D {x: 20.0, y: 250.0};

                        let v0 = ShadedVertex2 {x: -200.0, y: -200.0, h: 0.2};
                        let v1 = ShadedVertex2 {x: -200.0, y: 250.0, h: 1.0};
                        let v2 = ShadedVertex2 {x: 0.0, y: 250.0, h: 0.5};
                        

                        draw_filled_triangle(p1, p2, p3, 0x123524, &mut canvas);
                        draw_wireframe_triangle(p1,p2,p3, 0x6495ED, &mut canvas);
                        draw_shaded_triangle(v0,v1,v2, 0x9b111e, &mut canvas);
                        

                    }

                    buffer.present().unwrap();
                }
            }
            _ => (),
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    env_logger::init();

    // Wayland isn't working in WSL2 so we force x11 here. Otherwise we can 
    // let event_loop = EventLoop::event_loop::new()?;
    let event_loop = EventLoop::builder().with_x11().build()?;

    event_loop.set_control_flow(event_loop::ControlFlow::Poll);

    let mut app = App { window: None, surface: None, };
    event_loop.run_app(&mut app)?;

    Ok(())
}
