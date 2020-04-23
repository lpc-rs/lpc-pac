#[doc = "Reader of register AUTOCGOR"]
pub type R = crate::R<u32, super::AUTOCGOR>;
#[doc = "Writer for register AUTOCGOR"]
pub type W = crate::W<u32, super::AUTOCGOR>;
#[doc = "Register AUTOCGOR `reset()`'s with value 0"]
impl crate::ResetValue for super::AUTOCGOR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RAM0X`"]
pub type RAM0X_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RAM0X`"]
pub struct RAM0X_W<'a> {
    w: &'a mut W,
}
impl<'a> RAM0X_W<'a> {
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
#[doc = "Reader of field `RAM1`"]
pub type RAM1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RAM1`"]
pub struct RAM1_W<'a> {
    w: &'a mut W,
}
impl<'a> RAM1_W<'a> {
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
#[doc = "Reader of field `RAM2`"]
pub type RAM2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RAM2`"]
pub struct RAM2_W<'a> {
    w: &'a mut W,
}
impl<'a> RAM2_W<'a> {
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
#[doc = "Reader of field `RAM3`"]
pub type RAM3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RAM3`"]
pub struct RAM3_W<'a> {
    w: &'a mut W,
}
impl<'a> RAM3_W<'a> {
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
impl R {
    #[doc = "Bit 1 - When 1, automatic clock gating for RAMX and RAM0 are turned off."]
    #[inline(always)]
    pub fn ram0x(&self) -> RAM0X_R {
        RAM0X_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - When 1, automatic clock gating for RAM1 are turned off."]
    #[inline(always)]
    pub fn ram1(&self) -> RAM1_R {
        RAM1_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - When 1, automatic clock gating for RAM1 are turned off."]
    #[inline(always)]
    pub fn ram2(&self) -> RAM2_R {
        RAM2_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - When 1, automatic clock gating for RAM1 are turned off."]
    #[inline(always)]
    pub fn ram3(&self) -> RAM3_R {
        RAM3_R::new(((self.bits >> 4) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - When 1, automatic clock gating for RAMX and RAM0 are turned off."]
    #[inline(always)]
    pub fn ram0x(&mut self) -> RAM0X_W {
        RAM0X_W { w: self }
    }
    #[doc = "Bit 2 - When 1, automatic clock gating for RAM1 are turned off."]
    #[inline(always)]
    pub fn ram1(&mut self) -> RAM1_W {
        RAM1_W { w: self }
    }
    #[doc = "Bit 3 - When 1, automatic clock gating for RAM1 are turned off."]
    #[inline(always)]
    pub fn ram2(&mut self) -> RAM2_W {
        RAM2_W { w: self }
    }
    #[doc = "Bit 4 - When 1, automatic clock gating for RAM1 are turned off."]
    #[inline(always)]
    pub fn ram3(&mut self) -> RAM3_W {
        RAM3_W { w: self }
    }
}
