extern crate amqp;

use amqp::{Session, Basic};
use std::{thread, time};

fn process() -> String {
    "HELLO !".to_string()
}

fn conn_loop() {
    let mut session = match Session::open_url("amqp://rabbit") {
        Ok(session) => session,
        Err(error) => {
            println!("Can't create session: {:?}", error);
            return ;
        }
    };

    let mut channel = session.open_channel(1).ok().expect("Can't open channel!");
    let queue_name = "task";

    for get_result in channel.basic_get(queue_name, false) {
        println!("Received: {:?}", String::from_utf8_lossy(&get_result.body));
        get_result.ack();
        println!("Process: {:?}", process());
    }
}

fn main() {
    loop {
        conn_loop();
        // wait 3sec before trying to reconnect
        thread::sleep(time::Duration::from_millis(3000));
    }
}
