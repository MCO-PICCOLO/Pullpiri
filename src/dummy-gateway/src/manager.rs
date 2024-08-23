use tokio::sync::mpsc::{channel, Receiver, Sender};

use crate::filter::Filter;
use crate::listener::DdsData;
use crate::scenario::ResourceScenario;
use std::sync::Arc;
use tokio::sync::Mutex;

pub struct Manager {
    gear: Arc<Mutex<String>>,
    day: Arc<Mutex<String>>,
    rx_rest: Arc<Mutex<Receiver<ResourceScenario>>>,
    tx_dds: Sender<DdsData>,
    rx_dds: Arc<Mutex<Receiver<DdsData>>>,
    filters: Arc<Mutex<Vec<Filter>>>,
}

impl Manager {
    pub fn new(rx_rest: Receiver<ResourceScenario>) -> Self {
        let (tx_dds, rx_dds) = channel::<DdsData>(10);
        Manager {
            gear: Arc::new(Mutex::new(String::new())),
            day: Arc::new(Mutex::new(String::new())),
            rx_rest: Arc::new(Mutex::new(rx_rest)),
            tx_dds,
            rx_dds: Arc::new(Mutex::new(rx_dds)),
            filters: Arc::new(Mutex::new(Vec::new())),
        }
    }

    pub async fn run(&mut self) {
        tokio::spawn(launch_dds("gear", self.tx_dds.clone()));
        tokio::spawn(launch_dds("day", self.tx_dds.clone()));
        tokio::spawn(launch_dds("light", self.tx_dds.clone()));

        let arc_rx_rest = Arc::clone(&self.rx_rest);
        let arc_rx_dds = Arc::clone(&self.rx_dds);
        let arc_filters = Arc::clone(&self.filters);
        let arc_gear = Arc::clone(&self.gear);
        let arc_day = Arc::clone(&self.day);

        tokio::spawn(async move {
            let mut rx_dds = arc_rx_dds.lock().await;
            while let Some(data) = rx_dds.recv().await {
                if data.name == "gear" {
                    let mut gear = arc_gear.lock().await;
                    gear.clone_from(&data.value);
                } else if data.name == "day" {
                    let mut day = arc_day.lock().await;
                    day.clone_from(&data.value);
                }

                let mut filters = arc_filters.lock().await;
                if filters.is_empty() {
                    continue;
                }

                match data.name.as_str() {
                    "gear" => filters[0].set_status(0, &data.value).await,
                    "day" => filters[0].set_status(1, &data.value).await,
                    "light" => filters[0].receive_light(&data.value).await,
                    _ => continue,
                }
            }
        });

        let mut rx_rest = arc_rx_rest.lock().await;
        while let Some(scenario) = rx_rest.recv().await {
            // TODO parsing scenario
            // TODO get condition and DDS criteria
            if Some(false) == scenario.route {
                println!("#####\nscenario is deleted\n#####\n");
                let _ = common::etcd::delete(&format!("scenario/{}", scenario.name.clone())).await;
                self.remove_filter(0).await;
            } else if Some(true) == scenario.route {
                println!("{:#?}", scenario);
                let _ =
                    common::etcd::put(&format!("scenario/{}", scenario.name.clone()), "INACTIVE")
                        .await;
                self.launch_filter(&scenario).await;
            }
        }
    }

    async fn launch_filter(&mut self, scenario: &ResourceScenario) {
        println!("launch filter {}\n", &scenario.name);

        let mut gear_target = "";
        let mut day_target = "";

        let criteria = &scenario.condition.criteria;
        for criterion in criteria {
            if criterion.message.contains("Gear") {
                gear_target = &criterion.value;
                println!("gear target : {gear_target}");
            } else if criterion.message.contains("Day") {
                day_target = &criterion.value;
                println!("day target : {day_target}");
            }
        }

        let gear_current = self.gear.lock().await;
        let day_current = self.day.lock().await;

        let f = Filter::new(
            &scenario.name,
            gear_target,
            day_target,
            &gear_current,
            &day_current,
        )
        .await;
        let mut filters = self.filters.lock().await;
        filters.push(f);
    }

    async fn remove_filter(&mut self, index: usize) -> Option<Filter> {
        let mut filters = self.filters.lock().await;
        if index < filters.len() {
            Some(filters.remove(index))
        } else {
            None
        }
    }
}

async fn launch_dds(name: &str, tx_dds: Sender<DdsData>) {
    let l = crate::listener::dds::DdsListener::new(name, tx_dds);
    l.run().await;
}