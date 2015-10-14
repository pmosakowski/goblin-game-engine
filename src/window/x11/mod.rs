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
    visual: &'a mut xlib::Visual,
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
            let vis = xlib::XDefaultVisual(disp, scr);
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

            Ok(X11Window {
                width: width,
                height: height,
                display: &mut *disp,
                screen: scr,
                visual: &mut *vis,
                window: win,
                gc: gc,
            })
        }
    }

    pub fn update(&mut self) {
        unsafe {
            let mut buffer = Image::new(self.display, self.visual, 640, 480);

            for x in 0..640 {
                for y in 0..480 {
                    let pad = 00;
                    let red = (x % 256);
                    let green = (y % 256);
                    let blue = 256 - (x % 256);
                    let color = (pad << 24) + (red << 16) + (green << 8) + blue;
                    xlib::XPutPixel(buffer.ximage, x, y, color as u64);
                };
            };

            self.blit(&mut buffer);

            let mouse_events = xlib::ButtonPressMask | xlib::ButtonReleaseMask;
            xlib::XSelectInput(self.display, self.window, mouse_events);
            let mut e: xlib::XEvent = zeroed();
            loop {
                xlib::XNextEvent(self.display, &mut e);
                match e.get_type() {
                    xlib::ButtonRelease => {break;},
                    _ => {},
                }
            }
        }
    }

    pub fn blit(&mut self, img: &mut Image) {
        unsafe {
            xlib::XPutImage(self.display,
                            self.window,
                            self.gc,
                            img.ximage,
                            0,0,
                            0,0,
                            img.width,
                            img.height,)
        };
    }

    pub fn close(&mut self) {
        unsafe {
            xlib::XDestroyWindow(self.display, self.window);
            xlib::XCloseDisplay(self.display);
        }
    }
}

pub struct Image<'a> {
    ximage: &'a mut xlib::XImage,
    data: Vec<i8>,
    width: u32,
    height: u32,
}

impl<'a> Image<'a> {
    pub fn new(disp: &mut xlib::Display, vis: &mut xlib::Visual, width: u32, height: u32) -> Image<'a> {
        let bitmap_pad: i32 = 32;
        let bytes_per_line: i32 = (width as i32)*bitmap_pad/4;
        let mut data: Vec<i8> = vec![0; (width*height*4) as usize];
        let mut ximg_ref = unsafe {
            let ximg_ptr = xlib::XCreateImage(disp, vis, 24, xlib::ZPixmap, 0,
                                              data.as_mut_ptr(), width, height, bitmap_pad, bytes_per_line);
            xlib::XInitImage(ximg_ptr);
            &mut *ximg_ptr
        };

        Image {
            ximage: ximg_ref,
            data: data,
            width: width,
            height: height,
        }
    }
}
