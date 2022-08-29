
use bevy::prelude::*;
use bevy::text::Text2dBounds;

use crate::common::*;

pub fn dialogue_setup(mut commands: Commands
    , windows: Res<Windows>
    , asset_server: Res<AssetServer>
    , state: Res<DialogueState>
    , font: Res<Handle<Font>>)
{
    let window = windows.primary();
    let (width, height) = (window.width(), window.height());
    let scale = height/2160.;
    let left_character_place = Transform::from_xyz(-width/2. + width/4., 0., 0.)
        .with_scale((scale, scale, 1.).into());
    let right_character_place = Transform::from_xyz(width/2. - width/4., 0., 0.)
        .with_scale((scale, scale, 1.).into());
    
    commands.spawn()
        .insert(DialogueScene)
        .insert_bundle(SpriteBundle {
            texture: asset_server.load(state.left_character_path()),
            transform: left_character_place,
            ..default()
        });
    
    commands.spawn()
        .insert(DialogueScene)
        .insert_bundle(SpriteBundle {
            texture: asset_server.load(state.right_character_path()),
            transform: right_character_place,
            ..default()
        });

    let text_style = TextStyle {
        font: font.clone(),
        font_size: height/40.,
        color: Color::WHITE
    };
    let text_alignment = TextAlignment::CENTER;
    let text_place = |z| Transform::from_xyz(0., -height/4. - height/16., z);
    let outer_box_size = (width - width/4., height/4.);
    let text_size = (width - width/3., height/4. - height/12.);
    let text_bounds = Text2dBounds { 
        size: text_size.into()
    };

    commands.spawn_bundle(SpriteBundle {
        sprite: Sprite {
            color:Color::rgba(0., 0., 0., 0.75),
            custom_size: Some(outer_box_size.into()),
            ..default()
        },
        transform: Transform::from_xyz(0., -height/4. - height/16., 1.),
        ..default()
    }).insert(DialogueScene);
    
    commands.spawn_bundle(Text2dBundle {
        text: Text::from_section(state.text(), text_style).with_alignment(text_alignment),
        text_2d_bounds: text_bounds,
        transform: text_place(2.),
        ..default()
    }).insert(DialogueScene)
        .insert(DialogueText);

    commands.spawn_bundle(SpriteBundle {
        texture: asset_server.load("backgrounds/dialogue.png"),
        transform: Transform::from_scale((0.71, 0.71, 0.).into()),
        ..default()
    }).insert(DialogueScene);
}

pub fn dialogue_text(mut text: Query<&mut Text, With<DialogueText>>
    , state: Res<DialogueState>
    , font: Res<Handle<Font>>)
{
    let text_style = TextStyle {
        font: font.clone(),
        font_size: 18.,
        color: Color::WHITE
    };
    let text_alignment = TextAlignment::CENTER;
    let mut text = text.single_mut();
    *text.as_mut() = Text::from_section(state.text(), text_style).with_alignment(text_alignment);
}

pub fn dialogue_next(mut state: ResMut<DialogueState>
    , mut keys: ResMut<Input<KeyCode>>
    , mut game_state: ResMut<State<GameState>>)
{
    if keys.get_just_released().count() > 0 {
        match state.next_frame() {
            TransitionTo::Cooking => game_state.set(GameState::Cooking).ok(),
            TransitionTo::Intermission => game_state.set(GameState::Intermission).ok(),
            TransitionTo::Dialogue => None
        };
    }
    keys.clear();
}

pub fn dialogue_cleanup(mut commands: Commands, entities: Query<(Entity, &DialogueScene)>) {
    for (entity, _) in entities.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

#[derive(Component)]
pub struct DialogueText;

#[derive(Component)]
pub struct DialogueScene;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DialogueState(pub u32, pub u32);

pub enum TransitionTo {
    Cooking,
    Intermission,
    Dialogue
}

impl DialogueState {
    pub fn left_character_path(&self) -> &'static str {
        "characters/witch.png"
    }

    pub fn right_character_path(&self) -> &'static str {
        let DialogueState(id, _) = self;
        match id {
            0 | 1 | 2 => "characters/lady.png",
            3 | 4 | 5 => "characters/guard.png",
            6 | 7 | 8 => "characters/ghost.png",
            _ => panic!("Impossible")
        }
    }

    pub fn maximum(&self) -> u32 {
        let DialogueState(id, _) = self;
        match id {
            0 => 1,
            1 => 1,
            2 => 3,
            3 => 3,
            4 => 1,
            5 => 3,
            6 => 3,
            7 => 2,
            8 => 3,
            _ => 0,
        }
    }

    pub fn text(&self) -> &'static str {
        let DialogueState(id, frame) = self;
        match (id, frame) {
            (0, 0) => "Witch's Brew! Welcome to the village, I'm Matilda. I've always wanted to try the sweets made by witches back home and never hade the chance. Please would you mind brewing me something sweet?",
            (0, 1) => "Of course! Coming right up.",
            
            (1, 0) => "Oh my, this is positively delightful, I love it! Thank you so much, my sweet tooth is very satisfied",
            (1, 1) => "You're welcome! Come back again soon!",
            
            (2, 0) => "Ah, you really haven't perfected your craft yet have you? This isn't that sweet at all...",
            (2, 1) => "...",
            (2, 2) => "Oh don't worry dear, I'll come by tomorrow to give you more practice.",
            (2, 3) => "...",

            (3, 0) => "Lady witch, I'm part of Manor's guard for Duke Trichondri.",
            (3, 1) => "Oh, uh, welcome sir, what can I do for you?",
            (3, 2) => "I need something savory this evening Lady witch, please indulge me.",
            (3, 3) => "Okay! Coming right up!",

            (4, 0) => "Splendid! I've never had such a savory meal since my mother's home cooking! You have done a splendid job Lady witch, I'll be sure to tell my fellow guardsmen to visit your Witch's Brew.",
            (4, 1) => "You're too kind sir! You'll make me blush.",

            (5, 0) => "Disaster! You call this savory? What are you thinking!? You shouldn't be let near an ingredient or a stew for the rest of your life!",
            (5, 1) => "Dear sir! That is quite disrespectful!",
            (5, 2) => "Harumph! Maybe next time learn your way around salt shaker and seasoning palette!",
            (5, 3) => "...",

            (6, 0) => "BOO! Give me something spooky or I'll haunt you for the rest of your life!",
            (6, 1) => "Aww, you're so cute, I don't think I would mind!",
            (6, 2) => "Wait, hold on, I really want a spooky meal though! Listen, I'll haunt all your customers so they never return! Haha!",
            (6, 3) => "Alright! Alright! Calm down, I'll whip up something spooky just for you",

            (7, 0) => "Spooooky! I love it! I'll scare so many new people with this, Mwahahaha!",
            (7, 1) => "I'm glad you like it! Don't scare my customers okay?",
            (7, 2) => "No promises!",

            (8, 0) => "You call THIS spooky!? You call THIS scary!? I've never been so insulted in my afterlife!",
            (8, 1) => "Ah, wait! Give me a second chance cute ghost!",
            (8, 2) => "There are no second changes in the afterlife!",
            (8, 3) => "Isn't the afterlife a second chance?...",

            _ => "You're not suppose to see this."
        }
    }

    pub fn next_scene(&mut self, sweet: u32, savory: u32, spooky: u32) {
        let DialogueState(scene, frame) = self;
        *frame = 0;
        match (*scene, sweet, savory, spooky) {
            (0, x, _, _) if x > 75 => { *scene = 1; }
            (0, _, _, _) => { *scene = 2; }
            (1, _, _, _) | (2, _, _, _) => { *scene = 3; }
            (3, _, x, _) if x > 55 => { *scene = 4; }
            (3, _, _, _) => { *scene = 5; }
            (4, _, _, _) | (5, _, _, _) => { *scene = 6; }
            (6, _, _, x) if x > 20 => { *scene = 7; }
            (6, _, _, _) => { *scene = 8; }
            (7, _, _, _) | (8, _, _, _) => { *scene = 0; }
            _ => panic!("Impossible!")
        }
    }

    pub fn next_frame(&mut self) -> TransitionTo {
        let maximum = self.maximum();
        let mut last_frame = true;
        let DialogueState(scene, frame) = self;
        if *frame < maximum {
            *frame += 1;
            last_frame = false;
        }
        let cooking_scene = *scene == 0 || *scene == 3 || *scene == 6;
        if last_frame && cooking_scene {
            TransitionTo::Cooking
        } else if last_frame && !cooking_scene {
            TransitionTo::Intermission
        } else {
            TransitionTo::Dialogue
        }
    }
}
