use std::{env, fmt};

use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};

use crate::network::{
    GitHubAuthRequest, GitHubTempTokenResult, LoginForm, LoginResult, NetworkCommand,
    SwapTokenRequest, UserCreateRequest, UserCreateResult,

};
use crate::{GameState, MySystemSets};

pub struct LoginMenuPlugin;

impl Plugin for LoginMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::LoginMenu), add_ui_state)
            .add_systems(
                Update,
                (update_ui_state, login_menu)
                    .chain()
                    .run_if(in_state(GameState::LoginMenu))
                    .after(MySystemSets::EguiTop),
            )
            .add_systems(OnExit(GameState::LoginMenu), remove_ui_state);
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
    RegisterWindow,
    Registering,
    RegisteringError(String),
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
    password_repeat: String,
    login_status: LoginStatus,
}

impl UiState {
    fn main_interaction_blocked(&self) -> bool {
        self.window_interaction_blocked()
            || matches!(self.login_status, LoginStatus::RegisterWindow)
    }

    fn window_interaction_blocked(&self) -> bool {
        matches!(self.login_status, LoginStatus::Registering)
    }

    fn enable_register_window_button(&self) -> bool {
        !self.main_interaction_blocked()
    }

    fn enable_register_button(&self) -> bool {
        !self.window_interaction_blocked()
            && match self.login_status {
                LoginStatus::RegisterWindow => true,
                _ => panic!("illegal state"),
            }
    }

    fn enable_cancel_button(&self) -> bool {
        !self.window_interaction_blocked()
    }

    fn reset(&mut self) {
        self.login_status = LoginStatus::NotStarted;
        self.username.clear();
        self.password.clear();
        self.password_repeat.clear();
    }
}

impl Default for UiState {
    fn default() -> Self {
        Self {
            username: "root".to_owned(),
            password: "password".to_owned(),
            password_repeat: "password".to_owned(),
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
    mut state: ResMut<NextState<GameState>>,
    mut ui_state: ResMut<UiState>,
    mut login_results: EventReader<LoginResult>,
    mut register_results: EventReader<UserCreateResult>,
) {
    if let Some(connect_result) = login_results.iter().next() {
        assert!(matches!(
            ui_state.login_status,
            LoginStatus::LoggingIn | LoginStatus::GitHubRequest
        ));
        match connect_result {
            LoginResult::Success => {
                state.set(GameState::RoomMenu);
            }
            LoginResult::Failure(e) => {
                ui_state.login_status = LoginStatus::LoginError(format!("{e:?}"));
            }
            LoginResult::GitHubWait(GitHubTempTokenResult(result)) => match result {
                Ok(token) => {
                    let data = GitHubAuthData::new(&token.user_code, &token.device_code);
                    ui_state.login_status = LoginStatus::GitHubAuth(data);
                }
                Err(_) => ui_state.login_status = LoginStatus::NotStarted,
            },
        };
    }
    if let Some(register_result) = register_results.iter().next() {
        assert!(matches!(ui_state.login_status, LoginStatus::Registering));
        match register_result {
            Ok(_) => ui_state.login_status = LoginStatus::NotStarted,
            Err(e) => {
                ui_state.login_status = LoginStatus::RegisteringError(format!("{:?}", e));
            }
        }
    }
}

fn login_menu(
    mut egui_context: EguiContexts,
    mut ui_state: ResMut<UiState>,
    mut event_writer: EventWriter<NetworkCommand>,
) {
    let ui_state: &mut UiState = &mut ui_state;
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
        let register_button = ui.add_enabled(
            ui_state.enable_register_window_button(),
            egui::Button::new("Register"),
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
        if register_button.clicked() {
            ui_state.reset();
            ui_state.login_status = LoginStatus::RegisterWindow;
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

    match ui_state.login_status {
        LoginStatus::RegisterWindow | LoginStatus::Registering => {
            egui::Window::new("Register User").show(egui_context.ctx_mut(), |ui| {
                ui.horizontal(|ui| {
                    ui.label("Name:");
                    ui.text_edit_singleline(&mut ui_state.username);
                });
                ui.horizontal(|ui| {
                    ui.label("Password:");
                    ui.text_edit_singleline(&mut ui_state.password);
                });
                ui.horizontal(|ui| {
                    ui.label("Repeat Password:");
                    ui.text_edit_singleline(&mut ui_state.password_repeat);
                });
                ui.horizontal(|ui| {
                    let register_button = ui.add_enabled(
                        ui_state.enable_register_button(),
                        egui::Button::new("Register"),
                    );
                    if register_button.clicked() && ui_state.password == ui_state.password_repeat {
                        ui_state.login_status = LoginStatus::Registering;
                        event_writer.send(NetworkCommand::UserCreate(UserCreateRequest {
                            username: ui_state.username.to_string(),
                            password: ui_state.password.to_string(),
                        }));
                    }

                    let cancel_button = ui
                        .add_enabled(ui_state.enable_cancel_button(), egui::Button::new("Cancel"));
                    if cancel_button.clicked() {
                        ui_state.reset();
                    }
                });
            });
        }
        _ => {}
    }
}
