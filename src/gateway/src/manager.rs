use crate::event;
use crate::event::{parser, Event};
use crate::listener::EventListener;
use common::gateway::piccolo_gateway_service_server::PiccoloGatewayServiceServer;
use common::gateway::EventName;
use tokio::sync::mpsc;
use tonic::transport::Server;

pub struct GatewayManager {
    grpc_rx: mpsc::Receiver<EventName>,
    grpc_tx: mpsc::Sender<EventName>,
}

impl GatewayManager {
    pub fn new() -> Self {
        let (grpc_tx, grpc_rx) = mpsc::channel(100);
        GatewayManager { grpc_rx, grpc_tx }
    }

    pub async fn run(&mut self) {
        tokio::spawn(grpc_server(self.grpc_tx.clone()));

        while let Some(req) = self.grpc_rx.recv().await {
            let name = req.name.clone();
            if event::get(&name).is_some() {
                println!("Already exist scenario.\n");
                continue;
            }
            let event: Event = parser::parse(&name, req.target).await;

            println!("{:#?}\n", req);
            println!("{:#?}\n", event);

            event::insert(&name, event);
            tokio::spawn(launch_dds(name));
        }
    }
}

async fn grpc_server(tx: mpsc::Sender<EventName>) {
    let server = crate::grpc::receiver::GrpcServer {
        grpc_msg_tx: tx.clone(),
    };
    let addr = common::gateway::open_server()
        .parse()
        .expect("gateway address parsing error");

    println!("Piccolod gateway listening on {}", addr);

    let _ = Server::builder()
        .add_service(PiccoloGatewayServiceServer::new(server))
        .serve(addr)
        .await;
}

async fn launch_dds(name: String) {
    let l = crate::listener::dds::DdsEventListener::new(name);
    l.run().await;
}
