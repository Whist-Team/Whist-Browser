use bevy::prelude::*;
use bevy_egui::{egui, EguiContext};

use crate::network::{LoginForm, LoginResult, NetworkCommand};
use crate::{GameState, MySystemLabel};

pub struct LoginMenuPlugin;

impl Plugin for LoginMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(GameState::LoginMenu).with_system(add_ui_state))
            .add_system_set(
                SystemSet::on_update(GameState::LoginMenu)
                    .after(MySystemLabel::EguiTop)
                    .with_system(update_ui_state)
                    .with_system(login_menu.after(update_ui_state)),
            )
            .add_system_set(SystemSet::on_exit(GameState::LoginMenu).with_system(remove_ui_state));
    }
}

#[derive(Debug)]
enum LoginStatus {
    NotStarted,
    LoggingIn,
    LoginError(String),
}

impl LoginStatus {
    fn enable_login_button(&self) -> bool {
        matches!(self, LoginStatus::NotStarted | LoginStatus::LoginError(_))
    }

    fn enable_label(&self) -> bool {
        matches!(self, LoginStatus::LoggingIn | LoginStatus::LoginError(_))
    }
}

struct UiState {
    username: String,
    password: String,
    login_status: LoginStatus,
}

impl Default for UiState {
    fn default() -> Self {
        Self {
            username: "root".to_owned(),
            password: "password".to_owned(),
            login_status: LoginStatus::NotStarted,
        }
    }
}

fn add_ui_state(mut commands: Commands) {
    commands.init_resource::<UiState>();
}

fn remove_ui_state(mut commands: Commands) {
    commands.remove_resource::<UiState>();
}

fn update_ui_state(
    mut state: ResMut<State<GameState>>,
    mut ui_state: ResMut<UiState>,
    mut login_results: EventReader<LoginResult>,
) {
    if let Some(connect_result) = login_results.iter().next() {
        assert!(matches!(ui_state.login_status, LoginStatus::LoggingIn));
        match connect_result {
            LoginResult::Success => {
                state.set(GameState::RoomMenu).unwrap();
            }
            LoginResult::Failure(e) => {
                ui_state.login_status = LoginStatus::LoginError(format!("{:?}", e));
            }
        };
    }
}

fn login_menu(
    mut egui_context: ResMut<EguiContext>,
    mut ui_state: ResMut<UiState>,
    mut event_writer: EventWriter<NetworkCommand>,
) {
    egui::CentralPanel::default().show(egui_context.ctx_mut(), |ui| {
        ui.horizontal(|ui| {
            ui.label("Username: ");
            ui.text_edit_singleline(&mut ui_state.username);
        });
        ui.horizontal(|ui| {
            ui.label("Password: ");
            ui.text_edit_singleline(&mut ui_state.password); // TODO: hide password
        });
        let button = ui.add_enabled(
            ui_state.login_status.enable_login_button(),
            egui::Button::new("Login"),
        );
        if button.clicked() {
            ui_state.login_status = LoginStatus::LoggingIn;
            event_writer.send(NetworkCommand::Login(LoginForm::new(
                ui_state.username.as_str(),
                ui_state.password.as_str(),
            )));
        }
        ui.add_enabled(
            ui_state.login_status.enable_label(),
            egui::Label::new(format!("{:?}", ui_state.login_status)),
        );
    });
}
