use bevy::{self, app::App, ecs::{event::{Event, EventReader}, query::{Changed, With}, system::{Commands, Query, ResMut, Resource}}, ui::{widget::Button, Interaction}};

#[derive(Resource)]
pub struct CurrentState{
    screen_type: bool, // false means menu ui
    currentlevel: u32,
}


pub fn startup(mut command: Commands){
    command.insert_resource(CurrentState{screen_type: false, currentlevel: 0});
    //Load MenuUI
}
#[derive(Event)]
pub struct StageEvent(u8);

pub fn gamestatecheck(mut gamestate: ResMut<CurrentState>, interact: Query<&Interaction, (Changed<Interaction>,With<Button>)>, command: Commands){ //Communicates events with the event loader
    for item in interact.iter(){
        if *item == Interaction::Pressed{
           gamestate.screen_type = true;
        }
    }
    if gamestate.screen_type == true && gamestate.currentlevel == 0{
        //Load Into game
        gamestate.currentlevel = 1;
    }

  

}
fn return_stage_id(getstage: EventReader<StageEvent>) -> u8{
    for event in getstage.read(){
        return event.0;
    }
    
}

