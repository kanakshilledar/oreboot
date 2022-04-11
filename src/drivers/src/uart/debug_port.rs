use crate::model::{Driver, Result, NOT_IMPLEMENTED};
use crate::timer::hpet::HPET;
use consts::DeviceCtl;

pub struct DebugPort<D: Driver> {
    address: usize,
    d: D,
    timer: HPET,
}

impl<D: Driver> DebugPort<D> {
    pub fn new(address: usize, d: D) -> DebugPort<D> {
        DebugPort {
            address,
            d,
            timer: HPET::hpet0(),
        }
    }
}

impl<D: Driver> Driver for DebugPort<D> {
    fn init(&mut self) -> Result<()> {
        self.timer
            .enable()
            .map_err(|_| "DebugPort unusable: enable bit not set for HPET timer.")
    }

    // DebugPort can only be used to write, nothing here
    fn pread(&self, _data: &mut [u8], _offset: usize) -> Result<usize> {
        Ok(0)
    }

    // Just write out byte for byte :)
    fn pwrite(&mut self, data: &[u8], _offset: usize) -> Result<usize> {
        for &c in data {
            let mut s = [0u8; 1];
            s[0] = c;
            // 0.5 microseconds
            for _j in 0..125 {
                // shorter sleep time here so that it also works in 32 bit
                self.timer.sleep(4_000_000); // that's in fs
            }
            self.d.pwrite(&s, self.address).unwrap();
        }
        Ok(data.len())
    }

    fn ctl(&mut self, __d: DeviceCtl) -> Result<usize> {
        NOT_IMPLEMENTED
    }

    fn stat(&self, _data: &mut [u8]) -> Result<usize> {
        NOT_IMPLEMENTED
    }

    // Nothing to shut down here
    fn shutdown(&mut self) {}
}