use std::f32::consts::{self, PI};

use bevy::asset::AssetServer;
use bevy::math::Vec3;
use bevy::prelude::{Camera2dBundle, Component, Entity, NextState, With, Without};
use bevy::time::Stopwatch;
use bevy::transform::components::Transform;
use bevy::utils::default;

use bevy::{input::ButtonInput, prelude::{Commands, Event, EventReader, EventWriter, KeyCode, Query, Res, ResMut, Resource}, render::texture::Image, sprite::{Sprite, SpriteBundle}, time::{Time, Timer}};




use super::Physics;
use super::StageEvent;
#[derive(Event,Default)]
pub struct EnemyShoot(pub i8);

#[derive(Event, Default)]
pub struct AttackType(pub i8);

#[derive(Component, Default)]
pub struct Projectileref;
#[derive(Resource, Default)]
pub struct RotationCount{
    angle: f32,
    shift: f32,
}


#[derive(Resource, Default)]
pub struct Firingtimer{
    time:Stopwatch,

    id: i32
}
pub fn attackreg(mut firingevent: EventReader<EnemyShoot>, mut firingtype: EventWriter<AttackType> ){
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

pub fn deload_all(mut cmd: Commands, query: Query<Entity, (With<Physics::MainCamera>, With<Physics::Refplayerproj>, With<Physics::Refplayer>, With<Physics::Enemyproj>)>){
    for item in query.iter(){
        cmd.entity(item).despawn();
    }
}
pub fn projectilespawner(slow: Res<Physics::Slowdown>,mut rotation: ResMut<RotationCount>,mut cmd: Commands, mut timer: ResMut<Firingtimer>, time: Res<Time>, mut attacktype: EventReader<AttackType>,asset_server: Res<AssetServer>){

        
        timer.time.tick(time.delta());

        if (timer.time.elapsed_secs() *1000.).round() / 1000.0 / slow.rate >= 0.02{

            timer.id += 1;
            rotation.angle += consts::PI/10.; 



        
           
            if rotation.angle > consts::PI * 2.{
                rotation.angle = 0.;
                rotation.shift += consts::PI/15.;
            }
            if rotation.shift >= consts::PI * 2.{
                rotation.shift = 0.0
            }

           
            cmd.spawn(((SpriteBundle
                {sprite: Sprite{custom_size: Some(bevy::math::Vec2::new(Physics::ENEMYTESTPROJ,Physics::ENEMYTESTPROJ)), ..default()}
                ,texture: asset_server.load::<Image>("embedded://Hitbox.png"),transform: Transform::from_xyz(0.0, 0.0, 1.0)  
                , ..Default::default()}),Physics::Enemyproj {bullettype: 1, id: timer.id , angle: rotation.angle  + rotation.shift }, Projectileref));
            timer.time.reset();
        }

    }



pub fn movementpattern(mut projectilequery: Query<(&mut Transform, &Physics::Enemyproj), With<Projectileref>>, slow: Res<Physics::Slowdown>){
    

    if !projectilequery.is_empty(){

        for mut pos in projectilequery.iter_mut(){

            pos.0.translation.x = (pos.0.translation.x * 1000.0).round() /1000.0 ;
            pos.0.translation.y = (pos.0.translation.y * 1000.0).round() /1000.0 ;
            pos.0.translation += Vec3::new(pos.1.angle.cos() / slow.rate,pos.1.angle.sin() / slow.rate, 0.0);
        }
    }

}

pub fn despawnprojectile(mut cmd: Commands,mut projectilequery: Query<(&mut Transform, Entity), With<Physics::Enemyproj>>, screen: Query<&bevy::window::Window>){
    if !projectilequery.is_empty(){
        for x in projectilequery.iter_mut(){
            if [x.0.translation.x, x.0.translation.y] >= [screen.single().physical_width() as f32/2. + Physics::ENEMYTESTPROJ ,screen.single().physical_height() as f32/2. + Physics::ENEMYTESTPROJ] 
            ||[x.0.translation.x, x.0.translation.y] <= [screen.single().physical_width() as f32/-2. - Physics::ENEMYTESTPROJ,screen.single().physical_height() as f32/-2. - Physics::ENEMYTESTPROJ]{
                cmd.entity(x.1).despawn();
            }
        }
    }
}