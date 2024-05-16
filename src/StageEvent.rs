use bevy::{ecs::{entity::Entity, query::{Changed, With}, schedule::{NextState, States}, system::{Commands, Query, ResMut, Resource}}, ui::{widget::Button, Interaction}};


use crate::Ui;
#[derive(Resource, PartialEq)]#[derive(Default)]


pub struct ChangeLevelEvent;




#[derive(States, Debug, Clone, PartialEq, Eq, Hash)]
pub enum GameState {
    LoadingScreen,
    MainMenu,
    InGame,
}








pub fn gamestatecheck(mut change_state: ResMut<NextState<GameState>> ,mut titlescreen: Query<Entity, With<Ui::GUI>>, interact: Query<&Interaction, (Changed<Interaction>,With<Button>)>, mut command: Commands){ //Communicates events with the event loader
   
    for item in interact.iter(){
        if *item == Interaction::Pressed{
           change_state.set(GameState::InGame);
           if !titlescreen.is_empty(){
            for item in titlescreen.iter_mut(){
                command.entity(item).despawn()
            }
        }
        }
    }
    
    
  

}

