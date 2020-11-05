use std::f32;
use std::path::Path;
use std::rc::Rc;

use crate::driver::*;
use crate::solver::Solver;
use crate::strategy::Strategy;

use nalgebra::{Point2, Point3};
use nalgebra::{Translation3, UnitQuaternion, Vector3};

use kiss3d::camera::ArcBall;
use kiss3d::event::{Action, Key, WindowEvent};
use kiss3d::light::Light;
use kiss3d::resource::MeshManager;
use kiss3d::scene::SceneNode;
use kiss3d::text::Font;
use kiss3d::window::Window;

pub struct Driver3D {
    board_size: usize,
    window: Window,
    font: Rc<Font>,
    queens: Vec<SceneNode>,
    arc_ball: ArcBall,
}

impl Driver3D {
    pub fn new(board_size: usize) -> Self {
        let mut window = Window::new_with_size("N-Queen solver", 720, 720);

        let mut board = window.add_group();
        let tile_rotation =
            UnitQuaternion::from_axis_angle(&Vector3::x_axis(), f32::consts::PI / 2.);
        for i in 0..board_size {
            for j in 0..board_size {
                let mut tile = board.add_quad(1., 1., 1, 1);
                tile.append_rotation_wrt_center(&tile_rotation);
                tile.append_translation(&Translation3::new(i as f32, 0., j as f32));
                let c = if (i + j) % 2 == 0 { 0.1 } else { 0.9 };
                tile.set_color(c, c, c);
            }
        }

        let queen_obj = Path::new("resources/queen.obj");
        let queen_mtl = Path::new("resources/queen.mtl");

        let mut manager = MeshManager::new();
        MeshManager::load_obj(&queen_obj, &queen_mtl, "queen")
            .unwrap()
            .into_iter()
            .for_each(|(name, mesh, _)| {
                manager.add(mesh, &name[..]);
            });

        let mut queens = Vec::new();
        for i in 0..board_size {
            let mesh = manager.get("queen").unwrap();
            let mut queen = window.add_mesh(mesh, Vector3::new(0.8, 0.8, 0.8));
            queen.set_local_translation(Translation3::new(-1., 0.0, i as f32));
            queens.push(queen);
        }

        let eye = Point3::new(
            board_size as f32 / 2. - 1.0,
            12.0,
            board_size as f32 / 2. - 0.5 + 0.3,
        );
        let at = Point3::new(
            board_size as f32 / 2. - 1.0,
            0.0,
            board_size as f32 / 2. - 0.5,
        );
        let arc_ball = ArcBall::new(eye, at);

        Self {
            board_size,
            window,
            font: Font::default(),
            queens,
            arc_ball,
        }
    }
}

const USAGE: &str = r#"Usage:
Press <Space> to advance one step or maintain to advance quickly
Press <Enter> to go back at the origin
Press <Esc> to quit
Mouse left click to rotate
Mouse right click to shift
Mouse wheel to zoom
"#;

impl<T> Driver<T> for Driver3D
where
    T: Strategy,
{
    fn execute(&mut self, solver: Solver, strategy: T, interaction_mode: InteractionMode) {
        let mut solver = solver;
		let mut strategy = strategy;
		let mut end = false;

        self.window.set_light(Light::StickToCamera);

        while !self.window.should_close() {
            self.window.draw_text(
                USAGE,
                &Point2::origin(),
                30.0,
                &self.font,
                &Point3::new(1.0, 1.0, 1.0),
            );

            self.window.draw_text(
                &format!("Solutions found so far: {}", solver.solutions_found())[..],
                &Point2::new(0.0, 220.0),
                60.0,
                &self.font,
                &Point3::new(1.0, 1.0, 0.0),
			);

			if !end {
				let step = strategy.next_step(&mut solver);
				if let Some(config) = step {
					let mut positions = config
                    .configuration
                    .into_iter()
                    .map(|x| x as f32)
                    .collect::<Vec<f32>>();
					positions.resize(self.board_size, -1.);
					let it = self.queens.iter_mut().zip(positions.into_iter());
					for (i, (queen, pos)) in it.enumerate() {
						queen.set_local_translation(Translation3::new(pos, 0.0, i as f32));
					}
				} else {
					if let InteractionMode::WaitUser = interaction_mode {
						for event in self.window.events().iter() {
							match event.value {
								WindowEvent::Key(Key::Space, Action::Press, _) => {
									end = !strategy.has_next_batch(&mut solver);
								}
								_ => {}
							}
						}
					}
					else {
						end = !strategy.has_next_batch(&mut solver);
					}
				}
			} else {
				self.window.draw_text(
					"No other partial solution exists",
					&Point2::new(0.0, 280.0),
					60.0,
					&self.font,
					&Point3::new(1.0, 0.0, 0.0),
				);
			}

            self.window.render_with_camera(&mut self.arc_ball);
        }
    }
}
