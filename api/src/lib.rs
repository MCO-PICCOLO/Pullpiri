pub mod proto {
    pub mod apiserver {
        tonic::include_proto!("apiserver");
    }
    pub mod statemanager {
        tonic::include_proto!("statemanager");
    }
}
