macro_rules! define_id {
    ($name:ident) => {
        #[derive(Debug, Clone, PartialEq, Eq, Hash)]
        pub struct $name(String);

        impl $name {
            pub fn new(id: impl Into<String>) -> Self {
                Self(id.into())
            }

            pub fn as_str(&self) -> &str {
                &self.0
            }
        }

        impl From<&str> for $name {
            fn from(value: &str) -> Self {
                Self(value.to_string())
            }
        }

        impl From<String> for $name {
            fn from(value: String) -> Self {
                Self(value)
            }
        }

        impl std::fmt::Display for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                self.0.fmt(f)
            }
        }
    };
}

define_id!(DroneId);
define_id!(FlightPlanId);
define_id!(HazardId);
define_id!(MissionId);
define_id!(RouteId);
define_id!(WaypointId);

impl DroneId {
    pub fn display_name(&self) -> String {
        let splits: Vec<&str> = self.0.as_str().split("-").collect();
        if splits.len() == 2 {
            let numerical = splits[1];
            return format!("Drone {}", numerical);
        }

        self.0.clone()
    }
}

impl RouteId {
    pub fn display_name(&self) -> String {
        let splits: Vec<&str> = self.0.as_str().split("-").collect();
        if splits.len() == 2 {
            let numerical = splits[1];
            return format!("Route {}", numerical);
        }

        self.0.clone()
    }
}

impl HazardId {
    pub fn display_name(&self) -> String {
        let splits: Vec<&str> = self.0.as_str().split("-").collect();
        if splits.len() == 2 {
            let numerical = splits[1];
            return format!("Hazard {}", numerical);
        }

        self.0.clone()
    }
}

impl WaypointId {
    pub fn display_name(&self) -> String {
        let splits: Vec<&str> = self.0.as_str().split("-").collect();
        if splits.len() == 2 {
            let numerical = splits[1];
            return format!("Waypoint {}", numerical);
        }

        self.0.clone()
    }
}
