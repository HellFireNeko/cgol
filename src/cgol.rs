use amethyst::assets::{AssetStorage, Loader};
use amethyst::core::math::{Point3, Vector3};
use amethyst::core::Transform;
use amethyst::input::InputEvent;
use amethyst::prelude::*;
use amethyst::renderer::sprite::SpriteSheetHandle;
use amethyst::renderer::{Camera, ImageFormat, SpriteSheet, SpriteSheetFormat, Texture};
use amethyst::tiles::MapStorage;
use amethyst::tiles::TileMap;
use amethyst::winit::{Event, WindowEvent};
use amethyst::SimpleState;

use rand::Rng;

use log::info;

use crate::MAP_DIMS;
// use crate::data::{AutomataStorage, EntityData, EntityState, AutomataState};
use crate::data::{AutomataState, EntityState, SimpleTile};

pub struct ConwaysGameOfLife;

impl SimpleState for ConwaysGameOfLife {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;

        world.insert(AutomataState::default());

        info!("[INIT_SYSTEM] Preparing all the entities!");

        init_camera(world);
        init_world(world);

        info!("[INIT_SYSTEM] Done!");
    }

    fn handle_event(
        &mut self,
        data: StateData<'_, GameData<'_, '_>>,
        event: StateEvent,
    ) -> SimpleTrans {
        let mut automata_state = {
            let world = data.world;
            world.write_resource::<AutomataState>()
        };
        match event {
            StateEvent::Window(event) => match event {
                Event::WindowEvent { event, .. } => match event {
                    WindowEvent::CloseRequested => {
                        automata_state.running = false;
                        automata_state.quitting = true;
                        Trans::None
                    }
                    _ => Trans::None,
                },
                _ => Trans::None,
            },
            StateEvent::Input(event) => match event {
                InputEvent::ActionPressed(action) => {
                    info!("[KEY_PRESS] {action} was pressed");
                    match action.as_str() {
                        "pause" => {
                            automata_state.running = !automata_state.running;
                            Trans::None
                        }
                        "exit" => {
                            automata_state.running = false;
                            automata_state.quitting = true;
                            Trans::None
                        }
                        "step" => {
                            automata_state.running = false;
                            automata_state.should_step = true;
                            Trans::None
                        }
                        _ => Trans::None,
                    }
                }
                _ => Trans::None,
            },
            _ => Trans::None,
        }
    }

    fn fixed_update(&mut self, data: StateData<'_, GameData<'_, '_>>) -> SimpleTrans {
        let automata_state = {
            let world = data.world;
            world.read_resource::<AutomataState>()
        };

        if automata_state.quitting {
            return Trans::Quit;
        }
        Trans::None
    }
}

fn init_camera(world: &mut World) {
    let mut transform = Transform::default();
    transform.set_translation_xyz(0.0, 0.0, 1.0);

    world
        .create_entity()
        .with(Camera::standard_2d(800.0, 600.0))
        .with(transform)
        .build();
}

fn init_world(world: &mut World) {
    let sprite_sheet = get_sprite_sheet(world);

    let mut tilemap = TileMap::<SimpleTile>::new(
        Vector3::new(MAP_DIMS.0, MAP_DIMS.1, 1),
        Vector3::new(32, 32, 1),
        Some(sprite_sheet),
    );

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

    world
        .create_entity()
        .with(tilemap)
        .with(Transform::default())
        .build();
}

fn get_sprite_sheet(world: &mut World) -> SpriteSheetHandle {
    let loader = world.read_resource::<Loader>();
    let texture_storage = world.read_resource::<AssetStorage<Texture>>();
    let sprite_sheet_storage = world.read_resource::<AssetStorage<SpriteSheet>>();

    let texture_handle = loader.load("cellular.png", ImageFormat::default(), (), &texture_storage);

    loader.load(
        "cellular.ron",
        SpriteSheetFormat(texture_handle),
        (),
        &sprite_sheet_storage,
    )
}
