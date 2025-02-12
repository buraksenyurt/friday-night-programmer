use chrono::{DateTime, Utc};

#[derive(Debug)]
enum Service {
    Offline {
        name: String,
    },
    Online {
        name: String,
        address: String,
        active: bool,
        start_time: DateTime<Utc>,
    },
}

impl Service {
    fn run(&self, address: String, start_time: DateTime<Utc>) -> Result<Self> {
        match self {
            Service::Offline { name } => {
                let created = Service::Online {
                    name: name.clone(),
                    address,
                    active: true,
                    start_time,
                };
                Ok(created)
            }
            Service::Online { .. } => Err(AlreadyOnlineError),
        }
    }
}

#[derive(Debug, Clone)]
struct AlreadyOnlineError;

type Result<T> = std::result::Result<T, AlreadyOnlineError>;

fn main() {
    let redis = Service::Offline {
        name: "Redis".to_string(),
    };
    println!("{:#?}", redis);

    if let Ok(m) = redis.run("https:://127.0.0.1:5326".to_string(), Utc::now()) {
        println!("Redis service is online");
        println!("{:#?}", m);
    }
}
