#[doc = "Reader of register MCFG"]
pub type R = crate::R<u32, super::MCFG>;
#[doc = "Writer for register MCFG"]
pub type W = crate::W<u32, super::MCFG>;
#[doc = "Register MCFG `reset()`'s with value 0"]
impl crate::ResetValue for super::MCFG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SCANINC`"]
pub type SCANINC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SCANINC`"]
pub struct SCANINC_W<'a> {
    w: &'a mut W,
}
impl<'a> SCANINC_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
#[doc = "Reader of field `SUPPPREAMBLE`"]
pub type SUPPPREAMBLE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SUPPPREAMBLE`"]
pub struct SUPPPREAMBLE_W<'a> {
    w: &'a mut W,
}
impl<'a> SUPPPREAMBLE_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "Reader of field `CLOCKSEL`"]
pub type CLOCKSEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CLOCKSEL`"]
pub struct CLOCKSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> CLOCKSEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 2)) | (((value as u32) & 0x0f) << 2);
        self.w
    }
}
#[doc = "Reader of field `RESETMIIMGMT`"]
pub type RESETMIIMGMT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RESETMIIMGMT`"]
pub struct RESETMIIMGMT_W<'a> {
    w: &'a mut W,
}
impl<'a> RESETMIIMGMT_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 15)) | (((value as u32) & 0x01) << 15);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - SCAN INCREMENT. Set this bit to cause the MII Management hardware to perform read cycles across a range of PHYs. When set, the MII Management hardware will perform read cycles from address 1 through the value set in PHY ADDRESS\\[4:0\\]. Clear this bit to allow continuous reads of the same PHY."]
    #[inline(always)]
    pub fn scaninc(&self) -> SCANINC_R {
        SCANINC_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - SUPPRESS PREAMBLE. Set this bit to cause the MII Management hardware to perform read/write cycles without the 32-bit preamble field. Clear this bit to cause normal cycles to be performed. Some PHYs support suppressed preamble."]
    #[inline(always)]
    pub fn supppreamble(&self) -> SUPPPREAMBLE_R {
        SUPPPREAMBLE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bits 2:5 - CLOCK SELECT. This field is used by the clock divide logic in creating the MII Management Clock (MDC) which IEEE 802.3u defines to be no faster than 2.5 MHz. Some PHYs support clock rates up to 12.5 MHz, however. The AHB bus clock (HCLK) is divided by the specified amount. Refer to Table 160 below for the definition of values for this field."]
    #[inline(always)]
    pub fn clocksel(&self) -> CLOCKSEL_R {
        CLOCKSEL_R::new(((self.bits >> 2) & 0x0f) as u8)
    }
    #[doc = "Bit 15 - RESET MII MGMT. This bit resets the MII Management hardware."]
    #[inline(always)]
    pub fn resetmiimgmt(&self) -> RESETMIIMGMT_R {
        RESETMIIMGMT_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SCAN INCREMENT. Set this bit to cause the MII Management hardware to perform read cycles across a range of PHYs. When set, the MII Management hardware will perform read cycles from address 1 through the value set in PHY ADDRESS\\[4:0\\]. Clear this bit to allow continuous reads of the same PHY."]
    #[inline(always)]
    pub fn scaninc(&mut self) -> SCANINC_W {
        SCANINC_W { w: self }
    }
    #[doc = "Bit 1 - SUPPRESS PREAMBLE. Set this bit to cause the MII Management hardware to perform read/write cycles without the 32-bit preamble field. Clear this bit to cause normal cycles to be performed. Some PHYs support suppressed preamble."]
    #[inline(always)]
    pub fn supppreamble(&mut self) -> SUPPPREAMBLE_W {
        SUPPPREAMBLE_W { w: self }
    }
    #[doc = "Bits 2:5 - CLOCK SELECT. This field is used by the clock divide logic in creating the MII Management Clock (MDC) which IEEE 802.3u defines to be no faster than 2.5 MHz. Some PHYs support clock rates up to 12.5 MHz, however. The AHB bus clock (HCLK) is divided by the specified amount. Refer to Table 160 below for the definition of values for this field."]
    #[inline(always)]
    pub fn clocksel(&mut self) -> CLOCKSEL_W {
        CLOCKSEL_W { w: self }
    }
    #[doc = "Bit 15 - RESET MII MGMT. This bit resets the MII Management hardware."]
    #[inline(always)]
    pub fn resetmiimgmt(&mut self) -> RESETMIIMGMT_W {
        RESETMIIMGMT_W { w: self }
    }
}
