#[macro_use]
extern crate glium;

fn main() {
    println!("Welcome to opengl glium");

    use glium::glutin;

    // 1 . Creating an eventsLoop for handling window and device events
    let mut events_loop = glutin::EventsLoop::new();

    // 2 . Specify Window parameters using glium::glutin::WindowBuilder::new().
    //     These are window-specific attributes that have nothing to do with
    //     OpenGL.
    let window = glutin::WindowBuilder::new();

    // 3 . Specify Context parameters using glium::glutin::ContextBuilder::new().
    //     Here we specify OpenGL-specific attributes like multisampling or vsync.
    let context = glutin::ContextBuilder::new();

    // 4.  Create the OpenGL window (in glium, this is the Display):
    //     glium::Display::new(window, context, &events_loop).unwrap().
    //     This builds a Display using the given window and context attributes,
    //     and registers the window with the given events_loop.
    let display = glium::Display::new(window,context,&events_loop).unwrap();

    // 5. Keep the window open till the close event fires
    let mut closed = false;
    while !closed{
        // listing the events produced by application and waiting to be received
        events_loop.poll_events(|ev|{
            match ev {
                glutin::Event::WindowEvent {event, ..} => match event {
                    glutin::WindowEvent::CloseRequested => closed = true,
                    _ => (),
                },
                _ => (),
            }
        });
    }
}
