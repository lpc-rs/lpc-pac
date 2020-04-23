#[doc = "Reader of register ETHPHYSEL"]
pub type R = crate::R<u32, super::ETHPHYSEL>;
#[doc = "Writer for register ETHPHYSEL"]
pub type W = crate::W<u32, super::ETHPHYSEL>;
#[doc = "Register ETHPHYSEL `reset()`'s with value 0"]
impl crate::ResetValue for super::ETHPHYSEL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `PHY_SEL`"]
pub type PHY_SEL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PHY_SEL`"]
pub struct PHY_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> PHY_SEL_W<'a> {
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
    #[doc = "Bit 2 - PHY interface select."]
    #[inline(always)]
    pub fn phy_sel(&self) -> PHY_SEL_R {
        PHY_SEL_R::new(((self.bits >> 2) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - PHY interface select."]
    #[inline(always)]
    pub fn phy_sel(&mut self) -> PHY_SEL_W {
        PHY_SEL_W { w: self }
    }
}
