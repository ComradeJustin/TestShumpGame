

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
                        , background_color:Color::rgba_u8(26, 20, 35, 255).into() // Ui background main
                        , ..Default::default()
                    })
                    .with_children(
                        |parent|
                        {
                            parent.spawn(ButtonBundle{
                                style: Style{
                                     width: bevy::ui::Val::Px(150.),
                                     height: bevy::ui::Val::Px(50.), 
                                     bottom: Val::Px(-100.),
                                     position_type: bevy::ui::PositionType::Relative,
                                     border: UiRect::all(Val::Px(2.0)), 
                                     justify_content: JustifyContent::Center, 
                                     justify_items: bevy::ui::JustifyItems::Center,
                                     align_items: bevy::ui::AlignItems::Center, 
                                    
                                     ..default()},
                                     border_color: bevy::ui::BorderColor(Color::BLACK),
                                     background_color: Color::rgba_u8(119, 76, 96, 255).into(),
                                     ..default()
                            
                            }
                        )
                        .with_children(
                            |parent|
                            {
                                parent.spawn(
                                
                                TextBundle::from_section(
                                "START GAME",
                                 TextStyle
                                 {
                                    font_size: 18.,
                                    color: Color::rgba_u8(234, 205, 194, 255).into(),
                                    font: asset_server.load(r#"Fonts\Mira.ttf"#),
                                    ..default()



                                 }
                                
                                
                                ));
                            }
                        );



                            parent.spawn(
                                TextBundle::from_section("AMOGUS", TextStyle 
                                {
                                    font_size: 80. ,
                                    font: asset_server.load(r#"Fonts\Mira.ttf"#),
                                    color: Color::rgba_u8(234, 205, 194, 255).into()
                                
                                
                                
                                
                                })



                                .with_style(Style{
                                    position_type: bevy::ui::PositionType::Absolute
                                    ,  align_self: bevy::ui::AlignSelf::Center
                                    ,  justify_self: bevy::ui::JustifySelf::Center, height: Val::Percent(80.) 
                                    ,  ..default()}));
                        });
                });
                

}