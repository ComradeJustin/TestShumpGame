

use bevy::{a11y::accesskit::Vec2, ecs::system::Commands, hierarchy::{BuildChildren, Parent}, prelude::default, render::color::Color, text::{Text, Text2dBundle, TextSection, TextStyle}, ui::{node_bundles::{ButtonBundle, NodeBundle}, JustifyContent, Style, UiRect, Val}};



pub fn variable_text(input: String, color: Color, pos: Vec2 ){

    let x = Text2dBundle {
        text: Text {
            sections: vec![TextSection::new(format!("{}", input),TextStyle {color   ,..Default::default()},)],
            ..Default::default()
        },
        ..Default::default()
    };
    
}



pub fn render_title_screen(mut command: Commands){  
    command.spawn(NodeBundle
        {style: Style{
            width: bevy::ui::Val::Percent(100.)
            , height: bevy::ui::Val::Percent(100.)
            , justify_content: JustifyContent::SpaceBetween
            , ..default()}
        
        ,..default()})
        .with_children(
            |parent| 
            {parent.spawn(
                NodeBundle
                {style: Style
                    {
                        width: bevy::ui::Val::Percent(100.),
                        align_items: bevy::ui::AlignItems::Center   
                        , justify_content: JustifyContent::Center
                        , height: bevy::ui::Val::Percent(100.)
                        , ..Default::default()}
                        , background_color: Color::rgb(100., 0.65, 0.65).into()
                        , ..Default::default()
                    }).with_children(
                        |parent|
                        {
                            parent.spawn(ButtonBundle{
                                style: Style{width: bevy::ui::Val::Px(100.),
                                     height: bevy::ui::Val::Px(20.), 
                                     border: UiRect::all(Val::Px(0.5)), 
                                     justify_content: JustifyContent::Center, 
                                     align_items: bevy::ui::AlignItems::Center, 
                                     ..default()},
                                     border_color: bevy::ui::BorderColor(Color::PINK),
                                     background_color: Color::MIDNIGHT_BLUE.into(),
                                     ..default()
                                
                            });
    
                        });
                });
                

}