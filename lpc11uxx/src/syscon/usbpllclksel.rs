#[doc = "Register `USBPLLCLKSEL` reader"]
pub struct R(crate::R<USBPLLCLKSEL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<USBPLLCLKSEL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<USBPLLCLKSEL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<USBPLLCLKSEL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `USBPLLCLKSEL` writer"]
pub struct W(crate::W<USBPLLCLKSEL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<USBPLLCLKSEL_SPEC>;
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
impl From<crate::W<USBPLLCLKSEL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<USBPLLCLKSEL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "USB PLL clock source\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SEL_A {
    #[doc = "0: IRC. The USB PLL clock source must be switched to system oscillator for correct full-speed USB operation. The IRC is suitable for low-speed USB operation."]
    IRC_THE_USB_PLL_CLO = 0,
    #[doc = "1: System oscillator"]
    SYSTEM_OSCILLATOR = 1,
}
impl From<SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: SEL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `SEL` reader - USB PLL clock source"]
pub struct SEL_R(crate::FieldReader<u8, SEL_A>);
impl SEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        SEL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SEL_A {
        match self.bits {
            0 => SEL_A::IRC_THE_USB_PLL_CLO,
            1 => SEL_A::SYSTEM_OSCILLATOR,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `IRC_THE_USB_PLL_CLO`"]
    #[inline(always)]
    pub fn is_irc_the_usb_pll_clo(&self) -> bool {
        **self == SEL_A::IRC_THE_USB_PLL_CLO
    }
    #[doc = "Checks if the value of the field is `SYSTEM_OSCILLATOR`"]
    #[inline(always)]
    pub fn is_system_oscillator(&self) -> bool {
        **self == SEL_A::SYSTEM_OSCILLATOR
    }
}
impl core::ops::Deref for SEL_R {
    type Target = crate::FieldReader<u8, SEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SEL` writer - USB PLL clock source"]
pub struct SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> SEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SEL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "IRC. The USB PLL clock source must be switched to system oscillator for correct full-speed USB operation. The IRC is suitable for low-speed USB operation."]
    #[inline(always)]
    pub fn irc_the_usb_pll_clo(self) -> &'a mut W {
        self.variant(SEL_A::IRC_THE_USB_PLL_CLO)
    }
    #[doc = "System oscillator"]
    #[inline(always)]
    pub fn system_oscillator(self) -> &'a mut W {
        self.variant(SEL_A::SYSTEM_OSCILLATOR)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | (value as u32 & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - USB PLL clock source"]
    #[inline(always)]
    pub fn sel(&self) -> SEL_R {
        SEL_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - USB PLL clock source"]
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
#[doc = "USB PLL clock source select\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usbpllclksel](index.html) module"]
pub struct USBPLLCLKSEL_SPEC;
impl crate::RegisterSpec for USBPLLCLKSEL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [usbpllclksel::R](R) reader structure"]
impl crate::Readable for USBPLLCLKSEL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [usbpllclksel::W](W) writer structure"]
impl crate::Writable for USBPLLCLKSEL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets USBPLLCLKSEL to value 0"]
impl crate::Resettable for USBPLLCLKSEL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
