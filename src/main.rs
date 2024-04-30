use bevy::{app::{App, PluginGroup, Startup, Update}, asset::Assets, core_pipeline::{bloom::BloomSettings, core_2d::Camera2dBundle}, ecs::{entity::Entity, query::With, system::{Commands, NonSend, Query, ResMut}}, prelude::default, render::{color::Color, mesh::Mesh, texture::ImagePlugin, view::window}, sprite::{MaterialMesh2dBundle, Mesh2dHandle}, text::{Text, Text2dBundle, TextSection, TextStyle}, window::{EnabledButtons, PrimaryWindow, Window, WindowPlugin, WindowPosition, WindowResolution}, winit::WinitWindows, DefaultPlugins};
use Physics::{PlayerhitboxComp, Shotcounter};



mod Physics;
mod Ui;




fn main() {

    App::new()

        .add_plugins(DefaultPlugins::set(DefaultPlugins,WindowPlugin{ 
            primary_window: 
            Some(Window
                {title: "Amogus".into(), 
                name: Some("amogus2".into()), 
                resolution: WindowResolution::new(960., 720.).with_scale_factor_override(1.5), 
                position: bevy::window::WindowPosition::Centered(bevy::window::MonitorSelection::Current), 
                resizable: false,
                mode: bevy::window::WindowMode::Windowed, 
                enabled_buttons: EnabledButtons { minimize: true, maximize: false, close: true }, 
                ..Default::default() }),
                
                 ..Default::default()}).set(ImagePlugin::default_nearest()))

                 
        .add_systems(Startup, (camera,testui,Physics::spawnplayer, setup)) // Runs on startup
        .add_systems(Update, (Physics::input,Physics::physloop, Physics::guntimer, Physics::devmode))// Runs every frame
        .init_resource::<Shotcounter>().run(); // Runs the app


}

fn setup(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>,mut materials: ResMut<Assets<bevy::sprite::ColorMaterial>>, ){
    commands.init_resource::<Physics::GlobalChecker>();
    commands.init_resource::<Physics::Slowdown>();
    let x = commands.spawn(MaterialMesh2dBundle{mesh: Mesh2dHandle(meshes.add(bevy::math::primitives::Circle{radius: 5.0})), material: materials.add(Color::RED),..default()}).id();
    commands.entity(x).insert(PlayerhitboxComp);

}




fn testui( mut commands: Commands){ // Spawns a test ui
    let color = Color::RED;
    commands.spawn(Text2dBundle {
        text: Text {
            sections: vec![TextSection::new(
                format!("This is line one"),
                TextStyle {
                    color   ,..Default::default()
                },
            )],
            ..Default::default()
        },
        ..Default::default()
    });
}
fn camera(mut commands: Commands){
   
    commands.spawn((
        Camera2dBundle {
            camera: bevy::render::camera::Camera {
                hdr: true, // 1. HDR is required for bloom
                ..default()
            },
            tonemapping: bevy::core_pipeline::tonemapping::Tonemapping::TonyMcMapface, // 2. Using a tonemapper that desaturates to white is recommended
            ..default()
        },
        BloomSettings::OLD_SCHOOL, // 3. Enable bloom for the camera
    ));
   
}

    





