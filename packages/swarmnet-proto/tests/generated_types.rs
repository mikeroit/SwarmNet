use swarmnet_proto::swarmnet::v1::{
    DroneId, GeoPoint, MissionId, Route, Waypoint,
};

#[test]
fn can_construct_generated_types() {
    let drone_id = DroneId {
        value: "drone-001".to_string(),
    };

    let mission_id = MissionId {
        value: "mission-001".to_string(),
    };

    let waypoint = Waypoint {
        id: "wp-001".to_string(),
        position: Some(GeoPoint {
            latitude: 39.7392,
            longitude: -104.9903,
            altitude_meters: 120.0,
        }),
        target_speed_mps: 12.5,
        loiter_seconds: 0.0,
    };

    let route = Route {
        id: "route-001".to_string(),
        waypoints: vec![waypoint],
        estimated_distance_m: 1000.0,
        estimated_duration_s: 80.0,
    };

    assert_eq!(drone_id.value, "drone-001");
    assert_eq!(mission_id.value, "mission-001");
    assert_eq!(route.waypoints.len(), 1);
}
