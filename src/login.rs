use bevy::prelude::*;
use bevy_egui::{egui, EguiContext};

use crate::{GameState, MySystemLabel};

pub struct LoginMenuPlugin;

impl Plugin for LoginMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(GameState::LoginMenu).with_system(add_ui_state))
            .add_system_set(
                SystemSet::on_update(GameState::LoginMenu)
                    .after(MySystemLabel::EguiTop)
                    .with_system(login_menu),
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

fn login_menu(
    mut egui_context: ResMut<EguiContext>,
    mut ui_state: ResMut<UiState>,
    // mut event_writer: EventWriter<NetworkCommand>,
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
            /*event_writer.send(NetworkCommand::Login(
                ui_state.username.to_owned(),
                ui_state.password.to_owned(),
            ));*/
        }
    });
}
