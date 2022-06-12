use std::cell::RefCell;

pub trait Messager {
    fn send(&self, msg: &str);
}

pub struct LimitTracker<'a, T: Messager>{
    messager: &'a T,
    value: usize,
    max: usize,
}

impl<'a, T> LimitTracker<'a, T>
where
    T: Messager,
{
    pub fn new(messager: &T, max: usize) -> LimitTracker<T> {
        LimitTracker{
            messager,
            value: 0,
            max,
        }
    }

    pub fn set_value(&mut self, value: usize) {
        self.value = value;

        let percentage_of_max = self.value as f64 / self.max as f64;

        if percentage_of_max >= 1.0 {
            self.messager.send("Error: You are over your quota!");
        } else if percentage_of_max >= 0.9 {
            self.messager.send("Urgent: You've used up over 90% of your quota!");
        } else if percentage_of_max >= 0.75 {
            self.messager.send("Warning: You've used up over 75% of your quota!");
        }
    }
}

/******************* Test *******************/
struct MockMessager {
    pub sent_message: RefCell<Vec<String>>,
}

impl MockMessager {
    fn new() -> MockMessager {
        MockMessager {
            // Create a default emply vector in RefCell
            sent_message: RefCell::new(vec![]),
        }
    }
    
    fn get_message_record(&self) -> &RefCell<Vec<String>> {
        return &self.sent_message
    }
}

impl Messager for MockMessager {
    fn send(&self, message: &str) {
        self.sent_message.borrow_mut().push(String::from(message));
    }
}

fn all_warning_message() {
    let mock_messager = MockMessager::new();
    let mut limit_tracker = LimitTracker::new(&mock_messager, 100);

    limit_tracker.set_value(60);
    println!("limit tracker: {:?}", mock_messager.get_message_record());
    limit_tracker.set_value(80);
    println!("limit tracker: {:?}", mock_messager.get_message_record());
    limit_tracker.set_value(95);
    println!("limit tracker: {:?}", mock_messager.get_message_record());
    limit_tracker.set_value(105);
    println!("limit tracker: {:?}", mock_messager.get_message_record());
}

pub fn run() {
    // Test if all warning message sent properly
    all_warning_message();
}
