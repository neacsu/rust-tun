//            DO WHAT THE FUCK YOU WANT TO PUBLIC LICENSE
//                    Version 2, December 2004
//
// Copyleft (ↄ) meh. <meh@schizofreni.co> | http://meh.schizofreni.co
//
// Everyone is permitted to copy and distribute verbatim or modified
// copies of this license document, and changing it is allowed as long
// as the name is changed.
//
//            DO WHAT THE FUCK YOU WANT TO PUBLIC LICENSE
//   TERMS AND CONDITIONS FOR COPYING, DISTRIBUTION AND MODIFICATION
//
//  0. You just DO WHAT THE FUCK YOU WANT TO.

//! Linux specific functionality.

pub mod sys;

mod device;
pub use self::device::Device;

use crate::configuration::Configuration;
use crate::error::Result;

/// FreeBSD-only interface configuration.
/// packet information not exists in FreeBSD, so the `Default` is in `derive` list.
#[derive(Copy, Clone, Default, Debug)]
pub struct PlatformConfig {
    pub(crate) packet_information: bool,
}

impl PlatformConfig {
    pub fn packet_information(&mut self, value: bool) -> &mut Self {
        assert!(!value);
        self.packet_information = value;
        self
    }
}

/// Create a TUN device with the given name.
pub fn create(configuration: &Configuration) -> Result<Device> {
    Device::new(configuration)
}