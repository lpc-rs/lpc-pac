#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::EEMSSIG {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = r" Value of the field"]
pub struct DATA_SIGR {
    bits: u16,
}
impl DATA_SIGR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct PARITY_SIGR {
    bits: u16,
}
impl PARITY_SIGR {
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
    #[doc = "Bits 0:15 - BIST 16-bit signature calculated from only the data bytes"]
    #[inline]
    pub fn data_sig(&self) -> DATA_SIGR {
        let bits = {
            const MASK: u16 = 65535;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        };
        DATA_SIGR { bits }
    }
    #[doc = "Bits 16:31 - BIST 16-bit signature calculated from only the parity bits of the data bytes"]
    #[inline]
    pub fn parity_sig(&self) -> PARITY_SIGR {
        let bits = {
            const MASK: u16 = 65535;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        };
        PARITY_SIGR { bits }
    }
}
