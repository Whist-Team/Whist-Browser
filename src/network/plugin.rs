use bevy::prelude::*;
use bevy::tasks::AsyncComputeTaskPool;

use crate::network::*;

pub struct NetworkPlugin;

impl Plugin for NetworkPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<NetworkCommand>()
            .add_event::<ConnectResult>()
            .add_event::<LoginResult>()
            .add_startup_system(setup_worker)
            .add_system(send_network_events)
            .add_system(receive_network_events);
    }
}

#[derive(Debug)]
pub enum ConnectResult {
    Success,
    Failure(ConnectError),
}

#[derive(Debug)]
pub enum LoginResult {
    Success,
    Failure(LoginError),
}

#[derive(Debug, Clone)]
pub enum NetworkCommand {
    Connect(String),
    Login(LoginForm),
}

#[derive(Debug)]
enum NetworkResponse {
    ConnectSuccess,
    ConnectFailure(ConnectError),
    LoginSuccess,
    LoginFailure(LoginError),
}

type NetworkWorker = Worker<NetworkCommand, NetworkResponse>;
type NetworkWorkerFlipped = Worker<NetworkResponse, NetworkCommand>;

fn setup_worker(mut commands: Commands, thread_pool: Res<AsyncComputeTaskPool>) {
    commands.insert_resource(NetworkWorker::spawn(&thread_pool, network_worker));
}

async fn network_worker(mut worker: NetworkWorkerFlipped) {
    info!("network worker spawned");
    #[allow(unused_assignments)]
    let mut server_service: Option<ServerService> = None;
    while let Some(command) = worker.recv().await {
        info!("receiving command");
        match command {
            NetworkCommand::Connect(base_url) => {
                server_service = Some(ServerService::new(base_url));
                let res = server_service.as_ref().unwrap().check_connection().await;
                match res {
                    Ok(_) => worker.send(NetworkResponse::ConnectSuccess),
                    Err(e) => worker.send(NetworkResponse::ConnectFailure(e)),
                };
            }
            NetworkCommand::Login(login_form) => {
                let res = server_service.as_mut().unwrap().login(&login_form).await;
                match res {
                    Ok(_) => worker.send(NetworkResponse::LoginSuccess),
                    Err(e) => worker.send(NetworkResponse::LoginFailure(e)),
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
    mut connect_result: EventWriter<ConnectResult>,
    mut login_result: EventWriter<LoginResult>,
) {
    if let Some(mut network_worker) = network_worker {
        while let Ok(network_response) = network_worker.try_recv() {
            info!("response: {:?}", network_response);
            match network_response {
                NetworkResponse::ConnectSuccess => connect_result.send(ConnectResult::Success),
                NetworkResponse::ConnectFailure(e) => {
                    connect_result.send(ConnectResult::Failure(e))
                }
                NetworkResponse::LoginSuccess => login_result.send(LoginResult::Success),
                NetworkResponse::LoginFailure(e) => login_result.send(LoginResult::Failure(e)),
            }
        }
    }
}
