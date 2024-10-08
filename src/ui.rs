use std::collections::BTreeMap;

use bevy::diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin};
use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts, EguiPlugin, EguiSettings};

use crate::{GameState, MySystemSets};

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

fn configure_egui_settings(_egui_context: EguiContexts, mut query: Query<&mut EguiSettings>) {
    if let Ok(mut settings) = query.get_single_mut() {
        settings.scale_factor = 2.0;
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
