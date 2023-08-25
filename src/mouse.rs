use crate::{
    structure_ui::{overlaps_ui, UISprite},
    CombinedSheet, GameState,
};
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use rand::random;

#[derive(Component)]
pub struct EditorTool {
    is_left_clicked: bool,
}

#[derive(Component)]
pub struct BuiltItem;

#[derive(Component)]
struct Selected;

pub struct MousePlugin;

impl Plugin for MousePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_editor_tool)
            .add_systems(
                Update,
                (spawn_selected_item).run_if(in_state(GameState::Building)),
            )
            .add_systems(
                Update,
                (nothing).run_if(in_state(GameState::Erasing)),
                //TODO: hodně věcí brácho, ale ty to dáš. Pro Aničku <3.
                // Zbarvování buttonů lepší by to chtělo ...  A pak jsou na čase kolize. Asi kašli na ray casting,
                // i když to zní cool af.
                // Stačí přes souřadnice. Pak teda to mazání. Pak už možná sloupečky a export?
                //Případně funkce, co ještě půjdou udělat, jsou ctrlZ, lepší ukazování letadel,
                //deselektování nefunguje
            )
            .add_systems(Update, (movement));
    }
}

fn movement(
    mut editor_tool_query: Query<&mut Transform, With<EditorTool>>,
    windows_q: Query<&Window, With<PrimaryWindow>>,
) {
    for mut transform in editor_tool_query.iter_mut() {
        let window = windows_q.single();
        if let Some(position) = window.cursor_position() {
            transform.translation.x = position.x - (window.width() / 2.);
            transform.translation.y = -1. * position.y + (window.height() / 2.);
        }
    }
}

fn spawn_editor_tool(mut commands: Commands, texture_atlas: Res<CombinedSheet>) {
    let sprite = TextureAtlasSprite::new(0);

    let editor_tool = commands
        .spawn(SpriteSheetBundle {
            sprite,
            texture_atlas: texture_atlas.0.clone(),
            transform: Transform {
                translation: Vec3::new(100.0, 100.0, 900.0),
                scale: Vec3::splat(1.0),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(EditorTool {
            is_left_clicked: false,
        })
        .id();
    commands.entity(editor_tool);
}

fn spawn_selected_item(
    mut commands: Commands,
    texture_atlas: Res<CombinedSheet>,
    mut editor_tool_q: Query<(&TextureAtlasSprite, &Transform, &mut EditorTool), With<EditorTool>>,
    buttons: Res<Input<MouseButton>>,
    ui_q: Query<(&Transform, &UISprite), With<UISprite>>,
) {
    for (sprite, trans, mut tool) in editor_tool_q.iter_mut() {
        if !buttons.pressed(MouseButton::Left) {
            tool.is_left_clicked = false;
            return;
        }
        let editor_is_on_ui = overlaps_ui(trans, &ui_q);
        if tool.is_left_clicked || editor_is_on_ui {
            return;
        }
        let mut new_trans = trans.clone();
        new_trans.translation.z = trans.translation.z - (random::<f32>() * 100.) + 1.;

        let item = commands
            .spawn(SpriteSheetBundle {
                sprite: sprite.clone(),
                texture_atlas: texture_atlas.0.clone(),
                transform: new_trans,
                ..Default::default()
            })
            .insert(BuiltItem)
            .id();
        commands.entity(item);

        tool.is_left_clicked = true;
    }
}

fn nothing(cm: Commands) {}
