#[doc = "Reader of register TXDESCRIPTOR"]
pub type R = crate::R<u32, super::TXDESCRIPTOR>;
#[doc = "Writer for register TXDESCRIPTOR"]
pub type W = crate::W<u32, super::TXDESCRIPTOR>;
#[doc = "Register TXDESCRIPTOR `reset()`'s with value 0"]
impl crate::ResetValue for super::TXDESCRIPTOR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `TXD`"]
pub type TXD_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `TXD`"]
pub struct TXD_W<'a> {
    w: &'a mut W,
}
impl<'a> TXD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3fff_ffff << 2)) | (((value as u32) & 0x3fff_ffff) << 2);
        self.w
    }
}
impl R {
    #[doc = "Bits 2:31 - TxDescriptor. MSBs of transmit descriptor base address."]
    #[inline(always)]
    pub fn txd(&self) -> TXD_R {
        TXD_R::new(((self.bits >> 2) & 0x3fff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 2:31 - TxDescriptor. MSBs of transmit descriptor base address."]
    #[inline(always)]
    pub fn txd(&mut self) -> TXD_W {
        TXD_W { w: self }
    }
}
