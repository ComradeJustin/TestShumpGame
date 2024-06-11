use bevy::asset::AssetServer;
use bevy::transform::components::Transform;
use bevy::utils::default;

use bevy::{input::ButtonInput, prelude::{Commands, Event, EventReader, EventWriter, KeyCode, Query, Res, ResMut, Resource}, render::texture::Image, sprite::{Sprite, SpriteBundle}, time::{Time, Timer}};

use super::Physics;

#[derive(Event,Default)]
pub struct EnemyShoot(pub i8);

#[derive(Event, Default)]
pub struct AttackType(pub i8);



#[derive(Resource, Default)]
pub struct Firingtimer{
    time:Timer
}
pub fn pattern(mut firingevent: EventReader<EnemyShoot>, mut firingtype: EventWriter<AttackType> ){
    

    if !firingevent.is_empty(){
        for ev in firingevent.read(){
            match ev.0{
                1 => firingtype.send(AttackType(1)),
                2 => firingtype.send(AttackType(2)),
                _ => panic!("should not be called")
            };
        }
    }
    
    
}
pub fn reader(mut sendev: EventWriter<EnemyShoot>, key:  Res<ButtonInput<KeyCode>>){
    if key.just_pressed(KeyCode::KeyK){
        sendev.send(EnemyShoot(1));
    }
}
pub fn projectilespawnpattern(mut cmd: Commands, mut timer: ResMut<Firingtimer>, time: Res<Time>, mut attacktype: EventReader<AttackType>,asset_server: Res<AssetServer>){

    
    if !attacktype.is_empty(){
        timer.time.tick(time.delta());
        for ite in attacktype.read(){
            cmd.spawn(((SpriteBundle
                {sprite: Sprite{custom_size: Some(bevy::math::Vec2::new(Physics::ENEMYTESTPROJ,Physics::ENEMYTESTPROJ)), ..default()}
                ,texture: asset_server.load::<Image>("embedded://Hitbox.png"),transform: Transform::from_xyz(0.0, 0.0, 1.0)  
                , ..Default::default()}),Physics::Enemyproj));
        }
       
        
    }
}