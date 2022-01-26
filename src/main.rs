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

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
enum AppState {
    Init,
    Playing,
    Loading,
    CheckWin,
    Win,
    Fail
}

struct WinNoticeMenu { 
    win_notice_entity: Entity 
}

struct GameContainer {
    game_container_entity: Entity
}

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
        .init_resource::<GameState>()
        .add_asset::<CustomAsset>()
        .init_asset_loader::<CustomAssetLoader>()
        .add_state(AppState::Init)
        .add_system_set(
            SystemSet::on_enter(AppState::Init)
            .with_system(setup)
        )
        .add_system_set(
            SystemSet::on_update(AppState::Init)
            .with_system(init_game)
        )
        .add_system_set(
            SystemSet::on_exit(AppState::Init)
            .with_system(spawn_container)
        )
        .add_system_set(
            SystemSet::on_enter(AppState::Playing)
            .with_system(update_score)
        )
        .add_system_set(
            SystemSet::on_update(AppState::Fail)
            .with_system(handle_accept_win)
        )
        .add_system_set(
            SystemSet::on_enter(AppState::Fail)
            .with_system(cleanup_game_container)
        )
        .add_system_set(
            SystemSet::on_exit(AppState::Fail)
            .with_system(cleanup_win_notice)
        )
        .add_system_set(
            SystemSet::on_enter(AppState::Fail)
            .with_system(spawn_win_notice)
        )
        .add_system_set(
            SystemSet::on_update(AppState::Fail)
            .with_system(show_win_notice)
        )
        .add_system_set(
            SystemSet::on_update(AppState::Win)
            .with_system(handle_accept_win)
        )
        .add_system_set(
            SystemSet::on_enter(AppState::Win)
            .with_system(cleanup_game_container)
        )
        .add_system_set(
            SystemSet::on_exit(AppState::Win)
            .with_system(cleanup_win_notice)
        )
        .add_system_set(
            SystemSet::on_enter(AppState::Win)
            .with_system(spawn_win_notice)
        )
        .add_system_set(
            SystemSet::on_update(AppState::Win)
            .with_system(show_win_notice)
        )
        .add_system_set(
            SystemSet::on_enter(AppState::CheckWin)
            .with_system(check_win)
        )
        .add_system_set(
            SystemSet::on_update(AppState::Playing)
            .with_system(handle_keyboard)
            .label("input")
        )
        .add_system_set(
            SystemSet::on_update(AppState::Playing)
            .with_system(handle_button)
            .label("input")
        )
        .add_system_set(
            SystemSet::on_update(AppState::Playing)
            .with_system(update_text)
            .before("input")
        )
        .add_system_set(
            SystemSet::on_update(AppState::Loading)
            .with_system(check_keyboard)
            .before("input")
        )
        .add_system_set(
            SystemSet::on_update(AppState::Loading)
            .with_system(check_guesses)
        )
        .run();
}

#[derive(Component, Debug)]
struct Score { }


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
struct GameState {
    handle: Handle<CustomAsset>,
    guesses: Vec<String>,
    word: Option<String>,
    printed: bool,
    guess: String,
    column: usize,
    row: usize,
    wins: u32
}

#[derive(Component, Debug)]
struct WinNotice();

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
        "←",
        "z",
        "x",
        "c",
        "v",
        "b",
        "n",
        "m",
        "⏎",
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
            .spawn_bundle(ButtonBundle {
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
                              font: asset_server.load("fonts/FiraCode-Bold.ttf"),
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

fn handle_button(
    mut cell_query: Query<&mut Cell>,
    mut interaction_query: Query<
        (&Interaction, &Children),
        (Changed<Interaction>, With<Button>),
    >,
    mut state: ResMut<GameState>,
    mut text_query: Query<&mut Text, With<Key>>,
    custom_assets: ResMut<Assets<CustomAsset>>,
    mut app_state: ResMut<State<AppState>>,
) {
    for (interaction, children) in interaction_query.iter_mut() {
        for children in children.iter() {
            if let Ok(text) = text_query.get_mut(*children) {
                match *interaction {
                    Interaction::Clicked => {
                        let c = text.sections[0].value.chars().next().expect("Char!");
                        if c == '⏎' {
                            submit_guess(&mut state, &custom_assets, &mut app_state);
                        } else if c == '←' {
                            if state.column > 0 {
                                state.column -= 1;
                                handle_letter(&mut cell_query, &mut state, ' ');
                                state.column -= 1;
                            }
                        } else {
                            handle_letter(&mut cell_query, &mut state, c);
                        }
                    }
                    _ => {
                    }
                }
            }
        }
    }
}

fn spawn_win_notice(mut commands: Commands, asset_server: Res<AssetServer>) {
    let win_notice_entity = commands.spawn_bundle(NodeBundle {
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
            parent.spawn_bundle(ButtonBundle {
                style: Style {
                    position_type: PositionType::Relative,
                    size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
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
                color: Color::rgb(1.0, 1.0, 1.0).into(),
                ..Default::default()
            })
            .insert(WinNotice {})
            .with_children(|button_parent| {
                button_parent.spawn_bundle(TextBundle {
                    focus_policy: bevy::ui::FocusPolicy::Pass,
                    text: Text::with_section(
                              "",
                              TextStyle {
                                  font: asset_server.load("fonts/FiraCode-Bold.ttf"),
                                  font_size: 30.0,
                                  color: Color::rgb(0.2, 0.2, 0.2),
                              },
                              Default::default(),
                          ),
                          style: Style {
                              align_self: AlignSelf::Center,
                              ..Default::default()
                          },
                          ..Default::default()
                });
            });
        }).id();
    commands.insert_resource(WinNoticeMenu { win_notice_entity});
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
                                          font: asset_server.load("fonts/FiraCode-Bold.ttf"),
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

fn spawn_score(parent: &mut ChildBuilder, asset_server: &Res<AssetServer>) {
    parent.spawn_bundle(NodeBundle {
        style: Style {
            position_type: PositionType::Relative,
            position: Rect {
                ..Default::default()
            },
            flex_wrap: FlexWrap::WrapReverse,
            size: Size::new(Val::Auto, Val::Auto),
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
                margin: Rect::all(Val::Px(20.0)),
                ..Default::default()
            },
            color: Color::rgb(0.0, 0.0, 0.0).into(),
            ..Default::default()
        }).with_children(|parent| {
            // text
            parent.spawn_bundle(TextBundle {
                style: Style {
                    margin: Rect::all(Val::Px(5.0)),
                    ..Default::default()
                },
                text: Text::with_section(
                          "0",
                          TextStyle {
                              font: asset_server.load("fonts/FiraCode-Bold.ttf"),
                              font_size: 30.0,
                              color: Color::WHITE,
                          },
                          Default::default(),
                      ),
                      ..Default::default()
            }).insert(Score {});
        });
    });
}

fn spawn_container(mut commands: Commands, asset_server: Res<AssetServer>) {
    let game_entity = commands
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
            spawn_score(parent, &asset_server);
        }).id();

    commands.insert_resource(GameContainer {
        game_container_entity: game_entity
    })
}

fn setup(mut state: ResMut<GameState>, asset_server: Res<AssetServer>) {
    state.handle = asset_server.load("words.dict");
}

fn score_letter(state: &ResMut<GameState>, letter: &char, row: usize, col: usize) -> Option<u32> {
    if state.row == row {
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

fn score_key(state: &ResMut<GameState>, letter: &String) -> Option<u8> {
    if let Some(word) = &state.word {
        let guesses: String = state.guesses.iter().fold(String::new(), |mut guess_set, guess| {
            guess_set.push_str(guess.as_str());
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
    state: ResMut<GameState>,
    mut app_state: ResMut<State<AppState>>,
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
    app_state.set(AppState::CheckWin);
}

fn check_guesses(
    mut text_query: Query<( &Parent, &mut Text, &Cell )>,
    mut p_query: Query<&mut UiColor>,
    state: ResMut<GameState>,
    mut app_state: ResMut<State<AppState>>,
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
                            p.0 = Color::rgb(0.8, 0.8, 0.0);
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
    app_state.set(AppState::CheckWin);
}

fn update_text(
    mut text_query: Query<( &mut Text, &Cell ), Changed<Cell>>,
) {
    for ( mut text, cell ) in text_query.iter_mut() {
        if let Some(g) = &cell.guess {
            text.sections[0].value = g.to_string();
        }
    }
}

fn update_score(
    mut text_query: Query<( &mut Text, &Score)>,
    game_state: Res<GameState>
) {
    for ( mut text, _) in text_query.iter_mut() {
        text.sections[0].value = format!("{}", game_state.wins);
    }
}

fn handle_letter(query: &mut Query<&mut Cell>, state: &mut ResMut<GameState>, letter: char) {
    let cell = query.iter_mut().find(|cell| {
        cell.row == state.row && cell.column == state.column
    });
    if let Some(mut c) = cell {
        (*c).guess = Some(letter);
        state.column += 1;
        if letter != ' ' {
            state.guess.push(letter);
        } else {
            state.guess.pop();
        }
    }
}

fn handle_keyboard(
    mut query: Query<&mut Cell>,
    mut state: ResMut<GameState>,
    mut key_evr: EventReader<KeyboardInput>,
    custom_assets: ResMut<Assets<CustomAsset>>,
    mut app_state: ResMut<State<AppState>>,
    ) {
    use bevy::input::ElementState;

    for ev in key_evr.iter() {
        match ev.state {
            ElementState::Pressed => {
                match ev.key_code {
                    Some(KeyCode::Back) => {
                        if state.column > 0 {
                            state.column -= 1;
                            handle_letter(&mut query, &mut state, ' ');
                            state.column -= 1;
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
                        submit_guess(&mut state, &custom_assets, &mut app_state);
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

fn submit_guess(
    state: &mut ResMut<GameState>,
    custom_assets: &ResMut<Assets<CustomAsset>>,
    app_state: &mut ResMut<State<AppState>>,
    ) {
    if state.column == 5 {
        let custom_asset = custom_assets.get(&state.handle);

        if let Some(dict) = custom_asset {
            if dict.words.contains(&state.guess) {
                let guess = state.guess.clone();
                state.guesses.push(guess);
                state.row += 1;
                state.column = 0;
                app_state.set(AppState::Loading);
            }
        }
    }
}

fn init_game(
    mut state: ResMut<GameState>,
    custom_assets: ResMut<Assets<CustomAsset>>,
    mut app_state: ResMut<State<AppState>>
    ) {
    let custom_asset = custom_assets.get(&state.handle);

    if let Some(dict) = custom_asset {
        let num = rand::thread_rng().gen_range(0, dict.words.len() as i32);
        if let Some(word) = dict.words.get(num as usize) {
            state.word = Some(word.clone());
            app_state.set(AppState::Playing).expect("Could not start game!");
        }
    }

}

fn check_win(
    mut state: ResMut<GameState>,
    mut app_state: ResMut<State<AppState>>,
    ) {
    if let Some(w) = &state.word {
        if w.eq(&state.guess) {
            app_state.set(AppState::Win);
        } else if state.row == 6 {
            app_state.set(AppState::Fail);
        } else {
            app_state.set(AppState::Playing);
        }
        state.guess = String::new();
    }
}

fn show_win_notice(
    mut query: Query<(With<WinNotice>, &Children)>,
    mut q_text: Query<&mut Text>, 
    state: Res<GameState>,
    app_state: Res<State<AppState>>,
    ) {
    for ( _, children) in query.iter_mut() {
        for &child in children.iter() {
            let text = q_text.get_mut(child);
            if let Ok(mut t) = text {
                if *app_state.current() == AppState::Win{
                    t.sections[0].value = format!("The word was: {}. Congrats!\nStreak {} (nice!)\n(click)", state.word.clone().unwrap(), state.wins + 1);
                    
                } else {
                    t.sections[0].value = format!("Oh no! The word was: {}\n(click)", state.word.clone().unwrap());
                }
            }
        }

    }
}

fn handle_accept_win(
    mut interaction_query: Query<
        (&Interaction, &Children),
        (Changed<Interaction>, With<Button>),
    >,
    mut app_state: ResMut<State<AppState>>,
    mut game_state: ResMut<GameState>,
) {
    for (interaction, _) in interaction_query.iter_mut() {
        match *interaction {
            Interaction::Clicked => {
                game_state.row = 0;
                game_state.column = 0;
                game_state.guesses = Vec::new();
                if *app_state.current() == AppState::Win {
                    game_state.wins += 1;
                } else {
                    game_state.wins = 0;
                }
                app_state.set(AppState::Init).expect("Failed to transition to init");
            }
            _ => {
            }
        }
    }
}

fn cleanup_win_notice(mut commands: Commands, win_notice_data: Res<WinNoticeMenu>) {
    commands.entity(win_notice_data.win_notice_entity).despawn_recursive();
}

fn cleanup_game_container(mut commands: Commands, game_container_entity: Res<GameContainer>) {
    commands.entity(game_container_entity.game_container_entity).despawn_recursive();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn_bundle(UiCameraBundle::default());

    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}
