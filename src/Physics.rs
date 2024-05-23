

use std::path::PathBuf;

use bevy::{asset::{AssetServer, Assets}, ecs::{component::Component, entity::Entity, query::{With, Without}, system::{Commands, Query, Res, ResMut, Resource}}, hierarchy::BuildChildren, input::{keyboard::KeyCode, ButtonInput}, math::Vec3, prelude::default, render::{color::Color, mesh::Mesh, texture::Image}, sprite::{MaterialMesh2dBundle, Mesh2dHandle, Sprite, SpriteBundle}, time::Time, transform::components::Transform, window::Window};






#[derive(bevy::ecs::component::Component)]
pub struct PlayerhitboxComp;

#[derive(Resource, Default)]
pub struct Shotcounter {
    timesincelastshot: f32
}
#[derive(Component, Default)]
pub struct Enemyproj;




const PLAYERSPRITESIZE: f32 = 32.0;
const FIRERATE: f32 = 0.1;
const VELO:f32 = 7.0;
#[derive(Resource, Default)]
pub struct Slowdown{
    truefalsechecker: bool,
    rate: f32,
    count: f32
}






//Projectile movement and lifetime
pub fn physloop(mut transform: Query<(Entity, &mut Transform), With<Refplayerproj>>, slow: Res<Slowdown>, window: Query<&Window>, mut commands: Commands , playerpos:Query<&Transform, (With<Refplayer>, Without<Refplayerproj>)>){ 
    //Sets the movement speed on projectiles and other checks

    if !transform.is_empty(){
        for (projent, mut projpos) in transform.iter_mut(){

            projpos.translation.y += (VELO*2.0)  /  slow.rate; 
            if projpos.translation.y > window.single().height()/2.{
                commands.entity(projent).despawn();

            }

        }
    }


   
}
//Timer for firing
pub fn guntimer(mut counter: ResMut<Shotcounter>, time: Res<Time>,commands: Commands,asset_server: Res<AssetServer>,x: Query<&mut Transform, With<Refplayer>>,slow: Res<Slowdown>){ //Sets firerate
    let mut pos = x.single().translation;

    if counter.timesincelastshot + FIRERATE * slow.rate  <= time.elapsed_seconds_wrapped() {//Fire rate added  with delay
        counter.timesincelastshot = time.elapsed_seconds_wrapped();
        pos.y = PLAYERSPRITESIZE/2. + pos.y + 5.0;
        projectile(commands, asset_server, pos)
    }


}










//initialize spawning player
pub fn spawnplayer(mut commands: Commands,asset_server: Res<AssetServer>, mut meshes: ResMut<Assets<Mesh>>,mut materials: ResMut<Assets<bevy::sprite::ColorMaterial>>){

    let x = commands.spawn(
        ((MaterialMesh2dBundle
            {mesh: Mesh2dHandle(meshes.add(bevy::math::primitives::Circle{radius: PLAYERSPRITESIZE/10.}))
            , material: materials.add(Color::RED)
            ,..default()}),PlayerhitboxComp)).id();
            //Adds a hitbox as a child

    commands.spawn(
        (
            SpriteBundle
            {sprite: Sprite{custom_size: Some(bevy::math::Vec2::new(PLAYERSPRITESIZE, PLAYERSPRITESIZE)), ..default()}
            ,texture: asset_server.load::<Image>("embedded://OIP.png")
            ,transform: Transform::from_xyz(0.0, 0.0, 0.0)
            , ..Default::default()},Refplayer))
            .add_child(x);
}
    




pub fn gethitbox(hitbox: Query<Entity, With<PlayerhitboxComp>>){
    
}







//Input loop and clamping
pub fn input(key:  Res<ButtonInput<KeyCode>>,mut query: Query<&mut Transform, With<Refplayer>>, time: Res<Time>, mut slowcheck: ResMut<Slowdown>, windows: Query<&Window>){



    let window = windows.single();

    let up = key.pressed(KeyCode::KeyW) || key.pressed(KeyCode::ArrowUp);
    let down = key.pressed(KeyCode::KeyS) || key.pressed(KeyCode::ArrowDown);
    let left = key.pressed(KeyCode::KeyA) || key.pressed(KeyCode::ArrowLeft);
    let right = key.pressed(KeyCode::KeyD) || key.pressed(KeyCode::ArrowRight);

    let mut dirx= [1.0, 1.0];
    let mut diry = [1.0, 1.0];

    let mut playerpos = query.single_mut();
    
    

    if key.pressed(KeyCode::ShiftLeft) || key.pressed(KeyCode::ControlLeft){ 
        slowcheck.truefalsechecker = true;
        if slowcheck.rate < 5.0{
            slowcheck.rate = ramp_up_function(0.0, 1.5, 0.455, -3.1, 5.0, slowcheck.count);
        }

        if slowcheck.count > 15.0{
            slowcheck.rate = 5.0
        }
        else {
            slowcheck.count += 0.01_f32.log(time.delta_seconds())/20.0    ;
        }

    }
    else {
        if slowcheck.truefalsechecker == true{
            slowcheck.count = slowcheck.count/5.0
        }
        slowcheck.truefalsechecker = false;
        if slowcheck.rate > 1.0{
            slowcheck.count -= 0.01_f32.log(time.delta_seconds())/20.0;
            slowcheck.rate = ramp_up_function(0.0, 0.5, 0.455, -3.1, 5.0, slowcheck.count);
        }
        if slowcheck.count < 0.0{
            slowcheck.count = 0.0;
        }


    }

    if slowcheck.rate < 1.0{
        slowcheck.rate = 1.0;
    }
    if key.pressed(KeyCode::KeyF){
        println!("rate {} count {}", slowcheck.rate,slowcheck.count);
        println!("test {} {}", (VELO*2.).sqrt()*2_f32.sqrt(), 1.0  * VELO )
    }

  
    //Clamping to screen

    let windowbox = [window.height()/2., window.width()/2.];
    let playerclamp = [playerpos.translation.x - PLAYERSPRITESIZE/2., playerpos.translation.x + PLAYERSPRITESIZE/2., playerpos.translation.y - PLAYERSPRITESIZE/2., playerpos.translation.y + PLAYERSPRITESIZE/2. ];
    if playerclamp[2] <= -windowbox[0]{//Bottom border
        diry[0] = 0.;
        playerpos.translation.y = -windowbox[0] + PLAYERSPRITESIZE/2.;
        }   
    if playerclamp[3] >= windowbox[0]{//Top border
        diry[1] = 0.;
        playerpos.translation.y = windowbox[0] - PLAYERSPRITESIZE/2.;
        }
    if playerclamp[1] >= windowbox[1]{//Right border
        dirx[1] = 0.;
        playerpos.translation.x = windowbox[1] - PLAYERSPRITESIZE/2.;
        } 
    if playerclamp[0] <= -windowbox[1]{//Left border
        dirx[0] = 0.;
        playerpos.translation.x = -windowbox[1] + PLAYERSPRITESIZE/2.;
        } 
        
        

        // Diag movement check + calculation
    if (left || right) && (up || down) {



            if up && left && !down && !right 
            {
                playerpos.translation.x -= (VELO*2.).sqrt()*2_f32.sqrt() /slowcheck.rate * dirx[0];
                playerpos.translation.y += (VELO*2.).sqrt()*2_f32.sqrt() /slowcheck.rate *diry[1];

            }
            else 
            {
                if up && left && down{
                    playerpos.translation.x -= (VELO*2.).sqrt()*2_f32.sqrt() /slowcheck.rate * dirx[0];
                }
                if up && left && right{
                    playerpos.translation.y += (VELO*2.).sqrt()*2_f32.sqrt() /slowcheck.rate *diry[1];
                }
            }




            if up && right && !down && !left{
                playerpos.translation.x += (VELO*2.).sqrt()*2_f32.sqrt() /slowcheck.rate * dirx[1];
                playerpos.translation.y += (VELO*2.).sqrt()*2_f32.sqrt() /slowcheck.rate *diry[1];
            }
            




            if down && right && !up && !left  
            {
                playerpos.translation.x += (VELO*2.).sqrt()*2_f32.sqrt() /slowcheck.rate * dirx[1];
                playerpos.translation.y -= (VELO*2.).sqrt()*2_f32.sqrt() /slowcheck.rate *diry[0];
            }
            else 
            {
                if right && left && down{
                    playerpos.translation.y -=(VELO*2.).sqrt()*2_f32.sqrt() /slowcheck.rate *diry[0];
                }
                if up && down && right{
                    playerpos.translation.x += (VELO*2.).sqrt()*2_f32.sqrt() /slowcheck.rate * dirx[1];
                }
            }



            if down && left && !up && !right
            {
                playerpos.translation.x -= (VELO*2.).sqrt()*2_f32.sqrt() /slowcheck.rate * dirx[0];
                playerpos.translation.y -= (VELO*2.).sqrt()*2_f32.sqrt() /slowcheck.rate *diry[0];
            }
    }


    //basic wasd movement
        else    
        {
            
        if left
        {
            playerpos.translation.x -= 1.0  * VELO  /slowcheck.rate * dirx[0];
        }




        if right
        {
            playerpos.translation.x += 1.0  * VELO /slowcheck.rate * dirx[1];
        }
    
    
        if up
        {
            playerpos.translation.y += 1.0  * VELO /slowcheck.rate *diry[1];
        }

        if down
        {
            playerpos.translation.y -= 1.0  * VELO /slowcheck.rate *diry[0]; 

        }
    }

}



#[derive(bevy::ecs::component::Component)]
pub struct Refplayer;


#[derive(bevy::ecs::component::Component)]
pub struct Refplayerproj;
//Spawn projectile
pub fn projectile(mut commands: Commands,asset_server: Res<AssetServer>, pos: Vec3){
    let path = "embedded://R.png" ; 
    commands.spawn((SpriteBundle{texture: asset_server.load::<Image>(path),transform: Transform::from_xyz(pos.x - PLAYERSPRITESIZE/4., pos.y, pos.z), sprite:{Sprite{custom_size: Some(bevy::math::Vec2::new(8., 8.)), ..Default::default()}},..Default::default()},Refplayerproj));
    commands.spawn((SpriteBundle{texture: asset_server.load::<Image>(path),transform: Transform::from_xyz(pos.x + PLAYERSPRITESIZE/4., pos.y, pos.z), sprite:{Sprite{custom_size: Some(bevy::math::Vec2::new(8., 8.)), ..Default::default()}},..Default::default()},Refplayerproj));
}   

fn ramp_up_function(a:f32, s:f32, h:f32, v:f32, c:f32, time:f32) -> f32{ //My favorite function (Modified Logistic curve)
    return c*(1.0/(1.0+std::f32::consts::E.powf(-h*(time/s+v))))+a;
}
  
