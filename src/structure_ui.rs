use crate::{mouse::EditorTool, CombinedSheet, EraserSheet, GameState, PrimaryWindow};
use bevy::prelude::*;
use bevy::sprite::collide_aabb::collide;

#[derive(Component)]
struct EditorButton {
    index: usize,
}

#[derive(Component)]
struct EraserButton;

#[derive(Component)]
struct Selected;

#[derive(Component)]
pub struct UISprite {
    sprite_size: Vec2,
}

pub struct StructureUIPlugin;

impl Plugin for StructureUIPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (spawn_main_buttons, spawn_eraser))
            .add_systems(
                Update,
                (
                    unselected_button_interaction,
                    selected_button_interaction,
                    eraser_button_interaction,
                ),
            );
    }
}

fn eraser_button_interaction(
    mut next: ResMut<NextState<GameState>>,
    state: Res<State<GameState>>,
    mut eraser_button_q: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<EraserButton>),
    >,
    mut editor_tool_query: Query<(&mut TextureAtlasSprite), With<EditorTool>>,
) {
    for (interaction, mut color) in &mut eraser_button_q {
        match *interaction {
            Interaction::Pressed => {
                *color = Color::rgba(1., 0.8, 0.9, 0.9).into();
                if state.get() != &GameState::Erasing {
                    for mut sprite in editor_tool_query.iter_mut() {
                        sprite.index = 6;
                    }
                    next.set(GameState::Erasing);
                }
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

fn spawn_eraser(
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
                border: UiRect::all(Val::Px(3.)),
                ..default()
            },
            //image: assets.load("eraser.png").into(),
            border_color: Color::rgba(0., 0., 0., 1.0).into(),
            background_color: Color::NONE.into(),
            ..default()
        })
        .insert(EraserButton);

    let window = q_windows.single();
    let (w_width, w_height) = (window.width(), window.height());
    let scale = 80. / 256.;

    let texture = assets.get(&sheet.0).unwrap();
    let sprite_width = texture.size.x / texture.textures.len() as f32;
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
                        .insert(EditorButton { index: i });
                } else {
                    parent.spawn(button).insert(EditorButton { index: i });
                }
            }
        });

    let texture = assets.get(&sheet.0).unwrap();
    let sprite_width = texture.size.x / texture.textures.len() as f32;
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
                    scale: Vec3::splat(w_width / texture.size.x),
                    ..Default::default()
                },
                ..Default::default()
            })
            .insert(UISprite {
                sprite_size: Vec2::new(sprite_width, sprite_height),
            });
    }
}

///Handles interactions with the [TutorialButton].
/// # Arguments
/// * `commands` - [Commands].
/// * `loadtimer` - [Query] for [LoadTimer].
/// * `tutorial_interaction` - [Query] for [TutorialButton] and its [Interaction] when changed.
/// * `state` - Resource containing [State]. This game's states are defined in the [GameState] enum.
fn unselected_button_interaction(
    mut commands: Commands,
    mut selected: Query<(&mut BackgroundColor, Entity), (With<Selected>, With<EditorButton>)>,
    mut non_selected: Query<
        (&Interaction, &mut BackgroundColor, Entity, &EditorButton),
        (Changed<Interaction>, With<EditorButton>, Without<Selected>),
    >,
    mut editor_tool_query: Query<&mut TextureAtlasSprite, With<EditorTool>>,
    mut next: ResMut<NextState<GameState>>,
    state: Res<State<GameState>>,
) {
    let idle_color = Color::NONE.into();
    for (interaction, mut color, entity, button_index) in &mut non_selected {
        match *interaction {
            Interaction::Pressed => {
                if state.get() != &GameState::Building {
                    next.set(GameState::Building);
                }

                for (mut color, selected_button) in &mut selected {
                    commands.entity(selected_button).remove::<Selected>();
                    *color = idle_color;
                }

                commands.entity(entity).insert(Selected);

                *color = Color::rgba(0., 0., 0., 0.6).into();

                for mut sprite in editor_tool_query.iter_mut() {
                    sprite.index = button_index.index;
                }
            }
            Interaction::Hovered => {
                *color = Color::rgba(0., 0., 0., 0.4).into();
            }
            Interaction::None => {
                *color = idle_color;
            }
        }
    }
}

fn selected_button_interaction(
    mut previously_selected: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<EditorButton>, With<Selected>),
    >,
) {
    for (interaction, mut color) in &mut previously_selected {
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
