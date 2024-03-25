use std::ffi::CString;
use std::ptr;
use ash::extensions::ext::DebugUtils;
use ash::extensions::khr::{Surface, XlibSurface};
use ash::vk;
use winit::event::{Event, WindowEvent};
use winit::event_loop::{EventLoop};

const WINDOW_TITLE: &'static str = "Hexen";
const APPLICATION_VERSION: u32 = 0u32;
const ENGINE_VERSION: u32 = 0u32;
const API_VERSION: u32 = 0u32;

struct VulkanApp
{
    _entry: ash::Entry,
    instance: ash::Instance,
}

impl VulkanApp {

    pub fn new() -> VulkanApp {
        let entry = ash::Entry::linked();
        let instance = VulkanApp::create_instance(&entry);

        VulkanApp {
            _entry: entry,
            instance,
        }
    }

    fn init_window(event_loop: &EventLoop<()>) -> winit::window::Window {
        winit::window::Window::new( event_loop ).unwrap()
    }

    pub fn required_extension_names() -> Vec<*const i8> {
        vec![
            Surface::name().as_ptr(),
            XlibSurface::name().as_ptr(),
            DebugUtils::name().as_ptr(),
        ]
    }

    fn create_instance(entry: &ash::Entry) -> ash::Instance {

        let app_name = CString::new(WINDOW_TITLE).unwrap();
        let engine_name = CString::new(WINDOW_TITLE).unwrap();
        let app_info = vk::ApplicationInfo {
            s_type: vk::StructureType::APPLICATION_INFO,
            p_next: ptr::null(),
            p_application_name: app_name.as_ptr(),
            application_version: APPLICATION_VERSION,
            p_engine_name: engine_name.as_ptr(),
            engine_version: ENGINE_VERSION,
            api_version: API_VERSION,
        };

        let extension_names = Self::required_extension_names();

        let create_info = vk::InstanceCreateInfo {
            s_type: vk::StructureType::INSTANCE_CREATE_INFO,
            p_next: ptr::null(),
            flags: vk::InstanceCreateFlags::empty(),
            p_application_info: &app_info,
            pp_enabled_layer_names: ptr::null(),
            enabled_layer_count: 0,
            pp_enabled_extension_names: extension_names.as_ptr(),
            enabled_extension_count: extension_names.len() as u32,
        };

        let instance: ash::Instance = unsafe {
            entry
                .create_instance(&create_info, None)
                .expect("Failed to create instance!")
        };

        instance
    }

    fn draw_frame(&mut self) {

    }

    pub fn main_loop(mut self, event_loop: EventLoop<()> ) {
        event_loop.run( move |event, eventlooptarget| {

            match event {
                | Event::WindowEvent { event, .. } => {
                    match event {
                        | WindowEvent::CloseRequested => {
                            eventlooptarget.exit();
                        },
                        | WindowEvent::KeyboardInput { event, ..} => {
                            println!("Key pressed: {:?}", event.physical_key);
                        }
                        | WindowEvent::RedrawRequested => {
                            self.draw_frame();
                        },
                        _ => {
                        }
                    }
                },
                _ => {
                }
            }

        } ).unwrap();
    }

}

fn main() {
    let event_loop = EventLoop::new().unwrap();
    let _window = VulkanApp::init_window(&event_loop);

    let vulkan_app = VulkanApp::new();
    vulkan_app.main_loop(event_loop);
}