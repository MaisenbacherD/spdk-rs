///! TODO
use std::{marker::PhantomData, ptr::NonNull};

use crate::libspdk::{
    spdk_io_channel,
    spdk_io_channel_iter,
    spdk_io_channel_iter_get_channel,
    spdk_rs_io_channel_get_ctx,
};

/// Wrapper for SPDK `spdk_io_channel` structure.
///
/// # Generic Arguments
///
/// * `ChannelData`: TODO
#[derive(Debug)]
pub struct IoChannel<ChannelData> {
    /// TODO
    inner: NonNull<spdk_io_channel>,
    /// TODO
    _cd: PhantomData<ChannelData>,
}

impl<ChannelData> IoChannel<ChannelData> {
    /// Returns a reference to the channel data instance that this I/O channel
    /// owns.
    pub fn channel_data(&self) -> &ChannelData {
        unsafe {
            &*(spdk_rs_io_channel_get_ctx(self.inner.as_ptr())
                as *mut ChannelData)
        }
    }

    /// Returns a mutable reference to the channel data instance that this I/O
    /// channel owns.
    pub fn channel_data_mut(&mut self) -> &mut ChannelData {
        unsafe {
            &mut *(spdk_rs_io_channel_get_ctx(self.inner.as_ptr())
                as *mut ChannelData)
        }
    }

    /// Makes a new `IoChannel` wrapper from a raw SPDK structure pointer.
    ///
    /// # Arguments
    ///
    /// * `ptr`: TODO
    pub(crate) fn from_ptr(ptr: *mut spdk_io_channel) -> Self {
        Self {
            inner: NonNull::new(ptr).unwrap(),
            _cd: Default::default(),
        }
    }

    /// Makes a new `IoChannel` wrapper from a raw `spdk_io_channel_iter`
    /// pointer.
    ///
    /// # Arguments
    ///
    /// * `ptr`: TODO
    pub fn from_iter(ptr: *mut spdk_io_channel_iter) -> Self {
        let io_chan = unsafe { spdk_io_channel_iter_get_channel(ptr) };
        Self::from_ptr(io_chan)
    }
}

impl<ChannelData> Clone for IoChannel<ChannelData> {
    fn clone(&self) -> Self {
        Self {
            inner: self.inner,
            _cd: Default::default(),
        }
    }
}
