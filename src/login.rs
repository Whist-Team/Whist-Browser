use bevy::prelude::*;
use bevy_egui::{egui, EguiContext};

use crate::{GameState, MySystemLabel};

pub struct LoginMenuPlugin;

impl Plugin for LoginMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<LoginButtonPressed>()
            .add_system_set(SystemSet::on_enter(GameState::LoginMenu).with_system(add_ui_state))
            .add_system_set(
                SystemSet::on_update(GameState::LoginMenu)
                    .after(MySystemLabel::EguiTop)
                    .with_system(login_menu)
                    .with_system(on_login_button_pressed.after(login_menu)),
            )
            .add_system_set(SystemSet::on_exit(GameState::LoginMenu).with_system(remove_ui_state));
    }
}

struct UiState {
    username: String,
    password: String,
}

impl Default for UiState {
    fn default() -> Self {
        Self {
            username: "root".to_owned(),
            password: "password".to_owned(),
        }
    }
}

fn add_ui_state(mut commands: Commands) {
    commands.init_resource::<UiState>();
}

fn remove_ui_state(mut commands: Commands) {
    commands.remove_resource::<UiState>();
}

struct LoginButtonPressed(String, String);

fn login_menu(
    mut egui_context: ResMut<EguiContext>,
    mut ui_state: ResMut<UiState>,
    mut event_writer: EventWriter<LoginButtonPressed>,
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
        if ui.button("Login").clicked() {
            event_writer.send(LoginButtonPressed(
                ui_state.username.to_owned(),
                ui_state.password.to_owned(),
            ));
        }
    });
}

fn on_login_button_pressed(
    // mut server_service: ResMut<ServerService>,
    mut state: ResMut<State<GameState>>,
    mut event_reader: EventReader<LoginButtonPressed>,
) {
    if let Some(e) = event_reader.iter().next() {
        let LoginButtonPressed(username, password) = e;
        // TODO: remove log of pw
        info!("login as user '{}' with password '{}'", username, password);
        /*if let Err(e) = server_service.login(&LoginForm::new(username, password)) {
            panic!("{:#?}", e);
        }*/

        state.set(GameState::RoomMenu).unwrap();
    }
}
