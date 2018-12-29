#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::CTIME0 {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = r" Value of the field"]
pub struct SECONDSR {
    bits: u8,
}
impl SECONDSR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct MINUTESR {
    bits: u8,
}
impl MINUTESR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct HOURSR {
    bits: u8,
}
impl HOURSR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct DOWR {
    bits: u8,
}
impl DOWR {
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
    #[doc = "Bits 0:5 - Seconds value in the range of 0 to 59"]
    #[inline]
    pub fn seconds(&self) -> SECONDSR {
        let bits = {
            const MASK: u8 = 63;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        SECONDSR { bits }
    }
    #[doc = "Bits 8:13 - Minutes value in the range of 0 to 59"]
    #[inline]
    pub fn minutes(&self) -> MINUTESR {
        let bits = {
            const MASK: u8 = 63;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        MINUTESR { bits }
    }
    #[doc = "Bits 16:20 - Hours value in the range of 0 to 23"]
    #[inline]
    pub fn hours(&self) -> HOURSR {
        let bits = {
            const MASK: u8 = 31;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        HOURSR { bits }
    }
    #[doc = "Bits 24:26 - Day of week value in the range of 0 to 6"]
    #[inline]
    pub fn dow(&self) -> DOWR {
        let bits = {
            const MASK: u8 = 7;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        DOWR { bits }
    }
}
