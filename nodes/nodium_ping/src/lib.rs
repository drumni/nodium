use nodium_interface::{Node, SimpleNode};

#[no_mangle]
pub extern "C" fn create_node() -> *mut dyn Node {
    Box::into_raw(Box::new(SimpleNode {}))
}
