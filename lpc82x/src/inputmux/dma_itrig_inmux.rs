#[doc = "Reader of register DMA_ITRIG_INMUX[%s]"]
pub type R = crate::R<u32, super::DMA_ITRIG_INMUX>;
#[doc = "Writer for register DMA_ITRIG_INMUX[%s]"]
pub type W = crate::W<u32, super::DMA_ITRIG_INMUX>;
#[doc = "Register DMA_ITRIG_INMUX[%s]
`reset()`'s with value 0x0f"]
impl crate::ResetValue for super::DMA_ITRIG_INMUX {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0f
    }
}
#[doc = "Input mux register for trigger inputs 0 to 23 connected to DMA channel 0. Selects from ADC, SCT, ACMP, pin interrupts, and DMA requests.\n\nValue on reset: 15"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum INP_A {
    #[doc = "0: ADC_SEQA_IRQ"]
    ADC_SEQA_IRQ = 0,
    #[doc = "1: ADC_SEQB_IRQ"]
    ADC_SEQB_IRQ = 1,
    #[doc = "2: SCT_DMA0"]
    SCT_DMA0 = 2,
    #[doc = "3: SCT_DMA1"]
    SCT_DMA1 = 3,
    #[doc = "4: ACMP_O"]
    ACMP_O = 4,
    #[doc = "5: PININT0"]
    PININT0 = 5,
    #[doc = "6: PININT1"]
    PININT1 = 6,
    #[doc = "7: DMA trigger mux 0"]
    DMA_INMUX_INMUX0 = 7,
    #[doc = "8: DMA trigger mux 1"]
    DMA_INMUX_INMUX1 = 8,
}
impl From<INP_A> for u8 {
    #[inline(always)]
    fn from(variant: INP_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `INP`"]
pub type INP_R = crate::R<u8, INP_A>;
impl INP_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, INP_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(INP_A::ADC_SEQA_IRQ),
            1 => Val(INP_A::ADC_SEQB_IRQ),
            2 => Val(INP_A::SCT_DMA0),
            3 => Val(INP_A::SCT_DMA1),
            4 => Val(INP_A::ACMP_O),
            5 => Val(INP_A::PININT0),
            6 => Val(INP_A::PININT1),
            7 => Val(INP_A::DMA_INMUX_INMUX0),
            8 => Val(INP_A::DMA_INMUX_INMUX1),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `ADC_SEQA_IRQ`"]
    #[inline(always)]
    pub fn is_adc_seqa_irq(&self) -> bool {
        *self == INP_A::ADC_SEQA_IRQ
    }
    #[doc = "Checks if the value of the field is `ADC_SEQB_IRQ`"]
    #[inline(always)]
    pub fn is_adc_seqb_irq(&self) -> bool {
        *self == INP_A::ADC_SEQB_IRQ
    }
    #[doc = "Checks if the value of the field is `SCT_DMA0`"]
    #[inline(always)]
    pub fn is_sct_dma0(&self) -> bool {
        *self == INP_A::SCT_DMA0
    }
    #[doc = "Checks if the value of the field is `SCT_DMA1`"]
    #[inline(always)]
    pub fn is_sct_dma1(&self) -> bool {
        *self == INP_A::SCT_DMA1
    }
    #[doc = "Checks if the value of the field is `ACMP_O`"]
    #[inline(always)]
    pub fn is_acmp_o(&self) -> bool {
        *self == INP_A::ACMP_O
    }
    #[doc = "Checks if the value of the field is `PININT0`"]
    #[inline(always)]
    pub fn is_pinint0(&self) -> bool {
        *self == INP_A::PININT0
    }
    #[doc = "Checks if the value of the field is `PININT1`"]
    #[inline(always)]
    pub fn is_pinint1(&self) -> bool {
        *self == INP_A::PININT1
    }
    #[doc = "Checks if the value of the field is `DMA_INMUX_INMUX0`"]
    #[inline(always)]
    pub fn is_dma_inmux_inmux0(&self) -> bool {
        *self == INP_A::DMA_INMUX_INMUX0
    }
    #[doc = "Checks if the value of the field is `DMA_INMUX_INMUX1`"]
    #[inline(always)]
    pub fn is_dma_inmux_inmux1(&self) -> bool {
        *self == INP_A::DMA_INMUX_INMUX1
    }
}
#[doc = "Write proxy for field `INP`"]
pub struct INP_W<'a> {
    w: &'a mut W,
}
impl<'a> INP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: INP_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "ADC_SEQA_IRQ"]
    #[inline(always)]
    pub fn adc_seqa_irq(self) -> &'a mut W {
        self.variant(INP_A::ADC_SEQA_IRQ)
    }
    #[doc = "ADC_SEQB_IRQ"]
    #[inline(always)]
    pub fn adc_seqb_irq(self) -> &'a mut W {
        self.variant(INP_A::ADC_SEQB_IRQ)
    }
    #[doc = "SCT_DMA0"]
    #[inline(always)]
    pub fn sct_dma0(self) -> &'a mut W {
        self.variant(INP_A::SCT_DMA0)
    }
    #[doc = "SCT_DMA1"]
    #[inline(always)]
    pub fn sct_dma1(self) -> &'a mut W {
        self.variant(INP_A::SCT_DMA1)
    }
    #[doc = "ACMP_O"]
    #[inline(always)]
    pub fn acmp_o(self) -> &'a mut W {
        self.variant(INP_A::ACMP_O)
    }
    #[doc = "PININT0"]
    #[inline(always)]
    pub fn pinint0(self) -> &'a mut W {
        self.variant(INP_A::PININT0)
    }
    #[doc = "PININT1"]
    #[inline(always)]
    pub fn pinint1(self) -> &'a mut W {
        self.variant(INP_A::PININT1)
    }
    #[doc = "DMA trigger mux 0"]
    #[inline(always)]
    pub fn dma_inmux_inmux0(self) -> &'a mut W {
        self.variant(INP_A::DMA_INMUX_INMUX0)
    }
    #[doc = "DMA trigger mux 1"]
    #[inline(always)]
    pub fn dma_inmux_inmux1(self) -> &'a mut W {
        self.variant(INP_A::DMA_INMUX_INMUX1)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - Input mux register for trigger inputs 0 to 23 connected to DMA channel 0. Selects from ADC, SCT, ACMP, pin interrupts, and DMA requests."]
    #[inline(always)]
    pub fn inp(&self) -> INP_R {
        INP_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Input mux register for trigger inputs 0 to 23 connected to DMA channel 0. Selects from ADC, SCT, ACMP, pin interrupts, and DMA requests."]
    #[inline(always)]
    pub fn inp(&mut self) -> INP_W {
        INP_W { w: self }
    }
}
