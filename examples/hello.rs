use minigl::config::*;
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

    let framebuffer = ZBuffer::new(WIN_SIZE_X, WIN_SIZE_Y);
    mgl::init(framebuffer);
    //glTextSize(GL_TEXT_SIZE24x24);

    loop {
        handle_user_input(&mut event_pump);

        mgl::clear(MGLBit::COLOR).expect("error: MGL clear");
        // Draw a triangle
        mgl::matrix_mode(MGLMatrixMode::ModeModelView).expect("error: MGL matrix mode");
        mgl::load_identity().expect("error: MGL load identity");
        mgl::push_matrix().expect("error: MGL push matrix");
        let angle = 60.0;
        mgl::rotate(angle, 0.0, 0.0, 1.0).expect("error: MGL rotate");
        mgl::begin(MGLVertexMode::ModeTriangles).expect("error: MGL begin");
        mgl::color3i(0x33, 0x33, 0xff).expect("error: MGL color3i");
        mgl::vertex3f(-0.8, -0.8, 0.2).expect("error: MGL vertex3f");

        mgl::draw_text("Hello\n World!", 0, 0, MGLColor::RED).expect("error: MGL draw text");

        let screen = mgl::pbuffer().expect("error: MGL pbuffer");

        texture
            .update(None, &screen, WIN_PITCH)
            .expect("error: texture update");
        canvas
            .copy(&texture, None, None)
            .expect("error: canvas copy");
        canvas.present();
    }
}
