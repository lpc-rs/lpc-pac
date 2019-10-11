#[doc = "Reader of register INTEN"]
pub type R = crate::R<u32, super::INTEN>;
#[doc = "Writer for register INTEN"]
pub type W = crate::W<u32, super::INTEN>;
#[doc = "Register INTEN `reset()`'s with value 0"]
impl crate::ResetValue for super::INTEN {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `TMR_EN`"]
pub type TMR_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TMR_EN`"]
pub struct TMR_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> TMR_EN_W<'a> {
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
#[doc = "Reader of field `REMOVE_PU_EN`"]
pub type REMOVE_PU_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `REMOVE_PU_EN`"]
pub struct REMOVE_PU_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> REMOVE_PU_EN_W<'a> {
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
#[doc = "Reader of field `HNP_FAILURE_EN`"]
pub type HNP_FAILURE_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `HNP_FAILURE_EN`"]
pub struct HNP_FAILURE_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> HNP_FAILURE_EN_W<'a> {
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
#[doc = "Reader of field `HNP_SUCCES_EN`"]
pub type HNP_SUCCES_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `HNP_SUCCES_EN`"]
pub struct HNP_SUCCES_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> HNP_SUCCES_EN_W<'a> {
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
    #[doc = "Bit 0 - 1 = enable the corresponding bit in the IntSt register."]
    #[inline(always)]
    pub fn tmr_en(&self) -> TMR_EN_R {
        TMR_EN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - 1 = enable the corresponding bit in the IntSt register."]
    #[inline(always)]
    pub fn remove_pu_en(&self) -> REMOVE_PU_EN_R {
        REMOVE_PU_EN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - 1 = enable the corresponding bit in the IntSt register."]
    #[inline(always)]
    pub fn hnp_failure_en(&self) -> HNP_FAILURE_EN_R {
        HNP_FAILURE_EN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - 1 = enable the corresponding bit in the IntSt register."]
    #[inline(always)]
    pub fn hnp_succes_en(&self) -> HNP_SUCCES_EN_R {
        HNP_SUCCES_EN_R::new(((self.bits >> 3) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 1 = enable the corresponding bit in the IntSt register."]
    #[inline(always)]
    pub fn tmr_en(&mut self) -> TMR_EN_W {
        TMR_EN_W { w: self }
    }
    #[doc = "Bit 1 - 1 = enable the corresponding bit in the IntSt register."]
    #[inline(always)]
    pub fn remove_pu_en(&mut self) -> REMOVE_PU_EN_W {
        REMOVE_PU_EN_W { w: self }
    }
    #[doc = "Bit 2 - 1 = enable the corresponding bit in the IntSt register."]
    #[inline(always)]
    pub fn hnp_failure_en(&mut self) -> HNP_FAILURE_EN_W {
        HNP_FAILURE_EN_W { w: self }
    }
    #[doc = "Bit 3 - 1 = enable the corresponding bit in the IntSt register."]
    #[inline(always)]
    pub fn hnp_succes_en(&mut self) -> HNP_SUCCES_EN_W {
        HNP_SUCCES_EN_W { w: self }
    }
}
