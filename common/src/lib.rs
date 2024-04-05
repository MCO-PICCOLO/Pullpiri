pub mod apiserver {
    pub use api::proto::apiserver::*;
    pub const API_SERVER_OPEN: &str = "[::1]:50101";
    pub const API_SERVER_CONNECT: &str = "http://[::1]:50101";

    /** Followings are defined in api::proto::apiserver module.
    pub enum UpdateMethod {
        START = 0,
        STOP = 1,
        RESTART = 2,
        RELOAD = 3,
        ENABLE = 4,
        DISABLE = 5,
    }
    pub enum ControllerCommand {
        ListNode = 0,
        DaemonReload = 1,
    }
    pub enum NodeCommand {
        ListUnit = 0,
    }
    **/

    pub fn get_controller_command(
        cmd: api::proto::apiserver::ControllerCommand,
    ) -> api::proto::apiserver::ToServer {
        api::proto::apiserver::ToServer {
            to_server_content: Some(api::proto::apiserver::to_server::ToServerContent::Request(
                api::proto::apiserver::Request {
                    request_content: Some(
                        api::proto::apiserver::request::RequestContent::ControllerRequest(
                            api::proto::apiserver::ControllerRequest {
                                controller_command: cmd.into(),
                            },
                        ),
                    ),
                },
            )),
        }
    }

    pub fn get_node_command(
        cmd: api::proto::apiserver::NodeCommand,
        node_name: &str,
    ) -> api::proto::apiserver::ToServer {
        api::proto::apiserver::ToServer {
            to_server_content: Some(api::proto::apiserver::to_server::ToServerContent::Request(
                api::proto::apiserver::Request {
                    request_content: Some(
                        api::proto::apiserver::request::RequestContent::NodeRequest(
                            api::proto::apiserver::NodeRequest {
                                node_command: cmd.into(),
                                node_name: node_name.to_owned(),
                            },
                        ),
                    ),
                },
            )),
        }
    }

    pub fn get_unit_command(
        cmd: api::proto::apiserver::UpdateMethod,
        node_name: &str,
        unit_name: &str,
    ) -> api::proto::apiserver::ToServer {
        api::proto::apiserver::ToServer {
            to_server_content: Some(
                api::proto::apiserver::to_server::ToServerContent::UpdateWorkload(
                    api::proto::apiserver::UpdateWorkload {
                        update_method: cmd.into(),
                        node_name: node_name.to_owned(),
                        unit_name: unit_name.to_owned(),
                    },
                ),
            ),
        }
    }
}

pub mod statemanager {
    pub use api::proto::statemanager::*;
    pub const STATE_MANAGER_OPEN: &str = "[::1]:50010";
    pub const STATE_MANAGER_CONNECT: &str = "http://[::1]:50010";
}

pub mod etcd {
    pub const ETCD_ENDPOINT: &str = "127.0.0.1:2379";

    use etcd_client::{Client, Error};

    async fn get_client() -> Result<Client, Error> {
        Client::connect([ETCD_ENDPOINT], None).await
    }

    pub async fn put(key: &str, value: &str) -> Result<(), Error> {
        let mut client = get_client().await?;
        client.put(key, value, None).await?;
        Ok(())
    }

    pub async fn get(key: &str) -> Result<(), Error> {
        let mut client = get_client().await?;
        client.get(key, None).await?;
        Ok(())
    }

    pub async fn delete(key: &str) -> Result<(), Error> {
        let mut client = get_client().await?;
        client.delete(key, None).await?;
        Ok(())
    }
}
