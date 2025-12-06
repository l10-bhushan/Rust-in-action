#![allow(unused_variables)]
type Message = String;

#[derive(Debug)]
struct Mailbox {
    msg: Vec<Message>,
}

#[derive(Debug)]
struct Cubesat {
    sat_id: u64,
    mailbox: Mailbox,
}

impl Cubesat {
    fn recv(&mut self) -> Option<Message> {
        self.mailbox.msg.pop()
    }
}

#[derive(Debug)]
enum StatusMessage {
    OK,
}

fn check_status(sat_id: Cubesat) -> Cubesat {
    StatusMessage::OK;
    sat_id
}

struct GroundStation {}

impl GroundStation {
    fn send(&self, sat: &mut Cubesat, msg: Message) {
        sat.mailbox.msg.push(msg);
    }
}

fn main() {
    println!("Cubesat constellations");

    // 3 satellites
    let sat_a: Cubesat = Cubesat {
        sat_id: 0,
        mailbox: Mailbox { msg: vec![] },
    };
    let sat_b: Cubesat = Cubesat {
        sat_id: 1,
        mailbox: Mailbox { msg: vec![] },
    };
    let sat_c: Cubesat = Cubesat {
        sat_id: 2,
        mailbox: Mailbox { msg: vec![] },
    };

    let check_for_a = check_status(sat_a);
    let check_for_b = check_status(sat_b);
    let check_for_c = check_status(sat_c);

    println!(
        "a : {:?} , b : {:?} , c : {:?}",
        check_for_a, check_for_b, check_for_c
    );

    let check_for_a = check_status(check_for_a);
    let check_for_b = check_status(check_for_b);
    let check_for_c = check_status(check_for_c);

    println!(
        "a : {:?} , b : {:?} , c : {:?}",
        check_for_a, check_for_b, check_for_c
    )
}
