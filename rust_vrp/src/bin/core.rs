use std::sync::Arc;
use vrp_core::models::common::*;
use vrp_core::models::*;

struct DefineProblem {
    vehicles: problem::Fleet,
    jobs: problem::Jobs,
}

struct RoutingMatrix {
    distance: Vec<Vec<f64>>,
    duration: Vec<Vec<f64>>,
}

impl RoutingMatrix {
    fn new() -> RoutingMatrix {
        RoutingMatrix {
            distance: vec![Vec::new()],
            duration: vec![Vec::new()],
        }
    }
}

impl problem::TransportCost for RoutingMatrix {
    /// Returns time-dependent travel duration between locations specific for given actor.
    fn duration(
        &self,
        route: &solution::Route,
        from: Location,
        to: Location,
        departure: problem::TravelTime,
    ) -> Duration {
        0.0
    }

    /// Returns time-dependent travel distance between locations specific for given actor.
    fn distance(
        &self,
        route: &solution::Route,
        from: Location,
        to: Location,
        departure: problem::TravelTime,
    ) -> Duration {
        0.0
    }

    /// Returns time-independent travel duration between locations specific for given profile.
    fn duration_approx(&self, profile: &Profile, from: Location, to: Location) -> Duration {
        0.0
    }

    /// Returns time-independent travel distance between locations specific for given profile.
    fn distance_approx(&self, profile: &Profile, from: Location, to: Location) -> Distance {
        0.0
    }
}

impl DefineProblem {
    pub fn new(vehicles: problem::Fleet, jobs: problem::Jobs) -> DefineProblem {
        DefineProblem { vehicles, jobs }
    }

    fn create_driver(
        fixed: f64,
        per_distance: f64,
        per_driving_time: f64,
        per_waiting_time: f64,
        per_service_time: f64,
    ) -> Arc<problem::Driver> {
        Arc::new(problem::Driver {
            costs: problem::Costs {
                fixed,
                per_distance,
                per_driving_time,
                per_waiting_time,
                per_service_time,
            },
            dimens: Default::default(),
            details: vec![problem::DriverDetail {}],
        })
    }

    fn create_vehicle(
        depot: usize,
        fixed: f64,
        per_distance: f64,
        per_driving_time: f64,
        per_waiting_time: f64,
        per_service_time: f64,
    ) -> Arc<problem::Vehicle> {
        Arc::new(problem::Vehicle {
            profile: Profile::new(0, Some(1.0)),
            costs: problem::Costs {
                fixed,
                per_distance,
                per_driving_time,
                per_waiting_time,
                per_service_time,
            },
            dimens: Default::default(),
            details: vec![problem::VehicleDetail {
                start: Some(problem::VehiclePlace {
                    location: depot,
                    time: TimeInterval {
                        earliest: None,
                        latest: None,
                    },
                }),
                end: Some(problem::VehiclePlace {
                    location: depot,
                    time: TimeInterval {
                        earliest: None,
                        latest: None,
                    },
                }),
            }],
        })
    }

    fn create_fleet(depot: usize, num_drivers: u32, num_vehicles: u32) -> problem::Fleet {
        let mut drivers: Vec<Arc<problem::Driver>> = Vec::new();
        for _ in 0..num_drivers {
            drivers.push(DefineProblem::create_driver(25.0, 0.0002, 0.005, 0.0, 0.0));
        }
        let mut vehicles: Vec<Arc<problem::Vehicle>> = Vec::new();
        for _ in 0..num_vehicles {
            vehicles.push(DefineProblem::create_vehicle(
                depot, 25.0, 0.0002, 0.005, 0.0, 0.0,
            ));
        }
        problem::Fleet::new(drivers, vehicles, Box::new(|_| Box::new(|_| 0)))
    }

    fn create_job(location: usize, duration: f64) -> problem::Job {
        problem::Job::Single(Arc::new(problem::Single {
            places: vec![problem::Place {
                location: Some(location),
                duration: duration,
                times: vec![TimeSpan::Window(TimeWindow::max())],
            }],
            dimens: Default::default(),
        }))
    }

    fn create_jobs(locations: Vec<usize>, vehicles: &problem::Fleet) -> problem::Jobs {
        let mut new_jobs = Vec::new();
        for l in locations {
            new_jobs.push(DefineProblem::create_job(l, 0.0));
        }
        let transport: Arc<dyn problem::TransportCost + Sync + Send> =
            Arc::new(RoutingMatrix::new());
        problem::Jobs::new(vehicles, new_jobs, &transport)
    }
}

fn main() {
    let fleet = DefineProblem::create_fleet(1, 1, 3);
    let jobs = DefineProblem::create_jobs(vec![0, 1, 2], &fleet);
    let p = DefineProblem::new(fleet, jobs);
}
