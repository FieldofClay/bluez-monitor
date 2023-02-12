use bluez_async::{BluetoothSession, BluetoothEvent, AdapterEvent, DeviceEvent, DeviceInfo, AdapterInfo};
use futures::stream::StreamExt;
use serde::Serialize;
use serde_json::json;

#[derive(Serialize)]
struct DevInfo {
    pub mac_address: String,
    pub address_type: String,
    pub name: String,
    pub paired: bool,
    pub connected: bool,
    pub class: String,
    pub icon: String,
    pub trusted: bool,
}

#[derive(Serialize)]
struct AdapInfo {
    pub mac_address: String,
    pub address_type: String,
    pub name: String,
    pub alias: String,
    pub powered: bool,
    pub discovering: bool,
}

fn handle_device_info(info: &DeviceInfo) -> DevInfo{
    DevInfo {
        address_type: info.address_type.to_string(),
        name: info.name.clone().unwrap_or_default(),
        mac_address: info.mac_address.to_string(),
        icon: info.icon.clone().unwrap_or_default(),
        class: info.class.map(|c| format!("{:#010x}", c)).unwrap_or_default(),
        paired: info.paired,
        connected: info.connected,
        trusted: info.trusted,
    }
}

fn handle_adapter_info(info: &AdapterInfo) -> AdapInfo{
    AdapInfo { 
        mac_address: info.mac_address.to_string(),
        address_type: info.address_type.to_string(),
        name: info.name.clone(),
        alias: info.name.clone(),
        powered: info.powered,
        discovering: info.discovering }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a new BluetoothSession
    let (_, session) = BluetoothSession::new().await?;

    // Get the event stream
    let mut event_stream = session.event_stream().await?.fuse();

    // Listen to the stream of events and process each event
    while let Some(event) = event_stream.next().await {
        match event {
            BluetoothEvent::Adapter { id, event } => {
                let adapter_info = session.get_adapter_info(&id).await?;
                match event {
                    AdapterEvent::Discovering { discovering: _ } => {
                        let output = json!({
                            "event": "discovering",
                            "adapter_info": handle_adapter_info(&adapter_info),
                        });
                    
                        println!("{}", output.to_string());
                    }
                    AdapterEvent::Powered { powered: _ } => {
                        let output = json!({
                            "event": "powered",
                            "adapter_info": handle_adapter_info(&adapter_info),
                        });

                        println!("{}", output.to_string());
                    }
                    _ => {}
                }
            }
            BluetoothEvent::Device { id, event } => {
                let device_info = session.get_device_info(&id).await?;
                match event {
                    DeviceEvent::Connected { connected } => {
                        let output = json!({
                            "event": if connected { "connected" } else { "disconnected" },
                            "device_info": handle_device_info(&device_info),
                        });

                        println!("{}", output.to_string());
                    }
                    _ => {}
                }
            }
            _=> {}
        }
    }

    Ok(())
}
