
use bevy::prelude::*;
use bevy::text::Text2dBounds;

use crate::common::*;

pub fn cooking_setup(mut commands: Commands
    , windows: Res<Windows>
    , asset_server: Res<AssetServer>)
{
    // Spawn the item selections
    for i in 0..3 {
        for j in 0..3 {
            let path = CookingSelection::item_path(i, j);
            let (i, j) = (i as f32, j as f32);
            let transform = Transform::from_xyz(i*100., j*100., 0.)
                .with_scale((0.5, 0.5, 1.).into());
            commands.spawn()
                .insert(CookingScene)
                .insert_bundle(SpriteBundle {
                    texture: asset_server.load(path),
                    transform,
                    ..default()
                });
        }
    }

    let transform = Transform::from_xyz(-100., -100., 0.)
        .with_scale((0.25, 0.25, 1.).into());
    commands.spawn()
        .insert(CookingScene)
        .insert(CombinationItem)
        .insert_bundle(SpriteBundle {
            transform,
            ..default()
        });
}

pub fn cook(mut combination: Query<&mut Handle<Image>, With<CombinationItem>>
    , asset_server: Res<AssetServer>
    , input: Res<Input<KeyCode>>
    , mut selection: ResMut<CookingSelection>
    , mut game_state: ResMut<State<GameState>>)
{
    let mut confirm_selection = false;
    for code in input.get_just_released() {
        let new_selected = match code {
            KeyCode::Key1 | KeyCode::Numpad1 => 1,
            KeyCode::Key2 | KeyCode::Numpad2 => 1,
            KeyCode::Key3 | KeyCode::Numpad3 => 1,
            KeyCode::Key4 | KeyCode::Numpad4 => 1,
            KeyCode::Key5 | KeyCode::Numpad5 => 1,
            KeyCode::Key6 | KeyCode::Numpad6 => 1,
            KeyCode::Key7 | KeyCode::Numpad7 => 1,
            KeyCode::Key8 | KeyCode::Numpad8 => 1,
            KeyCode::Key9 | KeyCode::Numpad9 => 1,
            KeyCode::Return => { 
                confirm_selection = true;
                break
            }
            _ => 0
        };
        if new_selected != 0 {
            selection.update(new_selected);
        }
    }

    let mut combo = combination.single_mut();
    *combo = asset_server.load(selection.combination_path());

    if confirm_selection {
        game_state.set(GameState::Dialogue).ok();
    }
}

pub fn cooking_cleanup(mut commands: Commands, entities: Query<(Entity, &CookingScene)>) {
    for (entity, _) in entities.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

#[derive(Component)]
pub struct CookingScene;

#[derive(Component)]
pub struct CombinationItem;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CookingSelection(pub u32, pub u32);

impl CookingSelection {
    pub fn update(&mut self, next: u32) {
        let CookingSelection(first, second) = self;
        *first = *second;
        *second = next;
    }

    pub fn item_path(i: u32, j: u32) -> &'static str {
        match (i, j) {
            (0, 0) => "items/pumpkin_1.png",
            (1, 0) => "items/mushroom_1.png",
            (2, 0) => "items/mushroom_1.png",
            (0, 1) => "items/mushroom_1.png",
            (1, 1) => "items/mushroom_1.png",
            (2, 1) => "items/mushroom_1.png",
            (0, 2) => "items/mushroom_1.png",
            (1, 2) => "items/mushroom_1.png",
            (2, 2) => "items/mushroom_1.png",
            _ => panic!("Impossible")
        }
    }

    pub fn combination_path(&self) -> &'static str {
        let CookingSelection(first, second) = self;
        match (first, second) {
            (1, 2) => "combinations/pumpkin_mushroom_stew.png",
            _ => "combinations/pumpkin_mushroom_stew.png",
        }
    }
}
