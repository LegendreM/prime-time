extern crate amqp;
extern crate serde;
extern crate serde_json;

#[macro_use]
extern crate serde_derive;

use amqp::{Session, Basic, protocol};
use std::{thread, time};


#[derive(Deserialize)]
struct Task {
    task: u64,
    token: u32,
}

#[derive(Serialize)]
struct Update {
    token: u32,
    state: String,
    result: Option<String>,
}

fn check_prime(n: u64) -> bool {
    for x in 2..n {
        if n % x == 0 {
            return false;
        }
    }
    true
}

fn next_prime(n: u64) -> u64 {
    let mut v = n + 1;
    while !check_prime(v) {
        v = v + 1;
    }
    v
}

fn process(task: u64) -> String {
    let mut factor_list = Vec::new();
    let mut task = task;
    let mut prime = next_prime(1);

    while prime <= task {
        while task % prime == 0 {
            task = task / prime;
            factor_list.push(prime);
        }
        prime = next_prime(prime);
    }
    factor_list.iter().map(|x| format!("{}", x)).collect::<Vec<String>>().join("x")
}

fn post_update(channel: &mut amqp::Channel, token: u32, state: String, result: Option<String>) {
    let update_queue_name = "update";

    let update: Update = Update {token: token, state: state, result: result};
    
    let json_string = serde_json::to_string(&update).unwrap();

    channel.basic_publish(
        "",
        update_queue_name,
        true,
        false,
        protocol::basic::BasicProperties{ content_type: Some("text".to_string()), ..Default::default()},
        json_string.as_bytes().to_vec()
    );
}

fn conn_loop() {
    let mut session = match Session::open_url("amqp://rabbit") {
        Ok(session) => session,
        Err(error) => {
            println!("Can't create session: {:?}", error);
            return ;
        }
    };

    let mut channel_in = session.open_channel(1).ok().expect("Can't open channel!");
    let mut channel_out = session.open_channel(2).ok().expect("Can't open channel!");
    let queue_name = "task";

    for get_result in channel_in.basic_get(queue_name, false) {
        println!("Received: {:?}", String::from_utf8_lossy(&get_result.body));
        get_result.ack();
        let task : Task = match serde_json::from_str(&String::from_utf8_lossy(&get_result.body)) {
            Ok(task) => task,
            Err(e) => {
                println!("Error: {:?}", e);
                return ;
            },
        };
        post_update(&mut channel_out, task.token, "working".to_string(), None);
        let result = process(task.task);
        println!("Process: {:?}", result);
        post_update(&mut channel_out, task.token, "done".to_string(), Some(result));
    }
}

fn main() {
    loop {
        conn_loop();
        // wait 3sec before trying to reconnect
        thread::sleep(time::Duration::from_millis(3000));
    }
}
