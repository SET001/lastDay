use specs::{Builder, World, WorldExt, Dispatcher, DispatcherBuilder };
use ggez::{GameResult, Context};
use ggez::event::{self, KeyCode, KeyMods, MouseButton};
use ggez::timer;
use ggez::graphics;
use ggez::nalgebra::{Point2, Vector2};
use dotenv::dotenv;
use std::env;

use crate::components::*;
use crate::systems::*;

pub struct MainState{
	dispatcher: Dispatcher<'static, 'static>,
  world: World,
  font: graphics::Font
}

impl MainState{ 
	pub fn new(font: graphics::Font) -> MainState{
    dotenv().ok();
		let mut world = World::new();

    world.register::<ViewComponent>();    

    let mut dispatcher = DispatcherBuilder::new()
      .with(ControllerSystem, "ControllerSystem", &[])
      .with(ShooterSystem, "ShooterSystem", &[])
      .with(LinearMovement, "LinearMovement", &[])
      .with(CollisionSystem, "CollisionSystem", &[])
      .with(ZombieSpawner, "ZombieSpawner", &[])
      .with(OutOfScreenRemover, "OutOfScreenRemover", &[])
      .with(DamageOnCollide, "DamageOnCollide", &["CollisionSystem"])
      .with(TargetOnFraction, "TargetOnFraction", &[])
      .with(FollowTarget, "FollowTarget", &[])
      .build();
    
    dispatcher.setup(&mut world);

    world.create_entity()
    .with(Position{x: 400.0, y: 300.0})
    .with(ZombieSpawnerComponent{
      radius: 300.0,
      spawnRate: dotenv!("zombieSpawner.spawnRate").parse::<f32>().unwrap(),
      cooldown: 0.0,
      spawnInitially: dotenv!("zombieSpawner.spawnInitially").parse::<isize>().unwrap(),
      initiallySpawned: false
    })
    .build();

  world.create_entity()
    .with(Position{x: 400.0, y: 300.0})
    .with(RotationComponent(0.0))
    .with(ViewComponent::new (Views::Human))
    .with(FractionableComponent(Fractions::Humans))
    .with(ControllerComponent{
      movingLeft: false,
      movingRight: false,
      movingForward: false,
      movingBackward: false,
      isFiring: false,
    })
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
  fn mouse_button_up_event(&mut self,_ctx: &mut Context,_button: MouseButton,_x: f32,_y: f32,){
    let mut controllers = self.world.write_storage::<ControllerComponent>();
    for controller in (&mut controllers).join(){
      controller.isFiring = false;
    }
  }

  fn mouse_button_down_event(&mut self,_ctx: &mut Context,_button: MouseButton,_x: f32,_y: f32,){
    let mut controllers = self.world.write_storage::<ControllerComponent>();
    for controller in (&mut controllers).join(){
      controller.isFiring = true;
    }
  }

  fn mouse_motion_event(&mut self, _ctx: &mut Context, _x: f32, _y: f32, _dx: f32, _dy: f32){
    let controllers = self.world.read_storage::<ControllerComponent>();
    let mut rotations = self.world.write_storage::<RotationComponent>();
    let positions = self.world.write_storage::<Position>();

    for (_controller, rotation, position) in (&controllers, &mut rotations, &positions).join(){
      rotation.0 = (_y - position.y).atan2(_x - position.x);
    }
  }

  fn key_down_event( &mut self,
    _ctx: &mut Context,
    keycode: KeyCode,
    _keymod: KeyMods,
    _repeat: bool){
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
    _keymod: KeyMods){
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

	fn update(&mut self, ctx: &mut Context) -> GameResult {
    
    self.world.maintain();
    self.dispatcher.dispatch(&mut self.world);
    let mut view_comp = self.world.write_storage::<ViewComponent>();
    for view in (&mut view_comp).join() {
      if view.meshes.len() < 1 {
        match view.viewType{
          Views::Human => {
            let image = graphics::Image::new(ctx, "/player.png").unwrap();
            view.meshes.push(Box::new(image));

          },
          Views::Zombie => {
            let image = graphics::Image::new(ctx, "/zombie.png").unwrap();
            view.meshes.push(Box::new(image));
          },
          Views::Bullet => {
            let circle = graphics::Mesh::new_circle(
              ctx,
              graphics::DrawMode::fill(),
              Point2::new(0.0, 0.0),
              5.0,
              0.1,
              graphics::WHITE,
            ).unwrap();
            view.meshes.push(Box::new(circle));
          }
        }
      }
    }
    
		Ok(())
  }
  
	fn draw(&mut self, ctx: &mut Context) -> GameResult {
    
    let count = self.world.entities().join().count();

    let view_comp = self.world.read_storage::<ViewComponent>();
    let position_comp = self.world.read_storage::<Position>();
    let rotations = self.world.read_storage::<RotationComponent>();
   
    graphics::clear(ctx, graphics::BLACK);
    let scale = 0.3;
    for (view, position, rotation) in (&view_comp, &position_comp, &rotations).join() {
      let params = graphics::DrawParam::new()
        .rotation(rotation.0)
        .dest(Point2::new(
          position.x, position.y
        ))
        .scale(Vector2::new(scale, scale));
        //.offset(Point2::new(0.25, 0.5));    //  todo remove this
      for mesh in &view.meshes{
        mesh.draw(ctx, params).unwrap();
        if cfg!(feature="showDebugMeshes")  {
          //  draw debug mesh rectangle
          let dim = mesh. dimensions(ctx).unwrap();
          let width = dim.w;
          let height = dim.h;
          let debugRect = graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::stroke(5.0),
            graphics::Rect::new(0.0, 0.0, width-1.0, height-1.0),
            graphics::WHITE
          ).unwrap();
          graphics::draw(
            ctx, &debugRect, params
          ).unwrap();
          //  draw position marker
          let params = graphics::DrawParam::new()
            .dest(Point2::new(
              position.x, position.y
            ))
            .scale(Vector2::new(scale, scale));
          let circle = graphics::Mesh::new_circle(
            ctx,
            graphics::DrawMode::stroke(5.0),
            Point2::new(0.0, 0.0),
            4.0,
            0.1,
            graphics::Color{
              r: 0.0,
              g: 288.0,
              b: 0.0,
              a: 1.0
            },
          ).unwrap();
          graphics::draw(
            ctx, &circle, params
          ).unwrap();
        }
      }
    }
    if cfg!(feature="showDebugMeshes") {
      let collisions = self.world.read_storage::<CollisionComponent>();
      let shooters = self.world.read_storage::<ShooterComponent>();
      //  draw debug collision circle
      for (collision, position) in (&collisions, &position_comp).join() {
        let params = graphics::DrawParam::new()
          .dest(Point2::new(
            position.x, position.y
          ))
          .scale(Vector2::new(scale, scale));
        let circle = graphics::Mesh::new_circle(
          ctx,
          graphics::DrawMode::stroke(5.0),
          Point2::new(collision.center.x, collision.center.y),
          collision.radius,
          0.1,
          graphics::Color{
            r: 255.0,
            g: 0.0,
            b: 0.0,
            a: 1.0
          },
        ).unwrap();
        graphics::draw(
          ctx, &circle, params
        ).unwrap();
      }
      //  draw debug shooter origin
      for (shooter, position, rotation) in (&shooters, &position_comp, &rotations).join() {
        let params = graphics::DrawParam::new()
        .dest(Point2::new(
          position.x,
          position.y
        ))
        .rotation(rotation.0)
        .scale(Vector2::new(scale, scale));
        let circle = graphics::Mesh::new_circle(
          ctx,
          graphics::DrawMode::stroke(5.0),
          Point2::new(
            shooter.originOffset.x,
            shooter.originOffset.y
          ),
          5.0,
          0.1,
          graphics::WHITE,
        ).unwrap();
        graphics::draw(
          ctx, &circle, params
        ).unwrap();
      }
    }
    

    let dest_point = Point2::new(1.0, 10.0);
    let stext = format!("Entities: {}\nFPS: {}", count, timer::fps(ctx).floor());
    let counter_text = graphics::Text::new((stext, self.font, 48.0));
    graphics::draw(ctx, &counter_text, (dest_point,))?;
    graphics::present(ctx)?;
		Ok(())
	}
}