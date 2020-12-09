#[doc = "Reader of register EMCSYSCTRL"]
pub type R = crate::R<u32, super::EMCSYSCTRL>;
#[doc = "Writer for register EMCSYSCTRL"]
pub type W = crate::W<u32, super::EMCSYSCTRL>;
#[doc = "Register EMCSYSCTRL `reset()`'s with value 0x01"]
impl crate::ResetValue for super::EMCSYSCTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x01
    }
}
#[doc = "Reader of field `EMCSC`"]
pub type EMCSC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EMCSC`"]
pub struct EMCSC_W<'a> {
    w: &'a mut W,
}
impl<'a> EMCSC_W<'a> {
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
#[doc = "Reader of field `EMCRD`"]
pub type EMCRD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EMCRD`"]
pub struct EMCRD_W<'a> {
    w: &'a mut W,
}
impl<'a> EMCRD_W<'a> {
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
#[doc = "Reader of field `EMCBC`"]
pub type EMCBC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EMCBC`"]
pub struct EMCBC_W<'a> {
    w: &'a mut W,
}
impl<'a> EMCBC_W<'a> {
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
#[doc = "Reader of field `EMCFBCLKINSEL`"]
pub type EMCFBCLKINSEL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EMCFBCLKINSEL`"]
pub struct EMCFBCLKINSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> EMCFBCLKINSEL_W<'a> {
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
    #[doc = "Bit 0 - EMC Shift Control."]
    #[inline(always)]
    pub fn emcsc(&self) -> EMCSC_R {
        EMCSC_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - EMC Reset Disable."]
    #[inline(always)]
    pub fn emcrd(&self) -> EMCRD_R {
        EMCRD_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - External Memory Controller burst control."]
    #[inline(always)]
    pub fn emcbc(&self) -> EMCBC_R {
        EMCBC_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - External Memory Controller clock select."]
    #[inline(always)]
    pub fn emcfbclkinsel(&self) -> EMCFBCLKINSEL_R {
        EMCFBCLKINSEL_R::new(((self.bits >> 3) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - EMC Shift Control."]
    #[inline(always)]
    pub fn emcsc(&mut self) -> EMCSC_W {
        EMCSC_W { w: self }
    }
    #[doc = "Bit 1 - EMC Reset Disable."]
    #[inline(always)]
    pub fn emcrd(&mut self) -> EMCRD_W {
        EMCRD_W { w: self }
    }
    #[doc = "Bit 2 - External Memory Controller burst control."]
    #[inline(always)]
    pub fn emcbc(&mut self) -> EMCBC_W {
        EMCBC_W { w: self }
    }
    #[doc = "Bit 3 - External Memory Controller clock select."]
    #[inline(always)]
    pub fn emcfbclkinsel(&mut self) -> EMCFBCLKINSEL_W {
        EMCFBCLKINSEL_W { w: self }
    }
}
