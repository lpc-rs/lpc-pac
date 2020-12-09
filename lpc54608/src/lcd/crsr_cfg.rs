#[doc = "Reader of register CRSR_CFG"]
pub type R = crate::R<u32, super::CRSR_CFG>;
#[doc = "Writer for register CRSR_CFG"]
pub type W = crate::W<u32, super::CRSR_CFG>;
#[doc = "Register CRSR_CFG `reset()`'s with value 0"]
impl crate::ResetValue for super::CRSR_CFG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CRSRSIZE`"]
pub type CRSRSIZE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CRSRSIZE`"]
pub struct CRSRSIZE_W<'a> {
    w: &'a mut W,
}
impl<'a> CRSRSIZE_W<'a> {
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
#[doc = "Reader of field `FRAMESYNC`"]
pub type FRAMESYNC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FRAMESYNC`"]
pub struct FRAMESYNC_W<'a> {
    w: &'a mut W,
}
impl<'a> FRAMESYNC_W<'a> {
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
    #[doc = "Bit 0 - Cursor size selection."]
    #[inline(always)]
    pub fn crsrsize(&self) -> CRSRSIZE_R {
        CRSRSIZE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Cursor frame synchronization type."]
    #[inline(always)]
    pub fn framesync(&self) -> FRAMESYNC_R {
        FRAMESYNC_R::new(((self.bits >> 1) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Cursor size selection."]
    #[inline(always)]
    pub fn crsrsize(&mut self) -> CRSRSIZE_W {
        CRSRSIZE_W { w: self }
    }
    #[doc = "Bit 1 - Cursor frame synchronization type."]
    #[inline(always)]
    pub fn framesync(&mut self) -> FRAMESYNC_W {
        FRAMESYNC_W { w: self }
    }
}
