use bevy::{app::{App, First, FixedUpdate, Last, Plugin, PluginGroup, PostStartup, PostUpdate, PreUpdate, Startup, Update}, asset::Assets, core_pipeline::{bloom::BloomSettings, core_2d::Camera2dBundle}, ecs::{query::With, schedule::{common_conditions::{in_state, resource_equals}, IntoSystemConfigs, IntoSystemSetConfigs, OnEnter, SystemSet}, system::{Commands, NonSend, Query, ResMut}}, prelude::default, render::{camera::OrthographicProjection, color::Color, mesh::Mesh, texture::ImagePlugin, view::window}, sprite::{MaterialMesh2dBundle, Mesh2dHandle}, text::{Text, Text2dBundle, TextSection, TextStyle}, transform::{components::Transform, TransformSystem}, ui::update, window::{EnabledButtons, PrimaryWindow, Window, WindowPlugin, WindowPosition, WindowResolution}, winit::WinitWindows, DefaultPlugins};


use bevy_pixel_camera::{PixelCameraPlugin, PixelViewport, PixelZoom};
use Physics::{spawnplayer, PlayerhitboxComp, Shotcounter};
use StageEvent::GameState;


mod StageEvent;
mod Physics;
mod Ui;


fn main() {

    App::new()
        
        .add_plugins(PixelCameraPlugin)
        .add_plugins(DefaultPlugins::set(DefaultPlugins,WindowPlugin{ 
            primary_window: 
            Some(Window
                {title: "Amogus".into(), 
                name: Some("amogus2".into()), 
                resolution: WindowResolution::new(960., 720.).with_scale_factor_override(1.0), 
                position: bevy::window::WindowPosition::Centered(bevy::window::MonitorSelection::Primary), 
                resizable: false,
                mode: bevy::window::WindowMode::Windowed, 
                enabled_buttons: EnabledButtons { minimize: true, maximize: false, close: true }, 
                ..Default::default() }),
                
                 ..Default::default()}).set(ImagePlugin::default_nearest()))

        .insert_state(GameState::MainMenu)

        .add_plugins(StartupPlugin)
        .add_plugins(MaingamePlugin)
        
        .run(); // Runs the app


    
}

struct StartupPlugin;
impl Plugin for StartupPlugin{
    fn build(&self, app: &mut App) {
        app.init_resource::<Physics::Slowdown>();
        app.init_resource::<Shotcounter>();
        app.add_systems(bevy::app::PreStartup, startup); // Runs Before Loading in
        app.add_systems(OnEnter(GameState::MainMenu), Ui::render_title_screen); //Loads main Menu
        app.add_systems(PostUpdate, StageEvent::gamestatecheck);// Runs every frame since startup
       

    }
}

struct MaingamePlugin;
impl Plugin for MaingamePlugin{
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::InGame), spawnplayer); //Spawns player on entering states

        app.add_systems(FixedUpdate, (Physics::physloop,Physics::input,Physics::guntimer).before(TransformSystem::TransformPropagate).run_if(in_state(GameState::InGame)));
    }
}









fn startup(mut commands: Commands){


    
    let cam = Camera2dBundle {
        projection: OrthographicProjection {

            near: -1000.0,
            far: 1000.0,
            ..default()
        },
        camera: bevy::render::camera::Camera {
            clear_color: bevy::render::camera::ClearColorConfig::Custom(Color::BEIGE),
            
            ..default()
        },
        ..default()
    };
    commands.spawn((
        cam,
        PixelZoom::Fixed(1),
        PixelViewport,
    ));
    
   
}

    





