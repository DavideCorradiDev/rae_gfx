extern crate gfx_hal as hal;
extern crate raw_window_handle;

use hal::{
    window::{PresentationSurface as HalPresentationSurface, Surface as HalSurface},
    Instance as HalInstance,
};
use std::{
    cell::RefCell,
    fmt::{Debug, Formatter},
    mem::ManuallyDrop,
    ops::{Deref, DerefMut},
    rc::Rc,
};

use super::{Adapter, Backend, Gpu, Instance};

pub struct Surface {
    value: ManuallyDrop<<Backend as hal::Backend>::Surface>,
    instance: Rc<RefCell<Instance>>,
    adapter: Rc<RefCell<Adapter>>,
    gpu: Rc<RefCell<Gpu>>,
}

impl Surface {
    pub fn create(
        instance: Rc<RefCell<Instance>>,
        adapter: Rc<RefCell<Adapter>>,
        gpu: Rc<RefCell<Gpu>>,
        handle: &impl raw_window_handle::HasRawWindowHandle,
    ) -> Result<Self, hal::window::InitError> {
        let surface = unsafe { instance.borrow().create_surface(handle) }?;
        Ok(Self {
            value: ManuallyDrop::new(surface),
            instance,
            adapter,
            gpu,
        })
    }

    pub fn capabilities(&self) -> hal::window::SurfaceCapabilities {
        self.value
            .capabilities(&self.adapter.borrow().physical_device)
    }

    pub fn supported_formats(&self) -> Option<Vec<hal::format::Format>> {
        self.value
            .supported_formats(&self.adapter.borrow().physical_device)
    }

    pub fn configure_swapchain(
        &mut self,
        config: hal::window::SwapchainConfig,
    ) -> Result<(), hal::window::CreationError> {
        unsafe {
            self.value
                .configure_swapchain(&self.gpu.borrow().device, config)
        }
    }

    pub fn unconfigure_swapchain(&mut self) {
        unsafe { self.value.unconfigure_swapchain(&self.gpu.borrow().device) }
    }
}

impl Drop for Surface {
    fn drop(&mut self) {
        self.gpu.borrow().wait_idle().unwrap();
        unsafe {
            self.instance
                .borrow()
                .destroy_surface(ManuallyDrop::take(&mut self.value));
        }
    }
}

impl Deref for Surface {
    type Target = <Backend as hal::Backend>::Surface;
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl DerefMut for Surface {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.value
    }
}

impl Debug for Surface {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "Surface {{ value: {:?} }}", self.value)
    }
}
