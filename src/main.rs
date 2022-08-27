
use bevy::prelude::*;

mod common;
mod dialogue;
mod cooking;

use common::*;
use dialogue::*;
use cooking::*;

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            width: 1080.,
            height: 720.,
            resizable: false,
            present_mode: bevy::window::PresentMode::AutoVsync,
            ..default()
        })
        .add_state(GameState::Cooking)
        .insert_resource(DialogueState(0, 0))
        .insert_resource(CookingSelection(0, 0))
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_system_set(SystemSet::on_enter(GameState::Dialogue)
            .with_system(dialogue_setup))
        .add_system_set(SystemSet::on_update(GameState::Dialogue)
            .with_system(dialogue_text)
            .with_system(dialogue_next))
        .add_system_set(SystemSet::on_exit(GameState::Dialogue)
            .with_system(dialogue_cleanup))
        .add_system_set(SystemSet::on_enter(GameState::Cooking)
            .with_system(cooking_setup))
        .add_system_set(SystemSet::on_update(GameState::Cooking)
            .with_system(cook))
        .add_system_set(SystemSet::on_exit(GameState::Cooking)
            .with_system(cooking_cleanup))
        .add_system(bevy::window::close_on_esc)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn_bundle(Camera2dBundle::default());
}
