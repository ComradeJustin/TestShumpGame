

use bevy::{asset::AssetServer, audio::CpalSample, ecs::{component::Component, entity::Entity, query::With, system::{Commands, Query, Res, ResMut, Resource}, world::Ref}, input::{keyboard::KeyCode, ButtonInput}, math::{primitives::{Circle, Rectangle}, Vec3}, prelude::default, sprite::{Sprite, SpriteBundle}, time::{self, Time}, transform::{self, components::Transform}, utils::petgraph::data, window::Window};






#[derive(bevy::ecs::component::Component)]
pub struct PlayerhitboxComp;

#[derive(Resource, Default)]
pub struct Shotcounter {
    timesincelastshot: f32
}
#[derive(Component, Default)]
pub struct Enemyproj{
    bullet_type: u8,
    owner: u8,
    size: u8 
}



const playerspritesize: f32 = 25.0;


#[derive(Resource, Default)]
pub struct Slowdown{
    truefalsechecker: bool,
    rate: f32,
    count: f32
}
#[derive(Resource, Default)]
pub struct GlobalChecker{
    dev_mode: bool
}




struct playerdata{
    position:Vec3,
    hp:f32,
    velocity:f32,
    direction:f32
    
}
impl Default for playerdata{
    fn default() -> Self {
        Self { position: Default::default(), hp: Default::default(), velocity: 300.0, direction: Default::default() }
    }

}
//Projectile movement and lifetime
pub fn physloop(mut transform: Query<(Entity, &mut Transform), With<Refplayerproj>>, time: Res<Time>,slow: Res<Slowdown>, window: Query<&Window>, mut commands: Commands){ //Sets the movement speed on projectiles and other checks

    if !transform.is_empty(){
        for (projent, mut projpos) in transform.iter_mut(){
            projpos.translation.y += 500.0 * time.delta_seconds() /  slow.rate; 
            if projpos.translation.y > window.single().height()/2.{
                commands.entity(projent).despawn();

            }
            
        }
    }


   
}
//Timer for firing
pub fn guntimer(mut counter: ResMut<Shotcounter>, time: Res<Time>,commands: Commands,asset_server: Res<AssetServer>,x: Query<&mut Transform, With<Refplayer>>,slow: Res<Slowdown>){ //Sets firerate
    let pos = x.single().translation;
    let delay:f32 = 0.25; //Fire rate scales with delay
    if counter.timesincelastshot*time.delta_seconds() + delay*time.delta_seconds() * slow.rate  <= time.elapsed_seconds() * time.delta_seconds(){
        counter.timesincelastshot = time.elapsed_seconds();
        
        projectile(commands, asset_server, pos)
    }


}




pub fn devmode(key: Res<ButtonInput<KeyCode>>, mut devstate: ResMut<GlobalChecker>){
    if key.pressed(KeyCode::AltLeft){
        devstate.dev_mode = true;
    }
    if devstate.dev_mode == true{
        println!("dev")
    }
}





/*pub fn collision_check(mut commands: Commands, player: Query<&mut Transform, With<Refplayer>>, enemyproj: Query<(Entity, &Transform), With<Enemyproj>>){
        let playerpos = player.single().1;
        
    
}*/














//Input loop and clamping
pub fn input(key:  Res<ButtonInput<KeyCode>>,mut query: Query<&mut Transform, With<Refplayer>>, time: Res<Time>, mut slowcheck: ResMut<Slowdown>, windows: Query<&Window>){

    let data = playerdata::default();
    let velo  = data.velocity;

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
            slowcheck.rate = RampUpFunction(0.0, 1.5, 0.455, -3.1, 5.0, slowcheck.count);
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
            slowcheck.rate = RampUpFunction(0.0, 0.5, 0.455, -3.1, 5.0, slowcheck.count);
        }
        if slowcheck.count < 0.0{
            slowcheck.count = 0.0;
        }


    }

    if slowcheck.rate < 1.0{
        slowcheck.rate = 1.0;
    }
    if key.pressed(KeyCode::KeyF){
        println!("rate {} count {}", slowcheck.rate,slowcheck.count)
    }


    //Clamping to screen

    let windowbox = [window.height()/2., window.width()/2.];
    let playerclamp = [playerpos.translation.x - playerspritesize/2., playerpos.translation.x + playerspritesize/2., playerpos.translation.y - playerspritesize/2., playerpos.translation.y + playerspritesize/2. ];
    if playerclamp[2] <= -windowbox[0]{//Bottom border
        diry[0] = 0.;
        playerpos.translation.y = -windowbox[0] + playerspritesize/2.;
        }   
    if playerclamp[3] >= windowbox[0]{//Top border
        diry[1] = 0.;
        playerpos.translation.y = windowbox[0] - playerspritesize/2.;
        }
    if playerclamp[1] >= windowbox[1]{//Right border
        dirx[1] = 0.;
        playerpos.translation.x = windowbox[1] - playerspritesize/2.;
        } 
    if playerclamp[0] <= -windowbox[1]{//Left border
        dirx[0] = 0.;
        playerpos.translation.x = -windowbox[1] + playerspritesize/2.;
        } 
        
        

        // Diag movement check + calculation
    if (left || right) && (up || down) {



            if up && left && !down && !right 
            {
                playerpos.translation.x -= (1.0 + (velo * velo / 2.0).sqrt() * time.delta_seconds()) /slowcheck.rate * dirx[0];
                playerpos.translation.y += (1.0 + (velo * velo / 2.0).sqrt() * time.delta_seconds()) /slowcheck.rate *diry[1];
            }
            else 
            {
                if up && left && down{
                    playerpos.translation.x -= (1.0 + (velo * velo / 2.0).sqrt() * time.delta_seconds()) /slowcheck.rate * dirx[0];
                }
                if up && left && right{
                    playerpos.translation.y += (1.0 + (velo * velo / 2.0).sqrt() * time.delta_seconds()) /slowcheck.rate *diry[1];
                }
            }




            if up && right && !down && !left{
                playerpos.translation.x += (1.0 + (velo * velo / 2.0).sqrt() * time.delta_seconds()) /slowcheck.rate * dirx[1];
                playerpos.translation.y += (1.0 + (velo * velo / 2.0).sqrt() * time.delta_seconds()) /slowcheck.rate *diry[1];
            }
            




            if down && right && !up && !left  
            {
                playerpos.translation.x += (1.0 + (velo * velo / 2.0).sqrt() * time.delta_seconds()) /slowcheck.rate * dirx[1];
                playerpos.translation.y -= (1.0 + (velo * velo / 2.0).sqrt() * time.delta_seconds()) /slowcheck.rate *diry[0];
            }
            else 
            {
                if right && left && down{
                    playerpos.translation.y -=(1.0 + (velo * velo / 2.0).sqrt() * time.delta_seconds()) /slowcheck.rate *diry[0];
                }
                if up && down && right{
                    playerpos.translation.x += (1.0 + (velo * velo / 2.0).sqrt() * time.delta_seconds()) /slowcheck.rate * dirx[1];
                }
            }



            if down && left && !up && !right
            {
                playerpos.translation.x -= (1.0 + (velo * velo / 2.0).sqrt() * time.delta_seconds()) /slowcheck.rate * dirx[0];
                playerpos.translation.y -= (1.0 + (velo * velo / 2.0).sqrt() * time.delta_seconds()) /slowcheck.rate *diry[0];
            }
    }


    //basic wasd movement
        else    
        {
            
        if left
        {
            playerpos.translation.x -= 1.0 * time.delta_seconds() * velo  /slowcheck.rate * dirx[0];
        }




        if right
        {
            playerpos.translation.x += 1.0 * time.delta_seconds() * velo /slowcheck.rate * dirx[1];
        }
    
    
        if up
        {
            playerpos.translation.y += 1.0 * time.delta_seconds() * velo /slowcheck.rate *diry[1];
        }

        if down
        {
            playerpos.translation.y -= 1.0 * time.delta_seconds() * velo /slowcheck.rate *diry[0]; 

        }
    }

}



#[derive(bevy::ecs::component::Component)]
pub struct Refplayer;
//initialize spawning player
pub fn spawnplayer(mut commands: Commands,asset_server: Res<AssetServer>){
    commands.spawn((SpriteBundle{sprite: Sprite{custom_size: Some(bevy::math::Vec2::new(playerspritesize, playerspritesize)), ..default()},texture: asset_server.load(r#"OIP.png"#),transform: Transform::from_xyz(100.00, 0.0, 5.0), ..Default::default()},Refplayer));
}
#[derive(bevy::ecs::component::Component)]
pub struct Refplayerproj;
//Spawn projectile
pub fn projectile(mut commands: Commands,asset_server: Res<AssetServer>, pos: Vec3){
    commands.spawn((SpriteBundle{texture: asset_server.load(r#"R.png"#),transform: Transform::from_xyz(pos.x, pos.y+10.0, pos.z).with_scale(Vec3::splat(0.01)), ..Default::default()},Refplayerproj));
}

fn RampUpFunction(a:f32, s:f32, h:f32, v:f32, c:f32, time:f32) -> f32{ //My favorite function (Modified Logistic curve)
    return c*(1.0/(1.0+std::f32::consts::E.powf(-h*(time/s+v))))+a;
}
  
