pub mod xrmath;

use openxr as xr;

const GL_SRGB8_ALPHA8: u32 = 0x8C43;

pub struct Swapchain {
    pub swapchain: Option<xr::Swapchain<xr::OpenGL>>,
    pub resolution: (u32, u32),
}
impl Swapchain {
    fn empty() -> Self {
        Self {
            swapchain: None,
            resolution: (800, 800),
        }
    }
    fn new_from_session(
        session: &xr::Session<xr::OpenGL>,
        instance: &xr::Instance,
        system: openxr::SystemId,
    ) -> Self {
        let view_configuration_views = instance
            .enumerate_view_configuration_views(system, xr::ViewConfigurationType::PRIMARY_STEREO)
            .unwrap();
        let resolution = (
            view_configuration_views[0].recommended_image_rect_width,
            view_configuration_views[0].recommended_image_rect_height,
        );

        let sample_count = view_configuration_views[0].recommended_swapchain_sample_count;

        let swapchain_formats = session.enumerate_swapchain_formats().unwrap();
        if !swapchain_formats.contains(&GL_SRGB8_ALPHA8) {
            panic!("XR: Cannot use OpenGL GL_SRGB8_ALPHA8 swapchain format");
        }
        let swapchain_create_info: xr::SwapchainCreateInfo<xr::OpenGL> = xr::SwapchainCreateInfo {
            create_flags: xr::SwapchainCreateFlags::EMPTY,
            usage_flags: xr::SwapchainUsageFlags::COLOR_ATTACHMENT
                | xr::SwapchainUsageFlags::SAMPLED,
            format: GL_SRGB8_ALPHA8,
            sample_count: sample_count,
            width: resolution.0,
            height: resolution.1,
            face_count: 1,
            array_size: 2,
            mip_count: 1,
        };

        let swapchain = session.create_swapchain(&swapchain_create_info).unwrap();

        Self {
            swapchain: Some(swapchain),
            resolution,
        }
    }
    pub fn is_initialized(&self) -> bool {
        self.swapchain.is_none()
    }
    pub fn get_swapchain(&mut self) -> Option<&mut xr::Swapchain<xr::OpenGL>> {
        self.swapchain.as_mut()
    }
    pub fn get_images(&mut self) -> Option<u32> {
        let swapchain = self.get_swapchain();
        let swapchain = swapchain?;
        let swapchain_image = get_swapchain_image(swapchain);
        Some(swapchain_image)
    }
    pub fn get_subimages(
        &mut self,
    ) -> (
        xr::SwapchainSubImage<xr::OpenGL>,
        xr::SwapchainSubImage<xr::OpenGL>,
    ) {
        let resolution = self.resolution;

        let eye_rect_left = xr::Rect2Di {
            offset: xr::Offset2Di { x: 0, y: 0 },
            extent: xr::Extent2Di {
                width: resolution.0 as i32,
                height: resolution.1 as i32,
            },
        };
        let eye_rect_right = xr::Rect2Di {
            offset: xr::Offset2Di { x: 0, y: 0 },
            extent: xr::Extent2Di {
                width: resolution.0 as i32,
                height: resolution.1 as i32,
            },
        };
        let left_subimage: xr::SwapchainSubImage<xr::OpenGL> = openxr::SwapchainSubImage::new()
            .swapchain(&self.swapchain.as_ref().unwrap())
            .image_array_index(0)
            .image_rect(eye_rect_left);
        let right_subimage: xr::SwapchainSubImage<xr::OpenGL> = openxr::SwapchainSubImage::new()
            .swapchain(&self.swapchain.as_ref().unwrap())
            .image_array_index(1)
            .image_rect(eye_rect_right);
        (left_subimage, right_subimage)
    }
    pub fn release_images(&mut self) {
        let swapchain = self.get_swapchain().unwrap();
        swapchain.release_image().unwrap();
    }
}

pub struct OpenXR {
    //entry: xr::Entry,
    instance: xr::Instance,
    session: xr::Session<xr::OpenGL>,
    system: openxr::SystemId,
    pub swapchain: Swapchain,
    pub spaces: (Option<xr::Space>, Option<xr::Space>),
    pub session_state: xr::SessionState,
    pub views: Vec<xr::View>,
    frame_stream: xr::FrameStream<xr::OpenGL>,
    predicted_display_time: xr::Time,
}

impl OpenXR {
    pub fn new(backend: &mut crate::render::backend::Backend) -> Self {
        let entry = xr::Entry::linked();

        let extensions = entry
            .enumerate_extensions()
            .expect("Cannot enumerate extensions");
        let app_info = xr::ApplicationInfo::new().application_name("SlashMania");
        if !extensions.khr_opengl_enable {
            panic!("XR: OpenGL extension unsupported");
        }
        let extension_set = xr::ExtensionSet {
            khr_opengl_enable: true,
            ..Default::default()
        };
        let instance = entry.create_instance(app_info, &extension_set).unwrap();

        let instance_props = instance.properties().expect("Cannot load instance props");
        println!(
            "loaded instance: {} v{}",
            instance_props.runtime_name, instance_props.runtime_version
        );

        let system = instance
            .system(xr::FormFactor::HEAD_MOUNTED_DISPLAY)
            .unwrap();

        let info = unsafe { backend.xr_session_create_info() };
        let (session, frame_stream) = unsafe { instance.create_session(system, &info).unwrap() };
        session
            .begin(xr::ViewConfigurationType::PRIMARY_STEREO)
            .unwrap();

        let spaces = init_spaces(&session);

        let view_configuration_views = instance
            .enumerate_view_configuration_views(system, xr::ViewConfigurationType::PRIMARY_STEREO)
            .unwrap();
        let resolution = (
            view_configuration_views[0].recommended_image_rect_width,
            view_configuration_views[0].recommended_image_rect_height,
        );
        backend.dimmensions = resolution;

        OpenXR {
            //entry,
            instance,
            session,
            spaces,
            system,
            session_state: xr::SessionState::UNKNOWN,
            frame_stream,
            predicted_display_time: xr::Time::from_raw(0),
            swapchain: Swapchain::empty(),
            views: Vec::with_capacity(4),
        }
    }

    pub fn update(&mut self) {
        if self.swapchain.is_initialized() {
            let view_configuration_views = self
                .instance
                .enumerate_view_configuration_views(
                    self.system,
                    xr::ViewConfigurationType::PRIMARY_STEREO,
                )
                .unwrap();
            let resolution = (
                view_configuration_views[0].recommended_image_rect_width,
                view_configuration_views[0].recommended_image_rect_height,
            );
            if resolution != self.swapchain.resolution {
                self.swapchain.resolution = resolution;
                self.recreate_swapchain();
            }
        }

        let mut buffer = xr::EventDataBuffer::new();
        while let Some(event) = self.instance.poll_event(&mut buffer).unwrap() {
            use xr::Event::*;
            match event {
                SessionStateChanged(session_change) => {
                    self.session_state = session_change.state();
                    match session_change.state() {
                        xr::SessionState::EXITING | xr::SessionState::LOSS_PENDING => {
                            self.finish_session()
                        }
                        xr::SessionState::RUNNING => {
                            if self.swapchain.is_initialized() {
                                self.recreate_swapchain()
                            }
                        }
                        _ => {}
                    }
                }
                _ => {
                    println!("unhandled event");
                }
            }
        }
        let (_, views) = self
            .session
            .locate_views(self.predicted_display_time, self.spaces.0.as_ref().unwrap())
            .unwrap();
        self.views = views;
    }
    pub fn recreate_swapchain(&mut self) {
        self.swapchain = Swapchain::new_from_session(&self.session, &self.instance, self.system);
    }
    pub fn frame_stream_begin(&mut self) {
        let state = self.frame_stream.wait().unwrap();
        self.predicted_display_time = state.predicted_display_time;
        self.frame_stream.begin().unwrap();
    }
    pub fn frame_stream_end(&mut self) {
        let subimages = self.swapchain.get_subimages();
        let projection_view_left = xr::CompositionLayerProjectionView::new()
            .pose(self.views[0].pose)
            .fov(self.views[0].fov)
            .sub_image(subimages.0);
        let projection_view_right = xr::CompositionLayerProjectionView::new()
            .pose(self.views[1].pose)
            .fov(self.views[1].fov)
            .sub_image(subimages.1);
        let proj_views = [projection_view_left, projection_view_right];
        let projection = xr::CompositionLayerProjection::new().views(&proj_views);
        self.frame_stream
            .end(
                self.predicted_display_time,
                xr::EnvironmentBlendMode::OPAQUE,
                &[&projection],
            )
            .unwrap();
    }
    pub fn finish_session(&self) {}
}

pub fn init_spaces(session: &xr::Session<xr::OpenGL>) -> (Option<xr::Space>, Option<xr::Space>) {
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

    return (stage, view);
}

pub fn get_swapchain_image(swapchain: &mut xr::Swapchain<xr::OpenGL>) -> u32 {
    let images = swapchain.enumerate_images().unwrap();
    let image_id = swapchain.acquire_image().unwrap();
    swapchain.wait_image(xr::Duration::INFINITE).unwrap();
    let image = images[image_id as usize];
    image
}
