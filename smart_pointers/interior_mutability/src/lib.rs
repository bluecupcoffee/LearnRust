pub trait Messenger  {
    fn send(&self, msg: &str);
}

pub struct LimitTracker<'a, T: 'a + Messenger> {
    messenger: &'a T,
    value: usize,
    max: usize
}

impl<'a, T> LimitTracker<'a, T>
    where T: Messenger {
        pub fn new(messenger: &T, max: usize) -> LimitTracker<T> {
            LimitTracker {
                messenger,
                value: 0,
                max
            }
        }

        pub fn set_value(&mut self, value: usize) {
            self.value = value;

            let percentage_of_max = self.value as f64 / self.max as f64;

            if percentage_of_max >= 1.0 {
                self.messenger.send("Error: you are over your quota");
            } else if percentage_of_max >= 0.9 {
                self.messenger.send("Urgent. Used over 90% of quota");
            } else if percentage_of_max >= 0.75 {
                self.messenger.send("Warning. Used over 75% of quota");
            }
        }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    struct MockMessenger {
        sent_messages: RefCell<Vec<String>>
    }

    impl MockMessenger {
        fn new() -> MockMessenger {
            MockMessenger {
                sent_messages: RefCell::new(vec![])
            }
        }
    }

    impl Messenger for MockMessenger {
        fn send(&self, message: &str) {
            let mut borrow_one = self.sent_messages.borrow_mut();
            //let mut borrow_two = self.sent_messages.borrow_mut();
            borrow_one.push(String::from(message));
            //borrow_two.push(String::from(message));
        }
    }

    #[test]
    fn it_sends_an_over_75_percent_warning() {
        let mock = MockMessenger::new();
        let mut limit_tracker = LimitTracker::new(&mock, 100);

        limit_tracker.set_value(80);

        assert_eq!(mock.sent_messages.borrow().len(), 1);
    }

    #[test]
    fn it_sends_over_90_percent_warning() {
        let mock = MockMessenger::new();
        let mut limit_tracker = LimitTracker::new(&mock, 100);
        limit_tracker.set_value(91);
        assert_eq!(mock.sent_messages.borrow().len(), 1);
    }
}