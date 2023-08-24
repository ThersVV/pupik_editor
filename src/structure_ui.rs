use crate::{mouse::EditorTool, CombinedSheet, GameState, ImagePaths, PrimaryWindow};
use bevy::prelude::*;

#[derive(Component)]
struct EditorButton {
    index: usize,
}

#[derive(Component)]
struct Selected;

pub struct StructureUIPlugin;

impl Plugin for StructureUIPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::EditorUI), spawn_buttons)
            .add_systems(
                Update,
                (unselected_button_interaction, selected_button_interaction)
                    .run_if(in_state(GameState::EditorUI)),
            );
    }
}

///Spawns clickable background button together with the "How to play" button as its child
/// # Arguments
/// * `commands` - [Commands].
/// * `assets` - [AssetServer]. Used to load font.
fn spawn_buttons(
    mut commands: Commands,
    sheet: Res<CombinedSheet>,
    q_windows: Query<&Window, With<PrimaryWindow>>,
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
                /* parent.spawn(ImageBundle {
                    style: Style {
                        position_type: PositionType::Absolute,
                        max_width: Val::Percent(100.0 / 12.0),
                        max_height: Val::Percent(90.),
                        left: Val::Percent(100.0 / 6.0 * (i as f32) + 4.4),
                        bottom: Val::Percent(3.),
                        ..default()
                    },
                    image: assets.load(image_paths.vec[i].clone()).into(),
                    z_index: ZIndex::Local(2),
                    ..default()
                }) */
            }
        });

    for i in 0..6 {
        commands.spawn(SpriteSheetBundle {
            texture_atlas: sheet.0.clone(),
            sprite: TextureAtlasSprite::new(i),
            transform: Transform {
                translation: Vec3::new(
                    w_width * -0.5 + w_width / 6. * (i as f32 + 0.5),
                    w_height * -0.5 + 50.,
                    900.0,
                ),
                scale: Vec3::splat(w_width / 1932.),
                ..Default::default()
            },
            ..Default::default()
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
    mut previously_selected: Query<
        (&mut BackgroundColor, Entity),
        (With<Selected>, With<EditorButton>),
    >,
    mut non_selected: Query<
        (&Interaction, &mut BackgroundColor, Entity, &EditorButton),
        (Changed<Interaction>, With<EditorButton>, Without<Selected>),
    >,
    mut editor_tool_query: Query<(&mut TextureAtlasSprite, &mut Transform), With<EditorTool>>,
) {
    let idle_color = Color::NONE.into();
    for (interaction, mut color, entity, button_index) in &mut non_selected {
        match *interaction {
            Interaction::Pressed => {
                for (mut color, previously_selected_button) in &mut previously_selected {
                    commands
                        .entity(previously_selected_button)
                        .remove::<Selected>();
                    *color = idle_color;
                }
                commands.entity(entity).insert(Selected);
                *color = Color::rgba(0., 0., 0., 0.6).into();
                for (mut sprite, mut transform) in editor_tool_query.iter_mut() {
                    sprite.index = button_index.index;
                    if button_index.index == 2 || button_index.index == 4 {
                        transform.scale = Vec3::splat(0.6);
                    } else {
                        transform.scale = Vec3::splat(1.);
                    }
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
