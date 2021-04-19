#[doc = "Register `DMA_ITRIG_INMUX[%s]` reader"]
pub struct R(crate::R<DMA_ITRIG_INMUX_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMA_ITRIG_INMUX_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<DMA_ITRIG_INMUX_SPEC>> for R {
    fn from(reader: crate::R<DMA_ITRIG_INMUX_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DMA_ITRIG_INMUX[%s]` writer"]
pub struct W(crate::W<DMA_ITRIG_INMUX_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DMA_ITRIG_INMUX_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl core::convert::From<crate::W<DMA_ITRIG_INMUX_SPEC>> for W {
    fn from(writer: crate::W<DMA_ITRIG_INMUX_SPEC>) -> Self {
        W(writer)
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
#[doc = "Field `INP` reader - Input mux register for trigger inputs 0 to 23 connected to DMA channel 0. Selects from ADC, SCT, ACMP, pin interrupts, and DMA requests."]
pub struct INP_R(crate::FieldReader<u8, INP_A>);
impl INP_R {
    pub(crate) fn new(bits: u8) -> Self {
        INP_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<INP_A> {
        match self.bits {
            0 => Some(INP_A::ADC_SEQA_IRQ),
            1 => Some(INP_A::ADC_SEQB_IRQ),
            2 => Some(INP_A::SCT_DMA0),
            3 => Some(INP_A::SCT_DMA1),
            4 => Some(INP_A::ACMP_O),
            5 => Some(INP_A::PININT0),
            6 => Some(INP_A::PININT1),
            7 => Some(INP_A::DMA_INMUX_INMUX0),
            8 => Some(INP_A::DMA_INMUX_INMUX1),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `ADC_SEQA_IRQ`"]
    #[inline(always)]
    pub fn is_adc_seqa_irq(&self) -> bool {
        **self == INP_A::ADC_SEQA_IRQ
    }
    #[doc = "Checks if the value of the field is `ADC_SEQB_IRQ`"]
    #[inline(always)]
    pub fn is_adc_seqb_irq(&self) -> bool {
        **self == INP_A::ADC_SEQB_IRQ
    }
    #[doc = "Checks if the value of the field is `SCT_DMA0`"]
    #[inline(always)]
    pub fn is_sct_dma0(&self) -> bool {
        **self == INP_A::SCT_DMA0
    }
    #[doc = "Checks if the value of the field is `SCT_DMA1`"]
    #[inline(always)]
    pub fn is_sct_dma1(&self) -> bool {
        **self == INP_A::SCT_DMA1
    }
    #[doc = "Checks if the value of the field is `ACMP_O`"]
    #[inline(always)]
    pub fn is_acmp_o(&self) -> bool {
        **self == INP_A::ACMP_O
    }
    #[doc = "Checks if the value of the field is `PININT0`"]
    #[inline(always)]
    pub fn is_pinint0(&self) -> bool {
        **self == INP_A::PININT0
    }
    #[doc = "Checks if the value of the field is `PININT1`"]
    #[inline(always)]
    pub fn is_pinint1(&self) -> bool {
        **self == INP_A::PININT1
    }
    #[doc = "Checks if the value of the field is `DMA_INMUX_INMUX0`"]
    #[inline(always)]
    pub fn is_dma_inmux_inmux0(&self) -> bool {
        **self == INP_A::DMA_INMUX_INMUX0
    }
    #[doc = "Checks if the value of the field is `DMA_INMUX_INMUX1`"]
    #[inline(always)]
    pub fn is_dma_inmux_inmux1(&self) -> bool {
        **self == INP_A::DMA_INMUX_INMUX1
    }
}
impl core::ops::Deref for INP_R {
    type Target = crate::FieldReader<u8, INP_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INP` writer - Input mux register for trigger inputs 0 to 23 connected to DMA channel 0. Selects from ADC, SCT, ACMP, pin interrupts, and DMA requests."]
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
        self.w.bits = (self.w.bits & !0x0f) | (value as u32 & 0x0f);
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
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Trigger select register for DMA channel\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_itrig_inmux](index.html) module"]
pub struct DMA_ITRIG_INMUX_SPEC;
impl crate::RegisterSpec for DMA_ITRIG_INMUX_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dma_itrig_inmux::R](R) reader structure"]
impl crate::Readable for DMA_ITRIG_INMUX_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dma_itrig_inmux::W](W) writer structure"]
impl crate::Writable for DMA_ITRIG_INMUX_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DMA_ITRIG_INMUX[%s]
to value 0x0f"]
impl crate::Resettable for DMA_ITRIG_INMUX_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0f
    }
}
