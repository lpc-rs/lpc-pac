#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::FMSW0 {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = r" Value of the field"]
pub struct SW0_31_0R {
    bits: u32,
}
impl SW0_31_0R {
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
    #[doc = "Bits 0:31 - Word 0 of 128-bit signature (bits 31 to 0)."]
    #[inline]
    pub fn sw0_31_0(&self) -> SW0_31_0R {
        let bits = {
            const MASK: u32 = 4294967295;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u32
        };
        SW0_31_0R { bits }
    }
}
