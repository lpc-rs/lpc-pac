#[doc = "Reader of register CIIR"]
pub type R = crate::R<u32, super::CIIR>;
#[doc = "Writer for register CIIR"]
pub type W = crate::W<u32, super::CIIR>;
#[doc = "Register CIIR `reset()`'s with value 0"]
impl crate::ResetValue for super::CIIR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `IMSEC`"]
pub type IMSEC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IMSEC`"]
pub struct IMSEC_W<'a> {
    w: &'a mut W,
}
impl<'a> IMSEC_W<'a> {
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
#[doc = "Reader of field `IMMIN`"]
pub type IMMIN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IMMIN`"]
pub struct IMMIN_W<'a> {
    w: &'a mut W,
}
impl<'a> IMMIN_W<'a> {
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
#[doc = "Reader of field `IMHOUR`"]
pub type IMHOUR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IMHOUR`"]
pub struct IMHOUR_W<'a> {
    w: &'a mut W,
}
impl<'a> IMHOUR_W<'a> {
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
#[doc = "Reader of field `IMDOM`"]
pub type IMDOM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IMDOM`"]
pub struct IMDOM_W<'a> {
    w: &'a mut W,
}
impl<'a> IMDOM_W<'a> {
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
#[doc = "Reader of field `IMDOW`"]
pub type IMDOW_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IMDOW`"]
pub struct IMDOW_W<'a> {
    w: &'a mut W,
}
impl<'a> IMDOW_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
#[doc = "Reader of field `IMDOY`"]
pub type IMDOY_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IMDOY`"]
pub struct IMDOY_W<'a> {
    w: &'a mut W,
}
impl<'a> IMDOY_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
#[doc = "Reader of field `IMMON`"]
pub type IMMON_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IMMON`"]
pub struct IMMON_W<'a> {
    w: &'a mut W,
}
impl<'a> IMMON_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
#[doc = "Reader of field `IMYEAR`"]
pub type IMYEAR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IMYEAR`"]
pub struct IMYEAR_W<'a> {
    w: &'a mut W,
}
impl<'a> IMYEAR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - When 1, an increment of the Second value generates an interrupt."]
    #[inline(always)]
    pub fn imsec(&self) -> IMSEC_R {
        IMSEC_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - When 1, an increment of the Minute value generates an interrupt."]
    #[inline(always)]
    pub fn immin(&self) -> IMMIN_R {
        IMMIN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - When 1, an increment of the Hour value generates an interrupt."]
    #[inline(always)]
    pub fn imhour(&self) -> IMHOUR_R {
        IMHOUR_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - When 1, an increment of the Day of Month value generates an interrupt."]
    #[inline(always)]
    pub fn imdom(&self) -> IMDOM_R {
        IMDOM_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - When 1, an increment of the Day of Week value generates an interrupt."]
    #[inline(always)]
    pub fn imdow(&self) -> IMDOW_R {
        IMDOW_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - When 1, an increment of the Day of Year value generates an interrupt."]
    #[inline(always)]
    pub fn imdoy(&self) -> IMDOY_R {
        IMDOY_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - When 1, an increment of the Month value generates an interrupt."]
    #[inline(always)]
    pub fn immon(&self) -> IMMON_R {
        IMMON_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - When 1, an increment of the Year value generates an interrupt."]
    #[inline(always)]
    pub fn imyear(&self) -> IMYEAR_R {
        IMYEAR_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - When 1, an increment of the Second value generates an interrupt."]
    #[inline(always)]
    pub fn imsec(&mut self) -> IMSEC_W {
        IMSEC_W { w: self }
    }
    #[doc = "Bit 1 - When 1, an increment of the Minute value generates an interrupt."]
    #[inline(always)]
    pub fn immin(&mut self) -> IMMIN_W {
        IMMIN_W { w: self }
    }
    #[doc = "Bit 2 - When 1, an increment of the Hour value generates an interrupt."]
    #[inline(always)]
    pub fn imhour(&mut self) -> IMHOUR_W {
        IMHOUR_W { w: self }
    }
    #[doc = "Bit 3 - When 1, an increment of the Day of Month value generates an interrupt."]
    #[inline(always)]
    pub fn imdom(&mut self) -> IMDOM_W {
        IMDOM_W { w: self }
    }
    #[doc = "Bit 4 - When 1, an increment of the Day of Week value generates an interrupt."]
    #[inline(always)]
    pub fn imdow(&mut self) -> IMDOW_W {
        IMDOW_W { w: self }
    }
    #[doc = "Bit 5 - When 1, an increment of the Day of Year value generates an interrupt."]
    #[inline(always)]
    pub fn imdoy(&mut self) -> IMDOY_W {
        IMDOY_W { w: self }
    }
    #[doc = "Bit 6 - When 1, an increment of the Month value generates an interrupt."]
    #[inline(always)]
    pub fn immon(&mut self) -> IMMON_W {
        IMMON_W { w: self }
    }
    #[doc = "Bit 7 - When 1, an increment of the Year value generates an interrupt."]
    #[inline(always)]
    pub fn imyear(&mut self) -> IMYEAR_W {
        IMYEAR_W { w: self }
    }
}
