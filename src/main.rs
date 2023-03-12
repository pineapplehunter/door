use std::{
    sync::{Arc, Mutex},
    time::Duration,
};

use axum::{
    routing::{get, post},
    Router,
};
use std::thread::sleep;
use tokio::sync::mpsc::{channel, Receiver};
use tracing::info;

use crate::{
    doorlock::{DoorLock, DoorLockTrait},
    handlers::ServerState,
    key_management::load_keys,
};

mod doorlock;

#[derive(Debug)]
pub struct Open;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();

    let keys = load_keys().await?;
    info!("keys={keys:?}");

    let (tx, rx) = channel::<Open>(10);

    tokio::spawn(hardware_thread(rx));

    let app = Router::new()
        .route("/", get(handlers::index))
        .route("/", post(handlers::check_key))
        .with_state(ServerState {
            keys,
            open_sender: tx,
        });

    let addr = std::env::var("DOOR_PORT")
        .unwrap_or("0.0.0.0:8000".to_string())
        .parse()?;
    info!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
    Ok(())
}

async fn hardware_thread(rx: Receiver<Open>) {
    use tokio::task::spawn_blocking;
    info!("starting hardware thread");
    let mut rx = rx;
    let door_lock = Arc::new(Mutex::new(DoorLock::new()));
    while rx.recv().await.is_some() {
        let door_lock = door_lock.clone();
        spawn_blocking(move || {
            let mut door_lock = door_lock.lock().unwrap();
            door_lock.open();
            sleep(Duration::from_secs(10));
            loop {
                while door_lock.is_open() {
                    sleep(Duration::from_secs(1));
                }
                sleep(Duration::from_secs(3));
                if !door_lock.is_open() {
                    break;
                }
            }
            door_lock.close();
        })
        .await
        .unwrap();
    }
}

mod handlers {
    use std::collections::HashMap;

    use axum::{
        extract::State,
        response::{Html, IntoResponse},
        Form,
    };
    use serde::Deserialize;
    use tokio::sync::mpsc::Sender;

    use crate::{key_management::key_is_valid, Open};

    pub async fn index() -> impl IntoResponse {
        render_page(None)
    }

    #[derive(Debug, Deserialize)]
    pub struct PinCode {
        number: String,
    }

    #[derive(Clone, Debug)]
    pub struct ServerState {
        pub keys: Vec<String>,
        pub open_sender: Sender<Open>,
    }

    pub async fn check_key(
        State(ServerState { keys, open_sender }): State<ServerState>,
        Form(code): Form<PinCode>,
    ) -> impl IntoResponse {
        if key_is_valid(&code.number, &keys) {
            open_sender.send(Open).await.unwrap();
            render_page(Some("OK".to_string()))
        } else {
            render_page(Some("パスコードが間違っています".to_string()))
        }
    }

    fn render_page(message: Option<String>) -> Html<String> {
        static TEMPLATE: &str = include_str!("index.hbs");
        let hb = handlebars::Handlebars::new();
        let mut data = HashMap::new();
        if let Some(m) = message {
            data.insert("message", m);
        }
        Html::from(hb.render_template(TEMPLATE, &data).unwrap())
    }
}

mod key_management {
    use libreauth::oath::TOTPBuilder;
    use tokio::fs::read_to_string;

    pub async fn load_keys() -> Result<Vec<String>, std::io::Error> {
        let data = read_to_string("keys.txt").await?;
        Ok(data
            .split('\n')
            .filter(|x| x.len() == 32)
            .map(|x| x.to_owned())
            .collect())
    }

    pub fn key_is_valid(number: &str, keys: &[String]) -> bool {
        if number.len() != 6 {
            return false;
        }
        keys.iter().any(|key| {
            TOTPBuilder::new()
                .base32_key(key)
                .finalize()
                .unwrap()
                .is_valid(number)
        })
    }
}
