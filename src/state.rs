use std::{
    sync::OnceLock,
    time::{Duration, SystemTime},
};

use axum::response::sse::Event;
use dashmap::DashMap;
use tokio::sync::broadcast;

#[derive(Clone)]
pub enum Connectivity {
    Ready,
    Disconnect,
    Error,
    Disabled,
}

impl From<Connectivity> for String {
    fn from(value: Connectivity) -> Self {
        match value {
            Connectivity::Ready => "ready",
            Connectivity::Disconnect => "disconnected",
            Connectivity::Error => "error",
            Connectivity::Disabled => "disabled",
        }
        .to_string()
    }
}

impl From<&Connectivity> for String {
    fn from(value: &Connectivity) -> Self {
        match value {
            Connectivity::Ready => "ready",
            Connectivity::Disconnect => "disconnect",
            Connectivity::Error => "error",
            Connectivity::Disabled => "disabled",
        }
        .to_string()
    }
}

pub struct VenueDisplayInfo {
    pub venue_id: String,
    pub initial_time: String,
    pub update_time: String,
    pub connected_status: String,
}

pub struct PerConnection {
    venue_id: String,
    connectivities: Vec<Connectivity>,
    event_interval: Option<Duration>,
    initial_connection: SystemTime,
    event_updater: broadcast::Sender<Event>,
}
use rand::Rng;
impl PerConnection {
    pub fn new(
        venue_id: String,
        connectivities: Vec<Connectivity>,
        event_interval: Option<Duration>,
        event_updater: broadcast::Sender<Event>,
    ) -> Self {
        Self {
            venue_id,
            connectivities: if connectivities.is_empty() {
                vec![Connectivity::Ready]
            } else {
                connectivities
            },
            event_interval,
            initial_connection: SystemTime::now(),
            event_updater,
        }
    }

    pub async fn run(&mut self) {
        let event_interval = self.event_interval.take();
        let initial_established_connection_time = self.initial_connection;
        let venue_id = self.venue_id.clone();
        let event_updater = self.event_updater.clone();

        let initial_connectivity = self
            .connectivities
            .first()
            .cloned()
            .unwrap_or(Connectivity::Ready);

        utils::propagate(
            self.venue_id.clone(),
            &initial_connectivity,
            "new-venue",
            &self.initial_connection,
            None,
            &event_updater,
        );

        tokio::spawn(async move {
            if let Some(interval) = event_interval {
                let mut interval = tokio::time::interval(interval);
                let mut current_connection: Option<Connectivity> = None;
                loop {
                    interval.tick().await;
                    get_state().connections.entry(venue_id.clone()).and_modify(
                        |venue_monitor_instance| {
                            let num = rand::thread_rng()
                                .gen_range(0..venue_monitor_instance.connectivities.len());
                            current_connection =
                                venue_monitor_instance.connectivities.get(num).cloned();
                        },
                    );
                    if let Some(next_connectivity) = &current_connection {
                        utils::propagate(
                            venue_id.clone(),
                            next_connectivity,
                            &format!("venue-{}", venue_id),
                            &initial_established_connection_time,
                            Some(SystemTime::now()),
                            &event_updater,
                        );
                    }
                }
            }
        });
    }
}

pub struct InternalState {
    pub tx: broadcast::Sender<Event>,
    pub connections: DashMap<String, PerConnection>,
}

pub fn get_state() -> &'static InternalState {
    static INSTANCE: OnceLock<InternalState> = OnceLock::new();
    INSTANCE.get_or_init(|| {
        let (tx, _) = broadcast::channel(1000);
        InternalState {
            tx,
            connections: DashMap::new(),
        }
    })
}

pub fn seed_venue_with_default_connectivity(
    venue_name: String,
    update_time: usize,
) -> PerConnection {
    PerConnection::new(
        venue_name,
        vec![
            Connectivity::Ready,
            Connectivity::Disconnect,
            Connectivity::Error,
        ],
        Some(Duration::from_secs(update_time as u64)),
        get_state().tx.clone(),
    )
}

mod utils {
    use std::time::SystemTime;

    use axum::response::sse::Event;
    use chrono::{DateTime, Local};
    use tokio::sync::broadcast;

    use crate::web_service::views::components::venue_info_block;

    use super::{Connectivity, VenueDisplayInfo};

    pub fn to_time_format(time: &SystemTime) -> String {
        let date_time: DateTime<Local> = (*time).into();
        date_time.format("%-m/%-d/%Y, %-I:%M:%S %p").to_string()
    }

    pub fn propagate(
        venue_id: String,
        current_status: &Connectivity,
        event: &str,
        initial: &SystemTime,
        updated_time: Option<SystemTime>,
        updater: &broadcast::Sender<Event>,
    ) {
        let prepared_data = VenueDisplayInfo {
            venue_id: venue_id.clone(),
            initial_time: to_time_format(initial),
            update_time: if let Some(time) = updated_time {
                to_time_format(&time)
            } else {
                "Not Updated Yet".into()
            },
            connected_status: current_status.clone().into(),
        };

        let _res = updater.send(Event::default().event(event).data(venue_info_block::build(
            prepared_data,
            &format!("venue-{}", venue_id),
        )));
    }
}

// pub fn
