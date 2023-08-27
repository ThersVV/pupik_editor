use crate::{
    mouse::{BuiltItem, ItemName},
    structure_ui::{spawn_eraser, ApplyDefaultColoring, EraserButton, UISprite},
    ExportSheet,
};
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use std::fs::File;
use std::io::{BufWriter, Write};

#[derive(Component)]
pub struct ExportButton;

#[derive(Component)]
pub struct SingleUse;

pub struct ExportPlugin;

impl Plugin for ExportPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostStartup, spawn_export_button)
            .add_systems(Update, export_button_interaction);
    }
}

fn export(item_query: &Query<(&Transform, &ItemName), With<BuiltItem>>) {
    let file = File::create("./export").expect("unable to create file");
    let mut file = BufWriter::new(file);
    for (transform, name) in item_query.iter() {
        let trans = transform.translation;
        let line: String = (trans.x as i32).to_string()
            + " "
            + &(trans.y as i32).to_string()
            + " "
            + name.name
            + "\n";
        file.write_all(line.as_bytes())
            .expect("Unable to write into created file");
    }
}

pub fn spawn_export_button(
    mut commands: Commands,
    sheet: Res<ExportSheet>,
    q_windows: Query<&Window, With<PrimaryWindow>>,
    assets: Res<Assets<TextureAtlas>>,
) {
    commands
        .spawn(ButtonBundle {
            style: Style {
                position_type: PositionType::Absolute,
                width: Val::Px(80.),
                height: Val::Px(80.),
                right: Val::Percent(3.),
                top: Val::Percent(15.),
                border: UiRect::all(Val::Px(1.)),
                ..default()
            },
            //image: assets.load("eraser.png").into(),
            border_color: Color::rgba(0., 0., 0., 1.0).into(),
            background_color: Color::NONE.into(),
            ..default()
        })
        .insert(ExportButton)
        .insert(ApplyDefaultColoring)
        .insert(SingleUse);

    let window = q_windows.single();
    let (w_width, w_height) = (window.width(), window.height());
    let scale = 80. / 218.;

    let texture = assets.get(&sheet.0).unwrap();
    let sprite_width = texture.size.x / texture.len() as f32;
    let sprite_height = texture.size.y;

    commands
        .spawn(SpriteSheetBundle {
            sprite: TextureAtlasSprite::new(0),
            texture_atlas: sheet.0.clone(),
            transform: Transform {
                translation: Vec3::new(w_width * 0.47 - 40., w_height * 0.35 - 40., 900.),
                scale: Vec3::splat(scale),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(UISprite {
            sprite_size: Vec2::new(sprite_width * scale, sprite_height * scale),
        });
}
fn export_button_interaction(
    eraser_button_q: Query<&Interaction, (Changed<Interaction>, With<ExportButton>)>,
    item_query: Query<(&Transform, &ItemName), With<BuiltItem>>,
) {
    for interaction in eraser_button_q.iter() {
        match *interaction {
            Interaction::Pressed => {
                export(&item_query);
            }
            _ => {}
        }
    }
}
