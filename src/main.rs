
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
        .add_system_set(SystemSet::on_enter(GameState::Dialogue)
            .with_system(dialogue_setup))
        .add_system_set(SystemSet::on_update(GameState::Dialogue)
            .with_system(dialogue_text)
            .with_system(dialogue_next))
        .add_system_set(SystemSet::on_exit(GameState::Dialogue)
            .with_system(dialogue_cleanup))
        .add_system(bevy::window::close_on_esc)
        .run();
}

#[derive(Component)]
struct DialogueText(Handle<Font>);

#[derive(Component)]
struct DialogueScene;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct DialogueState(u32, u32);

impl DialogueState {
    fn left_character_path(&self) -> &'static str {
        let DialogueState(id, _) = self;
        match id {
            0 => "characters/witch.png",
            _ => "characters/witch.png",
        }
    }

    fn right_character_path(&self) -> &'static str {
        let DialogueState(id, _) = self;
        match id {
            0 => "characters/woman_blue.png",
            _ => "characters/woman_blue.png",
        }
    }

    fn maximum(&self) -> u32 {
        let DialogueState(id, _) = self;
        match id {
            0 => 3,
            _ => 0,
        }
    }

    fn text(&self) -> &'static str {
        let DialogueState(id, frame) = self;
        match (id, frame) {
            (0, 0) => "Hi",
            (0, 1) => "Hello!",
            (0, 2) => "This is awkward",
            (0, 3) => "Yeah it is.",
            _ => "meow"
        }
    }

    fn next(&mut self) -> bool {
        let maximum = self.maximum();
        let DialogueState(id, frame) = self;
        if *frame < maximum{
            *frame += 1;
            false
        } else {
            true
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum GameState {
    Dialogue,
    Cooking,
}

fn setup(mut commands: Commands) {
    commands.spawn_bundle(Camera2dBundle::default());
}

fn dialogue_setup(mut commands: Commands
    , windows: Res<Windows>
    , asset_server: Res<AssetServer>
    , state: Res<DialogueState>)
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

    let font = asset_server.load("fonts/FiraCode-Regular.ttf");
    let text_style = TextStyle {
        font: font.clone(),
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

    commands.spawn_bundle(SpriteBundle {
        sprite: Sprite {
            color:Color::rgba(0., 0., 0., 0.5),
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
    }).insert(DialogueText(font))
        .insert(DialogueScene);
}

fn dialogue_text(mut text: Query<&mut Text, With<DialogueText>>
    , dialogue_text: Query<&DialogueText>
    , state: Res<DialogueState>)
{
    let DialogueText(font) = dialogue_text.single();
    let text_style = TextStyle {
        font: font.clone(),
        font_size: 18.,
        color: Color::WHITE
    };
    let text_alignment = TextAlignment::CENTER;
    let mut text = text.single_mut();
    *text.as_mut() = Text::from_section(state.text(), text_style).with_alignment(text_alignment);
}

fn dialogue_next(mut state: ResMut<DialogueState>
    , buttons: Res<Input<MouseButton>>
    , mut game_state: ResMut<State<GameState>>)
{
    if buttons.any_just_released([MouseButton::Left, MouseButton::Right]) {
        let dialogue_finished = state.next();
        if dialogue_finished {
            game_state.set(GameState::Cooking).ok();
        }
    }
}

fn dialogue_cleanup(mut commands: Commands, entities: Query<(Entity, &DialogueScene)>) {
    for (entity, _) in entities.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
