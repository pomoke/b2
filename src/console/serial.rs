use alloc::boxed::Box;
use anyhow::{anyhow, Result};
use uefi::{prelude::BootServices, proto::console::serial::Serial, table::boot::ScopedProtocol};

use crate::io::Stream;
pub trait SerialConsole: Stream {
    /// Set baud rate of serial port.
    ///
    /// No plan to support non 8-bit transports.
    /// No parity and hard flow control as well.
    fn set_baud_rate(baud: i32) -> Result<()>;
}

/// EFI Serial Port
pub struct EFISerial<'a> {
    handler: ScopedProtocol<'a, Serial<'a>>,
}

impl<'a> EFISerial<'a> {
    /// Call init() only once, do not call twice!
    unsafe fn init() {}

    /// Convert serial console handle to the struct.
    fn from_efi_serial(serial: ScopedProtocol<'a, Serial<'a>>) -> Result<Self> {
        Ok(Self { handler: serial })
    }

    /// Acquire serial console handle from boot service.
    fn from_boot_service(bs: &'a mut BootServices) -> Result<Self> {
        let serial_handle = bs
            .get_handle_for_protocol::<Serial>()
            .map_err(|x| anyhow!("Failed to get serial handle, status {}", x.status()))?;
        let serial = bs
            .open_protocol_exclusive::<Serial>(serial_handle)
            .map_err(|x| anyhow!("Failed to open serial protocol, status {}", x.status()))?;
        Ok(Self { handler: serial })
    }
}

impl<'a> Stream for EFISerial<'a> {
    fn read(&mut self, buf: &mut [u8]) -> Result<i32> {
        Err(anyhow!("todo!"))
    }
    fn write(&mut self, buf: &[u8]) -> Result<i32> {
        Err(anyhow!("todo!"))
    }
}

impl<'a> SerialConsole for EFISerial<'a> {
    fn set_baud_rate(baud: i32) -> Result<()> {
        Err(anyhow!("todo!"))
    }
}
