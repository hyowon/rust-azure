import xlib::*;
import libc::*;
import cairo_xlib::*;
import xlib::bindgen::*;
import cairo::*;
import cairo::bindgen::*;
import cairo_xlib::bindgen::*;

#[link_args = "-L. -lcairo -lazure"]
#[nolink]
native mod m { }

#[test]
fn sanity_check() {
    AzureCSanityCheck();
}

const SIZEX: c_uint = 400 as c_uint;
const SIZEY: c_uint = 400 as c_uint;

const ExposureMask: c_long = (1 << 15) as c_long;
const ButtonPressMask: c_long = (1 << 2) as c_long;

const Expose: c_int = 12 as c_int;
const ButtonPress: c_int = 4 as c_int;

type XEventStub = {
    type_: c_int,
    padding: (
        int, int, int, int, int, int, int, int,
        int, int, int, int, int, int, int, int,
        int, int, int, int, int, int, int, int,
        int, int, int, int, int, int, int, int,
        int, int, int, int, int, int, int, int,
        int, int, int, int, int, int, int, int,
        int, int, int, int, int, int, int, int,
        int, int, int, int, int, int, int, int
    )
};

fn xexpose(event: *XEventStub) -> *XExposeEvent unsafe {
    unsafe::reinterpret_cast(ptr::addr_of((*event).padding))
}

#[test]
fn cairo_it_up() {
    let dpy = XOpenDisplay(ptr::null());
    assert(ptr::is_not_null(dpy));
    let scr = XDefaultScreen(dpy);
    let rootwin = XRootWindow(dpy, scr);
    let win = XCreateSimpleWindow(
        dpy, rootwin, 1 as c_int, 1 as c_int, SIZEX, SIZEY, 0 as c_uint,
        XBlackPixel(dpy, scr), XBlackPixel(dpy, scr));
    str::as_c_str("test") {|cstr|
        XStoreName(dpy, win, cstr);
    }
    XSelectInput(dpy, win, ExposureMask | ButtonPressMask);
    XMapWindow(dpy, win);

    let cs = cairo_xlib_surface_create(
        dpy, win, XDefaultVisual(dpy, 0 as c_int),
        SIZEX as c_int, SIZEY as c_int);

    let e = {
        type_: 0 as c_int,
        padding: (
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0
        )
    };
    let ep = ptr::addr_of(e);

    let dt = CreateDrawTargetForCairoSurface(cs);

    loop unsafe {
        XNextEvent(dpy, unsafe::reinterpret_cast(ep));
        log(error, *ep);
        log(error, *xexpose(ep));
        if e.type_ == Expose && (*xexpose(ep)).count < 1 as c_int {
            paint(dt);
        } else if e.type_ == ButtonPress {
            break;
        } else {
            paint(dt);
        }
    }

    ReleaseDrawTarget(dt);
    cairo_surface_destroy(cs);
    XCloseDisplay(dpy);
}

fn paint(dt: DrawTargetRef) {
    log(error, "painting");
    let rect = {
        x: 200f as Float,
        y: 200f as Float,
        width: 100f as Float,
        height: 100f as Float
    };
    let color = {
        r: 0f as Float,
        g: 1f as Float,
        b: 0f as Float,
        a: 1f as Float
    };
    let pattern = CreateColorPattern(ptr::addr_of(color));
    DrawTargetFillRect(
        dt,
        ptr::addr_of(rect),
        unsafe { unsafe::reinterpret_cast(pattern) });
    ReleaseColorPattern(pattern);
}