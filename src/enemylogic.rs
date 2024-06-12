use bevy::asset::AssetServer;
use bevy::math::Vec3;
use bevy::prelude::{Entity, With};
use bevy::transform::components::Transform;
use bevy::utils::default;

use bevy::{input::ButtonInput, prelude::{Commands, Event, EventReader, EventWriter, KeyCode, Query, Res, ResMut, Resource}, render::texture::Image, sprite::{Sprite, SpriteBundle}, time::{Time, Timer}};

use super::Physics;

#[derive(Event,Default)]
pub struct EnemyShoot(pub i8);

#[derive(Event, Default)]
pub struct AttackType(pub i8);

#[derive(Resource,Default)]
pub struct rotationaltime{
    theta: Timer
}




#[derive(Resource, Default)]
pub struct Firingtimer{
    time:Timer
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
pub fn projectilespawner(mut cmd: Commands, mut timer: ResMut<Firingtimer>, time: Res<Time>, mut attacktype: EventReader<AttackType>,asset_server: Res<AssetServer>){

    
    if !attacktype.is_empty(){
        let mut x:i32 = 0;
        timer.time.tick(time.delta());
        for ite in attacktype.read(){
            x += 1;
            cmd.spawn(((SpriteBundle
                {sprite: Sprite{custom_size: Some(bevy::math::Vec2::new(Physics::ENEMYTESTPROJ,Physics::ENEMYTESTPROJ)), ..default()}
                ,texture: asset_server.load::<Image>("embedded://Hitbox.png"),transform: Transform::from_xyz(0.0, 0.0, 1.0)  
                , ..Default::default()}),Physics::Enemyproj {bullettype: ite.0, id: x, angle: 0.0}));
        }
       
        
    }


}
pub fn movementpattern(mut projectilequery: Query<&mut Transform, With<Physics::Enemyproj>>, mut angle: ResMut<rotationaltime>, time: Res<Time>){
    angle.theta.tick(time.delta());
    
    if !projectilequery.is_empty(){
        for mut pos in projectilequery.iter_mut(){
            let dir = [( angle.theta.elapsed_secs()).sin(), ( angle.theta.elapsed_secs() ).cos()];
            pos.translation += Vec3::new(dir[1],dir[0], 0.0)
        }
    }
    if angle.theta.elapsed_secs() >= 10.0{
        angle.theta.reset();
    }
}

pub fn despawnprojectile(mut cmd: Commands,mut projectilequery: Query<(&mut Transform, Entity), With<Physics::Enemyproj>>, screen: Query<&bevy::window::Window>){
    if !projectilequery.is_empty(){
        for x in projectilequery.iter_mut(){
            if [x.0.translation.x, x.0.translation.y] >= [screen.single().physical_width() as f32/2. ,screen.single().physical_height() as f32/2.] 
            ||[x.0.translation.x, x.0.translation.y] <= [screen.single().physical_width() as f32/-2. ,screen.single().physical_height() as f32/-2.]{
                cmd.entity(x.1).despawn();
                println!("despawned!");
            }
        }
    }
}