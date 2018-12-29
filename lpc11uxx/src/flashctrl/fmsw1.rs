#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::FMSW1 {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = r" Value of the field"]
pub struct SW1_63_32R {
    bits: u32,
}
impl SW1_63_32R {
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
    #[doc = "Bits 0:31 - Word 1 of 128-bit signature (bits 63 to 32)."]
    #[inline]
    pub fn sw1_63_32(&self) -> SW1_63_32R {
        let bits = {
            const MASK: u32 = 4294967295;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u32
        };
        SW1_63_32R { bits }
    }
}
