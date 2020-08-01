use std::collections::HashMap;

#[derive(Default)]
struct Logger {
    log: HashMap<String, i32>,
}

impl Logger {
    fn new() -> Self {
        Logger {
            log: HashMap::new(),
        }
    }
    fn should_print_message(&mut self, timestamp: i32, message: String) -> bool {
        if let Some(&v) = self.log.get(&message) {
            if timestamp - v >= 10 {
                self.log.insert(message, timestamp);
            } else {
                return false;
            }
        } else {
            self.log.insert(message, timestamp);
        }
        return true;
    }
}

#[test]
fn test() {
    let mut logger = Logger::new();
    assert_eq!(logger.should_print_message(1, "foo".to_string()), true);
    assert_eq!(logger.should_print_message(2, "bar".to_string()), true);
    assert_eq!(logger.should_print_message(3, "foo".to_string()), false);
    assert_eq!(logger.should_print_message(8, "bar".to_string()), false);
    assert_eq!(logger.should_print_message(10, "foo".to_string()), false);
    assert_eq!(logger.should_print_message(11, "foo".to_string()), true);
}
