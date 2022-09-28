use bevy::log::{LogPlugin, LogSettings};
use local_ip_address::local_ip;
use logic_renet_demo::*;
use std::{
    net::{SocketAddr, UdpSocket},
    time::SystemTime,
};

fn create_renet_server() -> RenetServer {
    let server_addr = SocketAddr::new(local_ip().unwrap(), 42069);
    println!("Creating Server! {:?}", server_addr);

    let socket = UdpSocket::bind(server_addr).unwrap();
    let connection_config = RenetConnectionConfig::default();
    let server_config =
        ServerConfig::new(64, PROTOCOL_ID, server_addr, ServerAuthentication::Unsecure);
    let current_time = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap();
    RenetServer::new(current_time, server_config, connection_config, socket).unwrap()
}

fn main() {
    App::new()
        .insert_resource(LogSettings {
            filter: "info,wgpu_core=warn,wgpu_hal=off,rechannel=warn".into(),
            level: bevy::log::Level::DEBUG,
        })
        .insert_resource(WindowDescriptor {
            width: 1200.,
            height: 640.,
            title: "Voxel Server".to_string(),
            //present_mode: PresentMode::Immediate,
            resizable: false,
            ..Default::default()
        })
        .add_plugins(MinimalPlugins)
        .add_plugin(LogPlugin::default())
        .add_plugin(RenetServerPlugin)
        .insert_resource(create_renet_server())
        .add_system(server_ping)
        .run();
}

pub fn server_ping(mut server: ResMut<RenetServer>) {
    for client_id in server.clients_id().into_iter() {
        let reliable_channel_id = ReliableChannelConfig::default().channel_id;
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
