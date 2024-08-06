use dust_dds::{
    dds_async::domain_participant_factory::DomainParticipantFactoryAsync,
    infrastructure::{qos::QosKind, status::NO_STATUS},
    subscription::sample_info::{ANY_INSTANCE_STATE, ANY_SAMPLE_STATE, ANY_VIEW_STATE},
};
use tokio::sync::mpsc::Sender;

use crate::listener::DdsData;

// TOPIC NAME = /rt/piccolo/Gear_State
#[allow(non_snake_case)]
pub mod gearState {
    #[derive(Debug, dust_dds::topic_definition::type_support::DdsType)]
    pub struct DataType {
        pub state: String,
    }
}

// TOPIC NAME = /rt/piccolo/Day_Time
#[allow(non_snake_case)]
pub mod dayTime {
    #[derive(Debug, dust_dds::topic_definition::type_support::DdsType)]
    pub struct DataType {
        pub day: bool,
    }
}

/*
pub mod light {
    #[derive(Debug, dust_dds::topic_definition::type_support::DdsType)]
    pub struct DataType {
        pub light_on: String,
    }
}
*/

#[derive(Debug, Clone)]
pub struct DdsListener {
    name: String,
    tx: Sender<DdsData>,
}

impl Drop for DdsListener {
    fn drop(&mut self) {
        println!("drop DdsListener {}\n", self.name);
    }
}

impl DdsListener {
    pub fn new(name: &str, tx: Sender<DdsData>) -> Self {
        DdsListener {
            name: name.to_string(),
            tx,
        }
    }

    pub async fn run(&self) {
        let domain_id = 0;
        let participant_factory = DomainParticipantFactoryAsync::new();

        let participant = participant_factory
            .create_participant(domain_id, QosKind::Default, None, NO_STATUS)
            .await
            .unwrap();
        let subscriber = participant
            .create_subscriber(QosKind::Default, None, NO_STATUS)
            .await
            .unwrap();

        match self.name.as_str() {
            "gear" => {
                let topic = participant
                    .create_topic::<gearState::DataType>(
                        "/rt/piccolo/Gear_State",
                        "gearState::DataType",
                        QosKind::Default,
                        None,
                        NO_STATUS,
                    )
                    .await
                    .unwrap();
                let reader = subscriber
                    .create_datareader::<gearState::DataType>(
                        &topic,
                        QosKind::Default,
                        None,
                        NO_STATUS,
                    )
                    .await
                    .unwrap();

                loop {
                    if let Ok(data_samples) = reader
                        .take(10, ANY_SAMPLE_STATE, ANY_VIEW_STATE, ANY_INSTANCE_STATE)
                        .await
                    {
                        let data: gearState::DataType = data_samples[0].data().unwrap();
                        println!("Received: {:?}\n", data);

                        let msg = DdsData {
                            name: self.name.clone(),
                            value: data.state,
                        };
                        let _ = self.tx.send(msg).await;
                    }
                    std::thread::sleep(std::time::Duration::from_millis(100));
                }
            }
            "day" => {
                let topic = participant
                    .create_topic::<dayTime::DataType>(
                        "/rt/piccolo/Day_Time",
                        "day::DataType",
                        QosKind::Default,
                        None,
                        NO_STATUS,
                    )
                    .await
                    .unwrap();
                let reader = subscriber
                    .create_datareader::<dayTime::DataType>(&topic, QosKind::Default, None, NO_STATUS)
                    .await
                    .unwrap();

                loop {
                    if let Ok(data_samples) = reader
                        .take(10, ANY_SAMPLE_STATE, ANY_VIEW_STATE, ANY_INSTANCE_STATE)
                        .await
                    {
                        let data: dayTime::DataType = data_samples[0].data().unwrap();
                        println!("Received: {:?}\n", data);

                        let msg = DdsData {
                            name: self.name.clone(),
                            value: match data.day {
                                true => "day".to_string(),
                                false => "night".to_string(),
                            },
                        };
                        let _ = self.tx.send(msg).await;
                    }
                    std::thread::sleep(std::time::Duration::from_millis(100));
                }
            }
            _ => panic!("topic name is wrong"),
        };
    }
}