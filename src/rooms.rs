use bevy::prelude::*;
use bevy_egui::egui::Ui;
use bevy_egui::egui::scroll_area::ScrollBarVisibility;
use bevy_egui::{EguiContexts, egui};

use crate::network::{
    GameCreateRequest, GameCreateResult, GameJoinRequest, GameJoinResult, GameJoinStatus,
    GameListResult, GameReconnectResult, NetworkCommand,
};
use crate::{GameState, MySystemSets};

pub struct RoomMenuPlugin;

impl Plugin for RoomMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::RoomMenu), add_ui_state)
            .add_systems(
                Update,
                (update_ui_state, room_menu)
                    .chain()
                    .run_if(in_state(GameState::RoomMenu))
                    .in_set(MySystemSets::Egui),
            )
            .add_systems(OnExit(GameState::RoomMenu), remove_ui_state);
    }
}

#[derive(Debug)]
#[allow(dead_code)] // rust warns about an unread field, but we use it in the "Debug" impl
enum RoomStatus {
    Loading,
    Loaded,
    Error(String),
    JoinWindow,
    CreateWindow,
    Joining,
    CreatingAndJoining,
}

#[derive(Resource)]
struct UiState {
    room_status: RoomStatus,
    games: Vec<String>,
    selected: Option<String>,
    name: String,
    password: String,
}

impl UiState {
    fn main_interaction_blocked(&self) -> bool {
        self.window_interaction_blocked()
            || matches!(
                self.room_status,
                RoomStatus::Loading | RoomStatus::JoinWindow | RoomStatus::CreateWindow
            )
    }

    fn window_interaction_blocked(&self) -> bool {
        matches!(
            self.room_status,
            RoomStatus::Joining | RoomStatus::CreatingAndJoining
        )
    }

    fn enable_join_button(&self) -> bool {
        !self.main_interaction_blocked() && self.selected.is_some()
    }

    fn enable_create_button(&self) -> bool {
        !self.main_interaction_blocked()
    }

    fn enable_reload_button(&self) -> bool {
        !self.main_interaction_blocked()
    }

    fn enable_label(&self) -> bool {
        matches!(self.room_status, RoomStatus::Loading | RoomStatus::Error(_))
    }

    fn enable_join_create_button(&self) -> bool {
        !self.window_interaction_blocked()
            && match self.room_status {
                RoomStatus::JoinWindow => true,
                RoomStatus::CreateWindow => !self.name.is_empty(),
                _ => panic!("illegal state"),
            }
    }

    fn enable_cancel_button(&self) -> bool {
        !self.window_interaction_blocked()
    }

    fn reset(&mut self) {
        self.room_status = RoomStatus::Loaded;
        self.name.clear();
        self.password.clear();
    }
}

impl Default for UiState {
    fn default() -> Self {
        Self {
            room_status: RoomStatus::Loading,
            games: Vec::new(),
            selected: None,
            name: "".to_string(),
            password: "".to_string(),
        }
    }
}

fn game_to_string(game_id: impl AsRef<str>) -> String {
    format!("Game: {}", game_id.as_ref())
}

fn add_ui_state(mut commands: Commands) {
    info!("starting RoomMenu");
    commands.init_resource::<UiState>();
}

fn remove_ui_state(mut commands: Commands) {
    commands.remove_resource::<UiState>();
}

fn update_ui_state(
    mut state: ResMut<NextState<GameState>>,
    mut ui_state: ResMut<UiState>,
    mut game_list_results: EventReader<GameListResult>,
    mut game_join_results: EventReader<GameJoinResult>,
    mut game_reconnect_results: EventReader<GameReconnectResult>,
    mut game_create_results: EventReader<GameCreateResult>,
    mut event_writer: EventWriter<NetworkCommand>,
) {
    if matches!(ui_state.room_status, RoomStatus::Loading) {
        event_writer.send(NetworkCommand::GameReconnect);
    }
    if let Some(game_reconnect_result) = game_reconnect_results.read().last() {
        match &game_reconnect_result.0 {
            Ok(res) => match res.status {
                GameJoinStatus::Joined | GameJoinStatus::AlreadyJoined => match res.password {
                    Some(true) => {
                        ui_state.selected.clone_from(&res.room_id);
                        ui_state.room_status = RoomStatus::JoinWindow
                    }
                    _ => state.set(GameState::Ingame),
                },
                GameJoinStatus::NotJoined => {
                    event_writer.send(NetworkCommand::GetGameList);
                }
            },
            Err(e) => {
                ui_state.room_status = RoomStatus::Error(format!("{e:?}"));
            }
        }
    }
    if let Some(game_list_result) = game_list_results.read().last() {
        match &game_list_result.0 {
            Ok(game_list) => {
                ui_state.games.clone_from(&game_list.rooms);
                ui_state.room_status = RoomStatus::Loaded;
            }
            Err(e) => {
                ui_state.room_status = RoomStatus::Error(format!("{e:?}"));
            }
        }
    }
    if let Some(game_join_result) = game_join_results.read().last() {
        assert!(matches!(ui_state.room_status, RoomStatus::Joining));
        match &game_join_result.0 {
            Ok(res) => match res.status {
                GameJoinStatus::Joined | GameJoinStatus::AlreadyJoined => {
                    state.set(GameState::Ingame);
                }
                GameJoinStatus::NotJoined => {
                    ui_state.room_status = RoomStatus::Error("Not joined".to_string())
                }
            },
            Err(e) => {
                ui_state.room_status = RoomStatus::Error(format!("{e:?}"));
            }
        }
    }
    if let Some(game_create_result) = game_create_results.read().last() {
        assert!(matches!(
            ui_state.room_status,
            RoomStatus::CreatingAndJoining
        ));
        match &game_create_result.0 {
            Ok(_) => {
                state.set(GameState::Ingame);
            }
            Err(e) => {
                ui_state.room_status = RoomStatus::Error(format!("{e:?}"));
            }
        }
    }
}

fn room_menu(
    mut egui_context: EguiContexts,
    mut ui_state: ResMut<UiState>,
    mut event_writer: EventWriter<NetworkCommand>,
) {
    let ui_state: &mut UiState = &mut ui_state;
    egui::CentralPanel::default().show(egui_context.ctx_mut(), |ui| {
        ui.columns(2, |columns| {
            let ui_left: &mut Ui = &mut columns[0];

            ui_left.label("Rooms:");
            ui_left.separator();
            egui::ScrollArea::vertical()
                .auto_shrink([false, false])
                .scroll_bar_visibility(ScrollBarVisibility::AlwaysVisible)
                .max_height(ui_left.available_height() - 50.0)
                .show(ui_left, |ui| {
                    match &ui_state.room_status {
                        RoomStatus::Loading => {
                            ui.spinner();
                        }
                        _ => {
                            for game_id in &ui_state.games {
                                ui.selectable_value(
                                    &mut ui_state.selected,
                                    Some(game_id.to_string()),
                                    game_to_string(game_id),
                                );
                            }
                        }
                    };
                });
            ui_left.separator();

            let ui_right: &mut Ui = &mut columns[1];
            ui_right.label("Info:");
            // TODO: add game info
            ui_right.separator();
        });
        ui.horizontal(|ui| {
            let button =
                ui.add_enabled(ui_state.enable_reload_button(), egui::Button::new("Reload"));
            if button.clicked() {
                ui_state.room_status = RoomStatus::Loading;
                event_writer.send(NetworkCommand::GetGameList);
            }

            let button = ui.add_enabled(ui_state.enable_join_button(), egui::Button::new("Join"));
            if button.clicked() {
                ui_state.reset();
                ui_state.room_status = RoomStatus::JoinWindow;
            }

            let button =
                ui.add_enabled(ui_state.enable_create_button(), egui::Button::new("Create"));
            if button.clicked() {
                ui_state.reset();
                ui_state.room_status = RoomStatus::CreateWindow;
            }
        });
        ui.add_visible(
            ui_state.enable_label(),
            egui::Label::new(format!("{:?}", ui_state.room_status)),
        );
    });

    match ui_state.room_status {
        RoomStatus::JoinWindow | RoomStatus::Joining => {
            egui::Window::new("Joining game").show(egui_context.ctx_mut(), |ui| {
                ui.horizontal(|ui| {
                    ui.label("Password:");
                    ui.text_edit_singleline(&mut ui_state.password);
                });
                ui.horizontal(|ui| {
                    let button = ui.add_enabled(
                        ui_state.enable_join_create_button(),
                        egui::Button::new("Join"),
                    );
                    if button.clicked() {
                        ui_state.room_status = RoomStatus::Joining;
                        event_writer.send(NetworkCommand::GameJoin(
                            ui_state.selected.as_ref().unwrap().to_string(),
                            GameJoinRequest {
                                password: if ui_state.password.is_empty() {
                                    None
                                } else {
                                    Some(ui_state.password.to_string())
                                },
                            },
                        ));
                    }

                    let button = ui
                        .add_enabled(ui_state.enable_cancel_button(), egui::Button::new("Cancel"));
                    if button.clicked() {
                        ui_state.reset();
                    }
                });
            });
        }
        RoomStatus::CreateWindow | RoomStatus::CreatingAndJoining => {
            egui::Window::new("Create new game").show(egui_context.ctx_mut(), |ui| {
                ui.horizontal(|ui| {
                    ui.label("Name:");
                    ui.text_edit_singleline(&mut ui_state.name);
                });
                ui.horizontal(|ui| {
                    ui.label("Password:");
                    ui.text_edit_singleline(&mut ui_state.password);
                });
                // TODO: add min_player, max_player?
                ui.horizontal(|ui| {
                    let button = ui.add_enabled(
                        ui_state.enable_join_create_button(),
                        egui::Button::new("Create & join"),
                    );
                    if button.clicked() {
                        ui_state.room_status = RoomStatus::CreatingAndJoining;
                        event_writer.send(NetworkCommand::GameCreate(GameCreateRequest {
                            room_name: ui_state.name.to_string(),
                            password: if ui_state.password.is_empty() {
                                None
                            } else {
                                Some(ui_state.password.to_string())
                            },
                            min_player: None,
                            max_player: None,
                        }));
                    }

                    let button = ui
                        .add_enabled(ui_state.enable_cancel_button(), egui::Button::new("Cancel"));
                    if button.clicked() {
                        ui_state.reset();
                    }
                });
            });
        }
        _ => {}
    }

    if matches!(
        ui_state.room_status,
        RoomStatus::Joining | RoomStatus::CreatingAndJoining
    ) {
        egui::Window::new(match ui_state.room_status {
            RoomStatus::Joining => "Joining",
            RoomStatus::CreatingAndJoining => "Creating & Joining",
            _ => panic!("illegal state"),
        })
        .show(egui_context.ctx_mut(), |ui| {
            ui.spinner();
        });
    }
}
