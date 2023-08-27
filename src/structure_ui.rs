use crate::{
    export::{spawn_export_button, SingleUse},
    mouse::EditorTool,
    CombinedSheet, EraserSheet, GameState, PrimaryWindow,
};
use bevy::prelude::*;
use bevy::sprite::collide_aabb::collide;

#[derive(Component)]
struct EditorButton {
    index: usize,
}

#[derive(Component)]
pub struct EraserButton;

#[derive(Component)]
struct Selected;

#[derive(Component)]
pub struct ApplyDefaultColoring;

#[derive(Component)]
pub struct UISprite {
    pub sprite_size: Vec2,
}

pub struct StructureUIPlugin;

impl Plugin for StructureUIPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (spawn_main_buttons, spawn_eraser, vertical_bars))
            .add_systems(
                Update,
                (
                    update_freshly_unselected,
                    unselected_button_coloring,
                    selected_button_coloring,
                    eraser_button_interaction,
                    change_selection,
                    select_item,
                ),
            );
    }
}

fn eraser_button_interaction(
    mut next: ResMut<NextState<GameState>>,
    state: Res<State<GameState>>,
    eraser_button_q: Query<&Interaction, (Changed<Interaction>, With<EraserButton>)>,
    mut editor_tool_query: Query<&mut TextureAtlasSprite, With<EditorTool>>,
) {
    for interaction in eraser_button_q.iter() {
        match *interaction {
            Interaction::Pressed => {
                if state.get() != &GameState::Erasing {
                    for mut sprite in editor_tool_query.iter_mut() {
                        sprite.index = 6;
                    }
                    next.set(GameState::Erasing);
                }
            }
            _ => {}
        }
    }
}

pub fn overlaps_ui(
    player_trans: &Transform,
    ui_q: &Query<(&Transform, &UISprite), With<UISprite>>,
) -> bool {
    let player_translation = player_trans.translation;
    for (trans, uisprite) in ui_q.iter() {
        let ui_trans = trans.translation;
        let ui_size = uisprite.sprite_size;
        if collide(player_translation, Vec2::splat(1.), ui_trans, ui_size).is_some() {
            return true;
        }
    }
    return false;
}

pub fn spawn_eraser(
    mut commands: Commands,
    sheet: Res<EraserSheet>,
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
                top: Val::Percent(3.),
                border: UiRect::all(Val::Px(1.)),
                ..default()
            },
            //image: assets.load("eraser.png").into(),
            border_color: Color::rgba(0., 0., 0., 1.0).into(),
            background_color: Color::NONE.into(),
            ..default()
        })
        .insert(EraserButton)
        .insert(ApplyDefaultColoring);

    let window = q_windows.single();
    let (w_width, w_height) = (window.width(), window.height());
    let scale = 80. / 256.;

    let texture = assets.get(&sheet.0).unwrap();
    let sprite_width = texture.size.x / texture.len() as f32;
    let sprite_height = texture.size.y;
    commands
        .spawn(SpriteSheetBundle {
            sprite: TextureAtlasSprite::new(0),
            texture_atlas: sheet.0.clone(),
            transform: Transform {
                translation: Vec3::new(w_width * 0.47 - 40., w_height * 0.47 - 40., 900.),
                scale: Vec3::splat(scale),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(UISprite {
            sprite_size: Vec2::new(sprite_width * scale, sprite_height * scale),
        });
}

///Spawns clickable background button together with the "How to play" button as its child
/// # Arguments
/// * `commands` - [Commands].
/// * `assets` - [AssetServer]. Used to load font.
fn spawn_main_buttons(
    mut commands: Commands,
    sheet: Res<CombinedSheet>,
    q_windows: Query<&Window, With<PrimaryWindow>>,
    assets: Res<Assets<TextureAtlas>>,
) {
    let window = q_windows.single();
    let (w_width, w_height) = (window.width(), window.height());
    commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.),
                height: Val::Percent(15.),
                position_type: PositionType::Absolute,
                bottom: Val::Percent(0.),
                ..default()
            },
            background_color: Color::rgba(0.0, 0.0, 0.0, 0.2).into(), //pink
            ..default()
        })
        .with_children(|parent| {
            for i in 0..6 {
                let button = ButtonBundle {
                    style: Style {
                        position_type: PositionType::Absolute,
                        width: Val::Percent(100.0 / 6.0),
                        height: Val::Percent(100.0),
                        left: Val::Percent(100.0 / 6.0 * i as f32),
                        border: UiRect {
                            right: Val::Px(0.5),
                            top: Val::Px(1.0),
                            ..Default::default()
                        },
                        ..default()
                    },
                    border_color: Color::rgba(0., 0., 0., 0.6).into(),
                    background_color: BackgroundColor(Color::NONE),
                    z_index: ZIndex::Local(1),
                    ..default()
                };
                if i == 0 {
                    parent
                        .spawn(button)
                        .insert(Selected)
                        .insert(EditorButton { index: i })
                        .insert(ApplyDefaultColoring);
                } else {
                    parent
                        .spawn(button)
                        .insert(EditorButton { index: i })
                        .insert(ApplyDefaultColoring);
                }
            }
        });

    let texture = assets.get(&sheet.0).unwrap();
    let sprite_width = texture.size.x / texture.len() as f32;
    let sprite_height = texture.size.y;
    for i in 0..6 {
        commands
            .spawn(SpriteSheetBundle {
                texture_atlas: sheet.0.clone(),
                sprite: TextureAtlasSprite::new(i),
                transform: Transform {
                    translation: Vec3::new(
                        w_width * -0.5 + w_width / 6. * (i as f32 + 0.5),
                        w_height * -0.5 + 50.,
                        900.0,
                    ),
                    scale: Vec3::new(
                        (w_width / 6.) / sprite_width,
                        (w_height * 0.15) / sprite_height,
                        1.,
                    ),
                    ..Default::default()
                },
                ..Default::default()
            })
            .insert(UISprite {
                sprite_size: Vec2::new(w_width / 6., w_height * 0.15),
            });
    }
}

fn change_selection(
    mut commands: Commands,
    non_selected: Query<
        (&Interaction, Entity),
        (
            Changed<Interaction>,
            With<ApplyDefaultColoring>,
            Without<Selected>,
            Without<SingleUse>,
        ),
    >,
    selected: Query<Entity, (With<ApplyDefaultColoring>, With<Selected>)>,
) {
    for (interaction, entity) in non_selected.iter() {
        match *interaction {
            Interaction::Pressed => {
                for selected_button in selected.iter() {
                    commands.entity(selected_button).remove::<Selected>();
                }

                commands.entity(entity).insert(Selected);
            }
            _ => {}
        }
    }
}

fn select_item(
    non_selected: Query<
        (&Interaction, &EditorButton),
        (Changed<Interaction>, With<EditorButton>, Without<Selected>),
    >,
    mut editor_tool_query: Query<&mut TextureAtlasSprite, With<EditorTool>>,
    mut next: ResMut<NextState<GameState>>,
    state: Res<State<GameState>>,
) {
    for (interaction, button_index) in non_selected.iter() {
        match *interaction {
            Interaction::Pressed => {
                if state.get() != &GameState::Building {
                    next.set(GameState::Building);
                }

                for mut sprite in editor_tool_query.iter_mut() {
                    sprite.index = button_index.index;
                }
            }
            _ => {}
        }
    }
}

fn selected_button_coloring(
    mut selected: Query<
        (&Interaction, &mut BackgroundColor),
        (
            With<ApplyDefaultColoring>,
            Changed<Interaction>,
            With<Selected>,
        ),
    >,
) {
    for (interaction, mut color) in &mut selected {
        match *interaction {
            Interaction::Pressed => {
                *color = Color::rgba(1., 0.8, 0.9, 0.9).into();
            }
            Interaction::Hovered => {
                *color = Color::rgba(1., 0.8, 0.9, 0.8).into();
            }
            Interaction::None => {
                *color = Color::rgba(1., 0.8, 0.9, 0.6).into();
            }
        }
    }
}

fn unselected_button_coloring(
    mut unselected: Query<
        (&Interaction, &mut BackgroundColor),
        (
            With<ApplyDefaultColoring>,
            Changed<Interaction>,
            Without<Selected>,
        ),
    >,
) {
    for (interaction, mut color) in &mut unselected {
        match *interaction {
            Interaction::Pressed => {
                *color = Color::rgba(0., 0., 0., 0.75).into();
            }
            Interaction::Hovered => {
                *color = Color::rgba(0., 0., 0., 0.4).into();
            }
            Interaction::None => {
                *color = Color::NONE.into();
            }
        }
    }
}

fn update_freshly_unselected(
    mut unselected: RemovedComponents<Selected>,
    mut unselected_q: Query<
        (&Interaction, &mut BackgroundColor),
        (With<ApplyDefaultColoring>, Without<Selected>),
    >,
) {
    for entity in unselected.iter() {
        if let Ok((interaction, mut color)) = unselected_q.get_mut(entity) {
            match *interaction {
                Interaction::Pressed => {
                    *color = Color::rgba(0., 0., 0., 0.75).into();
                }
                Interaction::Hovered => {
                    *color = Color::rgba(0., 0., 0., 0.4).into();
                }
                Interaction::None => {
                    *color = Color::NONE.into();
                }
            }
        }
    }
}

fn vertical_bars(mut commands: Commands, q_windows: Query<&Window, With<PrimaryWindow>>) {
    let w_width = q_windows.single().width();
    commands.spawn(NodeBundle {
        style: Style {
            width: Val::Px((w_width - (1920. / 3.)) / 2.),
            height: Val::Percent(85.),
            left: Val::Percent(0.),
            position_type: PositionType::Absolute,
            top: Val::Percent(0.),
            border: UiRect {
                right: Val::Px(1.),
                ..Default::default()
            },
            ..default()
        },
        background_color: Color::rgba(0., 0., 0., 0.3).into(), //pink
        border_color: Color::rgba(0., 0., 0., 0.8).into(),
        ..default()
    });

    commands.spawn(NodeBundle {
        style: Style {
            width: Val::Px((w_width - (1920. / 3.)) / 2.),
            height: Val::Percent(85.),
            right: Val::Percent(0.),
            top: Val::Percent(0.),
            position_type: PositionType::Absolute,
            border: UiRect {
                left: Val::Px(1.),
                ..Default::default()
            },
            ..default()
        },
        background_color: Color::rgba(0., 0., 0., 0.3).into(), //pink
        border_color: Color::rgba(0., 0., 0., 0.8).into(),
        ..default()
    });
}
