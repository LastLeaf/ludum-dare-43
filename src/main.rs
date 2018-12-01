#[macro_use]
extern crate glayout;

use std::time::Duration;

mod resource;
mod cover;
mod level;
mod level_config;

extern {
    pub fn play_audio(id: i32);
    pub fn hide_loading();
}

fn loading() {
    let mut canvas = glayout::canvas::Canvas::new(0);
    canvas.ctx(|ctx| {
        ctx.set_clear_color(0.2, 0.2, 0.2, 1.);
        let pixel_ratio = ctx.device_pixel_ratio();
        ctx.set_canvas_size(1280, 720, pixel_ratio);
    });

    // load resource
    let mut loader = {
        let context = canvas.context();
        let mut ctx = context.borrow_mut();
        resource::ResourceLoader::new(ctx.canvas_config())
    };
    loader.wait(Duration::new(5, 0));
    loader.load_image("empty", "resource/empty.png");
    loader.load_image("unreachable", "resource/tree.png");
    loader.load_image("flower", "resource/home.png");
    loader.load_image("witch", "resource/witch.png");
    loader.load_image("monster", "resource/wolf.png");
    loader.load_image("blue", "resource/blue.png");
    loader.load_image("red", "resource/red.png");
    loader.load_image("red_with_teeth", "resource/red_with_teeth.png");
    loader.load_image("red_with_skin", "resource/red_with_skin.png");
    loader.load_image("red_with_teeth_skin", "resource/red_with_teeth_skin.png");
    resource::ResourceLoader::ended(loader, move |resource| {
        cover::Cover::show(canvas.context(), resource, "red");
    });
}

fn main() {
    glayout::init();
    glayout::set_log_level_num(-1);
    glayout::main_loop(loading);
}
