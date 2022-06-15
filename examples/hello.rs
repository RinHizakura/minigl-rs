use minigl::config::PIXEL_BYTES;
use minigl::mgl;
use minigl::zbuffer::ZBuffer;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::PixelFormatEnum;
use sdl2::EventPump;

fn handle_user_input(event_pump: &mut EventPump) {
    for event in event_pump.poll_iter() {
        match event {
            Event::Quit { .. }
            | Event::KeyDown {
                keycode: Some(Keycode::Escape),
                ..
            } => std::process::exit(0),
            _ => { /* do nothing */ }
        }
    }
}

pub const WIN_SIZE_X: usize = 640;
pub const WIN_SIZE_Y: usize = 480;
pub const WIN_PITCH: usize = WIN_SIZE_X * PIXEL_BYTES;

fn main() {
    println!("Hello");

    let screen = [0 as u8; WIN_PITCH * WIN_SIZE_Y];
    /* Init SDL */
    let sdl_context = sdl2::init().expect("error: SDL init");
    let video_subsystem = sdl_context.video().expect("error: SDL video");
    let mut event_pump = sdl_context.event_pump().expect("error: event_pump");

    let window = video_subsystem
        .window("hello", WIN_SIZE_X as u32, WIN_SIZE_Y as u32)
        .position_centered()
        .build()
        .expect("error: SDL window");

    let mut canvas = window
        .into_canvas()
        .build()
        .expect("error: SDL window canvas.");

    let creator = canvas.texture_creator();
    let mut texture = creator
        .create_texture_target(
            PixelFormatEnum::ARGB32,
            WIN_SIZE_X as u32,
            WIN_SIZE_Y as u32,
        )
        .expect("error: SDL texture");

    let change = false;

    let framebuffer = ZBuffer::new(WIN_SIZE_X, WIN_SIZE_Y);
    mgl::init(framebuffer);
    mgl::clear(0x10);
    //glTextSize(GL_TEXT_SIZE24x24);
    //glDrawText("Hello World!\nFrom TinyGL", 0, 0, 0x00FFFFFF);

    loop {
        handle_user_input(&mut event_pump);

        if change == true {
            texture
                .update(None, &screen, WIN_PITCH)
                .expect("error: texture update");
            canvas
                .copy(&texture, None, None)
                .expect("error: canvas copy");
            canvas.present();
        }
    }
}
