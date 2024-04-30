use bevy::{self, ecs::system::{Commands, Resource}};

#[derive(Resource)]
struct CurrentState{
    screen_type: bool,
    currentlevel: u32,
}


pub fn startup(command: Commands){
    
}

pub fn gamestatecheck(){
    
}