use crate::models::common::{Distance, Duration, Location, Profile, Timestamp};
use crate::models::problem::{ActivityCost, TransportCost};

pub struct TestTransportCost {}

impl TransportCost for TestTransportCost {
    fn duration(&self, _profile: Profile, from: Location, to: Location, _departure: Timestamp) -> Duration {
        fake_routing(from, to)
    }

    fn distance(&self, _profile: Profile, from: Location, to: Location, _departure: Timestamp) -> Distance {
        fake_routing(from, to)
    }
}

impl TestTransportCost {
    pub fn new() -> Self {
        Self {}
    }
}

pub fn fake_routing(from: Location, to: Location) -> f64 {
    (if to > from { to - from } else { from - to }) as f64
}

pub struct TestActivityCost {}

impl ActivityCost for TestActivityCost {}

impl TestActivityCost {
    pub fn new() -> Self {
        Self {}
    }
}