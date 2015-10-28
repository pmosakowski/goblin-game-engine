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
    fn xcb_connect(display: *const c_char,
                   screen: *mut c_int) -> *mut xcb_connection_t;

    fn xcb_disconnect(conn: *mut xcb_connection_t);

    fn xcb_get_setup(conn: *mut xcb_connection_t) -> *mut xcb_setup_t;

    fn xcb_flush(conn: *mut xcb_connection_t) -> c_int;

    fn xcb_generate_id(conn: *mut xcb_connection_t) -> uint32_t;

    fn xcb_setup_roots_iterator(setup: *mut xcb_setup_t) -> xcb_screen_iterator_t;

    fn xcb_create_window(conn:         *mut xcb_connection_t,
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

    fn xcb_destroy_window(conn: *mut xcb_connection_t,
                          window: xcb_window_t);

    fn xcb_map_window(conn: *mut xcb_connection_t,
                      window: xcb_window_t) -> xcb_void_cookie_t;

    fn xcb_create_gc(conn: *mut xcb_connection_t,
                     gc_id: xcb_gcontext_t,
                     drawable: xcb_drawable_t,
                     value_mask: xcb_gc_t,
                     value_list: *const uint32_t) -> xcb_void_cookie_t;

    fn xcb_poly_line (conn: *mut xcb_connection_t,
                      coordinate_mode: xcb_coord_mode_t,
                      drawable: xcb_drawable_t,
                      gc: xcb_gcontext_t,
                      num_points: uint32_t,
                      points: *const xcb_point_t) -> xcb_void_cookie_t;

    fn xcb_poly_rectangle(conn:           *mut xcb_connection_t,
                           drawable:       xcb_drawable_t,
                           gc:             xcb_gcontext_t,
                           rectangles_len: uint32_t,
                           rectangles:     *const xcb_rectangle_t) -> xcb_void_cookie_t;

    fn xcb_poly_fill_rectangle(conn:           *mut xcb_connection_t,
                               drawable:       xcb_drawable_t,
                               gc:             xcb_gcontext_t,
                               rectangles_len: uint32_t,
                               rectangles:     *const xcb_rectangle_t) -> xcb_void_cookie_t;

    fn xcb_wait_for_event(conn: *mut xcb_connection_t) -> *mut xcb_generic_event_t;

    fn xcb_create_pixmap(conn:      *mut xcb_connection_t,
                         depth:     uint8_t,
                         pixmap_id: xcb_pixmap_t,
                         drawable:  xcb_drawable_t,
                         width:     uint16_t,
                         height:    uint16_t) -> xcb_void_cookie_t;

    fn xcb_free_pixmap(conn:      *mut xcb_connection_t,
                       pixmap_id: xcb_pixmap_t) -> xcb_void_cookie_t;

    fn xcb_copy_area(conn:         *mut xcb_connection_t,
                     src_drawable: xcb_drawable_t,
                     dst_drawable: xcb_drawable_t,
                     gc:           xcb_gcontext_t,
                     src_x:        int16_t,
                     src_y:        int16_t,
                     dst_x:        int16_t,
                     dst_y:        int16_t,
                     width:        uint16_t,
                     height:       uint16_t) -> xcb_void_cookie_t;

    fn xcb_put_image(conn:     *mut xcb_connection_t,
                     format:   uint8_t,
                     drawable: xcb_drawable_t,
                     gc:       xcb_gcontext_t,
                     width:    uint16_t,
                     height:   uint16_t,
                     dst_x:    int16_t,
                     dst_y:    int16_t,
                     left_pad: uint8_t,
                     depth:    uint8_t,
                     data_len: uint32_t,
                     data:     *const uint8_t) -> xcb_void_cookie_t;
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

#[repr(C)]
struct xcb_point_t {
    x: int16_t,
    y: int16_t,
}

#[repr(C)]
struct xcb_rectangle_t {
    x: int16_t,
    y: int16_t,
    width: uint16_t,
    height: uint16_t,
}

#[repr(C)]
struct xcb_generic_event_t {
    response_type: xcb_event_type_t,
    pad0: uint8_t,
    sequence: uint16_t,
    pad: [uint32_t; 7],
    full_sequence: uint32_t,
}

// typedefs

type xcb_colormap_t = uint32_t;
type xcb_coord_mode_t = uint8_t;
type xcb_cw_t = uint32_t;
type xcb_event_mask_t = uint32_t;
type xcb_event_type_t = uint8_t;
type xcb_gc_t = uint32_t;
type xcb_gcontext_t = uint32_t;
type xcb_visualid_t = uint32_t;

type xcb_drawable_t = uint32_t;
type xcb_pixmap_t = xcb_drawable_t;
type xcb_window_t = xcb_drawable_t;

// xcb_gc_t constants

const XCB_GC_FUNCTION: xcb_gc_t = 1;
const XCB_GC_PLANE_MASK: xcb_gc_t = 2;
const XCB_GC_FOREGROUND: xcb_gc_t = 4;
const XCB_GC_BACKGROUND: xcb_gc_t = 8;
const XCB_GC_LINE_WIDTH: xcb_gc_t = 16;
const XCB_GC_LINE_STYLE: xcb_gc_t = 32;
const XCB_GC_CAP_STYLE: xcb_gc_t = 64;
const XCB_GC_JOIN_STYLE: xcb_gc_t = 128;
const XCB_GC_FILL_STYLE: xcb_gc_t = 256;
const XCB_GC_FILL_RULE: xcb_gc_t = 512;
const XCB_GC_TILE: xcb_gc_t = 1024;
const XCB_GC_STIPPLE: xcb_gc_t = 2048;
const XCB_GC_TILE_STIPPLE_ORIGIN_X: xcb_gc_t = 4096;
const XCB_GC_TILE_STIPPLE_ORIGIN_Y: xcb_gc_t = 8192;
const XCB_GC_FONT: xcb_gc_t = 16384;
const XCB_GC_SUBWINDOW_MODE: xcb_gc_t = 32768;
const XCB_GC_GRAPHICS_EXPOSURES: xcb_gc_t = 65536;
const XCB_GC_CLIP_ORIGIN_X: xcb_gc_t = 131072;
const XCB_GC_CLIP_ORIGIN_Y: xcb_gc_t = 262144;
const XCB_GC_CLIP_MASK: xcb_gc_t = 524288;
const XCB_GC_DASH_OFFSET: xcb_gc_t = 1048576;
const XCB_GC_DASH_LIST: xcb_gc_t = 2097152;
const XCB_GC_ARC_MODE: xcb_gc_t = 4194304;

// xcb_coord_mode_t constants

const XCB_COORD_MODE_ORIGIN: xcb_coord_mode_t = 0;
const XCB_COORD_MODE_PREVIOUS: xcb_coord_mode_t = 1;

// xcb_cw_t constants

const XCB_CW_BACK_PIXMAP: xcb_cw_t = 1;
const XCB_CW_BACK_PIXEL: xcb_cw_t = 2;
const XCB_CW_BORDER_PIXMAP: xcb_cw_t = 4;
const XCB_CW_BORDER_PIXEL: xcb_cw_t = 8;
const XCB_CW_BIT_GRAVITY: xcb_cw_t = 16;
const XCB_CW_WIN_GRAVITY: xcb_cw_t = 32;
const XCB_CW_BACKING_STORE: xcb_cw_t = 64;
const XCB_CW_BACKING_PLANES: xcb_cw_t = 128;
const XCB_CW_BACKING_PIXEL: xcb_cw_t = 256;
const XCB_CW_OVERRIDE_REDIRECT: xcb_cw_t = 512;
const XCB_CW_SAVE_UNDER: xcb_cw_t = 1024;
const XCB_CW_EVENT_MASK: xcb_cw_t = 2048;
const XCB_CW_DONT_PROPAGATE: xcb_cw_t = 4096;
const XCB_CW_COLORMAP: xcb_cw_t = 8192;
const XCB_CW_CURSOR: xcb_cw_t = 16384;

// xcb_event_mask_t constants

const XCB_EVENT_MASK_NO_EVENT: xcb_event_mask_t = 0;
const XCB_EVENT_MASK_KEY_PRESS: xcb_event_mask_t = 1;
const XCB_EVENT_MASK_KEY_RELEASE: xcb_event_mask_t = 2;
const XCB_EVENT_MASK_BUTTON_PRESS: xcb_event_mask_t = 4;
const XCB_EVENT_MASK_BUTTON_RELEASE: xcb_event_mask_t = 8;
const XCB_EVENT_MASK_ENTER_WINDOW: xcb_event_mask_t = 16;
const XCB_EVENT_MASK_LEAVE_WINDOW: xcb_event_mask_t = 32;
const XCB_EVENT_MASK_POINTER_MOTION: xcb_event_mask_t = 64;
const XCB_EVENT_MASK_POINTER_MOTION_HINT: xcb_event_mask_t = 128;
const XCB_EVENT_MASK_BUTTON_1_MOTION: xcb_event_mask_t = 256;
const XCB_EVENT_MASK_BUTTON_2_MOTION: xcb_event_mask_t = 512;
const XCB_EVENT_MASK_BUTTON_3_MOTION: xcb_event_mask_t = 1024;
const XCB_EVENT_MASK_BUTTON_4_MOTION: xcb_event_mask_t = 2048;
const XCB_EVENT_MASK_BUTTON_5_MOTION: xcb_event_mask_t = 4096;
const XCB_EVENT_MASK_BUTTON_MOTION: xcb_event_mask_t = 8192;
const XCB_EVENT_MASK_KEYMAP_STATE: xcb_event_mask_t = 16384;
const XCB_EVENT_MASK_EXPOSURE: xcb_event_mask_t = 32768;
const XCB_EVENT_MASK_VISIBILITY_CHANGE: xcb_event_mask_t = 65536;
const XCB_EVENT_MASK_STRUCTURE_NOTIFY: xcb_event_mask_t = 131072;
const XCB_EVENT_MASK_RESIZE_REDIRECT: xcb_event_mask_t = 262144;
const XCB_EVENT_MASK_SUBSTRUCTURE_NOTIFY: xcb_event_mask_t = 524288;
const XCB_EVENT_MASK_SUBSTRUCTURE_REDIRECT: xcb_event_mask_t = 1048576;
const XCB_EVENT_MASK_FOCUS_CHANGE: xcb_event_mask_t = 2097152;
const XCB_EVENT_MASK_PROPERTY_CHANGE: xcb_event_mask_t = 4194304;
const XCB_EVENT_MASK_COLOR_MAP_CHANGE: xcb_event_mask_t = 8388608;
const XCB_EVENT_MASK_OWNER_GRAB_BUTTON: xcb_event_mask_t = 16777216;

// xcb_event_type_t constants

const XCB_KEY_PRESS:         xcb_event_type_t = 2;
const XCB_KEY_RELEASE:       xcb_event_type_t = 3;
const XCB_BUTTON_PRESS:      xcb_event_type_t = 4;
const XCB_BUTTON_RELEASE:    xcb_event_type_t = 5;
const XCB_MOTION_NOTIFY:     xcb_event_type_t = 6;
const XCB_ENTER_NOTIFY:      xcb_event_type_t = 7;
const XCB_LEAVE_NOTIFY:      xcb_event_type_t = 8;
const XCB_FOCUS_IN:          xcb_event_type_t = 9;
const XCB_FOCUS_OUT:         xcb_event_type_t = 10;
const XCB_KEYMAP_NOTIFY:     xcb_event_type_t = 11;
const XCB_EXPOSE:            xcb_event_type_t = 12;
const XCB_GRAPHICS_EXPOSURE: xcb_event_type_t = 13;
const XCB_NO_EXPOSURE:       xcb_event_type_t = 14;
const XCB_VISIBILITY_NOTIFY: xcb_event_type_t = 15;
const XCB_CREATE_NOTIFY:     xcb_event_type_t = 16;
const XCB_DESTROY_NOTIFY:    xcb_event_type_t = 17;
const XCB_UNMAP_NOTIFY:      xcb_event_type_t = 18;
const XCB_MAP_NOTIFY:        xcb_event_type_t = 19;
const XCB_MAP_REQUEST:       xcb_event_type_t = 20;
const XCB_REPARENT_NOTIFY:   xcb_event_type_t = 21;
const XCB_CONFIGURE_NOTIFY:  xcb_event_type_t = 22;
const XCB_CONFIGURE_REQUEST: xcb_event_type_t = 23;
const XCB_GRAVITY_NOTIFY:    xcb_event_type_t = 24;
const XCB_RESIZE_REQUEST:    xcb_event_type_t = 25;
const XCB_CIRCULATE_NOTIFY:  xcb_event_type_t = 26;
const XCB_CIRCULATE_REQUEST: xcb_event_type_t = 27;
const XCB_PROPERTY_NOTIFY:   xcb_event_type_t = 28;
const XCB_SELECTION_CLEAR:   xcb_event_type_t = 29;
const XCB_SELECTION_REQUEST: xcb_event_type_t = 30;
const XCB_SELECTION_NOTIFY:  xcb_event_type_t = 31;
const XCB_COLORMAP_NOTIFY:   xcb_event_type_t = 32;
const XCB_CLIENT_MESSAGE:    xcb_event_type_t = 33;
const XCB_MAPPING_NOTIFY:    xcb_event_type_t = 34;
const XCB_GE_GENERIC:        xcb_event_type_t = 35;


pub struct Window<'a> {
    id: xcb_window_t,
    connection: &'a mut xcb_connection_t,
    pub width: u16,
    pub height: u16,
    screen: &'a xcb_screen_t,
}

impl<'a> Window<'a> {
    pub fn new(width: u16, height: u16) -> Result<Window<'a>, String> {
        let mut scr_num = 0;
        let mut conn = unsafe {
            &mut *xcb_connect(null(), &mut scr_num)
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

        let mask = XCB_CW_BACK_PIXEL | XCB_CW_EVENT_MASK;
        let values = vec![scr.black_pixel, XCB_EVENT_MASK_EXPOSURE];

        unsafe {
            xcb_create_window(conn, root_depth, win_id, root_id, 0, 0, width, height, 0, 0, root_visual, mask, values.as_ptr()); 
        };

        let win = Window {
            connection: conn,
            id: win_id,
            width: width,
            height: height,
            screen: scr,
        };

        unsafe {
            xcb_map_window(win.connection, win_id);
            xcb_flush(win.connection);
        };

        loop {
            let event = unsafe {
                &*xcb_wait_for_event(win.connection)
            };

            match event.response_type {
                XCB_EXPOSE => break,
                _ => (),
            };
        };

        Ok(win)
    }

    fn create_gc(win: &mut Window, colour: u32) -> xcb_gcontext_t {
        let gc_id = unsafe {
            xcb_generate_id(win.connection)
        };

        let mask = XCB_GC_FOREGROUND | XCB_GC_BACKGROUND | XCB_GC_FILL_STYLE;
        let value: [uint32_t;3] = [colour, win.screen.black_pixel, 0];
        unsafe {
            xcb_create_gc(win.connection, gc_id, win.id, mask, value.as_ptr());
        };

        gc_id
    }

    pub fn update(&mut self) {
        let white = self.screen.white_pixel.clone();
        let gc_fg = Window::create_gc(self, white);
        let gc_bg = Window::create_gc(self, 0x70707070);

        let points: Vec<xcb_point_t> = vec![
            xcb_point_t {x: 0, y:0},
            xcb_point_t {x: 1000, y:1000},
            xcb_point_t {x: 0, y:200},
            xcb_point_t {x: 200, y:0}];

        let clear_rect = xcb_rectangle_t {
            x: 0,
            y: 0,
            width: self.width,
            height: self.height,
        };

        let pixmap_id = unsafe {
            xcb_generate_id(self.connection)
        };

        unsafe {
            xcb_create_pixmap(self.connection,
                              self.screen.root_depth,
                              pixmap_id,
                              self.id,
                              self.width,
                              self.height);
            xcb_poly_fill_rectangle(self.connection,
                                    pixmap_id,
                                    gc_bg,
                                    1,
                                    &clear_rect);
            xcb_poly_line(self.connection,
                          XCB_COORD_MODE_ORIGIN,
                          pixmap_id,
                          gc_fg,
                          points.len() as u32,
                          points.as_ptr());
            xcb_copy_area(self.connection,
                          pixmap_id,
                          self.id,
                          gc_bg,
                          0, 0,
                          0, 0,
                          self.width, self.height);
            xcb_flush(self.connection);
        };
    }

    pub fn close(&mut self) {
        unsafe {
            xcb_destroy_window(self.connection, self.id);
            xcb_disconnect(self.connection);
        };
    }

    pub fn to_str(&self) -> String {
        format!("{:?}", self.screen)
    }
}
