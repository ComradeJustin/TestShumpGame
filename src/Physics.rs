

use std::path::PathBuf;

use bevy::{asset::{AssetServer, Assets}, ecs::{component::Component, entity::Entity, event::{Event, EventReader, EventWriter}, query::{With, Without}, system::{Commands, Query, Res, ResMut, Resource}}, hierarchy::BuildChildren, input::{keyboard::KeyCode, ButtonInput}, log::debug, math::Vec3, prelude::default, reflect::Reflect, render::{color::Color, mesh::Mesh, texture::Image}, sprite::{MaterialMesh2dBundle, Mesh2dHandle, Sprite, SpriteBundle}, time::{Stopwatch, Time}, transform::components::{GlobalTransform, Transform}, window::Window};






#[derive(bevy::ecs::component::Component)]
pub struct PlayerhitboxComp;
#[derive(bevy::ecs::component::Component)]
pub struct ProjectileSpeedDir{
    speedfactor: f32,
}

#[derive(Resource, Default)]
pub struct Shotcounter {
    timesincelastshot: f32
}
#[derive(Component, Default)]
pub struct Enemyproj{
    pub bullettype:i8,
    pub id: i32,
    pub angle: f32
}



#[derive(Event)]
pub struct PlayerVel(pub f32);

const PLAYERSPRITESIZE: f32 = 32.0;
const FIRERATE: f32 = 0.13;
const VELO:f32 = 3.0;
const HITBOXRADIUS:f32 = 15.0;
pub const ENEMYTESTPROJ:f32 = 30.0;
#[derive(Resource, Default)]
pub struct Slowdown{
    truefalsechecker: bool,
    pub rate: f32,
    count: f32,

}

#[derive(Resource, Default)]
pub struct PlayerData{
    pub lives:i32,
    pub points: i32,
    pub power: f32,
    pub iframes: bool,
    timer: Stopwatch,
    flash: Stopwatch,

}






//Projectile movement and lifetime
pub fn physloop(mut proj: Query<(Entity, &mut Transform, &ProjectileSpeedDir), With<Refplayerproj>>, slow: Res<Slowdown>, window: Query<&Window>
    ,mut commands: Commands ){ 
    
    //Sets the movement speed on projectiles and other checks

    if !proj.is_empty() {

        for (projent, mut projpos   , dir) in proj.iter_mut(){
            
            projpos.translation.y += (6.+ dir.speedfactor ) /  slow.rate ;

            projpos.translation.y = (projpos.translation.y*10.).round()/10.;
            projpos.translation.x = projpos.translation.x.round();



            if projpos.translation.y > window.single().physical_height() as f32/2.{
                commands.entity(projent).despawn();

            }

        }
    }
    


   
}
//Timer for firing
pub fn guntimer(mut counter: ResMut<Shotcounter>, time: Res<Time>,commands: Commands,
    asset_server: Res<AssetServer>,x: Query<&mut Transform, With<Refplayer>>,slow: Res<Slowdown>
    ,mut ev_veloread: EventReader<PlayerVel>){ //Sets firerate

    
    
    let mut pos = x.single().translation.round();
    if counter.timesincelastshot + FIRERATE * slow.rate  <= time.elapsed_seconds_wrapped() {//Fire rate added  with delay
        counter.timesincelastshot = time.elapsed_seconds_wrapped();
        pos.y = PLAYERSPRITESIZE/2. + pos.y + 2.0;
        pos.z = x.single().translation.z - 10.0;
        if !ev_veloread.is_empty(){

            projectile(commands, asset_server, pos, ev_veloread.read().peekable().peek().unwrap().0)
        }
        else {
            projectile(commands, asset_server, pos, 0.0)
        }
        
    }


}










//initialize spawning player
pub fn spawnplayer(mut commands: Commands,asset_server: Res<AssetServer>,mut pd: ResMut<PlayerData>){
    pd.lives = 3;
    pd.power = 0.0;
    pd.points = 0;
    pd.iframes = false;



    let x = commands.spawn((
        SpriteBundle
            {sprite: Sprite{custom_size: Some(bevy::math::Vec2::new(HITBOXRADIUS,HITBOXRADIUS)), ..default()}
            ,texture: asset_server.load::<Image>("embedded://Hitbox.png")
            , ..Default::default()},PlayerhitboxComp)).id();
            //Adds a hitbox as a child

    commands.spawn(
        (
            SpriteBundle
            {sprite: Sprite{custom_size: Some(bevy::math::Vec2::new(PLAYERSPRITESIZE, PLAYERSPRITESIZE)), ..default()}
            ,texture: asset_server.load::<Image>("embedded://OIP.png")
            ,transform: Transform::from_xyz(0.0, 0.0, -1.0)
            , ..Default::default()},Refplayer))
            .add_child(x);
    
    
    

}
    


// Calculates hitbox and makes enemy kill you

pub fn gethitbox(origin: Query<&GlobalTransform, With<PlayerhitboxComp>>, 
    enemy: Query<&Transform, With<Enemyproj>>,mut pd:  ResMut<PlayerData>
    ,time: Res<Time> ,mut player: Query<&mut Sprite, With<Refplayer>>, slowcheck: Res<Slowdown> ){

    let max:f32 = 100.0;

    if !enemy.is_empty() && pd.iframes == false{
        for entity in enemy.iter(){
            if (origin.single().translation().distance(entity.translation)).abs() <=  (ENEMYTESTPROJ + HITBOXRADIUS)/2. {
                pd.lives -= 1; //Lose life code
                pd.iframes = true;
                
            }
        }
    }



    if pd.iframes == true{


        if (pd.flash.elapsed_secs()*300./slowcheck.rate).clamp(max/2., max) >= max{
            pd.flash.reset()
        }
        pd.timer.tick(time.delta());
        pd.flash.tick(time.delta());




        if (pd.flash.elapsed_secs()*300./slowcheck.rate).clamp(max/2., max)  > max/2.0 {
            player.single_mut().color.set_a(1.0);
            player.single_mut().color.set_r(1.0);
            player.single_mut().color.set_g(1.0);
            player.single_mut().color.set_b(1.0);
        }
        else if (pd.flash.elapsed_secs()*300./slowcheck.rate).clamp(0.0, max/2.)  < max/2. {
            player.single_mut().color.set_a(0.2);
            player.single_mut().color.set_r(1.0);
            player.single_mut().color.set_g(0.0);
            player.single_mut().color.set_b(0.0);
        }

        

        println!("{}, {}", 


        (pd.flash.elapsed_secs()*300./slowcheck.rate).clamp(0.0, max/2.),
        (pd.flash.elapsed_secs()*300./slowcheck.rate).clamp(max/2., max)

        
    
    );
        

        

        if pd.timer.elapsed_secs() / slowcheck.rate  >= 5.0{
            pd.flash.reset();
            pd.iframes = false;
            pd.timer.reset();
            println!("{}", pd.lives);
            player.single_mut().color.set_r(1.0);
            player.single_mut().color.set_g(1.0);
            player.single_mut().color.set_b(1.0);
            player.single_mut().color.set_a(1.0);
        }
    }
   
}
    







//Input loop and clamping
pub fn input(key:  Res<ButtonInput<KeyCode>>,mut query: Query<&mut Transform, With<Refplayer>>, time: Res<Time>
    ,mut slowcheck: ResMut<Slowdown>, windows: Query<&Window>
    ,mut hitbox: Query<&mut Sprite, (With<PlayerhitboxComp>, Without<Refplayerproj>)>
    ,mut ev_ismoving: EventWriter<PlayerVel>){


    
    let window = windows.single();

    let up = key.pressed(KeyCode::KeyW) || key.pressed(KeyCode::ArrowUp);
    let down = key.pressed(KeyCode::KeyS) || key.pressed(KeyCode::ArrowDown);
    let left = key.pressed(KeyCode::KeyA) || key.pressed(KeyCode::ArrowLeft);
    let right = key.pressed(KeyCode::KeyD) || key.pressed(KeyCode::ArrowRight);

    let mut dirx= [1.0, 1.0];
    let mut diry = [1.0, 1.0];

    let mut playerpos = query.single_mut();
    


    if key.pressed(KeyCode::ShiftLeft) || key.pressed(KeyCode::ControlLeft){ 
        hitbox.single_mut().color.set_a(1.0);
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

        playerpos.translation = (playerpos.translation*1000.).round()/1000.;


    }
    else {



        playerpos.translation = playerpos.translation.round();  //Pixel perfect movement, as 1 unit in game is 1 unit in screen
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

        hitbox.single_mut().color.set_a(0.0);
    }

    if slowcheck.rate < 1.0{
        slowcheck.rate = 1.0;
    }

    


  
    //Clamping to screen

    let windowbox = [window.physical_height() as f32/2., window.physical_width() as f32/2.];
    let playerclamp = [playerpos.translation.x - PLAYERSPRITESIZE/2., playerpos.translation.x + PLAYERSPRITESIZE/2., playerpos.translation.y - PLAYERSPRITESIZE/2., playerpos.translation.y + PLAYERSPRITESIZE/2. ];
    if playerclamp[2] <= -windowbox[0]{//Bottom border
        diry[0] = 0.;
        playerpos.translation.y = (-windowbox[0] + PLAYERSPRITESIZE/2.).round();
        }   
    if playerclamp[3] >= windowbox[0]{//Top border
        diry[1] = 0.;
        playerpos.translation.y = (windowbox[0] - PLAYERSPRITESIZE/2.).round();
        }
    if playerclamp[1] >= windowbox[1]{//Right border
        dirx[1] = 0.;
        playerpos.translation.x = (windowbox[1] - PLAYERSPRITESIZE/2.).round();
        } 
    if playerclamp[0] <= -windowbox[1]{//Left border
        dirx[0] = 0.;
        playerpos.translation.x = (-windowbox[1] + PLAYERSPRITESIZE/2.).round();
        } 
        



    // Diag movement check + calculation
    if (left || right) && (up || down) {



            if up && left && !down && !right 
            {
                playerpos.translation.x -= (VELO*2.).sqrt()*2_f32.sqrt() /slowcheck.rate * dirx[0];
                playerpos.translation.y += (VELO*2.).sqrt()*2_f32.sqrt() /slowcheck.rate *diry[1];
                ev_ismoving.send(PlayerVel((VELO*2.).sqrt()*2_f32.sqrt()));

            }
            else 
            {
                if up && left && down{
                    playerpos.translation.x -= (VELO*2.).sqrt()*2_f32.sqrt() /slowcheck.rate * dirx[0];

                }
                if up && left && right{
                    playerpos.translation.y += (VELO*2.).sqrt()*2_f32.sqrt() /slowcheck.rate *diry[1];
                    ev_ismoving.send(PlayerVel((VELO*2.).sqrt()*2_f32.sqrt()));
                }
            }




            if up && right && !down && !left{
                playerpos.translation.x += (VELO*2.).sqrt()*2_f32.sqrt() /slowcheck.rate * dirx[1];
                playerpos.translation.y += (VELO*2.).sqrt()*2_f32.sqrt() /slowcheck.rate *diry[1];
                ev_ismoving.send(PlayerVel((VELO*2.).sqrt()*2_f32.sqrt()));
            }
            




            if down && right && !up && !left  
            {
                playerpos.translation.x += (VELO*2.).sqrt()*2_f32.sqrt() /slowcheck.rate * dirx[1];
                playerpos.translation.y -= (VELO*2.).sqrt()*2_f32.sqrt() /slowcheck.rate *diry[0];
                ev_ismoving.send(PlayerVel(-((VELO*2.).sqrt()*2_f32.sqrt())));
            }
            else 
            {
                if right && left && down{
                    playerpos.translation.y -=(VELO*2.).sqrt()*2_f32.sqrt() /slowcheck.rate *diry[0];
                    ev_ismoving.send(PlayerVel(-((VELO*2.).sqrt()*2_f32.sqrt())));
                }
                if up && down && right{
                    playerpos.translation.x += (VELO*2.).sqrt()*2_f32.sqrt() /slowcheck.rate * dirx[1];
                }
            }



            if down && left && !up && !right
            {
                ev_ismoving.send(PlayerVel(-((VELO*2.).sqrt()*2_f32.sqrt())));
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
            ev_ismoving.send(PlayerVel(VELO));

        }

        if down
        {
            playerpos.translation.y -= 1.0  * VELO /slowcheck.rate *diry[0]; 
            ev_ismoving.send(PlayerVel(-VELO));

        }
        
    }
    


}



#[derive(bevy::ecs::component::Component)]
pub struct Refplayer;


#[derive(bevy::ecs::component::Component)]
pub struct Refplayerproj;
//Spawn projectile
pub fn projectile(mut commands: Commands,asset_server: Res<AssetServer>, pos: Vec3, id: f32){
    let path = "embedded://R.png" ; 

    //Give Individual ID to each single projectile based on 3 states: And then Compensate doppler effect based on that 
    commands.spawn((SpriteBundle{texture: asset_server.load::<Image>(path),transform: Transform::from_xyz(pos.x - PLAYERSPRITESIZE/3., pos.y, pos.z), sprite:{Sprite{custom_size: Some(bevy::math::Vec2::new(8., 8.)), ..Default::default()}},..Default::default()}
    ,Refplayerproj, ProjectileSpeedDir{speedfactor: id}));
    
    commands.spawn(
        (SpriteBundle
            {texture: asset_server.load::<Image>(path)
            ,transform: Transform::from_xyz(pos.x + PLAYERSPRITESIZE/3., pos.y, pos.z)
            , sprite:{Sprite{custom_size: Some(bevy::math::Vec2::new(8., 8.))
            , ..Default::default()}}
            ,..Default::default()},Refplayerproj,ProjectileSpeedDir{speedfactor: id }));




}   

fn ramp_up_function(a:f32, s:f32, h:f32, v:f32, c:f32, time:f32) -> f32{ //My favorite function (Modified Logistic curve)
    return c*(1.0/(1.0+std::f32::consts::E.powf(-h*(time/s+v))))+a;
}
  
