use bevy::prelude::*;
use bevy::tasks::IoTaskPool;

use crate::network::*;

pub struct NetworkPlugin;

impl Plugin for NetworkPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<NetworkCommand>()
            .add_event::<ConnectSuccess>()
            .add_event::<ConnectFailure>()
            .add_startup_system(setup_worker)
            .add_system(send_network_events)
            .add_system(receive_network_events);
    }
}

pub struct ConnectSuccess;
pub struct ConnectFailure(ConnectError);

#[derive(Debug, Clone)]
pub enum NetworkCommand {
    Connect(String),
}

#[derive(Debug)]
enum NetworkResponse {
    ConnectSuccess,
    ConnectFailure(ConnectError),
}

type NetworkWorker = Worker<NetworkCommand, NetworkResponse>;
type NetworkWorkerFlipped = Worker<NetworkResponse, NetworkCommand>;

fn setup_worker(mut commands: Commands, thread_pool: Res<IoTaskPool>) {
    commands.insert_resource(NetworkWorker::spawn(&thread_pool, network_worker));
}

async fn network_worker(mut worker: NetworkWorkerFlipped) {
    #[allow(unused_assignments)]
    let mut server_service: Option<ServerService> = None;
    while let Some(command) = worker.recv().await {
        info!("receiving command");
        match command {
            NetworkCommand::Connect(base_url) => {
                server_service = Some(ServerService::new(base_url));
                let res = server_service.unwrap().check_connection().await;
                match res {
                    Ok(_) => worker.send(NetworkResponse::ConnectSuccess),
                    Err(e) => worker.send(NetworkResponse::ConnectFailure(e)),
                };
            }
        }
    }
}

fn send_network_events(
    network_worker: Option<Res<NetworkWorker>>,
    mut network_events_to_send: EventReader<NetworkCommand>,
) {
    if let Some(network_worker) = network_worker {
        for network_event in network_events_to_send.iter() {
            info!("event: {:?}", network_event);
            network_worker.send(network_event.to_owned());
        }
    }
}

fn receive_network_events(
    network_worker: Option<ResMut<NetworkWorker>>,
    mut connect_success: EventWriter<ConnectSuccess>,
    mut connect_failure: EventWriter<ConnectFailure>,
) {
    if let Some(mut network_worker) = network_worker {
        while let Ok(network_response) = network_worker.try_recv() {
            match network_response {
                NetworkResponse::ConnectSuccess => connect_success.send(ConnectSuccess),
                NetworkResponse::ConnectFailure(e) => connect_failure.send(ConnectFailure(e)),
            }
        }
    }
}
