#[doc = "Reader of register IST"]
pub type R = crate::R<u32, super::IST>;
#[doc = "Writer for register IST"]
pub type W = crate::W<u32, super::IST>;
#[doc = "Register IST `reset()`'s with value 0"]
impl crate::ResetValue for super::IST {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `PSTAT`"]
pub type PSTAT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PSTAT`"]
pub struct PSTAT_W<'a> {
    w: &'a mut W,
}
impl<'a> PSTAT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Pin interrupt status. Bit n returns the status, clears the edge interrupt, or inverts the active level of the pin selected in PINTSELn. Read 0: interrupt is not being requested for this interrupt pin. Write 0: no operation. Read 1: interrupt is being requested for this interrupt pin. Write 1 (edge-sensitive): clear rising- and falling-edge detection for this pin. Write 1 (level-sensitive): switch the active level for this pin (in the IENF register)."]
    #[inline(always)]
    pub fn pstat(&self) -> PSTAT_R {
        PSTAT_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Pin interrupt status. Bit n returns the status, clears the edge interrupt, or inverts the active level of the pin selected in PINTSELn. Read 0: interrupt is not being requested for this interrupt pin. Write 0: no operation. Read 1: interrupt is being requested for this interrupt pin. Write 1 (edge-sensitive): clear rising- and falling-edge detection for this pin. Write 1 (level-sensitive): switch the active level for this pin (in the IENF register)."]
    #[inline(always)]
    pub fn pstat(&mut self) -> PSTAT_W {
        PSTAT_W { w: self }
    }
}
