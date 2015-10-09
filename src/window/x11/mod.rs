extern crate x11 as libx11;
extern crate libc;

use self::libx11::xlib;
use std::ptr::{null, null_mut};
use std::mem::zeroed;

pub struct X11Window<'a> {
    pub width: u32,
    pub height: u32,
    display: &'a mut xlib::Display,
    screen: i32,
    window: xlib::Window,
    gc: xlib::GC,
}

impl<'a> X11Window<'a> {
    pub fn new(width: u32, height: u32) -> Result<X11Window<'a>, String> {
        unsafe {
            let disp = xlib::XOpenDisplay(null());
            if disp == null_mut() {
                return Err(String::from("Can't open display"))
            }
            let scr = xlib::XDefaultScreen(disp);
            let root_win = xlib::XRootWindow(disp,scr);
            let black = xlib::XBlackPixel(disp, scr);
            let white = xlib::XWhitePixel(disp, scr);
            let win = xlib::XCreateSimpleWindow(
                disp, root_win,
                0, 0, width, height,
                0, black, white
            );

            xlib::XMapWindow(disp, win);


            let event_mask = xlib::StructureNotifyMask;
            xlib::XSelectInput(disp, win, event_mask);
            let mut e: xlib::XEvent = zeroed();

            loop {
                xlib::XNextEvent(disp, &mut e);
                match e.get_type() {
                    xlib::MapNotify => {break;},
                    _ => {},
                }
            }
            let mut gcval: xlib::XGCValues = zeroed();
            let gc = xlib::XCreateGC(disp, win, 0, &mut gcval);

            xlib::XSetForeground(disp, gc, black);
            xlib::XDrawLine(disp, win, gc, 10, 10,190,190); //from-to
            xlib::XDrawLine(disp, win, gc, 10,190,190, 10);

            let mouse_events = xlib::ButtonPressMask | xlib::ButtonReleaseMask;
            xlib::XSelectInput(disp, win, mouse_events);
            loop {
                xlib::XNextEvent(disp, &mut e);
                match e.get_type() {
                    xlib::ButtonRelease => {break;},
                    _ => {},
                }
            }

            Ok(X11Window {
                width: width,
                height: height,
                display: &mut *disp,
                screen: scr,
                window: win,
                gc: gc,
            })
        }
    }

    pub fn close(&mut self) {
        unsafe {
            xlib::XDestroyWindow(self.display, self.window);
            xlib::XCloseDisplay(self.display);
        }
    }
}
