use openxr as xr;
use std::ffi::{c_void, CString};
use std::mem;
use std::os::raw::c_int;
use std::ptr;
use std::ptr::null_mut;
use x11::{glx, glx::arb, xlib};

type GlXcreateContextAttribsArb = unsafe extern "C" fn(
    dpy: *mut xlib::Display,
    fbc: glx::GLXFBConfig,
    share_context: glx::GLXContext,
    direct: xlib::Bool,
    attribs: *const c_int,
) -> glx::GLXContext;

pub struct Backend {
    pub context: glx::GLXContext,
    display: *mut xlib::Display,
    visual: *mut xlib::XVisualInfo,
    fb_config: *mut glx::GLXFBConfig,
    drawable: x11::xlib::Drawable,
    pub dimmensions: (u32, u32),
}

impl Backend {
    pub fn new() -> Self {
        let mut fbcount = 0;

        let attr = [
            glx::GLX_RGBA,
            glx::GLX_DEPTH_SIZE,
            24,
            glx::GLX_DOUBLEBUFFER,
            0,
        ];

        let visual_attribs = [0];

        let context_attribs = [
            arb::GLX_CONTEXT_MAJOR_VERSION_ARB,
            3,
            arb::GLX_CONTEXT_MINOR_VERSION_ARB,
            0,
            0,
        ];

        unsafe {
            let c_proc_name = CString::new("glXCreateContextAttribsARB").unwrap();
            let proc_addr = glx::glXGetProcAddress(c_proc_name.as_ptr() as *const u8);
            let glx_create_context_attribs =
                mem::transmute::<_, GlXcreateContextAttribsArb>(proc_addr);

            let display = xlib::XOpenDisplay(ptr::null());
            let root = xlib::XDefaultRootWindow(display);
            let visual = glx::glXChooseVisual(display, 0, attr.as_ptr() as *mut _);
            let fb_config = glx::glXChooseFBConfig(
                display,
                xlib::XDefaultScreen(display),
                visual_attribs.as_ptr(),
                &mut fbcount,
            );

            let context = glx_create_context_attribs(
                display,
                *fb_config,
                null_mut(),
                xlib::True,
                &context_attribs[0] as *const c_int,
            );
            if context.is_null() {
                panic!("glXCreateContextAttribsARB failed")
            }
            glx::glXMakeCurrent(display, root, context);

            Self {
                context,
                display,
                visual,
                fb_config,
                drawable: root,
                dimmensions: (800, 600),
            }
        }
    }
    pub unsafe fn xr_session_create_info(&self) -> xr::opengl::SessionCreateInfo {
        let visualid = { *self.visual }.visualid as u32;
        xr::opengl::SessionCreateInfo::Xlib {
            x_display: self.display as *mut _,
            glx_fb_config: *self.fb_config as *mut _,
            glx_drawable: self.drawable,
            visualid: visualid,
            glx_context: self.context as *mut _,
        }
    }
}

impl Drop for Backend {
    fn drop(&mut self) {
        unsafe {
            x11::xlib::XFree(self.fb_config as *mut _);
            x11::xlib::XFree(self.visual as *mut _);
            x11::xlib::XCloseDisplay(self.display);
        }
    }
}

unsafe impl glium::backend::Backend for Backend {
    fn swap_buffers(&self) -> Result<(), glium::SwapBuffersError> {
        unsafe {
            x11::glx::glXSwapBuffers(self.display, self.drawable);
        }
        Ok(())
    }

    unsafe fn get_proc_address(&self, symbol: &str) -> *const c_void {
        let addr = CString::new(symbol.as_bytes()).unwrap();
        let addr = addr.as_ptr();
        let proc_addr = glx::glXGetProcAddressARB(addr as *const _);
        match proc_addr {
            Some(proc_addr) => proc_addr as *const _,
            _ => ptr::null(),
        }
    }

    fn get_framebuffer_dimensions(&self) -> (u32, u32) {
        self.dimmensions
    }

    fn is_current(&self) -> bool {
        true
    }

    unsafe fn make_current(&self) {
        glx::glXMakeCurrent(self.display, self.drawable, self.context);
    }
}
