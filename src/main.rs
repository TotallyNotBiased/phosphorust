pub mod math;
pub mod primitive;

use math::{Point3D, Ray, Point2D};
use primitive::Sphere;
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

use crate::primitive::Primitive;

impl Point2D {
    fn project_viewport(&self, viewport: &Viewport, canvas: &Canvas, distance: f64) -> Point3D {
        let vx = self.x * (viewport.width as f64 / canvas.width as f64);
        let vy = self.y * (viewport.height as f64 / canvas.height as f64);

        Point3D::new(vx, vy, distance)
    }
}

pub struct Scene {
    pub origin: Point3D,
    pub objects: Vec<Box<dyn Primitive>>,
    pub background_color: u32,
}

impl Scene {
    pub fn new() -> Self {
        Self { 
            origin: Point3D { x: 0.0, y: 0.0, z: 0.0 },
            objects: Vec::new(),
            background_color: 0,
        }
    }

    pub fn add(&mut self, object: Box<dyn Primitive>) {
        self.objects.push(object);
    }

    fn trace_ray(&self, o: Point3D, d: Point3D, distance: f64, viewrange: usize) -> u32 {
        let mut closest_t = viewrange as f64;
        let mut closest_object: Option<&Box<dyn Primitive>> = None;
        let ray = Ray { origin: self.origin, direction: (d - o).normalize() };
        for object in &self.objects {
            
            let t = object.intersect(&ray);

            match t {
                Some(t) => { 
                    if (distance <= t && t <= viewrange as f64) && t < closest_t {
                        closest_t = t;
                        closest_object = Some(object);
                    }
                }
                None => {
                }
            }
        }

        match closest_object {
            None => self.background_color,
            Some(object) => object.color(),
        }
    }
}

struct Canvas<'a> {
    buffer: &'a mut [u32], 
    width: u32,
    height: u32,
}

impl<'a> Canvas<'a> {
    fn put_pixel(&mut self, p: Point2D, color: u32) {
        let x_norm = (self.width / 2) as f64 + p.x;
        let y_norm = (self.height / 2) as f64 - p.y - 1.0;
        let index = ((self.width as f64 * y_norm) + x_norm) as usize;

        self.buffer[index] = color;
    }
}

struct Viewport {
    width: f64,
    height: f64,
}

struct App {
    window: Option<Rc<Window>>,
    surface: Option<softbuffer::Surface<Rc<Window>, Rc<Window>>>,
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let window_attributes = Window::default_attributes()
            .with_title("phosphorust")
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

                        let distance: f64 = 1.0; // normalize distance
                        let viewrange: usize = 100;

                        let viewport = Viewport { // and viewport extents
                            width: 1.0,
                            height: 1.0,
                        };

                        // draw pixels here
                        
                        let mut scene = Scene::new();
                        scene.add(Box::new(
                                Sphere::new(Point3D::new(0.0, -1.0, 3.0), 1.0, 0xFF0000)));
                        scene.add(Box::new(
                                Sphere::new(Point3D::new(2.0, 0.0, 4.0), 1.0, 0x0000FF)));
                        scene.add(Box::new(
                                Sphere::new(Point3D::new(-2.0, 0.0, 4.0), 1.0, 0x00FF00)));

                        let o = Point3D::new(0.0, 0.0, 0.0); // camera origin
                        
                        for x in -(canvas.width as i32/2)..(canvas.width as i32/2) {
                            for y in -(canvas.height as i32/2)..(canvas.height as i32/2) {
                                let viewport_point = Point2D { x: x as f64, y: y as f64 };
                                let d = viewport_point.project_viewport(&viewport, &canvas, distance);
                                let color = scene.trace_ray(o, d, distance, viewrange);
                                canvas.put_pixel(viewport_point, color);
                            }
                        }

                        // let p3 = Point3D::new(0.0, 0.0, 0.0); 

                        // let p2 = p3.project2d();
                        // canvas.put_pixel(p2, 0x6495ED);
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
