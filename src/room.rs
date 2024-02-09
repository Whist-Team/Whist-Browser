use bevy::prelude::*;
use bevy_egui::egui::scroll_area::ScrollBarVisibility;
use bevy_egui::egui::Ui;
use bevy_egui::{egui, EguiContexts};

use crate::network::{NetworkCommand, RoomInfoResult, RoomPhase};
use crate::player::Player;
use crate::{GameState, Globals, MySystemSets};

pub struct RoomLobbyPlugin;

impl Plugin for RoomLobbyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::RoomLobby), add_ui_state)
            .add_systems(
                Update,
                (update_ui_state, lobby_menu)
                    .chain()
                    .run_if(in_state(GameState::RoomLobby))
                    .after(MySystemSets::EguiTop),
            )
            .add_systems(OnExit(GameState::RoomLobby), remove_ui_state);
    }
}

#[derive(Debug)]
enum RoomStatus {
    Error(String),
    WaitingForPlayers,
    ReadyToStart,
    Starting,
    Started,
}

#[derive(Resource)]
struct UiState {
    room_status: RoomStatus,
    name: String,
    password: bool,
    rubber_number: u8,
    game_number: u8,
    hand_number: u8,
    trick_number: u8,
    min_player: u8,
    max_player: u8,
    players: Vec<Player>,
    selected: Option<String>,
    phase: RoomPhase,
}

impl UiState {
    fn enable_start_button(&self) -> bool {
        matches!(self.room_status, RoomStatus::ReadyToStart)
    }
}

impl Default for UiState {
    fn default() -> Self {
        Self {
            room_status: RoomStatus::WaitingForPlayers,
            name: "".to_string(),
            password: false,
            phase: RoomPhase::Lobby,
            rubber_number: 0,
            game_number: 0,
            hand_number: 0,
            trick_number: 0,
            min_player: 0,
            max_player: 0,
            players: Vec::new(),
            selected: None,
        }
    }
}

fn add_ui_state(
    mut commands: Commands,
    globals: ResMut<Globals>,
    mut event_writer: EventWriter<NetworkCommand>,
) {
    commands.init_resource::<UiState>();
    commands.init_resource::<Globals>();

    let room_id = globals.room_id.clone().unwrap();
    event_writer.send(NetworkCommand::RoomInfo(room_id));
}

fn remove_ui_state(mut commands: Commands) {
    commands.remove_resource::<UiState>();
}

fn update_ui_state(
    mut ui_state: ResMut<UiState>,
    mut room_info_results: EventReader<RoomInfoResult>,
) {
    if let Some(room_info_result) = room_info_results.iter().last() {
        match &room_info_result.0 {
            Ok(room_info) => {
                ui_state.name = room_info.name.to_owned();
                ui_state.password = room_info.password.to_owned();
                ui_state.phase = room_info.phase.to_owned();
                ui_state.rubber_number = room_info.rubber_number.to_owned();
                ui_state.game_number = room_info.game_number.to_owned();
                ui_state.hand_number = room_info.hand_number.to_owned();
                ui_state.trick_number = room_info.trick_number.to_owned();
                ui_state.min_player = room_info.min_player.to_owned();
                ui_state.max_player = room_info.max_player.to_owned();
                ui_state.players = room_info.players.to_owned();

                match ui_state.phase {
                    RoomPhase::Lobby => {
                        if ui_state.players.len() < ui_state.min_player as usize {
                            ui_state.room_status = RoomStatus::WaitingForPlayers
                        }
                    }
                    RoomPhase::ReadyToStart => ui_state.room_status = RoomStatus::ReadyToStart,
                    RoomPhase::Playing => ui_state.room_status = RoomStatus::Started,
                }
            }
            Err(e) => ui_state.room_status = RoomStatus::Error(format!("{:?}", e)),
        }
    }
}

fn lobby_menu(
    mut egui_context: EguiContexts,
    mut ui_state: ResMut<UiState>,
    globals: Res<Globals>,
    mut event_writer: EventWriter<NetworkCommand>,
) {
    let ui_state: &mut UiState = &mut ui_state;
    egui::CentralPanel::default().show(egui_context.ctx_mut(), |ui| {
        ui.horizontal(|ui| {
            ui.label(format!("Room: {}", ui_state.name));
        });
        ui.columns(2, |columns| {
            let ui_left: &mut Ui = &mut columns[0];
            ui_left.label(format!(
                "Players: {}/{}",
                ui_state.players.len(),
                ui_state.max_player
            ));
            ui_left.separator();
            egui::ScrollArea::vertical()
                .auto_shrink([false, false])
                .scroll_bar_visibility(ScrollBarVisibility::AlwaysVisible)
                .max_height(ui_left.available_height() - 50.0)
                .show(ui_left, |ui| {
                    for player in &ui_state.players {
                        let username = player.clone().username;
                        ui.selectable_value(
                            &mut ui_state.selected,
                            Some(username.to_string()),
                            &username,
                        );
                    }
                });
            ui_left.separator();

            let ui_right: &mut Ui = &mut columns[1];
            ui_right.label("Info:");
            // TODO: add player info
            ui_right.separator();
        });
        let start_button =
            ui.add_enabled(ui_state.enable_start_button(), egui::Button::new("Start"));
        if start_button.clicked() {
            ui_state.room_status = RoomStatus::Starting;
            let room_id = globals.room_id.clone().unwrap();
            event_writer.send(NetworkCommand::StartRoom(room_id));
        }
    });
}
