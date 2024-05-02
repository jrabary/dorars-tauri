// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use dora_node_api::{dora_core::config::DataId, DoraNode, Event, IntoArrow, MetadataParameters};
use tokio::sync::mpsc;
use tokio::sync::Mutex;

struct AsyncProcInputTx {
    tx: Mutex<mpsc::Sender<CmdData>>,
}

#[derive(Debug)]
enum CmdData {
    Velocity { vx: f64, vy: f64 },
    Direction { theta: f64 },
}

#[tauri::command]
async fn message_handler(
    vx: f64,
    vy: f64,
    state: tauri::State<'_, AsyncProcInputTx>,
) -> Result<(), String> {
    let tx = state.tx.lock().await;

    let cmd = CmdData::Velocity { vx, vy };
    println!(
        "Receive cmd from the fronend will forward it to dora {:?}",
        cmd
    );

    tx.send(cmd).await.map_err(|e| e.to_string())
    // tx.send(message.to_string())
    //     .await
    //     .map_err(|e| e.to_string())
}

async fn dora_manager(
    mut rx: mpsc::Receiver<CmdData>,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let (mut node, mut events) = DoraNode::init_from_env().unwrap();

    while let Some(cmd) = rx.recv().await {
        match cmd {
            CmdData::Velocity { vx, vy } => {
                let msg = format!("Velocity: vx = {}, vy = {}", vx, vy);
                let output_id = DataId::from("velocity".to_owned());
                println!("Sending to dora velocity: {:?}", msg);
                node.send_output(output_id, MetadataParameters::default(), msg.into_arrow())
                    .unwrap();
            }
            CmdData::Direction { theta } => {
                let msg = format!("Direction: theta = {}", theta);
                let output_id = DataId::from("direction".to_owned());
                println!("Sending to dora direction: {:?}", msg);
                node.send_output(output_id, MetadataParameters::default(), msg.into_arrow())
                    .unwrap();
            }
        }
    }
    Ok(())
}

fn main() {
    let (tx, rx) = mpsc::channel(32);

    tauri::Builder::default()
        .manage(AsyncProcInputTx { tx: Mutex::new(tx) })
        .invoke_handler(tauri::generate_handler![message_handler])
        .setup(|app| {
            tauri::async_runtime::spawn(async move { dora_manager(rx).await });
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
