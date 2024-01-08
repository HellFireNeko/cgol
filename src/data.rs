use amethyst::core::math::Point3;
use amethyst::prelude::World;
use amethyst::tiles::Tile;

#[derive(Default, PartialEq, Eq, Clone, Copy, Debug)]
pub enum EntityState {
    Alive,
    #[default]
    Dead
}

#[derive(Default)]
pub struct AutomataState {
    pub running: bool,
    pub quitting: bool,
    pub should_step: bool
}

#[derive(Clone, Default)]
pub struct SimpleTile {
    pub state: EntityState
}

impl Tile for SimpleTile {
    fn sprite(&self, _coordinates: Point3<u32>, _world: &World) -> Option<usize> {
        match self.state {
            EntityState::Alive => Some(1),
            EntityState::Dead => Some(0),
        }
    }
}