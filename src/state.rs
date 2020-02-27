use specs::{Builder, World, WorldExt, Dispatcher, DispatcherBuilder, RunNow};
use ggez::{GameResult, Context};
use ggez::event::{self, KeyCode, KeyMods, MouseButton};
use ggez::timer;
use ggez::graphics;
use ggez::nalgebra::{Point2, Vector2};
use std::f32::consts::PI;
use nalgebra_glm::Vec2;

use dotenv::dotenv;
use ggez_goodies::camera::*;

use crate::components::*;
use crate::systems::*;

use crate::prefab::*;

const WINDOW_WIDTH: u32 = 1500;
const WINDOW_HEIGHT: u32 = 700;

const CAMERA_WIDTH: f32 = 1500.0;
const CAMERA_HEIGHT: f32 = 700.0;

pub struct MainState{
	dispatcher: Dispatcher<'static, 'static>,
  world: World,
  font: graphics::Font,
  camera: Camera,
  rotation: f32,
  controller: ControllerComponent,
}

impl MainState{ 
	pub fn new(font: graphics::Font) -> MainState{
    dotenv().ok();
		let mut world = World::new();
    

    world.register::<ViewComponent>();
    world.register::<CollisionComponent>();
    world.register::<RotationComponent>();
    world.register::<Position>();

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
    
    let mut init = InitSystem;
    init.run_now(&world);
    
    dispatcher.setup(&mut world);
 
    let camera = Camera::new(WINDOW_WIDTH, WINDOW_HEIGHT, CAMERA_WIDTH, CAMERA_HEIGHT);


		MainState {
			world,
      dispatcher,
      font,
      camera,
      controller: ControllerComponent::new(),
      rotation: 0.0,
		}
	}
}

fn setUpController(controller: &mut ControllerComponent, state: bool, keycode: KeyCode){
  match keycode {
    KeyCode::D => controller.movingRight = state,
    KeyCode::A => controller.movingLeft = state,
    KeyCode::W => controller.movingForward = state,
    KeyCode::S => controller.movingBackward = state,
    _ => ()
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
      let (cx, cy) = self.camera.world_to_screen_coords(Vec2::new(position.x as f32, position.y as f32));
      rotation.0 = (_y-cy as f32).atan2(_x-cx as f32);
      // println!("{:?}, rotation: {}, mouse: {}, {}, camera: {}, {}", position, rotation.0, _x, _y, cx, cy);
    }
  }

  fn key_down_event( &mut self,
    _ctx: &mut Context,
    keycode: KeyCode,
    _keymod: KeyMods,
    _repeat: bool){
      setUpController(&mut self.controller, true, keycode);
  }

  fn key_up_event( &mut self,
    ctx: &mut Context,
    keycode: KeyCode,
    _keymod: KeyMods){
      if keycode == KeyCode::Escape {
        event::quit(ctx);
      }
      setUpController(&mut self.controller, false, keycode);
  }

	fn update(&mut self, ctx: &mut Context) -> GameResult {
    self.dispatcher.dispatch(&mut self.world);
    self.world.maintain();
    let speed = 2.0;
    
    let controllers = self.world.read_storage::<ControllerComponent>();
    let mut positions = self.world.write_storage::<Position>();
    let rotations = self.world.read_storage::<RotationComponent>();
    
    for (_controller, position, rotation) in (&controllers, &mut positions, &rotations).join(){
      if self.controller.movingForward {
        position.x += speed * rotation.0.cos();
        position.y -= speed * rotation.0.sin();
      }
      if self.controller.movingBackward {
        position.x -= speed * rotation.0.cos();
        position.y += speed * rotation.0.sin();
      }
      if self.controller.movingLeft {
        let angle = rotation.0-PI/180.0*90.0;
        position.x += speed * angle.cos();
        position.y -= speed * angle.sin();
      }
      if self.controller.movingRight {
        let angle = rotation.0+PI/180.0*90.0;
        position.x += speed * angle.cos();
        position.y -= speed * angle.sin();
      }

      self.camera.move_to(Vec2::new(
        position.x,
        position.y
      ));
    }
    


    
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
          },
          Views::Background => {
            let image = graphics::Image::new(ctx, "/grass.png").unwrap();
            let mut batch = graphics::spritebatch::SpriteBatch::new(image);
            
            let (cx, cy) = self.camera.world_to_screen_coords(Vec2::new(0.0, 0.0));
            let grassSize = 10;
            for j in -grassSize..grassSize {
              for i in -grassSize..grassSize {
                let p = graphics::DrawParam::new().dest(Point2::new((cx+i*415) as f32,(cy+j*415) as f32));
                batch.add(p);
              }
            }
            view.meshes.push(Box::new(batch));
            // graphics::draw(ctx, &batch, param)?;
          }
        }
      }
    }
    
		Ok(())
  }
  
	fn draw(&mut self, ctx: &mut Context) -> GameResult {
    
    let count = self.world.entities().join().count();

    let view_comp = self.world.read_storage::<ViewComponent>();
    let positions = self.world.read_storage::<Position>();
    let rotations = self.world.read_storage::<RotationComponent>();
   
    graphics::clear(ctx, graphics::BLACK);
    let scale = 0.3;
    

    for (view, position, rotation) in (&view_comp, &positions, &rotations).join() {
      let (cx, cy) = self.camera.world_to_screen_coords(Vec2::new(position.x, position.y));
      // println!("Position {:?}, translated to camera coordinates {:?}", position, (cx, cy));
      let params = graphics::DrawParam::new()
        .rotation(rotation.0)
        .dest(Point2::new(cx as f32, cy as f32))
        .scale(Vector2::new(scale, scale));
        // .offset(Point2::new(0.25, 0.5));    //  todo remove this
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
              cx as f32, cy as f32
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
    //   //  draw debug collision circle
      for (collision, position, rotation) in (&collisions, &positions, &rotations).join() {
        let (cx, cy) = self.camera.world_to_screen_coords(Vec2::new(position.x, position.y));
        let params = graphics::DrawParam::new()
          .dest(Point2::new(
            cx as f32, cy as f32
          ))
          .rotation(rotation.0)
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
      for (shooter, position, rotation) in (&shooters, &positions, &rotations).join() {
        let (cx, cy) = self.camera.world_to_screen_coords(Vec2::new(position.x, position.y));
        let params = graphics::DrawParam::new()
        .dest(Point2::new(
          cx as f32,
          cy as f32
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