#[doc = "Reader of register EMCDLYCAL"]
pub type R = crate::R<u32, super::EMCDLYCAL>;
#[doc = "Writer for register EMCDLYCAL"]
pub type W = crate::W<u32, super::EMCDLYCAL>;
#[doc = "Register EMCDLYCAL `reset()`'s with value 0"]
impl crate::ResetValue for super::EMCDLYCAL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CALVALUE`"]
pub type CALVALUE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CALVALUE`"]
pub struct CALVALUE_W<'a> {
    w: &'a mut W,
}
impl<'a> CALVALUE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
#[doc = "Reader of field `START`"]
pub type START_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `START`"]
pub struct START_W<'a> {
    w: &'a mut W,
}
impl<'a> START_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | (((value as u32) & 0x01) << 14);
        self.w
    }
}
#[doc = "Reader of field `DONE`"]
pub type DONE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DONE`"]
pub struct DONE_W<'a> {
    w: &'a mut W,
}
impl<'a> DONE_W<'a> {
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
    #[doc = "Bits 0:7 - Returns the count of the approximately 50 MHz ring oscillator that occur during 32 clocks of the FRO 12 MHz."]
    #[inline(always)]
    pub fn calvalue(&self) -> CALVALUE_R {
        CALVALUE_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 14 - Start control bit for the EMC calibration counter."]
    #[inline(always)]
    pub fn start(&self) -> START_R {
        START_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Measurement completion flag."]
    #[inline(always)]
    pub fn done(&self) -> DONE_R {
        DONE_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - Returns the count of the approximately 50 MHz ring oscillator that occur during 32 clocks of the FRO 12 MHz."]
    #[inline(always)]
    pub fn calvalue(&mut self) -> CALVALUE_W {
        CALVALUE_W { w: self }
    }
    #[doc = "Bit 14 - Start control bit for the EMC calibration counter."]
    #[inline(always)]
    pub fn start(&mut self) -> START_W {
        START_W { w: self }
    }
    #[doc = "Bit 15 - Measurement completion flag."]
    #[inline(always)]
    pub fn done(&mut self) -> DONE_W {
        DONE_W { w: self }
    }
}
