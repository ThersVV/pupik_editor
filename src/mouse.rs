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
pub struct LocalZ {
    z: i32,
}

#[derive(Component)]
pub struct ItemName {
    pub name: &'static str,
}

#[derive(Component)]
pub struct BuiltItem;

#[derive(Component)]
pub struct BuiltButton {
    id: Entity,
}

#[derive(Component)]
struct Selected;

pub struct MousePlugin;

impl Plugin for MousePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_editor_tool)
            .add_systems(
                Update,
                spawn_selected_item.run_if(in_state(GameState::Building)),
            )
            .add_systems(Update, erase_item.run_if(in_state(GameState::Erasing)))
            .add_systems(Update, movement);
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
    windows_q: Query<&Window, With<PrimaryWindow>>,
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
        let name;
        match sprite.index {
            0 => name = "blackhole",
            1 => name = "rainbow",
            2 => name = "energybar",
            3 => name = "regular",
            4 => name = "plane",
            5 => name = "planet",
            _ => name = "",
        }
        let item = commands
            .spawn(SpriteSheetBundle {
                sprite: sprite.clone(),
                texture_atlas: texture_atlas.0.clone(),
                transform: new_trans,
                ..Default::default()
            })
            .insert(BuiltItem)
            .insert(ItemName { name })
            .id();

        let window = windows_q.single();
        if let Some(position) = window.cursor_position() {
            let z = i32::abs(random::<i32>()) * -1 - 1;
            let button = commands
                .spawn(ButtonBundle {
                    style: Style {
                        width: Val::Px(30.),
                        height: Val::Px(22.),
                        left: Val::Px(position.x - 15.),
                        top: Val::Px(position.y - 11.),
                        border: UiRect::all(Val::Px(1.)),
                        position_type: PositionType::Absolute,
                        ..Default::default()
                    },
                    z_index: ZIndex::Global(z),
                    border_color: Color::BLACK.into(),
                    ..Default::default()
                })
                .insert(LocalZ { z: z })
                .insert(BuiltButton { id: item })
                .id();
            commands.entity(button);
        }

        commands.entity(item);

        tool.is_left_clicked = true;
    }
}

fn erase_item(
    mut commands: Commands,
    button_q: Query<
        (&Interaction, Entity, &BuiltButton, &LocalZ),
        (Changed<Interaction>, With<BuiltButton>),
    >,
) {
    let mut max_z = i32::MIN;
    let mut max_z_entity = None;
    let mut max_z_sprite = None;
    for (interaction, entity, button, z) in button_q.iter() {
        match *interaction {
            Interaction::Pressed => {
                if z.z > max_z {
                    max_z = z.z;
                    max_z_entity = Some(entity);
                    max_z_sprite = Some(button.id);
                }
            }
            _ => {}
        }
    }
    if let Some(e) = max_z_entity {
        commands.entity(e).despawn();
        commands.entity(max_z_sprite.unwrap()).despawn();
    }
}
