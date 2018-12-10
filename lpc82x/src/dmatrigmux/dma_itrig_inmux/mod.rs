#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::DMA_ITRIG_INMUX {
    #[doc = r" Modifies the contents of the register"]
    #[inline]
    pub fn modify<F>(&self, f: F)
    where
        for<'w> F: FnOnce(&R, &'w mut W) -> &'w mut W,
    {
        let bits = self.register.get();
        let r = R { bits: bits };
        let mut w = W { bits: bits };
        f(&r, &mut w);
        self.register.set(w.bits);
    }
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
    #[doc = r" Writes to the register"]
    #[inline]
    pub fn write<F>(&self, f: F)
    where
        F: FnOnce(&mut W) -> &mut W,
    {
        let mut w = W::reset_value();
        f(&mut w);
        self.register.set(w.bits);
    }
    #[doc = r" Writes the reset value to the register"]
    #[inline]
    pub fn reset(&self) {
        self.write(|w| w)
    }
}
#[doc = "Possible values of the field `INP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INPR {
    #[doc = "ADC_SEQA_IRQ"]
    ADC_SEQA_IRQ,
    #[doc = "ADC_SEQB_IRQ"]
    ADC_SEQB_IRQ,
    #[doc = "SCT_DMA0"]
    SCT_DMA0,
    #[doc = "SCT_DMA1"]
    SCT_DMA1,
    #[doc = "ACMP_O"]
    ACMP_O,
    #[doc = "PININT0"]
    PININT0,
    #[doc = "PININT1"]
    PININT1,
    #[doc = "DMA trigger mux 0. (DMA_INMUX_INMUX0)."]
    DMA_TRIGGER_MUX_0,
    #[doc = "DMA trigger mux 1. (DMA_INMUX_INMUX1)"]
    DMA_TRIGGER_MUX_1,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl INPR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            INPR::ADC_SEQA_IRQ => 0,
            INPR::ADC_SEQB_IRQ => 1,
            INPR::SCT_DMA0 => 2,
            INPR::SCT_DMA1 => 3,
            INPR::ACMP_O => 4,
            INPR::PININT0 => 5,
            INPR::PININT1 => 6,
            INPR::DMA_TRIGGER_MUX_0 => 7,
            INPR::DMA_TRIGGER_MUX_1 => 8,
            INPR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> INPR {
        match value {
            0 => INPR::ADC_SEQA_IRQ,
            1 => INPR::ADC_SEQB_IRQ,
            2 => INPR::SCT_DMA0,
            3 => INPR::SCT_DMA1,
            4 => INPR::ACMP_O,
            5 => INPR::PININT0,
            6 => INPR::PININT1,
            7 => INPR::DMA_TRIGGER_MUX_0,
            8 => INPR::DMA_TRIGGER_MUX_1,
            i => INPR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `ADC_SEQA_IRQ`"]
    #[inline]
    pub fn is_adc_seqa_irq(&self) -> bool {
        *self == INPR::ADC_SEQA_IRQ
    }
    #[doc = "Checks if the value of the field is `ADC_SEQB_IRQ`"]
    #[inline]
    pub fn is_adc_seqb_irq(&self) -> bool {
        *self == INPR::ADC_SEQB_IRQ
    }
    #[doc = "Checks if the value of the field is `SCT_DMA0`"]
    #[inline]
    pub fn is_sct_dma0(&self) -> bool {
        *self == INPR::SCT_DMA0
    }
    #[doc = "Checks if the value of the field is `SCT_DMA1`"]
    #[inline]
    pub fn is_sct_dma1(&self) -> bool {
        *self == INPR::SCT_DMA1
    }
    #[doc = "Checks if the value of the field is `ACMP_O`"]
    #[inline]
    pub fn is_acmp_o(&self) -> bool {
        *self == INPR::ACMP_O
    }
    #[doc = "Checks if the value of the field is `PININT0`"]
    #[inline]
    pub fn is_pinint0(&self) -> bool {
        *self == INPR::PININT0
    }
    #[doc = "Checks if the value of the field is `PININT1`"]
    #[inline]
    pub fn is_pinint1(&self) -> bool {
        *self == INPR::PININT1
    }
    #[doc = "Checks if the value of the field is `DMA_TRIGGER_MUX_0`"]
    #[inline]
    pub fn is_dma_trigger_mux_0(&self) -> bool {
        *self == INPR::DMA_TRIGGER_MUX_0
    }
    #[doc = "Checks if the value of the field is `DMA_TRIGGER_MUX_1`"]
    #[inline]
    pub fn is_dma_trigger_mux_1(&self) -> bool {
        *self == INPR::DMA_TRIGGER_MUX_1
    }
}
#[doc = "Values that can be written to the field `INP`"]
pub enum INPW {
    #[doc = "ADC_SEQA_IRQ"]
    ADC_SEQA_IRQ,
    #[doc = "ADC_SEQB_IRQ"]
    ADC_SEQB_IRQ,
    #[doc = "SCT_DMA0"]
    SCT_DMA0,
    #[doc = "SCT_DMA1"]
    SCT_DMA1,
    #[doc = "ACMP_O"]
    ACMP_O,
    #[doc = "PININT0"]
    PININT0,
    #[doc = "PININT1"]
    PININT1,
    #[doc = "DMA trigger mux 0. (DMA_INMUX_INMUX0)."]
    DMA_TRIGGER_MUX_0,
    #[doc = "DMA trigger mux 1. (DMA_INMUX_INMUX1)"]
    DMA_TRIGGER_MUX_1,
}
impl INPW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            INPW::ADC_SEQA_IRQ => 0,
            INPW::ADC_SEQB_IRQ => 1,
            INPW::SCT_DMA0 => 2,
            INPW::SCT_DMA1 => 3,
            INPW::ACMP_O => 4,
            INPW::PININT0 => 5,
            INPW::PININT1 => 6,
            INPW::DMA_TRIGGER_MUX_0 => 7,
            INPW::DMA_TRIGGER_MUX_1 => 8,
        }
    }
}
#[doc = r" Proxy"]
pub struct _INPW<'a> {
    w: &'a mut W,
}
impl<'a> _INPW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: INPW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "ADC_SEQA_IRQ"]
    #[inline]
    pub fn adc_seqa_irq(self) -> &'a mut W {
        self.variant(INPW::ADC_SEQA_IRQ)
    }
    #[doc = "ADC_SEQB_IRQ"]
    #[inline]
    pub fn adc_seqb_irq(self) -> &'a mut W {
        self.variant(INPW::ADC_SEQB_IRQ)
    }
    #[doc = "SCT_DMA0"]
    #[inline]
    pub fn sct_dma0(self) -> &'a mut W {
        self.variant(INPW::SCT_DMA0)
    }
    #[doc = "SCT_DMA1"]
    #[inline]
    pub fn sct_dma1(self) -> &'a mut W {
        self.variant(INPW::SCT_DMA1)
    }
    #[doc = "ACMP_O"]
    #[inline]
    pub fn acmp_o(self) -> &'a mut W {
        self.variant(INPW::ACMP_O)
    }
    #[doc = "PININT0"]
    #[inline]
    pub fn pinint0(self) -> &'a mut W {
        self.variant(INPW::PININT0)
    }
    #[doc = "PININT1"]
    #[inline]
    pub fn pinint1(self) -> &'a mut W {
        self.variant(INPW::PININT1)
    }
    #[doc = "DMA trigger mux 0. (DMA_INMUX_INMUX0)."]
    #[inline]
    pub fn dma_trigger_mux_0(self) -> &'a mut W {
        self.variant(INPW::DMA_TRIGGER_MUX_0)
    }
    #[doc = "DMA trigger mux 1. (DMA_INMUX_INMUX1)"]
    #[inline]
    pub fn dma_trigger_mux_1(self) -> &'a mut W {
        self.variant(INPW::DMA_TRIGGER_MUX_1)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:3 - Trigger input number (decimal value) for DMA channel n (n = 0 to 8). All other values are reserved."]
    #[inline]
    pub fn inp(&self) -> INPR {
        INPR::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 15 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:3 - Trigger input number (decimal value) for DMA channel n (n = 0 to 8). All other values are reserved."]
    #[inline]
    pub fn inp(&mut self) -> _INPW {
        _INPW { w: self }
    }
}
