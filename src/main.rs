use bevy::{
    asset::{AssetLoader, LoadContext, LoadedAsset},
    prelude::*,
    reflect::TypeUuid,
    utils::BoxedFuture,
};
use rand::Rng;
use bevy::input::keyboard::KeyboardInput;

#[derive(Debug, TypeUuid)]
#[uuid = "39cadc56-aa9c-4543-8640-a018b74b5052"]
pub struct CustomAsset {
    pub words: Vec<String>,
}

#[derive(Default)]
pub struct CustomAssetLoader;

impl AssetLoader for CustomAssetLoader {
    fn load<'a>(
        &'a self,
        bytes: &'a [u8],
        load_context: &'a mut LoadContext,
    ) -> BoxedFuture<'a, Result<(), anyhow::Error>> {
        Box::pin(async move {
            let words = String::from_utf8(bytes.to_vec())?;
            let vec: Vec<String> = words.lines().map(|word| {
                String::from(word)
            }).collect();
            load_context.set_default_asset(LoadedAsset::new(CustomAsset {
                words: vec
            }));
            Ok(())
        })
    }

    fn extensions(&self) -> &[&str] {
        &["dict"]
    }
}

fn main() {
    App::new()
        .add_startup_system(setup_camera)
        .insert_resource(WindowDescriptor {
            vsync: false, // This is needed because of an issue with wgpu amdvlk
            ..Default::default()
        })
    .add_plugins(DefaultPlugins)
        .init_resource::<State>()
        .add_asset::<CustomAsset>()
        .init_asset_loader::<CustomAssetLoader>()
        .add_startup_system(setup)
        .add_startup_system(spawn_container)
        .add_system(print_on_load)
        .add_system(update_guesses)
        .add_system(check_guesses)
        .add_system(check_keyboard)
        .run();
}

#[derive(Component, Debug)]
struct Cell {
    row: usize,
    column: usize,
    guess: Option<char>
}

#[derive(Component, Debug)]
struct Key {
    key: String
}

#[derive(Default, Debug)]
struct State {
    handle: Handle<CustomAsset>,
    guesses: Vec<Vec<String>>,
    word: Option<String>,
    printed: bool,
    x: usize,
    y: usize,
}

fn spawn_keyboard(parent: &mut ChildBuilder, asset_server: &Res<AssetServer>) {
    let keys = vec![
        "q", 
        "w",
        "e",
        "r",
        "t", 
        "y",
        "u",
        "i",
        "o",
        "p",
        "a",
        "s",
        "d",
        "f",
        "g",
        "h",
        "j",
        "k",
        "l",
        " ",
        "⏎",
        "z",
        "x",
        "c",
        "v",
        "b",
        "n",
        "m",
        "⬾"
            ];

    parent.spawn_bundle(NodeBundle {
        style: Style {
            position_type: PositionType::Relative,
            position: Rect {
                ..Default::default()
            },
            flex_wrap: FlexWrap::WrapReverse,
            size: Size::new(Val::Px((40.*12.) + 50.), Val::Px((40.*3.) + 50.)),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            align_content: AlignContent::Center,
            ..Default::default()
        },
        color: Color::rgb(0.9, 0.9, 0.9).into(),
        ..Default::default()
    }).with_children(|parent| {
    for key in keys {
        parent
            .spawn_bundle(NodeBundle {
                style: Style {
                    position_type: PositionType::Relative,
                    max_size: Size::new(Val::Px(40.), Val::Px(40.)),
                    size: Size::new(Val::Px(40.), Val::Percent(40.0)),
                    margin: Rect {
                        top: Val::Px(5.),
                        left: Val::Px(5.),
                        right: Val::Px(5.),
                        bottom: Val::Px(5.),
                    },
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    align_content: AlignContent::Center,
                    ..Default::default()
                },
                color: Color::rgb(0.15, 0.15, 0.15).into(),
                ..Default::default()
            })
        .with_children(|parent| {
            // text
            parent.spawn_bundle(TextBundle {
                style: Style {
                    margin: Rect::all(Val::Px(5.0)),
                    ..Default::default()
                },
                text: Text::with_section(
                          key,
                          TextStyle {
                              font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                              font_size: 30.0,
                              color: Color::WHITE,
                          },
                          Default::default(),
                      ),
                      ..Default::default()
            }).insert(Key {
                key: key.to_string(),
            });
        });
    }
    });
}

fn spawn_grid(parent: &mut ChildBuilder, asset_server: &Res<AssetServer>) {
    parent.spawn_bundle(NodeBundle {
        style: Style {
            position_type: PositionType::Relative,
            position: Rect {
                ..Default::default()
            },
            flex_wrap: FlexWrap::WrapReverse,
            size: Size::new(Val::Px((40.*5.) + 50.), Val::Px((40.*6.) + 60.)),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            align_content: AlignContent::Center,
            ..Default::default()
        },
        color: Color::rgb(0.9, 0.9, 0.9).into(),
        ..Default::default()
    }).with_children(|parent| {
        parent.spawn_bundle(NodeBundle {
            style: Style {
                position_type: PositionType::Relative,
                position: Rect {
                    ..Default::default()
                },
                flex_wrap: FlexWrap::WrapReverse,
                size: Size::new(Val::Px((40.*5.) + 50.), Val::Px((40.*6.) + 60.)),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                align_content: AlignContent::Center,
                ..Default::default()
            },
            color: Color::rgb(0.9, 0.9, 0.9).into(),
            ..Default::default()
        }).with_children(|parent| {
            for row in 0..6 {
                for col in 0..5 {
                    parent
                        .spawn_bundle(NodeBundle {
                            style: Style {
                                position_type: PositionType::Relative,
                                max_size: Size::new(Val::Px(40.), Val::Px(40.)),
                                size: Size::new(Val::Px(40.), Val::Percent(40.0)),
                                margin: Rect {
                                    top: Val::Px(5.),
                                    left: Val::Px(5.),
                                    right: Val::Px(5.),
                                    bottom: Val::Px(5.),
                                },
                                align_items: AlignItems::Center,
                                justify_content: JustifyContent::Center,
                                align_content: AlignContent::Center,
                                ..Default::default()
                            },
                            color: Color::rgb(0.15, 0.15, 0.15).into(),
                            ..Default::default()
                        })
                    .with_children(|parent| {
                        // text
                        parent.spawn_bundle(TextBundle {
                            style: Style {
                                margin: Rect::all(Val::Px(5.0)),
                                ..Default::default()
                            },
                            text: Text::with_section(
                                      "",
                                      TextStyle {
                                          font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                          font_size: 30.0,
                                          color: Color::WHITE,
                                      },
                                      Default::default(),
                                  ),
                                  ..Default::default()
                        }).insert(Cell {
                            row: row as usize,
                            column: col as usize,
                            guess: None
                        });
                    });
                }
            }
        });
    });
}

fn spawn_container(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,
                position: Rect {
                    ..Default::default()
                },
                size: Size::new(Val::Percent(100.), Val::Percent(100.)),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                flex_direction: FlexDirection::Column,
                align_content: AlignContent::Center,
                ..Default::default()
            },
            color: Color::rgb(1.0, 1.0, 1.0).into(),
            ..Default::default()
        }).with_children(|parent| {
            spawn_keyboard(parent, &asset_server);
            spawn_grid(parent, &asset_server);
        });
}

fn setup(mut state: ResMut<State>, asset_server: Res<AssetServer>) {
    state.handle = asset_server.load("words.dict");
}

fn score_letter(state: &ResMut<State>, letter: &char, row: usize, col: usize) -> Option<u32> {
    if state.y == row {
        return None
    } else {
        let mut score = 0;
        let word = state.word.clone().expect("Word not set!");
        let mut chars = word.chars();
        if chars.any(|c| {
            c == *letter
        }) {
            score += 1;
        }

        let pos = word.chars().nth(col);

        if let Some(p) = pos {
            if p == *letter {
                score += 1;
            }
        }
        
        Some(score)
    }
}

fn score_key(state: &ResMut<State>, letter: &String) -> Option<u8> {
    if let Some(word) = &state.word {
        let guesses: String = state.guesses.iter().fold(String::new(), |mut guess_set, guess| {
            guess_set.push_str(guess.join("").as_str());
            guess_set
        });
        let letter_in_guesses = letter.chars().any(|c| {
            guesses.chars().any(|g| {
                g == c
            })
        });
        let has_word = word.chars().any(|c| {
            letter.chars().any(|g| {
                g == c
            })
        });
        if letter_in_guesses && !has_word {
            return Some(1)
        }
        if letter_in_guesses && has_word {
            return Some(2)
        }
        None
    } else {
        None 
    }
}


fn check_keyboard(
    mut key_query: Query<( &Parent, &mut Text, &Key)>,
    mut p_query: Query<&mut UiColor>,
    state: ResMut<State>,
) {
    for ( parent, _, key) in key_query.iter_mut() {
        let score = score_key(&state, &key.key);
        let parent_style = p_query.get_mut(parent.0);
        match parent_style {
            Ok(mut p) => {
                if let Some( s ) = score {
                    if s == 1 {
                        p.0 = Color::RED;
                    } 
                    if s == 2 {
                        p.0 = Color::GREEN;
                    }
                }
            }
            Err(err) => {
                println!("{}", err);
            }
        }
    }
}

fn check_guesses(
    mut text_query: Query<( &Parent, &mut Text, &Cell )>,
    mut p_query: Query<&mut UiColor>,
    state: ResMut<State>,
) {
    for ( parent, mut text, cell ) in text_query.iter_mut() {
        if let Some(g) = &cell.guess {
            let score = score_letter(&state, g, cell.row, cell.column);
            text.sections[0].value = g.to_string();
            let parent_style = p_query.get_mut(parent.0);
            match parent_style {
                Ok(mut p) => {
                    if let Some(s) = score {
                        if s == 1 {
                            p.0 = Color::YELLOW;
                        }
                        if s == 2 {
                            p.0 = Color::GREEN;
                        }
                    }
                }
                Err(err) => {
                    println!("{}", err);
                }
            }
            
        }
    }
}

fn handle_letter(query: &mut Query<&mut Cell>, state: &mut ResMut<State>, letter: char) {
    let cell = query.iter_mut().find(|cell| {
        cell.row == state.y && cell.column == state.x
    });
    if let Some(mut c) = cell {
        (*c).guess = Some(letter);
        state.x += 1;
    }
}

fn update_guesses(
    mut query: Query<&mut Cell>,
    mut state: ResMut<State>,
    mut key_evr: EventReader<KeyboardInput>,
    custom_assets: ResMut<Assets<CustomAsset>>
    ) {
    use bevy::input::ElementState;

    for ev in key_evr.iter() {
        match ev.state {
            ElementState::Pressed => {
                match ev.key_code {
                    Some(KeyCode::Back) => {
                        if state.x > 0 {
                            state.x -= 1;
                            handle_letter(&mut query, &mut state, ' ');
                            state.x -= 1;
                        }
                    }
                    Some(KeyCode::A) => {
                        handle_letter(&mut query, &mut state, 'a');
                    }
                    Some(KeyCode::B) => {
                        handle_letter(&mut query, &mut state, 'b');
                    }
                    Some(KeyCode::C) => {
                        handle_letter(&mut query, &mut state, 'c');
                    }
                    Some(KeyCode::D) => {
                        handle_letter(&mut query, &mut state, 'd');
                    }
                    Some(KeyCode::E) => {
                        handle_letter(&mut query, &mut state, 'e');
                    }
                    Some(KeyCode::F) => {
                        handle_letter(&mut query, &mut state, 'f');
                    }
                    Some(KeyCode::G) => {
                        handle_letter(&mut query, &mut state, 'g');
                    }
                    Some(KeyCode::H) => {
                        handle_letter(&mut query, &mut state, 'h');
                    }
                    Some(KeyCode::I) => {
                        handle_letter(&mut query, &mut state, 'i');
                    }
                    Some(KeyCode::J) => {
                        handle_letter(&mut query, &mut state, 'j');
                    }
                    Some(KeyCode::K) => {
                        handle_letter(&mut query, &mut state, 'k');
                    }
                    Some(KeyCode::L) => {
                        handle_letter(&mut query, &mut state, 'l');
                    }
                    Some(KeyCode::M) => {
                        handle_letter(&mut query, &mut state, 'm');
                    }
                    Some(KeyCode::N) => {
                        handle_letter(&mut query, &mut state, 'n');
                    }
                    Some(KeyCode::O) => {
                        handle_letter(&mut query, &mut state, 'o');
                    }
                    Some(KeyCode::P) => {
                        handle_letter(&mut query, &mut state, 'p');
                    }
                    Some(KeyCode::Q) => {
                        handle_letter(&mut query, &mut state, 'q');
                    }
                    Some(KeyCode::R) => {
                        handle_letter(&mut query, &mut state, 'r');
                    }
                    Some(KeyCode::S) => {
                        handle_letter(&mut query, &mut state, 's');
                    }
                    Some(KeyCode::T) => {
                        handle_letter(&mut query, &mut state, 't');
                    }
                    Some(KeyCode::U) => {
                        handle_letter(&mut query, &mut state, 'u');
                    }
                    Some(KeyCode::V) => {
                        handle_letter(&mut query, &mut state, 'v');
                    }
                    Some(KeyCode::W) => {
                        handle_letter(&mut query, &mut state, 'w');
                    }
                    Some(KeyCode::X) => {
                        handle_letter(&mut query, &mut state, 'x');
                    }
                    Some(KeyCode::Y) => {
                        handle_letter(&mut query, &mut state, 'y');
                    }
                    Some(KeyCode::Z) => {
                        handle_letter(&mut query, &mut state, 'z');
                    }
                    Some(KeyCode::Return) => {
                        if state.x == 5 {
                            let letters: Vec<&Cell> = query.iter().filter(|cell| {
                                cell.row == state.y
                            }).collect();
                            let mut guess= vec![
                                String::from("1"),
                                String::from("2"),
                                String::from("3"),
                                String::from("4"),
                                String::from("5")
                            ];

                            for letter in letters {
                                guess[letter.column] = letter.guess.clone().unwrap().to_string();
                            }

                            let custom_asset = custom_assets.get(&state.handle);

                            if let Some(dict) = custom_asset {
                                let guessed_word = guess.join("");
                                if dict.words.contains(&guessed_word) {
                                    state.guesses.push(guess);
                                    state.y += 1;
                                    state.x = 0;
                                }
                            }
                        }
                    }
                    _ => {
                    }
                };
            }
            _ => {
            }
        }
    }
}

fn print_on_load(mut state: ResMut<State>, custom_assets: ResMut<Assets<CustomAsset>>) {
    let custom_asset = custom_assets.get(&state.handle);
    if state.printed || custom_asset.is_none() {
        return;
    }

    if let Some(dict) = custom_asset {
        let num = rand::thread_rng().gen_range(0, dict.words.len() as i32);
        state.printed = true;
        if let Some(word) = dict.words.get(num as usize) {
            println!("{}", word);
            state.word = Some(word.clone());
        }
    }
    
}

fn setup_camera(mut commands: Commands) {
    commands.spawn_bundle(UiCameraBundle::default());

    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}
