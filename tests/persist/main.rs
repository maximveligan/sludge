use sludge::{
    components::{Name, Persistent},
    prelude::*,
};

fn roundtrip(space: &Space) -> Result<Space> {
    let mut bytes = Vec::<u8>::new();
    space
        .lua()
        .context(|lua| sludge::persist::persist(lua, space, &mut bytes))?;

    let new_space = Space::new()?;
    new_space
        .lua()
        .context(|lua| sludge::persist::unpersist(lua, &new_space, &mut &bytes[..]))?;

    Ok(new_space)
}

#[test]
fn persist_empty() -> Result<()> {
    let space = Space::new()?;
    roundtrip(&space)?;

    Ok(())
}

#[derive(Bundle)]
pub struct Fish {
    name: Name,
    persistent: Persistent,
}

#[test]
fn persist_simple() -> Result<()> {
    let space = Space::new()?;

    {
        let mut w = space.world_mut();
        for i in 0..100 {
            for j in 0..100 {
                let (x, y) = (i as f32, j as f32);
                w.spawn(Fish {
                    name: Name(format!("Fish ({}, {})", x, y)),
                    persistent: Persistent,
                });
            }
        }
    }

    let space = roundtrip(&space)?;
    let count = space
        .world()
        .query::<()>()
        .with::<Name>()
        .with::<Persistent>()
        .iter()
        .count();

    assert_eq!(count, 100 * 100);

    Ok(())
}
