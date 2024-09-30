pub mod dds;

pub trait EventListener {
    async fn run(&self);
}
