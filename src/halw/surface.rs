extern crate gfx_hal as hal;
extern crate raw_window_handle;

use hal::{window::Surface as HalSurface, Instance as HalInstance};
use std::{
    cell::RefCell,
    fmt::{Debug, Formatter},
    mem::ManuallyDrop,
    ops::{Deref, DerefMut},
    rc::Rc,
};

use super::{Adapter, Backend, Gpu, Instance};

pub struct Surface<'a> {
    value: ManuallyDrop<<Backend as hal::Backend>::Surface>,
    instance: &'a Instance,
    adapter: Rc<RefCell<Adapter>>,
    gpu: Rc<RefCell<Gpu>>,
}

impl<'a> Surface<'a> {
    pub fn create(
        instance: &'a Instance,
        adapter: Rc<RefCell<Adapter>>,
        gpu: Rc<RefCell<Gpu>>,
        handle: &impl raw_window_handle::HasRawWindowHandle,
    ) -> Result<Self, hal::window::InitError> {
        let surface = unsafe { instance.create_surface(handle) }?;
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

    pub fn configure_swapchain(
        &mut self,
        config: hal::window::SwapchainConfig,
    ) -> Result<(), hal::window::CreationError> {
        use hal::window::PresentationSurface;
        unsafe {
            self.value
                .configure_swapchain(&self.gpu.borrow().device, config)
        }
    }
}

impl<'a> Drop for Surface<'a> {
    fn drop(&mut self) {
        unsafe {
            self.instance
                .destroy_surface(ManuallyDrop::take(&mut self.value));
        }
    }
}

impl<'a> Deref for Surface<'a> {
    type Target = <Backend as hal::Backend>::Surface;
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl<'a> DerefMut for Surface<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.value
    }
}

impl<'a> Debug for Surface<'a> {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "Surface {{ value: {:?} }}", self.value)
    }
}
