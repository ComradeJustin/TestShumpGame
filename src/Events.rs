use bevy::{self, ecs::{query::{Changed, With}, system::{Commands, Query, ResMut, Resource}}, ui::{widget::Button, Interaction}};

#[derive(Resource)]
pub struct CurrentState{
    screen_type: bool, // false means menu ui
    currentlevel: u32,
}


pub fn startup(mut command: Commands){
    command.insert_resource(CurrentState{screen_type: false, currentlevel: 0});
    //Load MenuUI
}

pub fn gamestatecheck(mut gamestate: ResMut<CurrentState>, interact: Query<&Interaction, (Changed<Interaction>,With<Button>)>){
    for item in interact.iter(){
        if *item == Interaction::Pressed{
            println!("PRESSED")
        }
    }

  

}