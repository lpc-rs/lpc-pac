#[doc = "Reader of register CHANEN"]
pub type R = crate::R<u32, super::CHANEN>;
#[doc = "Writer for register CHANEN"]
pub type W = crate::W<u32, super::CHANEN>;
#[doc = "Register CHANEN `reset()`'s with value 0"]
impl crate::ResetValue for super::CHANEN {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `EN_CH0`"]
pub type EN_CH0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EN_CH0`"]
pub struct EN_CH0_W<'a> {
    w: &'a mut W,
}
impl<'a> EN_CH0_W<'a> {
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
#[doc = "Reader of field `EN_CH1`"]
pub type EN_CH1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EN_CH1`"]
pub struct EN_CH1_W<'a> {
    w: &'a mut W,
}
impl<'a> EN_CH1_W<'a> {
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
    #[doc = "Bit 0 - Enable channel 0. When 1, PDM channel 0 is enabled."]
    #[inline(always)]
    pub fn en_ch0(&self) -> EN_CH0_R {
        EN_CH0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Enable channel 1. When 1, PDM channel 1 is enabled."]
    #[inline(always)]
    pub fn en_ch1(&self) -> EN_CH1_R {
        EN_CH1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable channel 0. When 1, PDM channel 0 is enabled."]
    #[inline(always)]
    pub fn en_ch0(&mut self) -> EN_CH0_W {
        EN_CH0_W { w: self }
    }
    #[doc = "Bit 1 - Enable channel 1. When 1, PDM channel 1 is enabled."]
    #[inline(always)]
    pub fn en_ch1(&mut self) -> EN_CH1_W {
        EN_CH1_W { w: self }
    }
}
