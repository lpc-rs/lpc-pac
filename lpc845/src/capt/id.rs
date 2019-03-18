#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::ID {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = r" Value of the field"]
pub struct APERTURER {
    bits: u8,
}
impl APERTURER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct MINOR_REVR {
    bits: u8,
}
impl MINOR_REVR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct MAJOR_REVR {
    bits: u8,
}
impl MAJOR_REVR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct IDR {
    bits: u16,
}
impl IDR {
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
    #[doc = "Bits 0:7 - Aperture: encoded as (aperture size/4K) -1, so 0x00 is a 4 K aperture."]
    #[inline]
    pub fn aperture(&self) -> APERTURER {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        APERTURER { bits }
    }
    #[doc = "Bits 8:11 - Minor revision of module implementation, starting at 0. Software compatibility is expected between minor revisions."]
    #[inline]
    pub fn minor_rev(&self) -> MINOR_REVR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        MINOR_REVR { bits }
    }
    #[doc = "Bits 12:15 - Major revision of module implementation, starting at 0. There may not be software compatibility between major revisions."]
    #[inline]
    pub fn major_rev(&self) -> MAJOR_REVR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        MAJOR_REVR { bits }
    }
    #[doc = "Bits 16:31 - 1 if is Timeout."]
    #[inline]
    pub fn id(&self) -> IDR {
        let bits = {
            const MASK: u16 = 65535;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        };
        IDR { bits }
    }
}
