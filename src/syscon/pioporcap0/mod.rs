#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::PIOPORCAP0 {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = r" Value of the field"]
pub struct PIOSTATR {
    bits: u32,
}
impl PIOSTATR {
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
    #[doc = "Bits 0:17 - State of PIO0_17 through PIO0_0 at power-on reset"]
    #[inline]
    pub fn piostat(&self) -> PIOSTATR {
        let bits = {
            const MASK: u32 = 262143;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u32
        };
        PIOSTATR { bits }
    }
}
