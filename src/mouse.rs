use crate::{CombinedSheet, GameState};
use bevy::prelude::*;
use bevy::window::PrimaryWindow;

#[derive(Component)]
pub struct Player;

#[derive(Component)]
struct Selected;

pub struct MousePlugin;

impl Plugin for MousePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::EditorUI), spawn_player)
            .add_systems(Update, (movement).run_if(in_state(GameState::EditorUI)));
    }
}

fn movement(
    mut player_query: Query<&mut Transform, With<Player>>,
    q_windows: Query<&Window, With<PrimaryWindow>>,
) {
    for mut transform in player_query.iter_mut() {
        let window = q_windows.single();
        if let Some(position) = window.cursor_position() {
            transform.translation.x = position.x - (window.width() / 2.);
            transform.translation.y = -1. * position.y + (window.height() / 2.);
        }
    }
}

fn spawn_player(mut commands: Commands, texture_atlas: Res<CombinedSheet>) {
    let sprite = TextureAtlasSprite::new(0);

    let player = commands
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
        .insert(Player)
        .id();
    commands.entity(player);
}
