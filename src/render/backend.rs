use openxr as xr;
use std::ffi::{c_void, CString};
use std::ptr;
use x11::{glx, xlib};

pub struct Backend {
    context: glx::GLXContext,
    display: *mut xlib::Display,
    visual: *mut xlib::XVisualInfo,
    fb_config: glx::GLXFBConfig,
    drawable: x11::xlib::Drawable,
}

impl Backend {
    pub fn new() -> Self {
        let mut fbcount = 0;
        let visual_attribs = [0];

        let context_attribs = [
            glx::GLX_RGBA,
            glx::GLX_DEPTH_SIZE,
            24,
            glx::GLX_DOUBLEBUFFER,
            0,
        ];
        unsafe {
            let display = xlib::XOpenDisplay(ptr::null());
            let root = xlib::XDefaultRootWindow(display);
            let visual = glx::glXChooseVisual(display, 0, context_attribs.as_ptr() as *mut _);
            let context =
                glx::glXCreateContext(display, visual, 0 as glx::GLXContext, true as xlib::Bool);
            let fb_config = glx::glXChooseFBConfig(
                display,
                xlib::XDefaultScreen(display),
                visual_attribs.as_ptr(),
                &mut fbcount,
            );

            glx::glXMakeCurrent(display, root, context);

            Self {
                context,
                display,
                visual,
                fb_config: *fb_config,
                drawable: root,
            }
        }
    }
    pub unsafe fn xr_session_create_info(&self) -> xr::opengl::SessionCreateInfo {
        let visualid = { *self.visual }.visualid as u32;
        xr::opengl::SessionCreateInfo::Xlib {
            x_display: self.display as *mut _,
            glx_fb_config: self.fb_config as *mut _,
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
            x11::xlib::XCloseDisplay(self.display);
            x11::xlib::XFree(self.visual as *mut _);
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
        // NOTE: Use real dismensions
        (800, 600)
    }

    fn is_current(&self) -> bool {
        // Impl that
        true
    }

    unsafe fn make_current(&self) {
        glx::glXMakeCurrent(self.display, self.drawable, self.context);
    }
}
