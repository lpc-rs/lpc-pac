#[doc = "Reader of register PLL0STAT"]
pub type R = crate::R<u32, super::PLL0STAT>;
#[doc = "Reader of field `MSEL0`"]
pub type MSEL0_R = crate::R<u16, u16>;
#[doc = "Reader of field `NSEL0`"]
pub type NSEL0_R = crate::R<u8, u8>;
#[doc = "Reader of field `PLLE0_STAT`"]
pub type PLLE0_STAT_R = crate::R<bool, bool>;
#[doc = "Reader of field `PLLC0_STAT`"]
pub type PLLC0_STAT_R = crate::R<bool, bool>;
#[doc = "Reader of field `PLOCK0`"]
pub type PLOCK0_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bits 0:14 - Read-back for the PLL0 Multiplier value. This is the value currently used by PLL0, and is one less than the actual multiplier."]
    #[inline(always)]
    pub fn msel0(&self) -> MSEL0_R {
        MSEL0_R::new((self.bits & 0x7fff) as u16)
    }
    #[doc = "Bits 16:23 - Read-back for the PLL0 Pre-Divider value. This is the value currently used by PLL0, and is one less than the actual divider."]
    #[inline(always)]
    pub fn nsel0(&self) -> NSEL0_R {
        NSEL0_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bit 24 - Read-back for the PLL0 Enable bit. This bit reflects the state of the PLEC0 bit in PLL0CON after a valid PLL0 feed. When one, PLL0 is currently enabled. When zero, PLL0 is turned off. This bit is automatically cleared when Power-down mode is entered."]
    #[inline(always)]
    pub fn plle0_stat(&self) -> PLLE0_STAT_R {
        PLLE0_STAT_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Read-back for the PLL0 Connect bit. This bit reflects the state of the PLLC0 bit in PLL0CON after a valid PLL0 feed. When PLLC0 and PLLE0 are both one, PLL0 is connected as the clock source for the CPU. When either PLLC0 or PLLE0 is zero, PLL0 is bypassed. This bit is automatically cleared when Power-down mode is entered."]
    #[inline(always)]
    pub fn pllc0_stat(&self) -> PLLC0_STAT_R {
        PLLC0_STAT_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - Reflects the PLL0 Lock status. When zero, PLL0 is not locked. When one, PLL0 is locked onto the requested frequency. See text for details."]
    #[inline(always)]
    pub fn plock0(&self) -> PLOCK0_R {
        PLOCK0_R::new(((self.bits >> 26) & 0x01) != 0)
    }
}
