use x11::xlib::{_XDisplay};
use x11::xlib::{CurrentTime, GrabModeAsync, XGrabPointer, XUngrabPointer};

pub struct Grabber {
    display: *mut _XDisplay,
    root: u64,
}

impl Grabber {
    pub fn new(display: *mut _XDisplay, root: u64) -> Grabber {
        Grabber {
            display: display,
            root: root,
        }
    }

    pub unsafe fn force_generate_enter_events(&self) {
        XGrabPointer(
            self.display,
            self.root,
            0,
            0,
            GrabModeAsync,
            GrabModeAsync,
            0,
            0,
            CurrentTime,
        );
        XUngrabPointer(self.display, CurrentTime);
    }
}
