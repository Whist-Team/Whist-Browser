use bevy::prelude::ResMut;
use bevy_egui::{egui, EguiContext};

#[derive(Debug)]
enum RoomStatus {
    WaitingForPlayers,
    ReadyToStart,
    Starting,
    Started,
}

impl RoomStatus {
    pub(crate) fn enable_start_button(&self) -> bool {
        matches!(self, RoomStatus::ReadyToStart)
    }
}

struct UiState {
    room_status: RoomStatus,
}

impl UiState {
    fn enable_start_button(&self) -> bool {
        match self.room_status {
            RoomStatus::ReadyToStart => true,
            _ => false,
        }
    }
}

impl Default for UiState {
    fn default() -> Self {
        Self {
            room_status: RoomStatus::WaitingForPlayers,
        }
    }
}

fn update_ui_state(mut ui_state: ResMut<UiState>) {}

fn lobby_menu(mut egui_context: ResMut<EguiContext>, mut ui_state: ResMut<UiState>) {
    let ui_state: &mut UiState = &mut ui_state;
    egui::CentralPanel::default().show(egui_context.ctx_mut(), |ui| {
        let start_button = ui.add_enabled(
            ui_state.room_status.enable_start_button(),
            egui::Button::new("Start"),
        );
    });
}
