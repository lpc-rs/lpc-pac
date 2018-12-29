#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::LUTERRAD {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = r" Value of the field"]
pub struct LUTERRADR {
    bits: u16,
}
impl LUTERRADR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 2:10 - It the LUT Error bit (below) is 1, this read-only field contains the address in AF Lookup Table RAM, at which the Acceptance Filter encountered an error in the content of the tables."]
    #[inline]
    pub fn luterrad(&self) -> LUTERRADR {
        let bits = {
            const MASK: u16 = 511;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        };
        LUTERRADR { bits }
    }
}
