pub mod math;
pub mod primitive;

use math::{Point3D, Vector3, Ray, Point2D};
use primitive::Sphere;

use std::error::Error;
use std::num::NonZeroU32;
use std::rc::Rc;
use winit::application::ApplicationHandler;
use winit::event::WindowEvent;
use winit::event_loop::{self, ActiveEventLoop, EventLoop};
use winit::window::{Window, WindowId};

// comment out for wayland and change event_loop declaration in main()
use winit::platform::x11::EventLoopBuilderExtX11;

struct Canvas<'a> {
    buffer: &'a mut [u32], 
    width: u32,
    height: u32,
}

impl Point2D {
    fn project_viewport(&self, viewport: Viewport, canvas: Canvas, distance: f64) -> Point3D {
        let vx = self.x * (viewport.width as f64 / canvas.width as f64);
        let vy = self.y * (viewport.height as f64 / canvas.height as f64);

        Point3D::new(vx, vy, distance)
    }
}

impl<'a> Canvas<'a> {
    fn put_pixel(&mut self, p: Point2D, color: u32) {
        let x_norm = (self.width / 2) as f64 + p.x;
        let y_norm = (self.height / 2) as f64 - p.y;
        let index = ((self.width as f64 * y_norm) + x_norm) as usize;

        self.buffer[index] = color;
    }
}

struct Viewport {
    width: u32,
    height: u32,
}

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

                        // draw pixels here
                        
                        for x in -(canvas.width/2)..(canvas.width/2) {
                            for y in -(canvas.height/2)..(canvas.height/2) {
                                let d = Point3D.ne
 

                        let p3 = Point3D::new(0.0, 0.0, 0.0);

                        let p2 = p3.project2d();
                        canvas.put_pixel(p2, 0x6495ED);
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
