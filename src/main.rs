use std::time::SystemTime;
use std::sync::mpsc;
use std::thread;
use std::time::Duration;
use rand::Rng;
use chrono::{DateTime, Local, Utc};


fn main() {
    let (tx, rx) = mpsc::channel();
    let sensor = SensorNode{node_id: 1, sender: tx};
    let sensor1 = SensorNode{node_id: 2, sender: sensor.sender.clone()};
    let sensor2 = SensorNode{node_id: 3, sender: sensor.sender.clone()};
    let command = CommandNode{receiver: rx};
    
    thread::spawn(move || {
        sensor.run();
    });

    thread::spawn(move || {
        sensor1.run();
    });

    thread::spawn(move || {
        sensor2.run();
    });

    command.run();

    
}

fn print_reading(test : &Reading) {
    println!("{:?}", test);
}


#[derive(Debug)]
struct Reading {
    node_id : u8,
    timestamp : u64,
    value : f64,
}

struct SensorNode{
    node_id: u8,
    sender: mpsc::Sender<Reading>,
}

impl SensorNode {
    fn run(&self) {
        loop {
            let secret_number = rand::thread_rng().gen_range(1.0..=100.0);
            let timestamp = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_millis() as u64;
            let reading = Reading {
                node_id : self.node_id,
                timestamp : timestamp,
                value : secret_number,
            };
            self.sender.send(reading).unwrap();
            thread::sleep(Duration::from_millis(500));
        }
    }
}

struct CommandNode {
    receiver: mpsc::Receiver<Reading>,
}

impl CommandNode {
    fn run(&self) {
        for received in &self.receiver {
            let secs = (received.timestamp / 1000) as i64;
            let nsecs = ((received.timestamp % 1000) * 1_000_000) as u32;

            if let Some(utc_time) = DateTime::<Utc>::from_timestamp(secs, nsecs) {
                let local_time: DateTime<Local> = DateTime::from(utc_time);
                let formatted_time = local_time.format("%H:%M:%S%.3f").to_string();
                println!("Got {:.2} from thread {} at {}", received.value, received.node_id, formatted_time);
            } else {
                println!("Got {:.2} from thread {} at [Invalid Timestamp]", received.value, received.node_id);
            }
        }
    }
}