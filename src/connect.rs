use bevy::prelude::*;
use bevy_egui::{egui, EguiContext};

use crate::network::ServerService;
use crate::{GameState, MySystemLabel};

const INITIAL_URL: &str = "http://localhost:8080";

pub struct ConnectMenuPlugin;

impl Plugin for ConnectMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<ConnectButtonPressed>()
            .add_system_set(SystemSet::on_enter(GameState::ConnectMenu).with_system(add_ui_state))
            .add_system_set(
                SystemSet::on_update(GameState::ConnectMenu)
                    .after(MySystemLabel::EguiTop)
                    .with_system(connect_menu)
                    .with_system(on_connect_button_pressed /*.after(connect_menu)*/),
            )
            .add_system_set(
                SystemSet::on_exit(GameState::ConnectMenu).with_system(remove_ui_state),
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

fn add_ui_state(mut commands: Commands) {
    commands.init_resource::<UiState>();
}

fn remove_ui_state(mut commands: Commands) {
    commands.remove_resource::<UiState>();
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

fn on_connect_button_pressed(
    mut commands: Commands,
    mut state: ResMut<State<GameState>>,
    mut event_reader: EventReader<ConnectButtonPressed>,
) {
    if let Some(e) = event_reader.iter().next() {
        let url = e.0.as_str();
        info!("connecting to {}", url);
        let server_service = ServerService::new(url);
        /*if let Err(e) = server_service.check_connection().await {
            panic!("{:#?}", e);
        }*/

        commands.insert_resource(server_service);
        state.set(GameState::LoginMenu).unwrap();
    }
}
