#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::TIMER {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = r" Value of the field"]
pub struct VALUER {
    bits: u32,
}
impl VALUER {
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
    #[doc = "Bits 0:30 - Holds the current timer value of the down-counter. The initial value is loaded as IVALUE - 1 from the TIME_INTVALn register either at the end of the time interval if the LOAD bit in TIME_INTVALn is 0 and the timer is in repeat mode or immediately if LOAD = 1. When the timer is in idle state, reading this bit fields returns -1 (0x7FFF FFFF)."]
    #[inline]
    pub fn value(&self) -> VALUER {
        let bits = {
            const MASK: u32 = 2147483647;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u32
        };
        VALUER { bits }
    }
}
