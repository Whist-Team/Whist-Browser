use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};

use crate::network::{ConnectResult, NetworkCommand};
use crate::{GameState, MySystemSets};

const INITIAL_URL: &str = "http://localhost:8080";

pub struct ConnectMenuPlugin;

impl Plugin for ConnectMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::ConnectMenu), add_ui_state)
            .add_systems(
                Update,
                (update_ui_state, connect_menu)
                    .chain()
                    .run_if(in_state(GameState::ConnectMenu))
                    .in_set(MySystemSets::Egui),
            )
            .add_systems(OnExit(GameState::ConnectMenu), remove_ui_state);
    }
}

#[derive(Debug)]
#[allow(dead_code)] // rust warns about an unread field, but we use it in the "Debug" impl
enum ConnectStatus {
    NotStarted,
    Connecting,
    ConnectionError(String),
}

impl ConnectStatus {
    fn enable_connect_button(&self) -> bool {
        matches!(
            self,
            ConnectStatus::NotStarted | ConnectStatus::ConnectionError(_)
        )
    }

    fn enable_label(&self) -> bool {
        matches!(
            self,
            ConnectStatus::Connecting | ConnectStatus::ConnectionError(_)
        )
    }
}

#[derive(Resource)]
struct UiState {
    connect_url: String,
    connect_status: ConnectStatus,
}

impl Default for UiState {
    fn default() -> Self {
        Self {
            connect_url: INITIAL_URL.to_owned(),
            connect_status: ConnectStatus::NotStarted,
        }
    }
}

fn add_ui_state(mut commands: Commands) {
    info!("starting ConnectMenu");
    commands.init_resource::<UiState>();
}

fn remove_ui_state(mut commands: Commands) {
    commands.remove_resource::<UiState>();
}

fn update_ui_state(
    mut state: ResMut<NextState<GameState>>,
    mut ui_state: ResMut<UiState>,
    mut connect_results: EventReader<ConnectResult>,
) {
    if let Some(connect_result) = connect_results.read().next() {
        assert!(matches!(ui_state.connect_status, ConnectStatus::Connecting));
        match connect_result {
            ConnectResult::Success => {
                state.set(GameState::LoginMenu);
            }
            ConnectResult::Failure(e) => {
                ui_state.connect_status = ConnectStatus::ConnectionError(format!("{e:?}"));
            }
        };
    }
}

fn connect_menu(
    mut egui_context: EguiContexts,
    mut ui_state: ResMut<UiState>,
    mut event_writer: EventWriter<NetworkCommand>,
) {
    egui::CentralPanel::default().show(egui_context.ctx_mut(), |ui| {
        ui.horizontal(|ui| {
            ui.label("Connect to:");
            ui.text_edit_singleline(&mut ui_state.connect_url);
        });
        let button = ui.add_enabled(
            ui_state.connect_status.enable_connect_button(),
            egui::Button::new("Connect"),
        );
        if button.clicked() {
            ui_state.connect_status = ConnectStatus::Connecting;
            event_writer.send(NetworkCommand::Connect(ui_state.connect_url.to_owned()));
        }
        ui.add_visible(
            ui_state.connect_status.enable_label(),
            egui::Label::new(format!("{:?}", ui_state.connect_status)),
        );
    });
}
