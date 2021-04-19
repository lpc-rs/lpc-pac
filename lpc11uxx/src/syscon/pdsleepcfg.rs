#[doc = "Register `PDSLEEPCFG` reader"]
pub struct R(crate::R<PDSLEEPCFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PDSLEEPCFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<PDSLEEPCFG_SPEC>> for R {
    fn from(reader: crate::R<PDSLEEPCFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PDSLEEPCFG` writer"]
pub struct W(crate::W<PDSLEEPCFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PDSLEEPCFG_SPEC>;
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
impl core::convert::From<crate::W<PDSLEEPCFG_SPEC>> for W {
    fn from(writer: crate::W<PDSLEEPCFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "BOD power-down control for Deep-sleep and Power-down mode\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BOD_PD_A {
    #[doc = "0: Powered"]
    POWERED = 0,
    #[doc = "1: Powered down"]
    POWERED_DOWN = 1,
}
impl From<BOD_PD_A> for bool {
    #[inline(always)]
    fn from(variant: BOD_PD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BOD_PD` reader - BOD power-down control for Deep-sleep and Power-down mode"]
pub struct BOD_PD_R(crate::FieldReader<bool, BOD_PD_A>);
impl BOD_PD_R {
    pub(crate) fn new(bits: bool) -> Self {
        BOD_PD_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BOD_PD_A {
        match self.bits {
            false => BOD_PD_A::POWERED,
            true => BOD_PD_A::POWERED_DOWN,
        }
    }
    #[doc = "Checks if the value of the field is `POWERED`"]
    #[inline(always)]
    pub fn is_powered(&self) -> bool {
        **self == BOD_PD_A::POWERED
    }
    #[doc = "Checks if the value of the field is `POWERED_DOWN`"]
    #[inline(always)]
    pub fn is_powered_down(&self) -> bool {
        **self == BOD_PD_A::POWERED_DOWN
    }
}
impl core::ops::Deref for BOD_PD_R {
    type Target = crate::FieldReader<bool, BOD_PD_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BOD_PD` writer - BOD power-down control for Deep-sleep and Power-down mode"]
pub struct BOD_PD_W<'a> {
    w: &'a mut W,
}
impl<'a> BOD_PD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BOD_PD_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Powered"]
    #[inline(always)]
    pub fn powered(self) -> &'a mut W {
        self.variant(BOD_PD_A::POWERED)
    }
    #[doc = "Powered down"]
    #[inline(always)]
    pub fn powered_down(self) -> &'a mut W {
        self.variant(BOD_PD_A::POWERED_DOWN)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 3)) | ((value as u32 & 0x01) << 3);
        self.w
    }
}
#[doc = "Watchdog oscillator power-down control for Deep-sleep and Power-down mode\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WDTOSC_PD_A {
    #[doc = "0: Powered"]
    POWERED = 0,
    #[doc = "1: Powered down"]
    POWERED_DOWN = 1,
}
impl From<WDTOSC_PD_A> for bool {
    #[inline(always)]
    fn from(variant: WDTOSC_PD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WDTOSC_PD` reader - Watchdog oscillator power-down control for Deep-sleep and Power-down mode"]
pub struct WDTOSC_PD_R(crate::FieldReader<bool, WDTOSC_PD_A>);
impl WDTOSC_PD_R {
    pub(crate) fn new(bits: bool) -> Self {
        WDTOSC_PD_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WDTOSC_PD_A {
        match self.bits {
            false => WDTOSC_PD_A::POWERED,
            true => WDTOSC_PD_A::POWERED_DOWN,
        }
    }
    #[doc = "Checks if the value of the field is `POWERED`"]
    #[inline(always)]
    pub fn is_powered(&self) -> bool {
        **self == WDTOSC_PD_A::POWERED
    }
    #[doc = "Checks if the value of the field is `POWERED_DOWN`"]
    #[inline(always)]
    pub fn is_powered_down(&self) -> bool {
        **self == WDTOSC_PD_A::POWERED_DOWN
    }
}
impl core::ops::Deref for WDTOSC_PD_R {
    type Target = crate::FieldReader<bool, WDTOSC_PD_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WDTOSC_PD` writer - Watchdog oscillator power-down control for Deep-sleep and Power-down mode"]
pub struct WDTOSC_PD_W<'a> {
    w: &'a mut W,
}
impl<'a> WDTOSC_PD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WDTOSC_PD_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Powered"]
    #[inline(always)]
    pub fn powered(self) -> &'a mut W {
        self.variant(WDTOSC_PD_A::POWERED)
    }
    #[doc = "Powered down"]
    #[inline(always)]
    pub fn powered_down(self) -> &'a mut W {
        self.variant(WDTOSC_PD_A::POWERED_DOWN)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 6)) | ((value as u32 & 0x01) << 6);
        self.w
    }
}
impl R {
    #[doc = "Bit 3 - BOD power-down control for Deep-sleep and Power-down mode"]
    #[inline(always)]
    pub fn bod_pd(&self) -> BOD_PD_R {
        BOD_PD_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Watchdog oscillator power-down control for Deep-sleep and Power-down mode"]
    #[inline(always)]
    pub fn wdtosc_pd(&self) -> WDTOSC_PD_R {
        WDTOSC_PD_R::new(((self.bits >> 6) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 3 - BOD power-down control for Deep-sleep and Power-down mode"]
    #[inline(always)]
    pub fn bod_pd(&mut self) -> BOD_PD_W {
        BOD_PD_W { w: self }
    }
    #[doc = "Bit 6 - Watchdog oscillator power-down control for Deep-sleep and Power-down mode"]
    #[inline(always)]
    pub fn wdtosc_pd(&mut self) -> WDTOSC_PD_W {
        WDTOSC_PD_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Power-down states in deep-sleep mode\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pdsleepcfg](index.html) module"]
pub struct PDSLEEPCFG_SPEC;
impl crate::RegisterSpec for PDSLEEPCFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pdsleepcfg::R](R) reader structure"]
impl crate::Readable for PDSLEEPCFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pdsleepcfg::W](W) writer structure"]
impl crate::Writable for PDSLEEPCFG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PDSLEEPCFG to value 0xffff"]
impl crate::Resettable for PDSLEEPCFG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xffff
    }
}
