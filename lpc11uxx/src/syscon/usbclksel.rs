#[doc = "Register `USBCLKSEL` reader"]
pub struct R(crate::R<USBCLKSEL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<USBCLKSEL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<USBCLKSEL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<USBCLKSEL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `USBCLKSEL` writer"]
pub struct W(crate::W<USBCLKSEL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<USBCLKSEL_SPEC>;
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
impl From<crate::W<USBCLKSEL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<USBCLKSEL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "USB clock source. Values 0x2 and 0x3 are reserved.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SEL_A {
    #[doc = "0: USB PLL out"]
    USB_PLL_OUT = 0,
    #[doc = "1: Main clock"]
    MAIN_CLOCK = 1,
}
impl From<SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: SEL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `SEL` reader - USB clock source. Values 0x2 and 0x3 are reserved."]
pub struct SEL_R(crate::FieldReader<u8, SEL_A>);
impl SEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        SEL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SEL_A> {
        match self.bits {
            0 => Some(SEL_A::USB_PLL_OUT),
            1 => Some(SEL_A::MAIN_CLOCK),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `USB_PLL_OUT`"]
    #[inline(always)]
    pub fn is_usb_pll_out(&self) -> bool {
        **self == SEL_A::USB_PLL_OUT
    }
    #[doc = "Checks if the value of the field is `MAIN_CLOCK`"]
    #[inline(always)]
    pub fn is_main_clock(&self) -> bool {
        **self == SEL_A::MAIN_CLOCK
    }
}
impl core::ops::Deref for SEL_R {
    type Target = crate::FieldReader<u8, SEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SEL` writer - USB clock source. Values 0x2 and 0x3 are reserved."]
pub struct SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> SEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SEL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "USB PLL out"]
    #[inline(always)]
    pub fn usb_pll_out(self) -> &'a mut W {
        self.variant(SEL_A::USB_PLL_OUT)
    }
    #[doc = "Main clock"]
    #[inline(always)]
    pub fn main_clock(self) -> &'a mut W {
        self.variant(SEL_A::MAIN_CLOCK)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | (value as u32 & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - USB clock source. Values 0x2 and 0x3 are reserved."]
    #[inline(always)]
    pub fn sel(&self) -> SEL_R {
        SEL_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - USB clock source. Values 0x2 and 0x3 are reserved."]
    #[inline(always)]
    pub fn sel(&mut self) -> SEL_W {
        SEL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USB clock source select\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usbclksel](index.html) module"]
pub struct USBCLKSEL_SPEC;
impl crate::RegisterSpec for USBCLKSEL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [usbclksel::R](R) reader structure"]
impl crate::Readable for USBCLKSEL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [usbclksel::W](W) writer structure"]
impl crate::Writable for USBCLKSEL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets USBCLKSEL to value 0"]
impl crate::Resettable for USBCLKSEL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
