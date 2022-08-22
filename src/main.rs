
use bevy::prelude::*;
use bevy::text::Text2dBounds;

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            width: 1080.,
            height: 720.,
            resizable: false,
            present_mode: bevy::window::PresentMode::AutoVsync,
            ..default()
        })
        .add_state(GameState::Dialogue)
        .insert_resource(DialogueState(0, 0))
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_system_set(SystemSet::on_update(GameState::Dialogue).with_system(dialogue))
        .add_system(bevy::window::close_on_esc)
        .run();
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct DialogueState(u32, u32);

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum GameState {
    Dialogue,
    Cooking,
}

#[derive(Component)]
struct Character;

fn setup(mut commands: Commands, windows: Res<Windows>, asset_server: Res<AssetServer>) {
    let window = windows.primary();
    let (width, height) = (window.width(), window.height());
    let scale = height/2160.;
    let left_character_place = Transform::from_xyz(-width/2. + width/4., 0., 0.)
        .with_scale((scale, scale, 1.).into());
    let right_character_place = Transform::from_xyz(width/2. - width/4., 0., 0.)
        .with_scale((scale, scale, 1.).into());

    commands.spawn_bundle(Camera2dBundle::default());
    
    commands.spawn()
        .insert(Character)
        .insert_bundle(SpriteBundle {
            texture: asset_server.load("characters/witch.png"),
            transform: left_character_place,
            ..default()
        });
    
    commands.spawn()
        .insert_bundle(SpriteBundle {
            texture: asset_server.load("characters/woman_blue.png"),
            transform: right_character_place,
            ..default()
        });

    let font = asset_server.load("fonts/FiraCode-Regular.ttf");
    let text_style = TextStyle {
        font,
        font_size: 18.,
        color: Color::WHITE
    };
    let text_alignment = TextAlignment::CENTER;
    let text_place = |z| Transform::from_xyz(0., -height/4. - height/16., z);
    let outer_box_size = (width - width/4., height/4.);
    let text_size = (width - width/4., height/4. - height/12.);
    let text_bounds = Text2dBounds { 
        size: text_size.into()
    };
    let text_data = r#"
        I want to tell you a story.
        It's not long, it's not short, but it's my story.
        Listen closely young one, because I'm only going to tell you this story once.
        What?
        You don't want to hear it?
        Wait really?
        No, no come on, it's a good story!
        I'll dial it back a little, come on, man!
        You'll hear me out?
        Excellent, fantastic!
        Okay, so where was I...
    "#;
    commands.spawn_bundle(SpriteBundle {
        sprite: Sprite {
            color:Color::rgba(0., 0., 0., 0.5),
            custom_size: Some(outer_box_size.into()),
            ..default()
        },
        transform: Transform::from_xyz(0., -height/4. - height/16., 1.),
        ..default()
    });
    commands.spawn_bundle(Text2dBundle {
        text: Text::from_section(text_data, text_style).with_alignment(text_alignment),
        text_2d_bounds: text_bounds,
        transform: text_place(2.),
        ..default()
    });
}

fn dialogue(mut state: ResMut<DialogueState>, input: Res<Input<KeyCode>>) {
    
}
