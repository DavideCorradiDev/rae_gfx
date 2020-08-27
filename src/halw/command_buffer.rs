extern crate gfx_hal as hal;

use hal::pool::CommandPool as HalCommandPool;
use std::{
    cell::RefCell,
    fmt::{Debug, Formatter},
    mem::ManuallyDrop,
    ops::{Deref, DerefMut},
    rc::Rc,
};

use super::{Backend, CommandPool};

pub struct CommandBuffer {
    value: ManuallyDrop<<Backend as hal::Backend>::CommandBuffer>,
    pool: Rc<RefCell<CommandPool>>,
}

impl CommandBuffer {
    pub fn allocate_one(pool: Rc<RefCell<CommandPool>>, level: hal::command::Level) -> Self {
        let buffer = unsafe { pool.borrow_mut().allocate_one(level) };
        Self {
            value: ManuallyDrop::new(buffer),
            pool,
        }
    }

    pub fn allocate(
        pool: Rc<RefCell<CommandPool>>,
        level: hal::command::Level,
        num: usize,
    ) -> Vec<Self> {
        let mut list = Vec::with_capacity(num);
        unsafe { pool.borrow_mut().allocate(num, level, &mut list) };
        list.into_iter()
            .map(|buf| Self {
                value: ManuallyDrop::new(buf),
                pool: Rc::clone(&pool),
            })
            .collect()
    }
}

impl Drop for CommandBuffer {
    fn drop(&mut self) {
        println!("*** Dropping Command Buffer {:?}", self);
        unsafe {
            self.pool
                .borrow_mut()
                .free(std::iter::once(ManuallyDrop::take(&mut self.value)));
        }
    }
}

impl Deref for CommandBuffer {
    type Target = <Backend as hal::Backend>::CommandBuffer;
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl DerefMut for CommandBuffer {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.value
    }
}

impl Debug for CommandBuffer {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "CommandBuffer {{ value: {:?} }}", self.value)
    }
}
