#[doc = "Reader of register CONTROL"]
pub type R = crate::R<u32, super::CONTROL>;
#[doc = "Writer for register CONTROL"]
pub type W = crate::W<u32, super::CONTROL>;
#[doc = "Register CONTROL `reset()`'s with value 0x03"]
impl crate::ResetValue for super::CONTROL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x03
    }
}
#[doc = "Reader of field `E`"]
pub type E_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `E`"]
pub struct E_W<'a> {
    w: &'a mut W,
}
impl<'a> E_W<'a> {
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
#[doc = "Reader of field `M`"]
pub type M_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `M`"]
pub struct M_W<'a> {
    w: &'a mut W,
}
impl<'a> M_W<'a> {
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
#[doc = "Reader of field `L`"]
pub type L_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `L`"]
pub struct L_W<'a> {
    w: &'a mut W,
}
impl<'a> L_W<'a> {
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
impl R {
    #[doc = "Bit 0 - EMC Enable."]
    #[inline(always)]
    pub fn e(&self) -> E_R {
        E_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Address mirror."]
    #[inline(always)]
    pub fn m(&self) -> M_R {
        M_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Low-power mode."]
    #[inline(always)]
    pub fn l(&self) -> L_R {
        L_R::new(((self.bits >> 2) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - EMC Enable."]
    #[inline(always)]
    pub fn e(&mut self) -> E_W {
        E_W { w: self }
    }
    #[doc = "Bit 1 - Address mirror."]
    #[inline(always)]
    pub fn m(&mut self) -> M_W {
        M_W { w: self }
    }
    #[doc = "Bit 2 - Low-power mode."]
    #[inline(always)]
    pub fn l(&mut self) -> L_W {
        L_W { w: self }
    }
}
