#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::IDLE_CH {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = r" Value of the field"]
pub struct CHANR {
    bits: u8,
}
impl CHANR {
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
    #[doc = "Bits 4:7 - Idle channel. Reading the CHAN bits, returns the lowest idle timer channel. If all timer channels are running, CHAN = ."]
    #[inline]
    pub fn chan(&self) -> CHANR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        CHANR { bits }
    }
}
