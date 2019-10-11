#[doc = "Reader of register TRM"]
pub type R = crate::R<u32, super::TRM>;
#[doc = "Writer for register TRM"]
pub type W = crate::W<u32, super::TRM>;
#[doc = "Register TRM `reset()`'s with value 0"]
impl crate::ResetValue for super::TRM {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `VRANGE`"]
pub type VRANGE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `VRANGE`"]
pub struct VRANGE_W<'a> {
    w: &'a mut W,
}
impl<'a> VRANGE_W<'a> {
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
impl R {
    #[doc = "Bit 5 - 1.8V to 3.6V Vdd range: This bit MUST be set to '1' if operation below 2.7V is to be used. Failure to set this bit will result in invalid ADC results. Note: This bit will not be spec'd on parts that do not support operation below 2.7V"]
    #[inline(always)]
    pub fn vrange(&self) -> VRANGE_R {
        VRANGE_R::new(((self.bits >> 5) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 5 - 1.8V to 3.6V Vdd range: This bit MUST be set to '1' if operation below 2.7V is to be used. Failure to set this bit will result in invalid ADC results. Note: This bit will not be spec'd on parts that do not support operation below 2.7V"]
    #[inline(always)]
    pub fn vrange(&mut self) -> VRANGE_W {
        VRANGE_W { w: self }
    }
}
