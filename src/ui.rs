use std::collections::BTreeMap;

use bevy::diagnostic::{Diagnostics, FrameTimeDiagnosticsPlugin};
use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts, EguiPlugin, EguiSettings};

use crate::{GameState, MySystemSets};

pub const PROPORTIONAL_FONT: &[u8] = include_bytes!("../assets/font/fira_go/FiraGO-Regular.ttf");
pub const MONOSPACE_FONT: &[u8] = include_bytes!("../assets/font/fira_mono/FiraMono-Regular.ttf");

pub struct BaseUiPlugin;

impl Plugin for BaseUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(EguiPlugin)
            .insert_resource(EguiSettings { scale_factor: 2.0 })
            .add_system(setup_fonts.in_schedule(OnExit(GameState::LoadingAssets)));
        app.add_plugin(FrameTimeDiagnosticsPlugin::default())
            .add_system(fps_text.in_set(MySystemSets::EguiTop));
    }
}

fn setup_fonts(mut egui_context: EguiContexts) {
    let mut font_data: BTreeMap<String, egui::FontData> = BTreeMap::new();
    font_data.insert(
        "FiraMono-Regular".to_owned(),
        egui::FontData::from_static(PROPORTIONAL_FONT),
    );
    font_data.insert(
        "FiraGO-Regular".to_owned(),
        egui::FontData::from_static(MONOSPACE_FONT),
    );

    let mut families: BTreeMap<egui::FontFamily, Vec<String>> = BTreeMap::new();
    families.insert(
        egui::FontFamily::Proportional,
        vec!["FiraGO-Regular".to_owned(), "FiraMono-Regular".to_owned()],
    );
    families.insert(
        egui::FontFamily::Monospace,
        vec!["FiraMono-Regular".to_owned(), "FiraGO-Regular".to_owned()],
    );

    egui_context.ctx_mut().set_fonts(egui::FontDefinitions {
        font_data,
        families,
    });
}

fn fps_text(mut egui_context: EguiContexts, diagnostics: Res<Diagnostics>) {
    let fps = diagnostics
        .get(FrameTimeDiagnosticsPlugin::FPS)
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
