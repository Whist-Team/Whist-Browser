use bevy::prelude::*;
use bevy_egui::{egui, EguiContext};
use std::{env, fmt};

use crate::network::{GitHubAuthRequest, LoginForm, LoginResult, NetworkCommand, SwapTokenRequest};
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

pub struct GitHubAuthData {
    user_code: String,
    device_code: String,
}

impl GitHubAuthData {
    pub fn new(user_code: impl Into<String>, device_code: impl Into<String>) -> Self {
        Self {
            user_code: user_code.into(),
            device_code: device_code.into(),
        }
    }
}

impl fmt::Debug for GitHubAuthData {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("GitHubAuthData")
            .field("user_code", &self.user_code)
            .finish()
    }
}

#[derive(Debug)]
enum LoginStatus {
    NotStarted,
    LoggingIn,
    LoginError(String),
    GitHubRequest,
    GitHubAuth(GitHubAuthData),
}

impl LoginStatus {
    fn enable_login_button(&self) -> bool {
        matches!(self, LoginStatus::NotStarted | LoginStatus::LoginError(_))
    }

    fn enable_confirm_button(&self) -> bool {
        matches!(self, LoginStatus::GitHubAuth(_))
    }

    fn enable_label(&self) -> bool {
        matches!(
            self,
            LoginStatus::LoggingIn | LoginStatus::GitHubAuth(_) | LoginStatus::LoginError(_)
        )
    }

    fn github_device_code(&self) -> String {
        match self {
            LoginStatus::GitHubAuth(data) => data.device_code.clone(),
            _ => panic!("No device code found!"),
        }
    }
}

#[derive(Resource)]
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
        assert!(matches!(
            ui_state.login_status,
            LoginStatus::LoggingIn | LoginStatus::GitHubRequest
        ));
        match connect_result {
            LoginResult::Success => {
                state.set(GameState::RoomMenu).unwrap();
            }
            LoginResult::Failure(e) => {
                ui_state.login_status = LoginStatus::LoginError(format!("{e:?}"));
            }
            LoginResult::GitHubWait(result) => match result {
                Ok(token) => {
                    let data = GitHubAuthData::new(&token.user_code, &token.device_code);
                    ui_state.login_status = LoginStatus::GitHubAuth(data);
                }
                Err(_) => ui_state.login_status = LoginStatus::NotStarted,
            },
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
        let github_button = ui.add_enabled(
            ui_state.login_status.enable_login_button(),
            egui::Button::new("Github"),
        );
        let github_button_confirm = ui.add_enabled(
            ui_state.login_status.enable_confirm_button(),
            egui::Button::new("Confirm"),
        );
        if button.clicked() {
            ui_state.login_status = LoginStatus::LoggingIn;
            event_writer.send(NetworkCommand::Login(LoginForm::new(
                ui_state.username.as_str(),
                ui_state.password.as_str(),
            )));
        }
        if github_button.clicked() {
            let client_id = env::var("GITHUB_CLIENT_ID").unwrap();
            ui_state.login_status = LoginStatus::GitHubRequest;
            event_writer.send(NetworkCommand::GithubAuth(GitHubAuthRequest::new(
                client_id,
            )));
        }
        if github_button_confirm.clicked() {
            let device_code = ui_state.login_status.github_device_code();
            ui_state.login_status = LoginStatus::LoggingIn;
            event_writer.send(NetworkCommand::SwapToken(SwapTokenRequest::new(
                device_code,
            )));
        }
        ui.add_visible(
            ui_state.login_status.enable_label(),
            egui::Label::new(format!("{:?}", ui_state.login_status)),
        );
    });
}
