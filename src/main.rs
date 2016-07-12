#[macro_use]
extern crate objc;

use objc::Encode;
use objc::runtime::{Class, Object};

fn main() {
    unsafe {
        let cls = Class::get("NSObject").unwrap();
        let obj: *mut Object = msg_send![cls, new];
        let hash: usize = msg_send![obj, hash];
        let is_kind: bool = msg_send![obj, isKindOfClass:cls];
        println!("{:?}", hash);
        println!("{:?}", is_kind);
    }
}
