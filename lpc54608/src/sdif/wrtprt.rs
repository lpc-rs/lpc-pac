#[doc = "Reader of register WRTPRT"]
pub type R = crate::R<u32, super::WRTPRT>;
#[doc = "Writer for register WRTPRT"]
pub type W = crate::W<u32, super::WRTPRT>;
#[doc = "Register WRTPRT `reset()`'s with value 0"]
impl crate::ResetValue for super::WRTPRT {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `WRITE_PROTECT`"]
pub type WRITE_PROTECT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WRITE_PROTECT`"]
pub struct WRITE_PROTECT_W<'a> {
    w: &'a mut W,
}
impl<'a> WRITE_PROTECT_W<'a> {
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
impl R {
    #[doc = "Bit 0 - Write protect."]
    #[inline(always)]
    pub fn write_protect(&self) -> WRITE_PROTECT_R {
        WRITE_PROTECT_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Write protect."]
    #[inline(always)]
    pub fn write_protect(&mut self) -> WRITE_PROTECT_W {
        WRITE_PROTECT_W { w: self }
    }
}
