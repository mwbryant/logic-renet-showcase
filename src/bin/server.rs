use bevy::log::{LogPlugin, LogSettings};
use local_ip_address::local_ip;
use logic_renet_demo::*;
use std::{
    net::{SocketAddr, UdpSocket},
    time::SystemTime,
};

fn create_renet_server() -> RenetServer {
    let current_time = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap();

    /* Public hosting, requires port forwarding
    let rt = tokio::runtime::Runtime::new().unwrap();
    let public_ip = rt.block_on(public_ip::addr()).unwrap();
    let server_addr = SocketAddr::new(public_ip, 42069);
    */

    let server_addr = SocketAddr::new(local_ip().unwrap(), 42069);
    info!("Creating Server! {:?}", server_addr);

    let server_config =
        ServerConfig::new(64, PROTOCOL_ID, server_addr, ServerAuthentication::Unsecure);

    let connection_config = RenetConnectionConfig::default();

    let inbound_server_addr = SocketAddr::new(local_ip().unwrap(), 42069);
    let socket = UdpSocket::bind(inbound_server_addr).unwrap();

    RenetServer::new(current_time, server_config, connection_config, socket).unwrap()
}

fn main() {
    App::new()
        .insert_resource(LogSettings {
            filter: "info,wgpu_core=warn,wgpu_hal=off,rechannel=warn".into(),
            level: bevy::log::Level::DEBUG,
        })
        .add_plugins(MinimalPlugins)
        .add_plugin(LogPlugin::default())
        .add_plugin(RenetServerPlugin)
        .insert_resource(create_renet_server())
        .add_system(server_events)
        .add_system(server_ping)
        .run();
}

fn server_events(mut events: EventReader<ServerEvent>) {
    for event in events.iter() {
        match event {
            ServerEvent::ClientConnected(id, _user_data) => info!("Connected {}!", id),
            ServerEvent::ClientDisconnected(id) => info!("Disconnected {}!", id),
        }
    }
}

fn server_ping(mut server: ResMut<RenetServer>) {
    let reliable_channel_id = ReliableChannelConfig::default().channel_id;

    for client_id in server.clients_id().into_iter() {
        while let Some(message) = server.receive_message(client_id, reliable_channel_id) {
            let client_message = bincode::deserialize(&message).unwrap();
            match client_message {
                ClientMessage::Ping => {
                    info!("Got ping from {}!", client_id);
                    let pong = bincode::serialize(&ServerMessage::Pong).unwrap();
                    server.send_message(client_id, reliable_channel_id, pong);
                }
            }
        }
    }
}
