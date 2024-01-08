use crate::{
    data::{EntityState, SimpleTile},
    MAP_DIMS,
};
use amethyst::{
    core::math::Point3,
    ecs::{Join, System, WriteStorage},
    input::{InputHandler, StringBindings},
    shred::Read,
    tiles::{MapStorage, TileMap},
};
use rand::Rng;

pub struct InitSystem;

impl<'s> System<'s> for InitSystem {
    type SystemData = (
        WriteStorage<'s, TileMap<SimpleTile>>,
        Read<'s, InputHandler<StringBindings>>,
    );

    fn run(&mut self, (mut tilemaps, input): Self::SystemData) {
        if let Some(action) = input.action_is_down("initialize") {
            if action {
                for tilemap in (&mut tilemaps).join() {
                    let mut rng = rand::thread_rng();

                    for x in 0..=MAP_DIMS.0 {
                        for y in 0..=MAP_DIMS.1 {
                            if let Some(tile) = tilemap.get_mut(&Point3::new(x, y, 0)) {
                                tile.state = if rng.gen_range(0..=1) == 1 {
                                    EntityState::Alive
                                } else {
                                    EntityState::Dead
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
