use specs::{Builder, World, WorldExt, Dispatcher, DispatcherBuilder};
use ggez::{GameResult, Context};
use ggez::event::{self, Axis, Button, GamepadId, KeyCode, KeyMods, MouseButton};
use ggez::timer;
use ggez::graphics;
use ggez::nalgebra::Point2;

use crate::components::*;
use crate::systems::*;
// use components::*;

pub struct MainState{
	dispatcher: Dispatcher<'static, 'static>,
  world: World,
  font: graphics::Font
}

impl MainState{ 
	pub fn new(ctx: &mut Context, font: graphics::Font) -> MainState{
		let mut world = World::new();
		world.register::<Position>();
    // world.register::<Velocity>();
    // world.register::<Target>();
    world.register::<ControllerComponent>();
    world.register::<HumanViewComponent>();
    world.register::<RotationComponent>();
    // world.register::<ViewComponent<Player>>();

    world.create_entity()
      .with(Position{x: 100.0, y: 100.0})
      .with(RotationComponent(0.0))
      .with(HumanViewComponent::new(ctx))
      .with(ControllerComponent{
        movingLeft: false,
        movingRight: false,
        movingForward: false,
        movingBackward: false,
      })
      .build();

    let dispatcher = DispatcherBuilder::new()
      .with(ControllerSystem, "ControllerSystem", &[])
      // .with(LinearMovement, "LinearMovement", &[])
      // .with(ZombieSpawner, "ZombieSpawner", &[])
			.build();
    
		MainState {
			world,
      dispatcher,
      font
		}
	}
}

use specs::Join;
impl event::EventHandler for MainState {
  fn mouse_motion_event(&mut self, _ctx: &mut Context, _x: f32, _y: f32, _dx: f32, _dy: f32){
    let controllers = self.world.read_storage::<ControllerComponent>();
    let mut rotations = self.world.write_storage::<RotationComponent>();
    let positions = self.world.write_storage::<Position>();

    for (controller, rotation, position) in (&controllers, &mut rotations, &positions).join(){
      rotation.0 = (_y - position.y).atan2(_x - position.x);
    }
  }

  fn key_down_event( &mut self,
    ctx: &mut Context,
    keycode: KeyCode,
    keymod: KeyMods,
    repeat: bool){
      let mut controllers = self.world.write_storage::<ControllerComponent>();
      for controller in (&mut controllers).join(){
        let state = true;
        match keycode {
          KeyCode::D => controller.movingRight = state,
          KeyCode::A => controller.movingLeft = state,
          KeyCode::W => controller.movingForward = state,
          KeyCode::S => controller.movingBackward = state,
          _ => ()
        }
      }
  }

  fn key_up_event( &mut self,
    ctx: &mut Context,
    keycode: KeyCode,
    keymod: KeyMods){
      let mut controllers = self.world.write_storage::<ControllerComponent>();
      if keycode == KeyCode::Escape {
        event::quit(ctx);
      }

      for controller in (&mut controllers).join(){
        let state = false;
        match keycode {
          KeyCode::D => controller.movingRight = state,
          KeyCode::A => controller.movingLeft = state,
          KeyCode::W => controller.movingForward = state,
          KeyCode::S => controller.movingBackward = state,
          _ => ()
        }
      }
  }

	fn update(&mut self, _ctx: &mut Context) -> GameResult {
    self.dispatcher.dispatch(&mut self.world);
    // self.world.maintain();
		Ok(())
  }
  
	fn draw(&mut self, ctx: &mut Context) -> GameResult {
    

    let view_comp = self.world.read_storage::<HumanViewComponent>();
    let position_comp = self.world.read_storage::<Position>();
    let rotations = self.world.read_storage::<RotationComponent>();
    
    graphics::clear(ctx, graphics::BLACK);

    let count = (&view_comp, &position_comp).join().count();
    let offset = Point2::new(0.25, 0.5);
    for (view, position, rotation) in (&view_comp, &position_comp, &rotations).join() {
      let params = graphics::DrawParam::new();
      params.rotation(1.1);
      
      graphics::draw(
        ctx,
        &view.mesh,
        (
          Point2::new(position.x, position.y),
          rotation.0,
          offset,
          graphics::WHITE
        ),
      ).unwrap();
    }

    let dest_point = Point2::new(1.0, 10.0);
    let stext = format!("Entities: {}\nFPS: {}", count, timer::fps(ctx).floor());
    let counter_text = graphics::Text::new((stext, self.font, 48.0));
    graphics::draw(ctx, &counter_text, (dest_point,))?;
    graphics::present(ctx)?;
		Ok(())
	}
}