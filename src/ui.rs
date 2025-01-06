use crate::{GameState, MySystemSets};
use bevy::diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin};
use bevy::prelude::*;
use bevy_egui::{egui, EguiContextSettings, EguiContexts, EguiPlugin};
use egui::{FontData, FontDefinitions, FontFamily};
use std::sync::Arc;

pub const PROPORTIONAL_FONT: &[u8] = include_bytes!("../assets/font/fira_go/FiraGO-Regular.ttf");
pub const MONOSPACE_FONT: &[u8] = include_bytes!("../assets/font/fira_mono/FiraMono-Regular.ttf");

pub struct BaseUiPlugin;

impl Plugin for BaseUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(EguiPlugin)
            .add_systems(
                PreStartup,
                configure_egui_settings.in_set(MySystemSets::Egui),
            )
            .add_systems(
                OnEnter(GameState::LoadingAssets),
                setup_fonts.in_set(MySystemSets::Egui),
            );
        app.add_plugins(FrameTimeDiagnosticsPlugin)
            .add_systems(Update, fps_text.in_set(MySystemSets::EguiTop));
    }
}

fn configure_egui_settings(
    _egui_context: EguiContexts,
    mut query: Query<&mut EguiContextSettings>,
) {
    if let Ok(mut settings) = query.get_single_mut() {
        settings.scale_factor = 2.0;
    }
}

fn setup_fonts(mut egui_context: EguiContexts) {
    let mut font_defs = FontDefinitions::empty();
    font_defs.font_data.insert(
        "FiraMono-Regular".to_owned(),
        Arc::new(FontData::from_static(PROPORTIONAL_FONT)),
    );
    font_defs.font_data.insert(
        "FiraGO-Regular".to_owned(),
        Arc::new(FontData::from_static(MONOSPACE_FONT)),
    );

    font_defs.families.insert(
        FontFamily::Proportional,
        vec!["FiraGO-Regular".to_owned(), "FiraMono-Regular".to_owned()],
    );
    font_defs.families.insert(
        FontFamily::Monospace,
        vec!["FiraMono-Regular".to_owned(), "FiraGO-Regular".to_owned()],
    );

    egui_context.ctx_mut().set_fonts(font_defs);
}

fn fps_text(mut egui_context: EguiContexts, diagnostics: Res<DiagnosticsStore>) {
    let fps = diagnostics
        .get(&FrameTimeDiagnosticsPlugin::FPS)
        .unwrap()
        .average()
        .unwrap_or_default();
    egui::TopBottomPanel::top("fps_panel").show(egui_context.ctx_mut(), |ui| {
        ui.horizontal(|ui| {
            ui.label("FPS:");
            ui.monospace(format!("{fps:.0}"));
        });
    });
}
