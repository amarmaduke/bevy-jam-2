
use std::cmp::{min, max};

use bevy::prelude::*;
use bevy::text::Text2dBounds;

use crate::common::*;
use crate::dialogue::DialogueState;

pub fn cooking_setup(mut commands: Commands
    , mut selection: ResMut<CookingSelection>
    , asset_server: Res<AssetServer>)
{
    selection.reset();
    let description = [
        ["Press 5", "Press 3", "Press 1"],
        ["Press 6", "Press 4", "Press 2"]
    ];

    let font = asset_server.load("fonts/FiraCode-Regular.ttf");
    let text_style = TextStyle {
        font,
        font_size: 18.,
        color: Color::WHITE
    };

    // Spawn the item selections
    for i in 0..2 {
        for j in 0..3 {
            let path = CookingSelection::item_path(i, j);
            let text = description[i as usize][j as usize];
            let (i, j) = (i as f32, j as f32);
            commands.spawn()
                .insert(CookingScene)
                .insert_bundle(SpriteBundle {
                    texture: asset_server.load(path),
                    transform: Transform::from_xyz(i*150. + 150., j*150. - 100., 0.5)
                        .with_scale((0.5, 0.5, 1.).into()),
                    ..default()
                });

            commands.spawn()
                .insert(CookingScene)
                .insert_bundle(Text2dBundle {
                    text: Text::from_section(text, text_style.clone()),
                    transform: Transform::from_xyz(i*150. + 122., j*150. - 165., 2.),
                    ..default()
                });

            commands.spawn()
                .insert(CookingScene)
                .insert_bundle(SpriteBundle {
                    sprite: Sprite {
                        color: Color::rgba(0., 0., 0., 0.75),
                        custom_size: Some((100., 25.).into()),
                        ..default()
                    },
                    transform: Transform::from_xyz(i*150. + 150., j*150. - 175., 1.),
                    ..default()
                });
        }
    }

    commands.spawn()
        .insert(CookingScene)
        .insert(CombinationItem)
        .insert_bundle(SpriteBundle {
            transform: Transform::from_xyz(-300., 150., 0.5)
            .with_scale((0.35, 0.35, -1.).into()),
            ..default()
        });

    commands.spawn()
        .insert(CookingScene)
        .insert(SelectionText)
        .insert_bundle(Text2dBundle {
            text: Text::from_section("Nothing selected.", text_style.clone())
                .with_alignment(TextAlignment::CENTER),
            transform: Transform::from_xyz(225., -250., 2.),
            ..default()
        });

    commands.spawn()
        .insert(CookingScene)
        .insert_bundle(SpriteBundle {
            sprite: Sprite {
                color: Color::rgba(0., 0., 0., 0.75),
                custom_size: Some((250., 25.).into()),
                ..default()
            },
            transform: Transform::from_xyz(225., -250., 1.),
            ..default()
        });

    commands.spawn()
        .insert(CookingScene)
        .insert_bundle(Text2dBundle {
            text: Text::from_section("Return to confirm selection.", text_style.clone())
                .with_alignment(TextAlignment::CENTER),
            transform: Transform::from_xyz(225., -300., 2.),
            ..default()
        });

    commands.spawn()
        .insert(CookingScene)
        .insert_bundle(SpriteBundle {
            sprite: Sprite {
                color: Color::rgba(0., 0., 0., 0.75),
                custom_size: Some((300., 25.).into()),
                ..default()
            },
            transform: Transform::from_xyz(225., -300., 1.),
            ..default()
        });

    commands.spawn()
        .insert(CookingScene)
        .insert(CombinationDescription)
        .insert_bundle(Text2dBundle {
            text: Text::from_section("Combination description.", text_style)
                .with_alignment(TextAlignment::CENTER),
            text_2d_bounds: Text2dBounds {
                size: (380., 300.).into()
            },
            transform: Transform::from_xyz(-300., -200., 3.),
            ..default()
        });

    commands.spawn()
        .insert(CookingScene)
        .insert_bundle(SpriteBundle {
            sprite: Sprite {
                color: Color::rgba(0., 0., 0., 0.75),
                custom_size: Some((400., 300.).into()),
                ..default()
            },
            transform: Transform::from_xyz(-300., -200., 1.),
            ..default()
        });

    commands.spawn_bundle(SpriteBundle {
            texture: asset_server.load("backgrounds/cooking.png"),
            transform: Transform::from_scale((0.71, 0.71, -1.).into()),
            ..default()
        }).insert(CookingScene);
}

#[allow(clippy::too_many_arguments)]
pub fn cook(mut combination: Query<&mut Handle<Image>, With<CombinationItem>>
    , mut combination_text: Query<(&mut Text, &CombinationDescription), Without<SelectionText>>
    , mut selection_text: Query<(&mut Text, &SelectionText), Without<CombinationDescription>>
    , asset_server: Res<AssetServer>
    , mut input: ResMut<Input<KeyCode>>
    , mut selection: ResMut<CookingSelection>
    , mut dialogue_state: ResMut<DialogueState>
    , mut game_state: ResMut<State<GameState>>
    , font: Res<Handle<Font>>)
{
    let mut confirm_selection = false;
    let mut selection_updated = false;
    for code in input.get_just_released() {
        let new_selected = match code {
            KeyCode::Key1 | KeyCode::Numpad1 => 1,
            KeyCode::Key2 | KeyCode::Numpad2 => 2,
            KeyCode::Key3 | KeyCode::Numpad3 => 3,
            KeyCode::Key4 | KeyCode::Numpad4 => 4,
            KeyCode::Key5 | KeyCode::Numpad5 => 5,
            KeyCode::Key6 | KeyCode::Numpad6 => 6,
            KeyCode::Return => { 
                confirm_selection = true;
                break
            }
            _ => 0
        };
        if new_selected != 0 {
            selection.update(new_selected);
            selection_updated = true;
        }
    }
    input.clear();

    let (combo_path, combo_description, sweet, savory, spooky) = selection.combination_data();
    if selection_updated {
        let mut combo_image = combination.single_mut();
        *combo_image = asset_server.load(combo_path);

        let (mut combo_text, _) = combination_text.single_mut();
        let text_style = TextStyle {
            font: font.clone(),
            font_size: 18.,
            color: Color::WHITE
        };
        *combo_text = Text::from_section(combo_description, text_style)
            .with_alignment(TextAlignment::CENTER);

        let (mut selected_text, _) = selection_text.single_mut();
        let new_selected_text = selection.text(&font);
        *selected_text = new_selected_text;
    }
    
    if confirm_selection {
        dialogue_state.next_scene(sweet, savory, spooky);
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

#[derive(Component)]
pub struct CombinationDescription;

#[derive(Component)]
pub struct SelectionText;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CookingSelection(pub u32, pub u32);

impl CookingSelection {
    pub fn reset(&mut self) {
        let CookingSelection(first, second) = self;
        *first = 0;
        *second = 0;
    }

    pub fn update(&mut self, next: u32) {
        let CookingSelection(first, second) = self;
        *first = *second;
        *second = next;
    }

    pub fn item_path(i: u32, j: u32) -> &'static str {
        match (i, j) {
            (0, 0) => "items/chocolate.png",
            (1, 0) => "items/coffee_beans.png",
            (0, 1) => "items/egg.png",
            (1, 1) => "items/grapes.png",
            (0, 2) => "items/pumpkin.png",
            (1, 2) => "items/skull.png",
            _ => panic!("Impossible")
        }
    }

    pub fn text(&self, font: &Handle<Font>) -> Text {
        let CookingSelection(first, second) = self;
        let text = format!("Select ingredients {} and {}", first, second);
        let text_style = TextStyle {
            font: font.clone(),
            font_size: 18.,
            color: Color::WHITE
        };
        Text::from_section(text, text_style)
            .with_alignment(TextAlignment::CENTER)
    }

    // 1: Pumpkin
    // 2: Skull
    // 3: Egg
    // 4: Grapes
    // 5: Chocolate
    // 6: Coffee Beans
    pub fn combination_data(&self) -> (&'static str, &'static str, u32, u32, u32) {
        let CookingSelection(first, second) = self;
        let (i, j) = (min(first, second), max(first, second));
        match (i, j) {
            (1, 1) => ("combinations/pumpkin_mash.png",
                r#"Like mom's mash potatoes but with the superior vegetable. Pumpkin mash is the perfect fall side dish to round out any meaty plate."#
            , 35, 60, 5
            ),
            (1, 2) => ("combinations/failure.png",
                r#"In a puff of smoke, pumpkins and skulls just don't seem to mix!"#
                , 0, 0, 0
            ),
            (1, 3) => ("combinations/pumpkin_pie.png",
                r#"A classic favorite. Pumpkin pie is delicious but not too sweet. The kids will love it and their teeth will too."#
                , 70, 30, 0
            ),
            (1, 4) => ("combinations/failure.png",
                r#"In a puff of smoke, pumpkins and grapes just don't seem to mix!"#
                , 0, 0, 0
            ),
            (1, 5) => ("combinations/pumpkin_spice_chocolate_bar.png",
                r#"Chocolate is always better cut with an earthy flavor, but make no mistake these candy bars are packed with sugar! Enjoy your lovable pumpkin aroma in a new chocolatey way."#
                , 85, 15, 0
            ),
            (1, 6) => ("combinations/pumpkin_spice_latte.png",
                r#"Not just for your coffee enthusiast! Pumpkin spice is as Halloween as it gets. Don't sleep on a tasty spicy treat that's quite sweet."#
                , 50, 50, 0
            ),
            (2, 2) => ("combinations/failure.png",
                r#"In a puff of smoke, skulls just don't seem to mix!"#
                , 0, 0, 0
            ),
            (2, 3) => ("combinations/failure.png",
                r#"In a puff of smoke, skull and egg just doesn't seem to mix!"#
                , 0, 0, 0
            ),
            (2, 4) => ("combinations/bone_marrow_wine.png",
                r#"What's better than an afternoon in front of the fire place with a cool glass of Bone Marrow Wine? Unlike other wine's you might try this one is infused with the life force of a once living being. Perfect for your fledgling vampire friends."#
                , 0, 60, 40
            ),
            (2, 5) => ("combinations/chocolate_skull_fondue.png",
                r#"Chocolate fondue is great on it's own, so why not pair it with the dread of death? Skull fondue is like ecstasy with a dash of existential crisis. Zombies seem to love it the most!"#
                , 60, 0, 40
            ),
            (2, 6) => ("combinations/failure.png",
                r#"In a puff of smoke, skull and coffee beans just doesn't seem to mix!"#
                , 0, 0, 0
            ),
            (3, 3) => ("combinations/egg_over_easy.png",
                r#"Eggs over easy is as old as bread. Can you really go wrong with this one? Just enjoy yourself a nice lightly seasoned egg already!"#
                , 5, 90, 5
            ),
            (3, 4) => ("combinations/egg_salad.png",
                r#"Grapes in an egg salad are the perfect sweet kick to an otherwise savory meal. Just imagine they're eye balls if it's not gruesome enough for you!"#
                , 5, 80, 15
            ),
            (3, 5) => ("combinations/chocolate_chip_cookie.png",
                r#"Chocolate chip cookies are the perfect treat for a young boy and girl. Especially if you have to fatten them up first!"#
                , 100, 0, 0
            ),
            (3, 6) => ("combinations/egg_coffee.png",
                r#"Egg coffee is known for it's double layers, a bottom layer of coffee and a top layer of tasty egg cream. Check your moustaches after this one fellas."#
                , 90, 10, 0
            ),
            (4, 4) => ("combinations/grape_jam.png",
                r#"Grape jam is what brings everybody together! Is that a pigeon I see sitting on the lid of the jar? Hey, hey, what are you doing! Stop stealing my grape jam! I spent valuable hours on that! Hey! Stop it!"#
                , 60, 20, 20
            ),
            (4, 5) => ("combinations/chocolate_covered_grapes.png",
                r#"The only thing better than chocolate covered strawberries are chocolate covered grapes! Did I remind you to just imagine they're eyeballs yet?"#
                , 95, 5, 0
            ),
            (4, 6) => ("combinations/failure.png",
                r#"In a puff of smoke, grapes and coffee beans just doesn't seem to mix!"#
                , 0, 0, 0
            ),
            (5, 5) => ("combinations/molten_dark_chocolate_candy.png",
                r#"Nothing is as delicious as a chocolate ball filled with molten chocolate. This lava is so hot it will sear right through your mandible. You'll scream in agony as you go in for another bite. The horror of it all is irresistible!"#
                , 80, 0, 20
            ),
            (5, 6) => ("combinations/chocolate_espresso.png",
                r#"On the move with a sweet tooth? Chocolate espresso is lot a shot of pure adrenaline!"#
                , 50, 50, 0
            ),
            (6, 6) => ("combinations/black_coffee.png",
                r#"One time I a werewolf told me that black coffee was the only solution to a transformation hangover. If it's just alcohol or something worse, give a straight black coffee a try to restart your senses."#
                , 0, 100, 0
            ),
            _ => ("",
                r#"Combination description."#
                , 0, 0, 0
            ),
        }
    }
}
