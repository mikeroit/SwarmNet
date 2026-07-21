fn main() {
    prost_build::compile_protos(
        &[
            "../../proto/swarmnet/v1/common.proto",
            "../../proto/swarmnet/v1/drone.proto",
            "../../proto/swarmnet/v1/mission.proto",
            "../../proto/swarmnet/v1/hazard.proto",
            "../../proto/swarmnet/v1/route.proto",
            "../../proto/swarmnet/v1/telemetry.proto",
            "../../proto/swarmnet/v1/flight_plan.proto",
        ],
        &["../../proto"],
    )
    .expect("Failed to compile protobuf files");
}
