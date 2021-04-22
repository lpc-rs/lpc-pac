#[doc = "Register `SCT0_INMUX[%s]` reader"]
pub struct R(crate::R<SCT0_INMUX_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SCT0_INMUX_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<SCT0_INMUX_SPEC>> for R {
    fn from(reader: crate::R<SCT0_INMUX_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SCT0_INMUX[%s]` writer"]
pub struct W(crate::W<SCT0_INMUX_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SCT0_INMUX_SPEC>;
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
impl core::convert::From<crate::W<SCT0_INMUX_SPEC>> for W {
    fn from(writer: crate::W<SCT0_INMUX_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Input mux register for SCT input n (n = 0 to 3). 0 = sct input 0 1=sct input 1 2= sct input 2 3= sct input 3 4= adc_thcmp_irq 5 = comparator out 6=arm_txev 7=debug_halted\n\nValue on reset: 15"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum INP_N_A {
    #[doc = "0: SCT_PIN0"]
    SCT_PIN0 = 0,
    #[doc = "1: SCT_PIN1"]
    SCT_PIN1 = 1,
    #[doc = "2: SCT_PIN2"]
    SCT_PIN2 = 2,
    #[doc = "3: SCT_PIN3"]
    SCT_PIN3 = 3,
    #[doc = "4: ADC_THCMP_IRQ"]
    ADC_THCMP_IRQ = 4,
    #[doc = "5: ACMP_O"]
    ACMP_O = 5,
    #[doc = "6: ARM_TXEV"]
    ARM_TXEV = 6,
    #[doc = "7: DEBUG_HALTED"]
    DEBUG_HALTED = 7,
}
impl From<INP_N_A> for u8 {
    #[inline(always)]
    fn from(variant: INP_N_A) -> Self {
        variant as _
    }
}
#[doc = "Field `INP_N` reader - Input mux register for SCT input n (n = 0 to 3). 0 = sct input 0 1=sct input 1 2= sct input 2 3= sct input 3 4= adc_thcmp_irq 5 = comparator out 6=arm_txev 7=debug_halted"]
pub struct INP_N_R(crate::FieldReader<u8, INP_N_A>);
impl INP_N_R {
    pub(crate) fn new(bits: u8) -> Self {
        INP_N_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<INP_N_A> {
        match self.bits {
            0 => Some(INP_N_A::SCT_PIN0),
            1 => Some(INP_N_A::SCT_PIN1),
            2 => Some(INP_N_A::SCT_PIN2),
            3 => Some(INP_N_A::SCT_PIN3),
            4 => Some(INP_N_A::ADC_THCMP_IRQ),
            5 => Some(INP_N_A::ACMP_O),
            6 => Some(INP_N_A::ARM_TXEV),
            7 => Some(INP_N_A::DEBUG_HALTED),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `SCT_PIN0`"]
    #[inline(always)]
    pub fn is_sct_pin0(&self) -> bool {
        **self == INP_N_A::SCT_PIN0
    }
    #[doc = "Checks if the value of the field is `SCT_PIN1`"]
    #[inline(always)]
    pub fn is_sct_pin1(&self) -> bool {
        **self == INP_N_A::SCT_PIN1
    }
    #[doc = "Checks if the value of the field is `SCT_PIN2`"]
    #[inline(always)]
    pub fn is_sct_pin2(&self) -> bool {
        **self == INP_N_A::SCT_PIN2
    }
    #[doc = "Checks if the value of the field is `SCT_PIN3`"]
    #[inline(always)]
    pub fn is_sct_pin3(&self) -> bool {
        **self == INP_N_A::SCT_PIN3
    }
    #[doc = "Checks if the value of the field is `ADC_THCMP_IRQ`"]
    #[inline(always)]
    pub fn is_adc_thcmp_irq(&self) -> bool {
        **self == INP_N_A::ADC_THCMP_IRQ
    }
    #[doc = "Checks if the value of the field is `ACMP_O`"]
    #[inline(always)]
    pub fn is_acmp_o(&self) -> bool {
        **self == INP_N_A::ACMP_O
    }
    #[doc = "Checks if the value of the field is `ARM_TXEV`"]
    #[inline(always)]
    pub fn is_arm_txev(&self) -> bool {
        **self == INP_N_A::ARM_TXEV
    }
    #[doc = "Checks if the value of the field is `DEBUG_HALTED`"]
    #[inline(always)]
    pub fn is_debug_halted(&self) -> bool {
        **self == INP_N_A::DEBUG_HALTED
    }
}
impl core::ops::Deref for INP_N_R {
    type Target = crate::FieldReader<u8, INP_N_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INP_N` writer - Input mux register for SCT input n (n = 0 to 3). 0 = sct input 0 1=sct input 1 2= sct input 2 3= sct input 3 4= adc_thcmp_irq 5 = comparator out 6=arm_txev 7=debug_halted"]
pub struct INP_N_W<'a> {
    w: &'a mut W,
}
impl<'a> INP_N_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: INP_N_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "SCT_PIN0"]
    #[inline(always)]
    pub fn sct_pin0(self) -> &'a mut W {
        self.variant(INP_N_A::SCT_PIN0)
    }
    #[doc = "SCT_PIN1"]
    #[inline(always)]
    pub fn sct_pin1(self) -> &'a mut W {
        self.variant(INP_N_A::SCT_PIN1)
    }
    #[doc = "SCT_PIN2"]
    #[inline(always)]
    pub fn sct_pin2(self) -> &'a mut W {
        self.variant(INP_N_A::SCT_PIN2)
    }
    #[doc = "SCT_PIN3"]
    #[inline(always)]
    pub fn sct_pin3(self) -> &'a mut W {
        self.variant(INP_N_A::SCT_PIN3)
    }
    #[doc = "ADC_THCMP_IRQ"]
    #[inline(always)]
    pub fn adc_thcmp_irq(self) -> &'a mut W {
        self.variant(INP_N_A::ADC_THCMP_IRQ)
    }
    #[doc = "ACMP_O"]
    #[inline(always)]
    pub fn acmp_o(self) -> &'a mut W {
        self.variant(INP_N_A::ACMP_O)
    }
    #[doc = "ARM_TXEV"]
    #[inline(always)]
    pub fn arm_txev(self) -> &'a mut W {
        self.variant(INP_N_A::ARM_TXEV)
    }
    #[doc = "DEBUG_HALTED"]
    #[inline(always)]
    pub fn debug_halted(self) -> &'a mut W {
        self.variant(INP_N_A::DEBUG_HALTED)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | (value as u32 & 0x0f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - Input mux register for SCT input n (n = 0 to 3). 0 = sct input 0 1=sct input 1 2= sct input 2 3= sct input 3 4= adc_thcmp_irq 5 = comparator out 6=arm_txev 7=debug_halted"]
    #[inline(always)]
    pub fn inp_n(&self) -> INP_N_R {
        INP_N_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Input mux register for SCT input n (n = 0 to 3). 0 = sct input 0 1=sct input 1 2= sct input 2 3= sct input 3 4= adc_thcmp_irq 5 = comparator out 6=arm_txev 7=debug_halted"]
    #[inline(always)]
    pub fn inp_n(&mut self) -> INP_N_W {
        INP_N_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "input select register for SCT\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sct0_inmux](index.html) module"]
pub struct SCT0_INMUX_SPEC;
impl crate::RegisterSpec for SCT0_INMUX_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sct0_inmux::R](R) reader structure"]
impl crate::Readable for SCT0_INMUX_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sct0_inmux::W](W) writer structure"]
impl crate::Writable for SCT0_INMUX_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SCT0_INMUX[%s]
to value 0x0f"]
impl crate::Resettable for SCT0_INMUX_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0f
    }
}
