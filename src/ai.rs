use specs::prelude::*;
use rand::prelude::*;
use crate::components::*;

const ENEMY_MOVEMENT_SPEED: i32 = 6;

pub struct AI;

impl<'a> System<'a> for AI {
    type SystemData = (
        ReadStorage<'a, Enemy>,
        WriteStorage<'a, Velocity>,
    );

    fn run(&mut self, mut data: Self::SystemData) {
        let mut rng = thread_rng();
        for (_, vel) in (&data.0, &mut data.1).join() {
            if rng.gen_range(0..1) == 0 {
                vel.speed = ENEMY_MOVEMENT_SPEED;
                vel.direction = match rng.gen_range(0..4) {
                    0 => Direction::Up,
                    1 => Direction::Down,
                    2 => Direction::Left,
                    3 => Direction::Right,
                    _ => unreachable!(),
                }
            }
            
        }
    }
}