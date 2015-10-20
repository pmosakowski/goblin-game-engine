#![allow(non_camel_case_types)]

extern crate libc;
use self::libc::{
    c_char,
    c_int,
    int16_t, 
    uint8_t, 
    uint16_t, 
    uint32_t
};
use std::ptr::null;

#[link(name="xcb")]
extern {
    fn xcb_connect(display: *const c_char, screen: *mut c_int) -> *mut xcb_connection_t;
    fn xcb_disconnect(conn: *mut xcb_connection_t);

    fn xcb_get_setup(conn: *mut xcb_connection_t) -> *mut xcb_setup_t;
    fn xcb_flush(conn: *mut xcb_connection_t) -> c_int;
    fn xcb_generate_id(conn: *mut xcb_connection_t) -> uint32_t;
    fn xcb_setup_roots_iterator(setup: *mut xcb_setup_t) -> xcb_screen_iterator_t;
    fn xcb_create_window(
        conn:         *mut xcb_connection_t,
        depth:        uint8_t,
        win:          xcb_window_t,
        parent:       xcb_window_t,
        x:            int16_t,
        y:            int16_t,
        width:        uint16_t,
        height:       uint16_t,
        border_width: uint16_t,
        class:        uint16_t,
        visual:       xcb_visualid_t,
        value_mask:   uint32_t,
        value_list:   *const uint32_t) -> xcb_void_cookie_t;

    fn xcb_map_window(conn: *mut xcb_connection_t, window: xcb_window_t) -> xcb_void_cookie_t;
}

// opaque structs
enum xcb_connection_t {}
enum xcb_setup_t {}
enum xcb_void_cookie_t {}

#[derive(Debug)]
#[repr(C)]
struct xcb_screen_t {
    root:                  xcb_window_t,
    default_colormap:      xcb_colormap_t,
    white_pixel:           uint32_t,
    black_pixel:           uint32_t,
    current_input_masks:   uint32_t,
    width_in_pixels:       uint16_t,
    height_in_pixels:      uint16_t,
    width_in_millimeters:  uint16_t,
    height_in_millimeters: uint16_t,
    min_installed_maps:    uint16_t,
    max_installed_maps:    uint16_t,
    root_visual:           xcb_visualid_t,
    backing_stores:        uint8_t,
    save_unders:           uint8_t,
    root_depth:            uint8_t,
    allowed_depths_len:    uint8_t,
}

#[repr(C)]
struct xcb_screen_iterator_t {
    data: *mut xcb_screen_t,
    rem: c_int,
    index: c_int,
}

type xcb_colormap_t = uint32_t;
type xcb_visualid_t = uint32_t;
type xcb_window_t = uint32_t;

#[derive(Debug)]
pub struct Window<'a> {
    id: xcb_window_t,
    pub width: u16,
    pub height: u16,
    screen: &'a xcb_screen_t,
}

impl<'a> Window<'a> {
    pub fn new(width: u16, height: u16) -> Result<Window<'a>, String> {
        let mut scr_num = 0;
        let conn = unsafe {
            xcb_connect(null(), &mut scr_num)
        };
        
        let setup = unsafe {
            xcb_get_setup(conn)
        };
        
        let scr = unsafe {
            let iter = xcb_setup_roots_iterator(setup);
            &(*iter.data)
        };

        let root_id = (*scr).root;
        let root_visual = (*scr).root_visual;
        let root_depth = (*scr).root_depth;
       
        let win_id = unsafe {
            xcb_generate_id(conn)
        };

        unsafe {
            xcb_create_window(conn, root_depth, win_id, root_id, 0, 0, width, height, 0, 0, root_visual, 0, null()); 
        };

        let win = Window {
            id: win_id,
            width: width,
            height: height,
            screen: scr,
        };

        unsafe {
            xcb_map_window(conn, win_id);
            xcb_flush(conn);
        }

        Ok(win)
    }
}
