extern crate sludge as sloodge;

use {
    sloodge::{
        assets::{DefaultCache, Key},
        conf::Conf,
        event::EventHandler,
        filesystem::Filesystem,
        graphics::text::*,
        graphics::*,
        prelude::*,
    },
    std::{env, path::PathBuf},
};

mod sludge {
    pub use ::sludge::sludge::*;
}

struct MainState {
    space: Space,
    text: Text,
}

impl MainState {
    pub fn new(gfx: Graphics) -> Result<MainState> {
        let global = {
            let mut resources = OwnedResources::new();

            let mut fs = Filesystem::new("text-example", "Maxim Veligan, Sean Leffler")?;
            if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
                let mut path = PathBuf::from(manifest_dir);
                path.push("resources");
                log::info!("Adding resource path {}", path.display());
                fs.mount(&path, true);
            }

            resources.insert(fs);
            resources.insert(gfx);

            SharedResources::from(resources)
        };

        let space = Space::with_global_resources(global)?;
        space
            .resources()
            .borrow_mut()
            .insert(DefaultCache::new(space.resources().clone()));

        let font_atlas_key = Key::from_structured(&FontAtlasKey::new(
            "/font.ttf",
            20,
            CharacterListType::AsciiSubset,
        ))?;
        let atlas = space
            .fetch_mut::<DefaultCache>()
            .get::<FontAtlas>(&font_atlas_key)?;
        let mut text = Text::from_cached(&mut *space.fetch_mut(), atlas);
        text.set_text("Hello World!", Color::GREEN);

        Ok(MainState { space, text })
    }
}

impl EventHandler for MainState {
    type Args = ();

    fn init(ctx: Graphics, _: ()) -> Result<Self> {
        Self::new(ctx)
    }

    fn update(&mut self) -> Result<()> {
        Ok(())
    }

    fn draw(&mut self) -> Result<()> {
        let Self { space, text } = self;
        let gfx = &mut *space.fetch_mut::<Graphics>();

        gfx.set_projection(Orthographic3::new(0., 1280., 960., 0., -1., 1.));
        gfx.begin_default_pass(PassAction::default());
        gfx.apply_default_pipeline();
        gfx.apply_transforms();
        gfx.draw(
            text,
            InstanceParam::new()
                .translate2(Vector2::new(20., 140.))
                .scale2(Vector2::repeat(40.)),
        );
        gfx.end_pass();
        gfx.commit_frame();
        Ok(())
    }
}

fn main() -> Result<()> {
    sloodge::event::run::<MainState>(
        Conf {
            window_title: "Hello world!".to_string(),
            window_width: 320 * 4,
            window_height: 240 * 4,
            ..Conf::default()
        },
        (),
    );

    Ok(())
}
