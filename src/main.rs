#![allow(non_snake_case)]
#[macro_use]
extern crate dotenv_codegen;

use ggez::{GameResult};
use ggez;
use ggez::event;
use ggez::conf;
use ggez::graphics;
use std::path;
use std::env;

mod state;
mod components;
mod systems;

use crate::state::MainState;


fn main() -> GameResult{
  let resource_dir = if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
    let mut path = path::PathBuf::from(manifest_dir);
    path.push("resources");
    path
  } else {
    path::PathBuf::from("./resources")
  };

  let cb = ggez::ContextBuilder::new("Last Day", "ggez").add_resource_path(resource_dir);
  let (ctx, event_loop) = &mut cb.build()?;

  //  TODO: move this to config
  // let screen_width = 2560.0;
  // let screen_height = 1080.0;
  let screen_width = 1500.0;
  let screen_height = 700.0;
  graphics::set_mode(ctx, conf::WindowMode::default().dimensions(screen_width, screen_height)).unwrap();
  graphics::set_screen_coordinates(ctx, graphics::Rect::new(0.0, 0.0, screen_width, screen_height)).unwrap();
  // ggez::graphics::set_fullscreen(ctx, conf::FullscreenType::Desktop)?;

  let font = graphics::Font::new(ctx, "/DejaVuSerif.ttf").unwrap();
  let state = &mut MainState::new(font);
  event::run(ctx, event_loop, state)?;
  Ok(())
}