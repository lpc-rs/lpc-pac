#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::PLL0STAT {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = r" Value of the field"]
pub struct MSEL0R {
    bits: u16,
}
impl MSEL0R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct NSEL0R {
    bits: u8,
}
impl NSEL0R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct PLLE0_STATR {
    bits: bool,
}
impl PLLE0_STATR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
#[doc = r" Value of the field"]
pub struct PLLC0_STATR {
    bits: bool,
}
impl PLLC0_STATR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
#[doc = r" Value of the field"]
pub struct PLOCK0R {
    bits: bool,
}
impl PLOCK0R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:14 - Read-back for the PLL0 Multiplier value. This is the value currently used by PLL0, and is one less than the actual multiplier."]
    #[inline]
    pub fn msel0(&self) -> MSEL0R {
        let bits = {
            const MASK: u16 = 32767;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        };
        MSEL0R { bits }
    }
    #[doc = "Bits 16:23 - Read-back for the PLL0 Pre-Divider value. This is the value currently used by PLL0, and is one less than the actual divider."]
    #[inline]
    pub fn nsel0(&self) -> NSEL0R {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        NSEL0R { bits }
    }
    #[doc = "Bit 24 - Read-back for the PLL0 Enable bit. This bit reflects the state of the PLEC0 bit in PLL0CON after a valid PLL0 feed. When one, PLL0 is currently enabled. When zero, PLL0 is turned off. This bit is automatically cleared when Power-down mode is entered."]
    #[inline]
    pub fn plle0_stat(&self) -> PLLE0_STATR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        PLLE0_STATR { bits }
    }
    #[doc = "Bit 25 - Read-back for the PLL0 Connect bit. This bit reflects the state of the PLLC0 bit in PLL0CON after a valid PLL0 feed. When PLLC0 and PLLE0 are both one, PLL0 is connected as the clock source for the CPU. When either PLLC0 or PLLE0 is zero, PLL0 is bypassed. This bit is automatically cleared when Power-down mode is entered."]
    #[inline]
    pub fn pllc0_stat(&self) -> PLLC0_STATR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 25;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        PLLC0_STATR { bits }
    }
    #[doc = "Bit 26 - Reflects the PLL0 Lock status. When zero, PLL0 is not locked. When one, PLL0 is locked onto the requested frequency. See text for details."]
    #[inline]
    pub fn plock0(&self) -> PLOCK0R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 26;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        PLOCK0R { bits }
    }
}
