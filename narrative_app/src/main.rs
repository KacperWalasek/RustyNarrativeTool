use glow::HasContext;
use glutin::{event::*, event_loop::*, platform::run_return::EventLoopExtRunReturn, *};

fn main() {
    let mut el = EventLoop::new();
    let builder = window::WindowBuilder::new()
        .with_title("Hello world!")
        .with_inner_size(dpi::LogicalSize::new(1024.0, 768.0));
    let wc = ContextBuilder::new().build_windowed(builder, &el).unwrap();
    let windowed_context = unsafe { wc.make_current() }.unwrap();
    let gl = unsafe {
        glow::Context::from_loader_function(|s| windowed_context.get_proc_address(s).cast())
    };
    el.run_return(|event, _, control_flow| match event {
        Event::WindowEvent {
            event: WindowEvent::CloseRequested,
            ..
        } => *control_flow = ControlFlow::Exit,
        Event::RedrawRequested(..) => {
            unsafe {
                gl.clear_color(0.1, 0.7, 1.0, 1.0);
                gl.clear(glow::COLOR_BUFFER_BIT);
            }
            windowed_context.swap_buffers().unwrap();
        }
        _ => (),
    });
}
