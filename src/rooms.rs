use bevy::prelude::*;
use bevy_egui::{egui, EguiContext};

use crate::network::{GameListResult, NetworkCommand};
use crate::{GameState, MySystemLabel};

pub struct RoomMenuPlugin;

impl Plugin for RoomMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(GameState::RoomMenu).with_system(add_ui_state))
            .add_system_set(
                SystemSet::on_update(GameState::RoomMenu)
                    .after(MySystemLabel::EguiTop)
                    .with_system(update_ui_state)
                    .with_system(room_menu.after(update_ui_state)),
            )
            .add_system_set(SystemSet::on_exit(GameState::RoomMenu).with_system(remove_ui_state));
    }
}
#[derive(Debug)]
enum RoomStatus {
    Loading,
    Loaded(Vec<String>),
    LoadingError(String),
}

impl RoomStatus {
    fn enable_reload_button(&self) -> bool {
        matches!(self, RoomStatus::Loaded(_) | RoomStatus::LoadingError(_))
    }

    fn enable_label(&self) -> bool {
        matches!(self, RoomStatus::Loading | RoomStatus::LoadingError(_))
    }
}

struct UiState {
    room_status: RoomStatus,
}

impl Default for UiState {
    fn default() -> Self {
        Self {
            room_status: RoomStatus::Loading,
        }
    }
}

fn add_ui_state(mut commands: Commands, mut event_writer: EventWriter<NetworkCommand>) {
    commands.init_resource::<UiState>();
    event_writer.send(NetworkCommand::GetGameList);
}

fn remove_ui_state(mut commands: Commands) {
    commands.remove_resource::<UiState>();
}

fn update_ui_state(
    mut ui_state: ResMut<UiState>,
    mut game_list_results: EventReader<GameListResult>,
) {
    if let Some(game_list_result) = game_list_results.iter().next() {
        assert!(matches!(ui_state.room_status, RoomStatus::Loading));
        match game_list_result {
            Ok(game_list) => {
                ui_state.room_status = RoomStatus::Loaded(game_list.games.to_owned());
            }
            Err(e) => {
                ui_state.room_status = RoomStatus::LoadingError(format!("{:?}", e));
            }
        };
    }
}

fn room_menu(
    mut egui_context: ResMut<EguiContext>,
    mut ui_state: ResMut<UiState>,
    mut event_writer: EventWriter<NetworkCommand>,
) {
    egui::CentralPanel::default().show(egui_context.ctx_mut(), |ui| {
        ui.label("Rooms:");
        let button = ui.add_enabled(
            ui_state.room_status.enable_reload_button(),
            egui::Button::new("Reload"),
        );
        if button.clicked() {
            ui_state.room_status = RoomStatus::Loading;
            event_writer.send(NetworkCommand::GetGameList);
        }
        egui::ScrollArea::vertical().show(ui, |ui| {
            match &ui_state.room_status {
                RoomStatus::Loaded(rooms) => {
                    for room in rooms {
                        ui.label(room);
                    }
                }
                RoomStatus::Loading => {
                    ui.spinner();
                }
                _ => {}
            };
        });
        ui.add_visible(
            ui_state.room_status.enable_label(),
            egui::Label::new(format!("{:?}", ui_state.room_status)),
        );
    });
}
