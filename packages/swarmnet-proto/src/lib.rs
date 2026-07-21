pub mod swarmnet {
    pub mod v1 {
        include!(concat!(env!("OUT_DIR"), "/swarmnet.v1.rs"));
    }
}
