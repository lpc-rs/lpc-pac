#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::DMA_ITRIG_PINMUX {
    #[doc = r"Modifies the contents of the register"]
    #[inline(always)]
    pub fn modify<F>(&self, f: F)
    where
        for<'w> F: FnOnce(&R, &'w mut W) -> &'w mut W,
    {
        let bits = self.register.get();
        self.register.set(f(&R { bits }, &mut W { bits }).bits);
    }
    #[doc = r"Reads the contents of the register"]
    #[inline(always)]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
    #[doc = r"Writes to the register"]
    #[inline(always)]
    pub fn write<F>(&self, f: F)
    where
        F: FnOnce(&mut W) -> &mut W,
    {
        self.register.set(
            f(&mut W {
                bits: Self::reset_value(),
            })
            .bits,
        );
    }
    #[doc = r"Reset value of the register"]
    #[inline(always)]
    pub const fn reset_value() -> u32 {
        0x1f
    }
    #[doc = r"Writes the reset value to the register"]
    #[inline(always)]
    pub fn reset(&self) {
        self.register.set(Self::reset_value())
    }
}
#[doc = r"Reader of the field"]
pub type INP_N_R = crate::FR<u8, u8>;
#[doc = r"Proxy"]
pub struct _INP_NW<'a> {
    w: &'a mut W,
}
impl<'a> _INP_NW<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | ((value as u32) & 0x1f);
        self.w
    }
}
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:4 - Trigger input number (decimal value) to DMA channel n. All other values are reserved. 0 = ADC0_SEQA_IRQ 1 = ADC0_SEQB_IRQ 2 = CT16B0_MAT0 3 = CT16B1_MAT0 4 = CT32B0_MAT0 5 = CT16B1_MAT0 6 = PINT0 ( pin interrupt 0) 7 = PINT1 (pin interrupt1 ) 8 = SCT0_DMA0 9 = SCT0_DMA1 10 = SCT1_DMA0 11 = SCT1_DMA1"]
    #[inline(always)]
    pub fn inp_n(&self) -> INP_N_R {
        INP_N_R::new((self.bits() & 0x1f) as u8)
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:4 - Trigger input number (decimal value) to DMA channel n. All other values are reserved. 0 = ADC0_SEQA_IRQ 1 = ADC0_SEQB_IRQ 2 = CT16B0_MAT0 3 = CT16B1_MAT0 4 = CT32B0_MAT0 5 = CT16B1_MAT0 6 = PINT0 ( pin interrupt 0) 7 = PINT1 (pin interrupt1 ) 8 = SCT0_DMA0 9 = SCT0_DMA1 10 = SCT1_DMA0 11 = SCT1_DMA1"]
    #[inline(always)]
    pub fn inp_n(&mut self) -> _INP_NW {
        _INP_NW { w: self }
    }
}
