use crate::event::{self, checker};
use crate::grpc::sender;
use dust_dds::{
    dds_async::domain_participant_factory::DomainParticipantFactoryAsync,
    infrastructure::{qos::QosKind, status::NO_STATUS},
    subscription::sample_info::{ANY_INSTANCE_STATE, ANY_SAMPLE_STATE, ANY_VIEW_STATE},
};

#[allow(non_snake_case)]
pub mod gearState {
    #[derive(Debug, dust_dds::topic_definition::type_support::DdsType)]
    pub struct DataType {
        pub gear: String,
    }
}

pub struct DdsEventListener {
    name: String,
}

impl DdsEventListener {
    pub fn new(name: String) -> Self {
        DdsEventListener { name }
    }
}

impl Drop for DdsEventListener {
    fn drop(&mut self) {
        println!("drop DdsEventListener {}\n", self.name);
    }
}

impl super::EventListener for DdsEventListener {
    async fn run(&self) {
        let domain_id = 0;
        let participant_factory = DomainParticipantFactoryAsync::new();

        let participant = participant_factory
            .create_participant(domain_id, QosKind::Default, None, NO_STATUS)
            .await
            .unwrap();

        let topic = participant
            .create_topic::<gearState::DataType>(
                "rt/piccolo/gear_state",
                "gearState::DataType",
                QosKind::Default,
                None,
                NO_STATUS,
            )
            .await
            .unwrap();

        let subscriber = participant
            .create_subscriber(QosKind::Default, None, NO_STATUS)
            .await
            .unwrap();

        let reader = subscriber
            .create_datareader::<gearState::DataType>(&topic, QosKind::Default, None, NO_STATUS)
            .await
            .unwrap();

        loop {
            if let Ok(data_samples) = reader
                .take(10, ANY_SAMPLE_STATE, ANY_VIEW_STATE, ANY_INSTANCE_STATE)
                .await
            {
                let data = data_samples[0].data().unwrap();
                println!("Received: {:?}\n", data);

                if checker::check(&self.name, &data.gear) {
                    notify_to_destination(&self.name).await;
                    break;
                }
            }
            std::thread::sleep(std::time::Duration::from_millis(100));
        }
    }
}

async fn notify_to_destination(key: &str) {
    let e = event::remove(key).unwrap();
    let action_key = e.action_key;

    println!(
        "target destination : {}, target action: {}",
        e.target_dest, action_key
    );

    let _ = sender::to_statemanager(&action_key).await;
}
