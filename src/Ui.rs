
use std::process::Command;

use bevy::{a11y::accesskit::Vec2, render::color::Color, text::{Text, Text2dBundle, TextSection, TextStyle}};



pub fn variable_text(input: String, color: Color, pos: Vec2 ){

    let x = Text2dBundle {
        text: Text {
            sections: vec![TextSection::new(format!("{}", input),TextStyle {color   ,..Default::default()},)],
            ..Default::default()
        },
        ..Default::default()
    };
    
}