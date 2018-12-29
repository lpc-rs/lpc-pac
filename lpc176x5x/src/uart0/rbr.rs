#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::RBR {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = r" Value of the field"]
pub struct RBRR {
    bits: u8,
}
impl RBRR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:7 - The UARTn Receiver Buffer Register contains the oldest received byte in the UARTn Rx FIFO."]
    #[inline]
    pub fn rbr(&self) -> RBRR {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        RBRR { bits }
    }
}
