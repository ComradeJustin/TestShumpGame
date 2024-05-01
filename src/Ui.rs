

use bevy::{a11y::accesskit::Vec2, asset::AssetServer, ecs::system::{Commands, Res}, hierarchy::{BuildChildren, ChildBuilder, Parent}, prelude::default, render::color::Color, text::{Text, Text2dBundle, TextSection, TextStyle}, ui::{node_bundles::{ButtonBundle, NodeBundle, TextBundle}, JustifyContent, Style, UiRect, Val}};



pub fn variable_text(input: String, color: Color, pos: Vec2 ){

    let x = Text2dBundle {
        text: Text {
            sections: vec![TextSection::new(format!("{}", input),TextStyle {color   ,..Default::default()},)],
            ..Default::default()
        },
        ..Default::default()
    };
    
}



pub fn render_title_screen(mut command: Commands, asset_server: Res<AssetServer>){  
    command.spawn(NodeBundle
        {style: Style{
            width: bevy::ui::Val::Percent(100.)
            , height: bevy::ui::Val::Percent(100.)
            , justify_content: JustifyContent::SpaceBetween
            , ..default()}
        
        ,..default()})
        .with_children(
            |parent| 
            {
                
                
                parent.spawn(NodeBundle{

                    style: Style
                    {
                        width: bevy::ui::Val::Percent(100.),
                        align_items: bevy::ui::AlignItems::Center   
                        , justify_content: JustifyContent::Center
                        , height: bevy::ui::Val::Percent(100.)
                        , ..Default::default()}
                        , background_color: Color::rgb(100., 0.65, 0.65).into()
                        , ..Default::default()
                    })
                    .with_children(
                        |parent|
                        {
                            parent.spawn(ButtonBundle{
                                style: Style{
                                    width: bevy::ui::Val::Px(120.),
                                     height: bevy::ui::Val::Px(40.), 
                                     bottom: Val::Px(-100.),
                                     position_type: bevy::ui::PositionType::Relative,
                                     border: UiRect::all(Val::Px(0.5)), 
                                     justify_content: JustifyContent::Center, 
                                     align_items: bevy::ui::AlignItems::Center, 
                                     ..default()},
                                     border_color: bevy::ui::BorderColor(Color::BLACK),
                                     background_color: Color::rgb(100., 10., 0.).into(),
                                     ..default()
                            
                            }
                        )
                        .with_children(
                            |parent|
                            {
                                parent.spawn(
                                
                                TextBundle::from_section(
                                "Start Game",
                                 TextStyle
                                 {
                                    font_size: 15.,
                                    color: Color::rgb(0.,0.,0.),
                                    font: asset_server.load(r#"assets\fonts\font.ttf"#),



                                 }
                                
                                
                                ));
                            }
                        );



                            parent.spawn(
                                TextBundle::from_section("Amogus Sussy ", TextStyle {font_size: 80. ,  ..default()})
                                .with_style(Style{position_type: bevy::ui::PositionType::Absolute,  align_self: bevy::ui::AlignSelf::Center
                                    ,  justify_self: bevy::ui::JustifySelf::Center, height: Val::Percent(80.) ,  ..default()}));
                        });
                });
                

}