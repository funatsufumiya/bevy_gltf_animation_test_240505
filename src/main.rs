use bevy::animation::RepeatAnimation;
use bevy::gltf::Gltf;
use bevy::prelude::*;
use bevy_asset_loader::asset_collection::AssetCollection;
use bevy_asset_loader::loading_state::config::ConfigureLoadingState;
use bevy_asset_loader::loading_state::{LoadingState, LoadingStateAppExt};
use bevy_debug_text_overlay::{screen_print, OverlayPlugin};
use bevy_gltf_components::ComponentsFromGltfPlugin;
use bevy_registry_export::ExportRegistryPlugin;

#[derive(AssetCollection, Resource)]
struct GltfAssets {
    #[asset(path = "test_scale.gltf")]
    gltf: Handle<Gltf>,

    #[asset(path = "test_scale.gltf#Animation0")]
    cube_animation: Handle<AnimationClip>,
}

#[derive(Component)]
struct MyGltfObject;

#[derive(Component, Reflect, Default, Debug)]
#[reflect(Component)]
struct MyGltfCube;

#[derive(Clone, Eq, PartialEq, Debug, Hash, Default, States)]
pub enum AssetLoadingState {
    #[default]
    Loading,
    Loaded,
}

// fn load_gltf(
//     mut commands: Commands,
//     assets: Res<AssetServer>,
// ) {
//     let gltf = assets.load("test_scale.gltf");
//     commands.insert_resource(MyAssetPack(gltf));
// }

fn spawn_camera_and_light(mut commands: Commands) {
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(0.0, 0.0, 5.0),
        ..default()
    });

    commands.insert_resource(AmbientLight {
        brightness: 0.5,
        color: Color::WHITE,
        // ..default()
    });
}

fn spawn_gltf_objects(
    mut commands: Commands,
    my: Res<GltfAssets>,
    assets_gltf: Res<Assets<Gltf>>,
) {
    println!("Spawning GLTF objects");

    // if the GLTF has loaded, we can navigate its contents
    if let Some(gltf) = assets_gltf.get(&my.gltf) {
        println!("GLTF loaded!");
        // spawn the first scene in the file
        commands.spawn((SceneBundle {
            scene: gltf.scenes[0].clone(),
            visibility: Visibility::Hidden,
            ..default()
        }, MyGltfObject));

        // spawn the scene
        // commands.spawn(SceneBundle {
        //     scene: gltf.named_scenes["CubeAction"].clone(),
        //     transform: Transform::from_xyz(1.0, 2.0, 3.0),
        //     ..Default::default()
        // });

        // PERF: the `.clone()`s are just for asset handles, don't worry :)
    }
}

fn play_animation(
    time: Res<Time>,
    mut query: Query<&mut AnimationPlayer, Added<AnimationPlayer>>,
    my: Res<GltfAssets>,
    assets_animations: Res<Assets<AnimationClip>>,
) {
    // println!("Playing animation");
    for mut player in query.iter_mut() {
        // println!("Playing animation!!!");

        // print total time
        let animation = assets_animations.get(&my.cube_animation).unwrap();
        println!("Total time: {:?}", animation.duration());

        // player.repeat();
        player.play(my.cube_animation.clone()).repeat();
    }
}

fn print_scale(
    query: Query<&Transform, With<MyGltfCube>>,
) {
    for transform in query.iter() {
        // println!("Scale: {:?}", transform.scale);
        screen_print!("Scale: {:?}", transform.scale);
    }
}

fn main (){
    let mut app = App::new();

    app
        .add_plugins(DefaultPlugins)
        .add_plugins(OverlayPlugin {
            font_size: 40.0,
            ..default()
        })
        .add_plugins(ExportRegistryPlugin::default())
        .add_plugins(ComponentsFromGltfPlugin::default())
        ;

    app
        .register_type::<MyGltfCube>()
        ;

    let mut loading_state = 
            LoadingState::new(AssetLoadingState::Loading)
                .continue_to_state(AssetLoadingState::Loaded);

    app
        .init_state::<AssetLoadingState>()
        .add_loading_state(
            loading_state
                .load_collection::<GltfAssets>()
        );
    
    app
        // .add_systems(Startup, load_gltf)
        .add_systems(OnEnter(AssetLoadingState::Loaded), spawn_camera_and_light)
        .add_systems(OnEnter(AssetLoadingState::Loaded), spawn_gltf_objects)
        .add_systems(Update, bevy::window::close_on_esc)
        .add_systems(Update, play_animation.run_if(in_state(AssetLoadingState::Loaded)))
        .add_systems(Update, print_scale.run_if(in_state(AssetLoadingState::Loaded)).after(play_animation))
        ;

    app.run();
}