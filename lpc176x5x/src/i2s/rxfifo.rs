#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::RXFIFO {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = r" Value of the field"]
pub struct I2SRXFIFOR {
    bits: u32,
}
impl I2SRXFIFOR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:31 - 8 x 32-bit transmit FIFO."]
    #[inline]
    pub fn i2srxfifo(&self) -> I2SRXFIFOR {
        let bits = {
            const MASK: u32 = 4294967295;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u32
        };
        I2SRXFIFOR { bits }
    }
}
