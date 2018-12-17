#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::FMSW2 {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = r" Value of the field"]
pub struct SW2_95_64R {
    bits: u32,
}
impl SW2_95_64R {
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
    #[doc = "Bits 0:31 - Word 2 of 128-bit signature (bits 95 to 64)."]
    #[inline]
    pub fn sw2_95_64(&self) -> SW2_95_64R {
        let bits = {
            const MASK: u32 = 4294967295;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u32
        };
        SW2_95_64R { bits }
    }
}
