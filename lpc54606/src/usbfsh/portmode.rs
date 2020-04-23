#[doc = "Reader of register PORTMODE"]
pub type R = crate::R<u32, super::PORTMODE>;
#[doc = "Writer for register PORTMODE"]
pub type W = crate::W<u32, super::PORTMODE>;
#[doc = "Register PORTMODE `reset()`'s with value 0"]
impl crate::ResetValue for super::PORTMODE {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ID`"]
pub type ID_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ID`"]
pub struct ID_W<'a> {
    w: &'a mut W,
}
impl<'a> ID_W<'a> {
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
#[doc = "Reader of field `ID_EN`"]
pub type ID_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ID_EN`"]
pub struct ID_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> ID_EN_W<'a> {
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
#[doc = "Reader of field `DEV_ENABLE`"]
pub type DEV_ENABLE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DEV_ENABLE`"]
pub struct DEV_ENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> DEV_ENABLE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | (((value as u32) & 0x01) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Port ID pin value."]
    #[inline(always)]
    pub fn id(&self) -> ID_R {
        ID_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 8 - Port ID pin pull-up enable."]
    #[inline(always)]
    pub fn id_en(&self) -> ID_EN_R {
        ID_EN_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 16 - 1: device 0: host."]
    #[inline(always)]
    pub fn dev_enable(&self) -> DEV_ENABLE_R {
        DEV_ENABLE_R::new(((self.bits >> 16) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Port ID pin value."]
    #[inline(always)]
    pub fn id(&mut self) -> ID_W {
        ID_W { w: self }
    }
    #[doc = "Bit 8 - Port ID pin pull-up enable."]
    #[inline(always)]
    pub fn id_en(&mut self) -> ID_EN_W {
        ID_EN_W { w: self }
    }
    #[doc = "Bit 16 - 1: device 0: host."]
    #[inline(always)]
    pub fn dev_enable(&mut self) -> DEV_ENABLE_W {
        DEV_ENABLE_W { w: self }
    }
}
