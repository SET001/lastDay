use ggez::{GameResult};
use ggez;
use ggez::event;
use ggez::graphics;
use ggez::nalgebra::Point2;

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

  let font = graphics::Font::new(ctx, "/DejaVuSerif.ttf").unwrap();
  let state = &mut MainState::new(ctx, font);
  event::run(ctx, event_loop, state)?;
  Ok(())
}