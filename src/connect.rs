use bevy::prelude::*;
use bevy_egui::{egui, EguiContext};

use crate::{GameState, MySystemLabel};

const INITIAL_URL: &str = "http://localhost:8080";

pub struct ConnectMenuPlugin;

impl Plugin for ConnectMenuPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<UiState>()
            .add_event::<ConnectButtonPressed>()
            .add_system_set(
                SystemSet::on_update(GameState::ConnectMenu)
                    .after(MySystemLabel::EguiTop)
                    .with_system(connect_menu)
                    .with_system(on_connect_button_pressed.after(connect_menu)),
            );
    }
}

struct UiState {
    connect_url: String,
}

impl Default for UiState {
    fn default() -> Self {
        Self {
            connect_url: INITIAL_URL.to_owned(),
        }
    }
}

struct ConnectButtonPressed(String);

fn connect_menu(
    mut egui_context: ResMut<EguiContext>,
    mut ui_state: ResMut<UiState>,
    mut event_writer: EventWriter<ConnectButtonPressed>,
) {
    egui::CentralPanel::default().show(egui_context.ctx_mut(), |ui| {
        ui.horizontal(|ui| {
            ui.label("Connect to: ");
            ui.text_edit_singleline(&mut ui_state.connect_url);
        });
        if ui.button("Connect").clicked() {
            event_writer.send(ConnectButtonPressed(ui_state.connect_url.to_owned()));
        }
    });
}

fn on_connect_button_pressed(mut event_reader: EventReader<ConnectButtonPressed>) {
    if let Some(e) = event_reader.iter().next() {
        info!("connecting to {}", e.0);
    }
}
