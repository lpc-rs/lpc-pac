#[doc = "Reader of register LE"]
pub type R = crate::R<u32, super::LE>;
#[doc = "Writer for register LE"]
pub type W = crate::W<u32, super::LE>;
#[doc = "Register LE `reset()`'s with value 0"]
impl crate::ResetValue for super::LE {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `LED`"]
pub type LED_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `LED`"]
pub struct LED_W<'a> {
    w: &'a mut W,
}
impl<'a> LED_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x7f) | ((value as u32) & 0x7f);
        self.w
    }
}
#[doc = "Reader of field `LEE`"]
pub type LEE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LEE`"]
pub struct LEE_W<'a> {
    w: &'a mut W,
}
impl<'a> LEE_W<'a> {
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
    #[doc = "Bits 0:6 - Line-end delay."]
    #[inline(always)]
    pub fn led(&self) -> LED_R {
        LED_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bit 16 - LCD Line end enable."]
    #[inline(always)]
    pub fn lee(&self) -> LEE_R {
        LEE_R::new(((self.bits >> 16) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:6 - Line-end delay."]
    #[inline(always)]
    pub fn led(&mut self) -> LED_W {
        LED_W { w: self }
    }
    #[doc = "Bit 16 - LCD Line end enable."]
    #[inline(always)]
    pub fn lee(&mut self) -> LEE_W {
        LEE_W { w: self }
    }
}
