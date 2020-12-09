#[doc = "Reader of register FFLR"]
pub type R = crate::R<u32, super::FFLR>;
#[doc = "Writer for register FFLR"]
pub type W = crate::W<u32, super::FFLR>;
#[doc = "Register FFLR `reset()`'s with value 0"]
impl crate::ResetValue for super::FFLR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `FIFOFullLevel`"]
pub type FIFOFULLLEVEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `FIFOFullLevel`"]
pub struct FIFOFULLLEVEL_W<'a> {
    w: &'a mut W,
}
impl<'a> FIFOFULLLEVEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - FIFO full level. The number of bytes left in FIFO, below which the FIFOFULL or SupressData signal is asserted. For example, setting this value to 15 causes data trace suppression or processor stalling, if enabled, when there are less than 15 free bytes in the FIFO."]
    #[inline(always)]
    pub fn fifofull_level(&self) -> FIFOFULLLEVEL_R {
        FIFOFULLLEVEL_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - FIFO full level. The number of bytes left in FIFO, below which the FIFOFULL or SupressData signal is asserted. For example, setting this value to 15 causes data trace suppression or processor stalling, if enabled, when there are less than 15 free bytes in the FIFO."]
    #[inline(always)]
    pub fn fifofull_level(&mut self) -> FIFOFULLLEVEL_W {
        FIFOFULLLEVEL_W { w: self }
    }
}
