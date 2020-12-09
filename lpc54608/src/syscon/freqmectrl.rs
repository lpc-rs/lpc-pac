#[doc = "Reader of register FREQMECTRL"]
pub type R = crate::R<u32, super::FREQMECTRL>;
#[doc = "Writer for register FREQMECTRL"]
pub type W = crate::W<u32, super::FREQMECTRL>;
#[doc = "Register FREQMECTRL `reset()`'s with value 0"]
impl crate::ResetValue for super::FREQMECTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CAPVAL`"]
pub type CAPVAL_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `CAPVAL`"]
pub struct CAPVAL_W<'a> {
    w: &'a mut W,
}
impl<'a> CAPVAL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3fff) | ((value as u32) & 0x3fff);
        self.w
    }
}
#[doc = "Reader of field `PROG`"]
pub type PROG_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PROG`"]
pub struct PROG_W<'a> {
    w: &'a mut W,
}
impl<'a> PROG_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | (((value as u32) & 0x01) << 31);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:13 - Stores the capture result which is used to calculate the frequency of the target clock. This field is read-only."]
    #[inline(always)]
    pub fn capval(&self) -> CAPVAL_R {
        CAPVAL_R::new((self.bits & 0x3fff) as u16)
    }
    #[doc = "Bit 31 - Set this bit to one to initiate a frequency measurement cycle. Hardware clears this bit when the measurement cycle has completed and there is valid capture data in the CAPVAL field (bits 13:0)."]
    #[inline(always)]
    pub fn prog(&self) -> PROG_R {
        PROG_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:13 - Stores the capture result which is used to calculate the frequency of the target clock. This field is read-only."]
    #[inline(always)]
    pub fn capval(&mut self) -> CAPVAL_W {
        CAPVAL_W { w: self }
    }
    #[doc = "Bit 31 - Set this bit to one to initiate a frequency measurement cycle. Hardware clears this bit when the measurement cycle has completed and there is valid capture data in the CAPVAL field (bits 13:0)."]
    #[inline(always)]
    pub fn prog(&mut self) -> PROG_W {
        PROG_W { w: self }
    }
}
