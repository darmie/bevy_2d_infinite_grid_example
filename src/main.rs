use bevy::{
    prelude::*,
    sprite::Material2dPlugin,
};
#[cfg(any(feature = "webgl2", feature="webgpu"))]
use bevy::asset::AssetMetaCheck;
use bevy_infinity_graph::*;


fn main() {
   let mut app =  App::new();
    #[cfg(any(feature = "webgl2", feature="webgpu"))]
    {
        app.insert_resource(AssetMetaCheck::Never);
        app.add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                // provide the ID selector string here
                canvas: Some("#infinite-2d-graph-canvas".into()),
                // ... any other window properties ...
                ..default()
            }),
            ..default()
        }));
    }
    #[cfg(not(any(feature = "webgl2", feature="webgpu")))]
    app.add_plugins(DefaultPlugins);

    app.init_resource::<MaterialId>()
        .add_plugins((Material2dPlugin::<InfinityGridMaterial2D>::default(),))
        .add_systems(Startup, setup)
        .add_systems(Update, update_material)
        .run();
}