use bevy::{app::{App, First, PluginGroup, PostStartup, Startup, Update}, asset::Assets, core_pipeline::{bloom::BloomSettings, core_2d::Camera2dBundle}, ecs::{entity::Entity, query::With, schedule::{common_conditions::resource_equals, IntoSystemConfigs, IntoSystemSetConfigs, SystemSet}, system::{Commands, NonSend, Query, ResMut}}, prelude::default, render::{color::Color, mesh::Mesh, texture::ImagePlugin, view::window}, sprite::{MaterialMesh2dBundle, Mesh2dHandle}, text::{Text, Text2dBundle, TextSection, TextStyle}, ui::update, window::{EnabledButtons, PrimaryWindow, Window, WindowPlugin, WindowPosition, WindowResolution}, winit::WinitWindows, DefaultPlugins};

use Physics::{spawnplayer, PlayerhitboxComp, Shotcounter};


mod Events;
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


        .add_systems(bevy::app::PreStartup, (camera, setup, Ui::render_title_screen, Events::startup)) // Runs Before Loading in
        .add_systems(Update, Events::gamestatecheck)// Runs every frame since startup
        .add_systems(PostStartup, spawnplayer)
        .add_systems(Update, (Physics::guntimer,Physics::input, Physics::physloop).run_if( resource_equals(Events::CurrentState{ screen_type: true, currentlevel: 1 })))  
        .run(); // Runs the app


    
}

fn setup(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>,mut materials: ResMut<Assets<bevy::sprite::ColorMaterial>>, ){

    commands.init_resource::<Physics::GlobalChecker>();
    commands.init_resource::<Physics::Slowdown>();
    let x = commands.spawn(MaterialMesh2dBundle{mesh: Mesh2dHandle(meshes.add(bevy::math::primitives::Circle{radius: 5.0})), material: materials.add(Color::RED),..default()}).id();
    commands.entity(x).insert(PlayerhitboxComp);
    commands.init_resource::<Shotcounter>()
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

                ..default()
            },
         
            ..default()
        },

    ));
   
}

    





