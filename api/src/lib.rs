pub mod proto {
    pub mod piccolo {
        tonic::include_proto!("piccolo");
    }
    pub mod filter {
        tonic::include_proto!("filter");
    }
}
