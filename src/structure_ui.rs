use crate::GameState;
use bevy::prelude::*;

#[derive(Component)]
struct EditorButton {
    selected: bool,
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
fn spawn_buttons(mut commands: Commands, assets: Res<AssetServer>) {
    let image_vec: Vec<UiImage> = Vec::from([
        assets.load("planet_sheet.png").into(),
        assets.load("blackhole_sheet.png").into(),
        assets.load("energy_sheet.png").into(),
        assets.load("duha.png").into(),
        assets.load("lovesheet.png").into(),
        assets.load("plane_sheet1.png").into(),
    ]);
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
                parent
                    .spawn(ButtonBundle {
                        style: Style {
                            position_type: PositionType::Relative,
                            width: Val::Percent(100.0 / 6.0),
                            height: Val::Percent(100.0),
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
                    })
                    .insert(EditorButton { selected: false });

                parent.spawn(ImageBundle {
                    style: Style {
                        position_type: PositionType::Absolute,
                        max_width: Val::Percent(100.0 / 12.0),
                        max_height: Val::Percent(90.),
                        left: Val::Percent(100.0 / 6.0 * (i as f32) + 4.4),
                        bottom: Val::Percent(3.),
                        ..default()
                    },
                    image: image_vec[i].clone(),
                    ..default()
                });
            }
        });
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
        (With<EditorButton>, With<Selected>),
    >,
    mut non_selected: Query<
        (&Interaction, &mut BackgroundColor, Entity),
        (Changed<Interaction>, With<EditorButton>, Without<Selected>),
    >,
) {
    for (interaction, mut color, entity) in &mut non_selected {
        match *interaction {
            Interaction::Pressed => {
                for (mut color, previously_selected_button) in &mut previously_selected {
                    commands
                        .entity(previously_selected_button)
                        .remove::<Selected>();
                    *color = Color::rgba(0., 0., 0., 0.1).into();
                }
                commands.entity(entity).insert(Selected);
                *color = Color::rgba(0., 0., 0., 0.6).into();
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

fn selected_button_interaction(
    mut commands: Commands,
    mut previously_selected: Query<
        (&Interaction, &mut BackgroundColor, Entity),
        (Changed<Interaction>, With<EditorButton>, With<Selected>),
    >,
) {
    for (interaction, mut color, entity) in &mut previously_selected {
        match *interaction {
            Interaction::Pressed => {
                *color = Color::rgba(1., 0.8, 0.9, 0.6).into();
            }
            Interaction::Hovered => {
                *color = Color::rgba(1., 0.8, 0.9, 0.5).into();
            }
            Interaction::None => {
                *color = Color::rgba(1., 0.8, 0.9, 0.4).into();
            }
        }
    }
}
