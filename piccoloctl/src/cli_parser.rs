use clap::{Args, Parser, Subcommand};

#[derive(Parser, Debug)]
pub struct Arguments {
    #[clap(subcommand)]
    /// command name.
    pub command: Command,
}

#[derive(Subcommand, Debug)]
pub enum Command {
    /// Returns information of all known nodes
    ListNode,
    /// Reload all unit files
    DaemonReload,
    /// Returns all loaded units on node
    ListUnit(Node),
    /// Start named unit
    Start(Unit),
    /// Stop named unit
    Stop(Unit),
    /// Restart named unit
    Restart(Unit),
    /// Reload named unit
    Reload(Unit),
    /// Enable one unit file
    Enable(Unit),
    /// Disable one unit file
    Disable(Unit),
}

#[derive(Args, Debug)]
pub struct Node {
    /// node name
    pub node_name: String,
}

#[derive(Args, Debug)]
pub struct Unit {
    /// node name
    pub node_name: String,
    /// unit name
    pub unit_name: String,
}
