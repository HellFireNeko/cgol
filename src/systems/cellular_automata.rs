use amethyst::{
    core::math::Point3,
    ecs::{Join, System, WriteStorage},
    shred::Write,
    tiles::{MapStorage, TileMap},
};

use crate::{
    data::{AutomataState, EntityState, SimpleTile},
    MAP_DIMS,
};

#[derive(Default)]
pub struct CelluarAutomataSystem;

impl<'s> System<'s> for CelluarAutomataSystem {
    type SystemData = (
        WriteStorage<'s, TileMap<SimpleTile>>,
        Write<'s, AutomataState>,
    );

    fn run(&mut self, (mut tilemaps, mut state): Self::SystemData) {
        if !state.running {
            if !state.should_step {
                return;
            }
        }

        for tilemap in (&mut tilemaps).join() {
            let mut next_map_state = tilemap.clone();

            for x in 0..=MAP_DIMS.0 {
                for y in 0..=MAP_DIMS.1 {
                    let pos = Point3::new(x, y, 0);

                    if let Some((Some(current_state), Some(mut_state))) =
                        (tilemap.get(&pos), next_map_state.get_mut(&pos)).into()
                    {
                        let alive_neighbors = count_alive_neighbors(&tilemap, x, y);

                        let new_state = match current_state.state {
                            EntityState::Alive if alive_neighbors == 2 || alive_neighbors == 3 => EntityState::Alive,
                            EntityState::Dead if alive_neighbors == 3 => EntityState::Alive,
                            _ => EntityState::Dead
                        };

                        mut_state.state = new_state;
                    }
                }
            }

            *tilemap = next_map_state;
        }

        state.should_step = false;
    }
}

fn count_alive_neighbors(tilemap: &TileMap<SimpleTile>, x: u32, y: u32) -> usize {
    let x = x as i32;
    let y = y as i32;
    let neighbors: [(i32, i32); 8] = [
        (x - 1, y - 1),
        (x, y - 1),
        (x + 1, y - 1),
        (x - 1, y),
        (x + 1, y),
        (x - 1, y + 1),
        (x, y + 1),
        (x + 1, y + 1),
    ];

    neighbors
        .iter()
        .filter(|&&(nx, ny)| {
            if nx < 0 || ny < 0 {
                return false;
            }
            tilemap
                .get(&Point3::new(nx as u32, ny as u32, 0))
                .map_or(false, |tile| tile.state == EntityState::Alive)
        })
        .count()
}
