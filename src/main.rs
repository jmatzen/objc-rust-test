use objc::declare::ClassDecl;
use objc::msg_send;
use objc::rc::autoreleasepool;
use objc::runtime::{Class, Object};
use objc::sel;
use objc::sel_impl;

use objc::class;

#[derive(Debug)]
#[repr(C)]
struct NSPoint {
    x: f64,
    y: f64,
}

#[derive(Debug)]
#[repr(C)]
struct NSSize {
    width: f64,
    height: f64,
}

#[derive(Debug)]
#[repr(C)]
struct NSRect {
    origin: NSPoint,
    size: NSSize,
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn do_test() {
        unsafe {
            autoreleasepool(|| {
                let ns_screen = class!(NSScreen);
                let main_screen: *mut Object = msg_send![ns_screen, mainScreen];
                let visible_frame: NSRect = msg_send![main_screen, visibleFrame];
                println!("{:#?}", visible_frame);
            });
        }
    }
}
