use std::cell::RefCell;
use std::rc::Rc;
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

#[derive(Debug, PartialEq)] // Adding PartialEq trait
pub struct Data {
    pub name: String,
    pub data_type: String,
    pub time: u64,
    pub value: String,
}

pub trait EventHandler: FnMut(&Data) {
    fn as_any(&mut self) -> &mut dyn std::any::Any;
}

impl<T: FnMut(&Data) + 'static> EventHandler for T {
    fn as_any(&mut self) -> &mut dyn std::any::Any {
        self
    }
}

pub trait Node: Send {
    fn get_name(&self) -> String;
    fn handle_event(&self, event: &str) -> Option<String>;
    fn register_event_handler(&self, event: &str, handler: Rc<RefCell<dyn EventHandler>>);
    fn remove_event_handler(&self, event: &str);
    fn trigger_event(&self, event: &str, data: Data);
    fn get_inputs(&self) -> Vec<Data>;

    fn start_input_monitoring(&mut self) {
        thread::spawn(move || {
            let (tx, rx) = mpsc::channel();
            loop {
                let data = Data {
                    name: "input".to_string(),
                    data_type: "string".to_string(),
                    time: 0,
                    value: "Hello".to_string(),
                };
                tx.send(data).unwrap();
                thread::sleep(Duration::from_secs(1));
            }
        });
    }

    fn get_event_handler(&self, event: &str) -> Option<Rc<RefCell<dyn EventHandler>>>;
}
