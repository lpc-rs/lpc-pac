#[doc = "Reader of register UDCAH"]
pub type R = crate::R<u32, super::UDCAH>;
#[doc = "Writer for register UDCAH"]
pub type W = crate::W<u32, super::UDCAH>;
#[doc = "Register UDCAH `reset()`'s with value 0"]
impl crate::ResetValue for super::UDCAH {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `UDCA_ADDR`"]
pub type UDCA_ADDR_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `UDCA_ADDR`"]
pub struct UDCA_ADDR_W<'a> {
    w: &'a mut W,
}
impl<'a> UDCA_ADDR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01ff_ffff << 7)) | (((value as u32) & 0x01ff_ffff) << 7);
        self.w
    }
}
impl R {
    #[doc = "Bits 7:31 - Start address of the UDCA."]
    #[inline(always)]
    pub fn udca_addr(&self) -> UDCA_ADDR_R {
        UDCA_ADDR_R::new(((self.bits >> 7) & 0x01ff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 7:31 - Start address of the UDCA."]
    #[inline(always)]
    pub fn udca_addr(&mut self) -> UDCA_ADDR_W {
        UDCA_ADDR_W { w: self }
    }
}
