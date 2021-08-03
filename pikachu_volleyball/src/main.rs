use amethyst::{
    core::transform::TransformBundle,
    input::{InputBundle, StringBindings},
    prelude::*,
    renderer::{
        plugins::{RenderFlat2D, RenderToWindow},
        types::DefaultBackend,
        RenderingBundle,
    },
    ui::{RenderUi, UiBundle},
    utils::application_root_dir,
};

pub struct PikachuVolleyball;

impl SimpleState for PikachuVolleyball{

}

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());

    let app_root = application_root_dir()?;


    let config_dir = app_root.join("config");
    let asseets_dir = app_root.join("assets");
    let config_display_path = config_dir.join("display.ron");
   
    let game_data = GameDataBuilder::default()
        .with_bundle(
            RenderingBundle::<DefaultBackend>::new()
                .with_plugin(
                    RenderToWindow::from_config_path(config_display_path)?
                        .with_clear([0.0, 0.0, 0.0, 1.0]),
                ) 
                .with_plugin(RenderFlat2D::default()),
        )?
        .with_bundle(TransformBundle::new())?;

    let mut game = Application::new(asseets_dir, PikachuVolleyball, game_data)?;
    game.run();

    Ok(())
}