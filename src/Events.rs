use bevy::{self, app::App, ecs::{component, entity::Entity, event::{Event, EventReader}, query::{Changed, With}, system::{Commands, Query, Res, ResMut, Resource}}, hierarchy::BuildChildren, transform::commands, ui::{widget::Button, Interaction}};


use crate::Ui;
#[derive(Resource, PartialEq)]
pub struct CurrentState{
    pub screen_type: bool, // false means menu ui
    pub currentlevel: u32,
}


pub fn startup(mut command: Commands){
    command.insert_resource(CurrentState{screen_type: false, currentlevel: 0});
    //Load MenuUI
}


pub fn gamestatecheck(mut gamestate: ResMut<CurrentState>, mut titlescreen: (Query<Entity, With<Ui::GUI>>), interact: Query<&Interaction, (Changed<Interaction>,With<Button>)>, mut command: Commands){ //Communicates events with the event loader
   
    for item in interact.iter(){
        if *item == Interaction::Pressed{
           gamestate.screen_type = true;
        }
    }
    if gamestate.screen_type == true && gamestate.currentlevel == 0{
        //Load Into game
        if !titlescreen.is_empty(){
            for item in titlescreen.iter_mut(){
                command.entity(item).despawn()
            }
        }
        else {
            gamestate.currentlevel = 1
        }
        


    }

  

}
pub fn return_current_level(gamestate: Res<CurrentState>) -> u32{
    return gamestate.currentlevel
}

