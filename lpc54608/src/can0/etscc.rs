#[doc = "Reader of register ETSCC"]
pub type R = crate::R<u32, super::ETSCC>;
#[doc = "Writer for register ETSCC"]
pub type W = crate::W<u32, super::ETSCC>;
#[doc = "Register ETSCC `reset()`'s with value 0"]
impl crate::ResetValue for super::ETSCC {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ETCP`"]
pub type ETCP_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `ETCP`"]
pub struct ETCP_W<'a> {
    w: &'a mut W,
}
impl<'a> ETCP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07ff) | ((value as u32) & 0x07ff);
        self.w
    }
}
#[doc = "Reader of field `ETCE`"]
pub type ETCE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ETCE`"]
pub struct ETCE_W<'a> {
    w: &'a mut W,
}
impl<'a> ETCE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | (((value as u32) & 0x01) << 31);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:10 - External timestamp prescaler value."]
    #[inline(always)]
    pub fn etcp(&self) -> ETCP_R {
        ETCP_R::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bit 31 - External timestamp counter enable."]
    #[inline(always)]
    pub fn etce(&self) -> ETCE_R {
        ETCE_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:10 - External timestamp prescaler value."]
    #[inline(always)]
    pub fn etcp(&mut self) -> ETCP_W {
        ETCP_W { w: self }
    }
    #[doc = "Bit 31 - External timestamp counter enable."]
    #[inline(always)]
    pub fn etce(&mut self) -> ETCE_W {
        ETCE_W { w: self }
    }
}
