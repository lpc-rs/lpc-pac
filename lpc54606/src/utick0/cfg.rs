#[doc = "Reader of register CFG"]
pub type R = crate::R<u32, super::CFG>;
#[doc = "Writer for register CFG"]
pub type W = crate::W<u32, super::CFG>;
#[doc = "Register CFG `reset()`'s with value 0"]
impl crate::ResetValue for super::CFG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CAPEN0`"]
pub type CAPEN0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CAPEN0`"]
pub struct CAPEN0_W<'a> {
    w: &'a mut W,
}
impl<'a> CAPEN0_W<'a> {
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
#[doc = "Reader of field `CAPEN1`"]
pub type CAPEN1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CAPEN1`"]
pub struct CAPEN1_W<'a> {
    w: &'a mut W,
}
impl<'a> CAPEN1_W<'a> {
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
#[doc = "Reader of field `CAPEN2`"]
pub type CAPEN2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CAPEN2`"]
pub struct CAPEN2_W<'a> {
    w: &'a mut W,
}
impl<'a> CAPEN2_W<'a> {
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
#[doc = "Reader of field `CAPEN3`"]
pub type CAPEN3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CAPEN3`"]
pub struct CAPEN3_W<'a> {
    w: &'a mut W,
}
impl<'a> CAPEN3_W<'a> {
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
#[doc = "Reader of field `CAPPOL0`"]
pub type CAPPOL0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CAPPOL0`"]
pub struct CAPPOL0_W<'a> {
    w: &'a mut W,
}
impl<'a> CAPPOL0_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
#[doc = "Reader of field `CAPPOL1`"]
pub type CAPPOL1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CAPPOL1`"]
pub struct CAPPOL1_W<'a> {
    w: &'a mut W,
}
impl<'a> CAPPOL1_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
        self.w
    }
}
#[doc = "Reader of field `CAPPOL2`"]
pub type CAPPOL2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CAPPOL2`"]
pub struct CAPPOL2_W<'a> {
    w: &'a mut W,
}
impl<'a> CAPPOL2_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
        self.w
    }
}
#[doc = "Reader of field `CAPPOL3`"]
pub type CAPPOL3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CAPPOL3`"]
pub struct CAPPOL3_W<'a> {
    w: &'a mut W,
}
impl<'a> CAPPOL3_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u32) & 0x01) << 11);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Enable Capture 0. 1 = Enabled, 0 = Disabled."]
    #[inline(always)]
    pub fn capen0(&self) -> CAPEN0_R {
        CAPEN0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Enable Capture 1. 1 = Enabled, 0 = Disabled."]
    #[inline(always)]
    pub fn capen1(&self) -> CAPEN1_R {
        CAPEN1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Enable Capture 2. 1 = Enabled, 0 = Disabled."]
    #[inline(always)]
    pub fn capen2(&self) -> CAPEN2_R {
        CAPEN2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Enable Capture 3. 1 = Enabled, 0 = Disabled."]
    #[inline(always)]
    pub fn capen3(&self) -> CAPEN3_R {
        CAPEN3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Capture Polarity 0. 0 = Positive edge capture, 1 = Negative edge capture."]
    #[inline(always)]
    pub fn cappol0(&self) -> CAPPOL0_R {
        CAPPOL0_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Capture Polarity 1. 0 = Positive edge capture, 1 = Negative edge capture."]
    #[inline(always)]
    pub fn cappol1(&self) -> CAPPOL1_R {
        CAPPOL1_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Capture Polarity 2. 0 = Positive edge capture, 1 = Negative edge capture."]
    #[inline(always)]
    pub fn cappol2(&self) -> CAPPOL2_R {
        CAPPOL2_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Capture Polarity 3. 0 = Positive edge capture, 1 = Negative edge capture."]
    #[inline(always)]
    pub fn cappol3(&self) -> CAPPOL3_R {
        CAPPOL3_R::new(((self.bits >> 11) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable Capture 0. 1 = Enabled, 0 = Disabled."]
    #[inline(always)]
    pub fn capen0(&mut self) -> CAPEN0_W {
        CAPEN0_W { w: self }
    }
    #[doc = "Bit 1 - Enable Capture 1. 1 = Enabled, 0 = Disabled."]
    #[inline(always)]
    pub fn capen1(&mut self) -> CAPEN1_W {
        CAPEN1_W { w: self }
    }
    #[doc = "Bit 2 - Enable Capture 2. 1 = Enabled, 0 = Disabled."]
    #[inline(always)]
    pub fn capen2(&mut self) -> CAPEN2_W {
        CAPEN2_W { w: self }
    }
    #[doc = "Bit 3 - Enable Capture 3. 1 = Enabled, 0 = Disabled."]
    #[inline(always)]
    pub fn capen3(&mut self) -> CAPEN3_W {
        CAPEN3_W { w: self }
    }
    #[doc = "Bit 8 - Capture Polarity 0. 0 = Positive edge capture, 1 = Negative edge capture."]
    #[inline(always)]
    pub fn cappol0(&mut self) -> CAPPOL0_W {
        CAPPOL0_W { w: self }
    }
    #[doc = "Bit 9 - Capture Polarity 1. 0 = Positive edge capture, 1 = Negative edge capture."]
    #[inline(always)]
    pub fn cappol1(&mut self) -> CAPPOL1_W {
        CAPPOL1_W { w: self }
    }
    #[doc = "Bit 10 - Capture Polarity 2. 0 = Positive edge capture, 1 = Negative edge capture."]
    #[inline(always)]
    pub fn cappol2(&mut self) -> CAPPOL2_W {
        CAPPOL2_W { w: self }
    }
    #[doc = "Bit 11 - Capture Polarity 3. 0 = Positive edge capture, 1 = Negative edge capture."]
    #[inline(always)]
    pub fn cappol3(&mut self) -> CAPPOL3_W {
        CAPPOL3_W { w: self }
    }
}
