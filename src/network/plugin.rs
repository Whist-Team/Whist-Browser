use bevy::prelude::*;
use bevy::tasks::IoTaskPool;

use crate::network::*;

pub struct NetworkPlugin;

impl Plugin for NetworkPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup_worker);
    }
}

#[derive(Debug)]
enum NetworkCommand {
    Connect(String),
}

#[derive(Debug)]
enum NetworkResponse {
    ConnectSuccess,
}

#[derive(Debug)]
enum NetworkError {
    ConnectFailure(ConnectError),
}

type NetworkWorker = Worker<NetworkCommand, Result<NetworkResponse, NetworkError>>;
type NetworkWorkerFlipped = Worker<Result<NetworkResponse, NetworkError>, NetworkCommand>;

fn setup_worker(mut commands: Commands, thread_pool: Res<IoTaskPool>) {
    commands.insert_resource(NetworkWorker::spawn(&thread_pool, network_worker));
}

async fn network_worker(mut worker: NetworkWorkerFlipped) {
    #[allow(unused_assignments)]
    let mut server_service: Option<ServerService> = None;
    while let Some(command) = worker.recv().await {
        match command {
            NetworkCommand::Connect(base_url) => {
                server_service = Some(ServerService::new(base_url));
                let res = server_service.unwrap().check_connection().await;
                match res {
                    Ok(_) => worker.send(Ok(NetworkResponse::ConnectSuccess)),
                    Err(e) => worker.send(Err(NetworkError::ConnectFailure(e))),
                };
            }
        }
    }
}

/*fn request_data(data_fetcher: Option<ResMut<NetworkWorker>>) {
    if let Some(fetcher) = data_fetcher {
        fetcher.send(());
    }
}

fn fetch_data(mut commands: Commands, data_fetcher: Option<ResMut<NetworkWorker>>) {
    if let Some(mut fetcher) = data_fetcher {
        while let Ok(fetch_result) = fetcher.try_recv() {
            if let Ok(data) = fetch_result {
                commands.insert_resource(data);
            }
        }
    }
}*/

/*fn show(egui_ctx: ResMut<EguiContext>, data: Option<Res<Data>>) {
    egui::TopBottomPanel::top("top_panel").show(egui_ctx.ctx(), |ui| {
        if let Some(data) = data.as_ref() {
            ui.label(&data.text);
        }
    });
}*/
