use crate::{
    mouse::{BuiltItem, ItemName},
    structure_ui::{ApplyDefaultColoring, UISprite},
    {ExportSheet, UiState}, WhiteSheet,
};
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy_egui::{egui::Rgba, *};
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
            .add_systems(Update, export_button_interaction)
            .add_systems(Startup, user_input_background)
            .add_systems(Update, user_input)
            /* .add_startup_system(configure_visuals_system) */;
        //.add_startup_system(configure_ui_state_system);
    }
}

fn export(item_query: &Query<(&Transform, &ItemName), With<BuiltItem>>, ui_state: &Res<UiState>) {
    if let Err(_) = std::fs::create_dir_all("./export") {
        return;
    };
    if !ui_state.ready_to_export {
        return;
    };
    let export_path = "./structures/".to_owned() + if ui_state.name == "" {"export"} else {&ui_state.name};
    let weight = &ui_state.weight_s;
    let file = File::create(export_path).expect("Unable to create file");
    let mut file = BufWriter::new(file);
    file.write_all((weight.to_owned() + "\n").as_bytes()).expect("Unable to write into created file");
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


fn user_input_background(
    mut commands: Commands,
    sheet: Res<WhiteSheet>,
    q_windows: Query<&Window, With<PrimaryWindow>>,){

        let window = q_windows.single();
        let (w_width, w_height) = (window.width(), window.height());

        let scale = (w_width - (1920. / 3.)) / 2.;
        commands
        .spawn(SpriteSheetBundle {
            sprite: TextureAtlasSprite::new(0),
            texture_atlas: sheet.0.clone(),
            transform: Transform {
                translation: Vec3::new((1920. / 3.) / 2. + scale / 2., w_height * 0.117, 901.),
                scale: Vec3::new(scale,165.,1.),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(UISprite {
            sprite_size: Vec2::new(scale, 165.),
        });
}
fn user_input(
    mut ui_state: ResMut<UiState>,
    q_windows: Query<&Window, With<PrimaryWindow>>,
    mut contexts: EguiContexts,
) {
    let ctx = contexts.ctx_mut();
    let window = q_windows.single();
    let (w_width, w_height) = (window.width(), window.height());
    let x = (w_width + (1920. / 3.)) / 2. + 10.;

    let mut style: egui::Style = (*ctx.style()).clone();
    for (_text_style, font_id) in style.text_styles.iter_mut() {
        font_id.size = 16.5; 
    }
    ctx.set_style(style);

    egui::Area::new("area")
        .fixed_pos(egui::pos2(x, w_height * 0.28))
        .show(ctx, |ui| {

            ui.horizontal(|ui| {
                ui.colored_label(
                    Rgba::BLACK,"File name:");
                ui.text_edit_singleline(&mut ui_state.name);
            });
            
            ui_state.name = ui_state.name.replace('.', "");

            ui.horizontal(|ui| {
                ui.colored_label(
                    Rgba::BLACK,"Relative weight:");
                ui.text_edit_singleline(&mut ui_state.weight_s);
            });
            ui_state.weight_s = ui_state.weight_s.trim().replace(',', ".");
            if let Err(_e) = ui_state.weight_s.parse::<f64>(){
                ui.colored_label(
                    Rgba::RED,
                    "PLEASE ENTER A VALID FLOAT NUMBER",
                );
                ui_state.ready_to_export = false;
            } else {
                ui_state.ready_to_export = true;

            }

            ui.colored_label(
                Rgba::BLACK,
                "Some example weights:\n   Rainbow is 0.2\n   Basic enemy is 119\n   Energy bar is 12",
            );
        });
        
}

fn export_button_interaction(
    eraser_button_q: Query<&Interaction, (Changed<Interaction>, With<ExportButton>)>,
    item_query: Query<(&Transform, &ItemName), With<BuiltItem>>,
    ui_state: Res<UiState>,
) {
    for interaction in eraser_button_q.iter() {
        match *interaction {
            Interaction::Pressed => {
                export(&item_query, &ui_state);
            }
            _ => {}
        }
    }
}
