#[doc = "Reader of register RSID"]
pub type R = crate::R<u32, super::RSID>;
#[doc = "Writer for register RSID"]
pub type W = crate::W<u32, super::RSID>;
#[doc = "Register RSID `reset()`'s with value 0"]
impl crate::ResetValue for super::RSID {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `POR`"]
pub type POR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `POR`"]
pub struct POR_W<'a> {
    w: &'a mut W,
}
impl<'a> POR_W<'a> {
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
#[doc = "Reader of field `EXTR`"]
pub type EXTR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EXTR`"]
pub struct EXTR_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTR_W<'a> {
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
#[doc = "Reader of field `WDTR`"]
pub type WDTR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WDTR`"]
pub struct WDTR_W<'a> {
    w: &'a mut W,
}
impl<'a> WDTR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = "Reader of field `BODR`"]
pub type BODR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BODR`"]
pub struct BODR_W<'a> {
    w: &'a mut W,
}
impl<'a> BODR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Assertion of the POR signal sets this bit, and clears all of the other bits in this register. But if another Reset signal (e.g., External Reset) remains asserted after the POR signal is negated, then its bit is set. This bit is not affected by any of the other sources of Reset."]
    #[inline(always)]
    pub fn por(&self) -> POR_R {
        POR_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Assertion of the RESET signal sets this bit. This bit is cleared only by software or POR."]
    #[inline(always)]
    pub fn extr(&self) -> EXTR_R {
        EXTR_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - This bit is set when the Watchdog Timer times out and the WDTRESET bit in the Watchdog Mode Register is 1. This bit is cleared only by software or POR."]
    #[inline(always)]
    pub fn wdtr(&self) -> WDTR_R {
        WDTR_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - This bit is set when the VDD(REG)(3V3) voltage reaches a level below the BOD reset trip level (typically 1.85 V under nominal room temperature conditions). If the VDD(REG)(3V3) voltage dips from the normal operating range to below the BOD reset trip level and recovers, the BODR bit will be set to 1. If the VDD(REG)(3V3) voltage dips from the normal operating range to below the BOD reset trip level and continues to decline to the level at which POR is asserted (nominally 1 V), the BODR bit is cleared. If the VDD(REG)(3V3) voltage rises continuously from below 1 V to a level above the BOD reset trip level, the BODR will be set to 1. This bit is cleared only by software or POR. Note: Only in the case where a reset occurs and the POR = 0, the BODR bit indicates if the VDD(REG)(3V3) voltage was below the BOD reset trip level or not."]
    #[inline(always)]
    pub fn bodr(&self) -> BODR_R {
        BODR_R::new(((self.bits >> 3) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Assertion of the POR signal sets this bit, and clears all of the other bits in this register. But if another Reset signal (e.g., External Reset) remains asserted after the POR signal is negated, then its bit is set. This bit is not affected by any of the other sources of Reset."]
    #[inline(always)]
    pub fn por(&mut self) -> POR_W {
        POR_W { w: self }
    }
    #[doc = "Bit 1 - Assertion of the RESET signal sets this bit. This bit is cleared only by software or POR."]
    #[inline(always)]
    pub fn extr(&mut self) -> EXTR_W {
        EXTR_W { w: self }
    }
    #[doc = "Bit 2 - This bit is set when the Watchdog Timer times out and the WDTRESET bit in the Watchdog Mode Register is 1. This bit is cleared only by software or POR."]
    #[inline(always)]
    pub fn wdtr(&mut self) -> WDTR_W {
        WDTR_W { w: self }
    }
    #[doc = "Bit 3 - This bit is set when the VDD(REG)(3V3) voltage reaches a level below the BOD reset trip level (typically 1.85 V under nominal room temperature conditions). If the VDD(REG)(3V3) voltage dips from the normal operating range to below the BOD reset trip level and recovers, the BODR bit will be set to 1. If the VDD(REG)(3V3) voltage dips from the normal operating range to below the BOD reset trip level and continues to decline to the level at which POR is asserted (nominally 1 V), the BODR bit is cleared. If the VDD(REG)(3V3) voltage rises continuously from below 1 V to a level above the BOD reset trip level, the BODR will be set to 1. This bit is cleared only by software or POR. Note: Only in the case where a reset occurs and the POR = 0, the BODR bit indicates if the VDD(REG)(3V3) voltage was below the BOD reset trip level or not."]
    #[inline(always)]
    pub fn bodr(&mut self) -> BODR_W {
        BODR_W { w: self }
    }
}
