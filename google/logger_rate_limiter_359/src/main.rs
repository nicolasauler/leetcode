use std::collections::HashMap;

struct Logger {
    last_appearance: HashMap<String, i32>,
}

impl Logger {
    fn new() -> Self {
        Self {
            last_appearance: HashMap::new(),
        }
    }

    fn should_print_message(&mut self, timestamp: i32, message: String) -> bool {
        let mut should_print = true;

        self.last_appearance
            .entry(message)
            .and_modify(|t| {
                if timestamp - *t >= 10 {
                    *t = timestamp;
                } else {
                    should_print = false;
                }
            })
            .or_insert(timestamp);

        should_print
    }
}

fn main() {
    let messages = vec![
        (1, "foo"),
        (2, "bar"),
        (3, "foo"),
        (8, "bar"),
        (10, "foo"),
        (11, "foo"),
    ];
    let mut obj = Logger::new();
    for message in messages {
        let (timestamp, msg) = message;
        let ret_1 = obj.should_print_message(timestamp, msg.to_owned());
        println!("{ret_1}");
    }
}
