mod cgol;
mod data;
mod systems;

use crate::data::SimpleTile;
use amethyst::{
    core::TransformBundle,
    input::{InputBundle, StringBindings},
    prelude::*,
    renderer::{
        plugins::{RenderFlat2D, RenderToWindow},
        types::DefaultBackend,
        RenderingBundle,
    },
    tiles::RenderTiles2D,
    utils::application_root_dir,
};
use cgol::ConwaysGameOfLife;
use systems::{CameraSystem, CelluarAutomataSystem, InitSystem};

pub const MAP_DIMS: (u32, u32) = (150, 90);

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());

    let app_root = application_root_dir()?;

    let config_path = app_root.join("config");

    let display_config_path = config_path.join("display.ron");
    let bindings_path = config_path.join("bindings.ron");

    let game_data = GameDataBuilder::default()
        .with_bundle(TransformBundle::new())?
        .with_bundle(
            RenderingBundle::<DefaultBackend>::new()
                .with_plugin(
                    RenderToWindow::from_config_path(display_config_path)?
                        .with_clear([0.0, 0.0, 0.0, 1.0]),
                )
                .with_plugin(RenderFlat2D::default())
                .with_plugin(RenderTiles2D::<SimpleTile>::default())
        )?
        .with_bundle(InputBundle::<StringBindings>::new().with_bindings_from_file(bindings_path)?)?
        .with(CameraSystem::default(), "camera_system", &[])
        .with(CelluarAutomataSystem::default(), "cellular_system", &[])
        .with(InitSystem, "init", &[])
        //.with(RenderSystem, "render_system", &[])
        ;

    let assets_dir = app_root.join("assets");

    let mut game = Application::new(assets_dir, ConwaysGameOfLife, game_data)?;
    game.run();

    Ok(())
}
