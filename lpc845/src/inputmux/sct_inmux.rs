#[doc = "Reader of register SCT_INMUX[%s]"]
pub type R = crate::R<u32, super::SCT_INMUX>;
#[doc = "Writer for register SCT_INMUX[%s]"]
pub type W = crate::W<u32, super::SCT_INMUX>;
#[doc = "Register SCT_INMUX[%s] `reset()`'s with value 0x0f"]
impl crate::ResetValue for super::SCT_INMUX {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0f
    }
}
#[doc = "Reader of field `INP_N`"]
pub type INP_N_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `INP_N`"]
pub struct INP_N_W<'a> {
    w: &'a mut W,
}
impl<'a> INP_N_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - Input mux register for SCT input n (n = 0 to 3). 0 = sct input 0 1=sct input 1 2= sct input 2 3= sct gpio input 3 4= adc_thcmp_irq 5 = comparator out 6 = timer ct32b0 match2 7=gpio_int_bmatch 8=arm_txev 9=debug_halted"]
    #[inline(always)]
    pub fn inp_n(&self) -> INP_N_R {
        INP_N_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Input mux register for SCT input n (n = 0 to 3). 0 = sct input 0 1=sct input 1 2= sct input 2 3= sct gpio input 3 4= adc_thcmp_irq 5 = comparator out 6 = timer ct32b0 match2 7=gpio_int_bmatch 8=arm_txev 9=debug_halted"]
    #[inline(always)]
    pub fn inp_n(&mut self) -> INP_N_W {
        INP_N_W { w: self }
    }
}
