#[doc = "Reader of register RXDESCRIPTOR"]
pub type R = crate::R<u32, super::RXDESCRIPTOR>;
#[doc = "Writer for register RXDESCRIPTOR"]
pub type W = crate::W<u32, super::RXDESCRIPTOR>;
#[doc = "Register RXDESCRIPTOR `reset()`'s with value 0"]
impl crate::ResetValue for super::RXDESCRIPTOR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RXDESCRIPTOR`"]
pub type RXDESCRIPTOR_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `RXDESCRIPTOR`"]
pub struct RXDESCRIPTOR_W<'a> {
    w: &'a mut W,
}
impl<'a> RXDESCRIPTOR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3fff_ffff << 2)) | (((value as u32) & 0x3fff_ffff) << 2);
        self.w
    }
}
impl R {
    #[doc = "Bits 2:31 - MSBs of receive descriptor base address."]
    #[inline(always)]
    pub fn rxdescriptor(&self) -> RXDESCRIPTOR_R {
        RXDESCRIPTOR_R::new(((self.bits >> 2) & 0x3fff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 2:31 - MSBs of receive descriptor base address."]
    #[inline(always)]
    pub fn rxdescriptor(&mut self) -> RXDESCRIPTOR_W {
        RXDESCRIPTOR_W { w: self }
    }
}
