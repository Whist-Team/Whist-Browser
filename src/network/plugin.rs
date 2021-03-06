use bevy::prelude::*;
use bevy::tasks::IoTaskPool;

use crate::network::*;

pub struct NetworkPlugin;

impl Plugin for NetworkPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<NetworkCommand>()
            .add_event::<ConnectResult>()
            .add_event::<LoginResult>()
            .add_event::<GameListResult>()
            .add_event::<GameJoinResult>()
            .add_event::<GameCreateResult>()
            .add_event::<WebSocketCommand>()
            .add_startup_system(setup_worker)
            .add_system(send_network_events)
            .add_system(receive_network_events)
            .add_system(send_websocket_commands)
            .add_system(receive_websocket_responses);
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
    GetGameList,
    GameJoin(String, GameJoinRequest),
    GameCreate(GameCreateRequest),
}

#[derive(Debug)]
enum NetworkResponse {
    ConnectSuccess,
    ConnectFailure(ConnectError),
    LoginSuccess,
    LoginFailure(LoginError),
    GameList(GameListResult),
    GameJoin(GameJoinResult),
    GameCreate(GameCreateResult),
}

#[derive(Debug)]
pub enum WebSocketCommand {
    Connect(String),
}

#[derive(Debug)]
enum WebSocketResponse {
    ConnectSuccess,
    ConnectError,
    Event(Event),
    Error,
}

type NetworkWorker = Worker<NetworkCommand, NetworkResponse>;
type NetworkWorkerFlipped = Worker<NetworkResponse, NetworkCommand>;

type WebSocketWorker = Worker<(), WebSocketResponse>;
type WebSocketWorkerFlipped = Worker<WebSocketResponse, ()>;

fn setup_worker(mut commands: Commands, thread_pool: Res<IoTaskPool>) {
    commands.insert_resource(NetworkWorker::spawn(&thread_pool, network_worker));
}

async fn network_worker(mut worker: NetworkWorkerFlipped) {
    info!("network worker spawned");
    let mut server_service: Option<ServerService> = None;
    while let Some(command) = worker.recv().await {
        info!("receiving network command {:?}", command);
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
            NetworkCommand::GetGameList => {
                worker.send(NetworkResponse::GameList(
                    server_service.as_ref().unwrap().get_games().await,
                ));
            }
            NetworkCommand::GameJoin(id, game_join_request) => {
                worker.send(NetworkResponse::GameJoin(
                    server_service
                        .as_ref()
                        .unwrap()
                        .join_game(id, &game_join_request)
                        .await,
                ));
            }
            NetworkCommand::GameCreate(game_create_request) => {
                worker.send(NetworkResponse::GameCreate(
                    server_service
                        .as_ref()
                        .unwrap()
                        .create_game(&game_create_request)
                        .await,
                ));
            }
        }
    }
}

fn send_network_events(
    network_worker: Option<ResMut<NetworkWorker>>,
    mut network_events_to_send: EventReader<NetworkCommand>,
) {
    if let Some(network_worker) = network_worker {
        for network_event in network_events_to_send.iter() {
            info!("network event: {:?}", network_event);
            network_worker.send(network_event.to_owned());
        }
    }
}

fn receive_network_events(
    network_worker: Option<ResMut<NetworkWorker>>,
    mut connect_result: EventWriter<ConnectResult>,
    mut login_result: EventWriter<LoginResult>,
    mut game_list_result: EventWriter<GameListResult>,
    mut game_join_result: EventWriter<GameJoinResult>,
    mut game_create_result: EventWriter<GameCreateResult>,
) {
    if let Some(mut network_worker) = network_worker {
        while let Ok(Some(network_response)) = network_worker.try_recv() {
            info!("worker response: {:?}", network_response);
            match network_response {
                NetworkResponse::ConnectSuccess => connect_result.send(ConnectResult::Success),
                NetworkResponse::ConnectFailure(e) => {
                    connect_result.send(ConnectResult::Failure(e))
                }
                NetworkResponse::LoginSuccess => login_result.send(LoginResult::Success),
                NetworkResponse::LoginFailure(e) => login_result.send(LoginResult::Failure(e)),
                NetworkResponse::GameList(result) => game_list_result.send(result),
                NetworkResponse::GameJoin(result) => game_join_result.send(result),
                NetworkResponse::GameCreate(result) => game_create_result.send(result),
            }
        }
    }
}

fn send_websocket_commands(
    mut commands: Commands,
    thread_pool: Res<IoTaskPool>,
    mut websocket_events: EventReader<WebSocketCommand>,
) {
    for websocket_event in websocket_events.iter() {
        info!("websocket event: {:?}", websocket_event);
        match websocket_event {
            WebSocketCommand::Connect(url) => {
                let url = url.to_owned();
                let worker = WebSocketWorker::spawn(
                    &thread_pool,
                    |worker: WebSocketWorkerFlipped| async move {
                        match WebSocket::connect(url).await {
                            Ok((_sender, mut receiver)) => {
                                worker.send(WebSocketResponse::ConnectSuccess);
                                while let Result::<Event, _>::Ok(event) = receiver.recv_json().await
                                {
                                    worker.send(WebSocketResponse::Event(event));
                                }
                                worker.send(WebSocketResponse::Error);
                            }
                            Err(_) => {
                                worker.send(WebSocketResponse::ConnectError);
                            }
                        }
                    },
                );
                commands.insert_resource(worker);
            }
        }
    }
}

fn receive_websocket_responses(websocket_worker: Option<ResMut<WebSocketWorker>>) {
    if let Some(mut websocket_worker) = websocket_worker {
        while let Ok(Some(websocket_response)) = websocket_worker.try_recv() {
            info!("websocket response: {:?}", websocket_response);
            match websocket_response {
                WebSocketResponse::ConnectSuccess => {}
                WebSocketResponse::ConnectError => panic!("websocket connect error"),
                WebSocketResponse::Event(event) => todo!("handle {:?} event", event),
                WebSocketResponse::Error => panic!("websocket error"),
            }
        }
    }
}
