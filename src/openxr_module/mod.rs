use openxr as xr;

const GL_RGBA8: u32 = 0x8058;

pub struct OpenXR {
    pub entry: xr::Entry,
    pub instance: xr::Instance,
    pub session: xr::Session<xr::OpenGL>,
    pub spaces: (Option<xr::Space>, Option<xr::Space>),
    pub session_state: xr::SessionState,
    frame_stream: xr::FrameStream<xr::OpenGL>,
    predicted_display_time: xr::Time,
    swap_chain: Option<xr::Swapchain<xr::OpenGL>>
}

impl OpenXR {
    pub fn new(backend: &crate::render::backend::Backend) -> Self{
        let entry = xr::Entry::linked();
        let extensions = entry.enumerate_extensions().expect("Cannot enumerate extensions");
        let app_info = xr::ApplicationInfo::new().application_name("SlashMania");
        if !extensions.khr_opengl_enable {
            panic!("XR: OpenGL extension unsupported");
        }
        let extension_set = xr::ExtensionSet {
            khr_opengl_enable: true,
            ..Default::default()
        };
        let instance = entry.create_instance(app_info, &extension_set,).unwrap();

        let instance_props = instance.properties().expect("Cannot load instance props");
        println!("loaded instance: {} v{}", instance_props.runtime_name, instance_props.runtime_version);

        let system = instance.system(xr::FormFactor::HEAD_MOUNTED_DISPLAY).unwrap();

        let info = backend.xr_session_create_info();
        let (session, frame_stream) = unsafe{
            instance.create_session(system, &info).unwrap()
        };
        session.begin(xr::ViewConfigurationType::PRIMARY_STEREO).unwrap();

        let spaces = init_spaces(&session);

        OpenXR{
            entry,
            instance,
            session,
            spaces,
            session_state: xr::SessionState::UNKNOWN,
            frame_stream,
            predicted_display_time: xr::Time::from_raw(0),
            swap_chain: None
        }
    }

    pub fn update(&mut self) {
        let mut buffer = xr::EventDataBuffer::new();
        while let Some(event) = self.instance.poll_event(&mut buffer).unwrap() {
            use xr::Event::*;
            match event {
                SessionStateChanged(session_change) => {
                    println!("session state changed to {:?} at t={:?}", session_change.state(), session_change.time());
                    self.session_state = session_change.state();
                    match session_change.state() {
                        xr::SessionState::EXITING | xr::SessionState::LOSS_PENDING => self.finish_session(),
                        xr::SessionState::RUNNING => {
                            if self.swap_chain.is_none() {
                                self.create_swapchain()
                            }
                        },
                        _ => {}
                    }
                },
                _ => {
                    println!("unhandled event");
                }
            }
        }
    }
    pub fn create_swapchain(&mut self){
        let swapchain_formats = self.session.enumerate_swapchain_formats().unwrap();
        if !swapchain_formats.contains(&GL_RGBA8) {
            for format in swapchain_formats{
                println!("Format: {:04x}", format);
            }
            panic!("XR: Cannot use OpenGL GL_RGBA8 swapchain format");
        }

        let swapchain_create_info: xr::SwapchainCreateInfo<xr::OpenGL> = xr::SwapchainCreateInfo{
            create_flags: xr::SwapchainCreateFlags::EMPTY,
            usage_flags: xr::SwapchainUsageFlags::COLOR_ATTACHMENT,
            format: GL_RGBA8,
            sample_count: 1,
            // NOTE: Change resolution to correct
            width: 800,
            height: 600,
            face_count: 1,
            array_size: 1,
            mip_count: 1
        };
        self.swap_chain = Some(self.session.create_swapchain(&swapchain_create_info).unwrap());
    }
    pub fn frame_stream_begin(&mut self){
        let state = self.frame_stream.wait().unwrap();
        self.predicted_display_time = state.predicted_display_time;
        self.frame_stream.begin().unwrap();
    }
    pub fn frame_stream_end(&mut self){
        let swap_chain = self.swap_chain.as_ref().unwrap();
        let eye_rect = xr::Rect2Di{
            offset: xr::Offset2Di{
                x: 0,
                y: 0
            },
            // NOTE: Use actual resolution
            extent: xr::Extent2Di{
                width: 800,
                height: 600
            }
        };
        let time = self.predicted_display_time;
        // NOTE: Probably we should move it away from frame_stream_end

        let left_subimage: xr::SwapchainSubImage<xr::OpenGL> = openxr::SwapchainSubImage::new()
            .swapchain(swap_chain)
            .image_rect(eye_rect);
        let right_subimage: xr::SwapchainSubImage<xr::OpenGL> = openxr::SwapchainSubImage::new()
            .swapchain(swap_chain)
            .image_rect(eye_rect);

        let projection_view_left = xr::CompositionLayerProjectionView::new().sub_image(left_subimage);
        let projection_view_right = xr::CompositionLayerProjectionView::new().sub_image(right_subimage);
        let views = [projection_view_left, projection_view_right];
        let projection = xr::CompositionLayerProjection::new().views(&views);
        self.frame_stream.end(time, xr::EnvironmentBlendMode::OPAQUE, &[&projection]).unwrap();
    }
    pub fn get_swapchain_image(&mut self) -> Option<u32>{
        let swapchain = self.swap_chain.as_mut()?;
        let images = swapchain.enumerate_images().unwrap();
        let image_id = swapchain.acquire_image().unwrap();
        swapchain.wait_image(xr::Duration::INFINITE).unwrap();
        let image = images[image_id as usize];
        Some(image)
    }
    pub fn release_swapchain_image(&mut self){
        let swapchain = self.swap_chain.as_mut().unwrap();
        swapchain.release_image().unwrap();
    }
    pub fn finish_session(&self) {

    }
}

pub fn init_spaces(session: &xr::Session<xr::OpenGL>) -> (Option<xr::Space>, Option<xr::Space>){
    let space_tys = session.enumerate_reference_spaces().unwrap();
    let has_stage = space_tys.contains(&xr::ReferenceSpaceType::STAGE);
    let has_view = space_tys.contains(&xr::ReferenceSpaceType::VIEW);

    let stage = if has_stage {
        Some(
            session
                .create_reference_space(
                    xr::ReferenceSpaceType::STAGE,
                    xr::Posef {
                        position: xr::Vector3f {
                            x: 0.0,
                            y: 0.0,
                            z: 0.0,
                        },
                        orientation: xr::Quaternionf {
                            w: 1.0,
                            x: 0.0,
                            y: 0.0,
                            z: 0.0,
                        },
                    },
                )
                .unwrap(),
        )
    } else {
        None
    };

    let view = if has_view {
        Some(
            session
                .create_reference_space(
                    xr::ReferenceSpaceType::VIEW,
                    xr::Posef {
                        position: xr::Vector3f {
                            x: 0.0,
                            y: 0.0,
                            z: 0.0,
                        },
                        orientation: xr::Quaternionf {
                            w: 1.0,
                            x: 0.0,
                            y: 0.0,
                            z: 0.0,
                        },
                    },
                )
                .unwrap(),
        )
    } else {
        None
    };

    return (stage, view)
}
