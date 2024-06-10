use bevy::{input::ButtonInput, prelude::{Commands, Event, EventReader, EventWriter, KeyCode, Query, Res}};



#[derive(Event)]
pub struct EnemyShoot(pub i8);

pub fn pattern(mut commands: Commands, mut firingevent: EventReader<EnemyShoot>){
    

    if !firingevent.is_empty(){
        for ev in firingevent.read(){
            match ev.0{
                1 => println!("what the sigma"),
                _ => panic!("should not be called")
            }
        }
    }
    
    
}
pub fn reader(mut sendev: EventWriter<EnemyShoot>, key:  Res<ButtonInput<KeyCode>>){
    if key.pressed(KeyCode::KeyK){
        sendev.send(EnemyShoot(1));
    }
}