#[doc = "Reader of register SYSOSCCTRL"]
pub type R = crate::R<u32, super::SYSOSCCTRL>;
#[doc = "Writer for register SYSOSCCTRL"]
pub type W = crate::W<u32, super::SYSOSCCTRL>;
#[doc = "Register SYSOSCCTRL `reset()`'s with value 0"]
impl crate::ResetValue for super::SYSOSCCTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `BYPASS`"]
pub type BYPASS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BYPASS`"]
pub struct BYPASS_W<'a> {
    w: &'a mut W,
}
impl<'a> BYPASS_W<'a> {
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
#[doc = "Reader of field `FREQRANGE`"]
pub type FREQRANGE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FREQRANGE`"]
pub struct FREQRANGE_W<'a> {
    w: &'a mut W,
}
impl<'a> FREQRANGE_W<'a> {
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
impl R {
    #[doc = "Bit 0 - oscillator (Xtal) Test Mode input (Active High)"]
    #[inline(always)]
    pub fn bypass(&self) -> BYPASS_R {
        BYPASS_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - oscillator low / high transconductance selection input (Active High) 1-20MHz '0' : 15-50MHz '1'"]
    #[inline(always)]
    pub fn freqrange(&self) -> FREQRANGE_R {
        FREQRANGE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - oscillator (Xtal) Test Mode input (Active High)"]
    #[inline(always)]
    pub fn bypass(&mut self) -> BYPASS_W {
        BYPASS_W { w: self }
    }
    #[doc = "Bit 1 - oscillator low / high transconductance selection input (Active High) 1-20MHz '0' : 15-50MHz '1'"]
    #[inline(always)]
    pub fn freqrange(&mut self) -> FREQRANGE_W {
        FREQRANGE_W { w: self }
    }
}
