/*
*
* X11 support is currently broken, and may never be fixed.
*
* I have a preference against unsafe code, but the X protocols require unsafe code.
*
* Chunks was intended to be solely for Wayland Compositors - that being said - if
* someone can get X11 to work properly, be my guest.
*
*/

use gdk4_x11::{X11Display, X11Surface};
use gio::prelude::Cast;
use gtk4::{
    prelude::NativeExt,
    prelude::{GtkWindowExt, WidgetExt},
    ApplicationWindow,
};
use x11::xlib::{
    Display, PMaxSize, PMinSize, PPosition, PSize, PropModeAppend, PropModeReplace, Window,
    XChangeProperty, XInternAtom, XMoveResizeWindow, XSetWMNormalHints, XSizeHints, XA_ATOM,
};

pub struct X11 {
    x: i32,
    y: i32,
    width: i32,
    height: i32,
    always_on_top: bool,
    stick: bool,
}

impl X11 {
    pub fn new(x: i32, y: i32, width: i32, height: i32, always_on_top: bool, stick: bool) -> Self {
        Self {
            x,
            y,
            width,
            height,
            always_on_top,
            stick,
        }
    }

    pub fn setup_window(self, chunk: &ApplicationWindow) {
        chunk.connect_realize(move |w| {
            if let Some(display) = w.display().downcast_ref::<X11Display>() {
                let surface = w.surface().expect("Window should have a surface");
                let x11_surface = surface
                    .downcast_ref::<X11Surface>()
                    .expect("Surface should be an X11Surface");
                let xid = x11_surface.xid() as Window;

                unsafe {
                    let x11_display = display.xrootwindow() as *mut Display;

                    XMoveResizeWindow(
                        x11_display,
                        xid,
                        self.x,
                        self.y,
                        self.width as u32,
                        self.height as u32,
                    );

                    if self.always_on_top {
                        let atom = XInternAtom(
                            x11_display,
                            "_NET_WM_STATE_ABOVE\0".as_ptr() as *const _,
                            0,
                        );
                        XChangeProperty(
                            x11_display,
                            xid,
                            XInternAtom(x11_display, "_NET_WM_STATE\0".as_ptr() as *const _, 0),
                            XA_ATOM,
                            32,
                            PropModeReplace,
                            &atom as *const _ as *const u8,
                            1,
                        );
                    }

                    if self.stick {
                        let atom = XInternAtom(
                            x11_display,
                            "_NET_WM_STATE_STICKY\0".as_ptr() as *const _,
                            0,
                        );
                        XChangeProperty(
                            x11_display,
                            xid,
                            XInternAtom(x11_display, "_NET_WM_STATE\0".as_ptr() as *const _, 0),
                            XA_ATOM,
                            32,
                            PropModeAppend,
                            &atom as *const _ as *const u8,
                            1,
                        );
                    }

                    let mut hints: XSizeHints = std::mem::zeroed();
                    hints.flags = PPosition | PSize | PMinSize | PMaxSize;
                    hints.x = self.x;
                    hints.y = self.y;
                    hints.width = self.width;
                    hints.height = self.height;
                    hints.min_width = self.width;
                    hints.min_height = self.height;
                    hints.max_width = self.width;
                    hints.max_height = self.height;

                    XSetWMNormalHints(x11_display, xid, &mut hints);
                }
            }
        });

        chunk.present();
    }
}
