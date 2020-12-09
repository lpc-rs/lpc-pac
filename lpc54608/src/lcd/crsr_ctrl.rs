#[doc = "Reader of register CRSR_CTRL"]
pub type R = crate::R<u32, super::CRSR_CTRL>;
#[doc = "Writer for register CRSR_CTRL"]
pub type W = crate::W<u32, super::CRSR_CTRL>;
#[doc = "Register CRSR_CTRL `reset()`'s with value 0"]
impl crate::ResetValue for super::CRSR_CTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CRSRON`"]
pub type CRSRON_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CRSRON`"]
pub struct CRSRON_W<'a> {
    w: &'a mut W,
}
impl<'a> CRSRON_W<'a> {
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
#[doc = "Reader of field `CRSRNUM1_0`"]
pub type CRSRNUM1_0_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CRSRNUM1_0`"]
pub struct CRSRNUM1_0_W<'a> {
    w: &'a mut W,
}
impl<'a> CRSRNUM1_0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | (((value as u32) & 0x03) << 4);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Cursor enable."]
    #[inline(always)]
    pub fn crsron(&self) -> CRSRON_R {
        CRSRON_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bits 4:5 - Cursor image number."]
    #[inline(always)]
    pub fn crsrnum1_0(&self) -> CRSRNUM1_0_R {
        CRSRNUM1_0_R::new(((self.bits >> 4) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Cursor enable."]
    #[inline(always)]
    pub fn crsron(&mut self) -> CRSRON_W {
        CRSRON_W { w: self }
    }
    #[doc = "Bits 4:5 - Cursor image number."]
    #[inline(always)]
    pub fn crsrnum1_0(&mut self) -> CRSRNUM1_0_W {
        CRSRNUM1_0_W { w: self }
    }
}
