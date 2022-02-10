///Register `HWVADST10` reader
pub struct R(crate::R<HWVADST10_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HWVADST10_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HWVADST10_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HWVADST10_SPEC>) -> Self {
        R(reader)
    }
}
///Register `HWVADST10` writer
pub struct W(crate::W<HWVADST10_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HWVADST10_SPEC>;
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
impl From<crate::W<HWVADST10_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HWVADST10_SPEC>) -> Self {
        W(writer)
    }
}
///Stage 0
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ST10_A {
    ///0: Normal operation, waiting for HWVAD trigger event (stage 0).
    NORMAL = 0,
    ///1: Reset internal interrupt flag by writing a '1' pulse.
    RESET = 1,
}
impl From<ST10_A> for bool {
    #[inline(always)]
    fn from(variant: ST10_A) -> Self {
        variant as u8 != 0
    }
}
///Field `ST10` reader - Stage 0
pub struct ST10_R(crate::FieldReader<bool, ST10_A>);
impl ST10_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ST10_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> ST10_A {
        match self.bits {
            false => ST10_A::NORMAL,
            true => ST10_A::RESET,
        }
    }
    ///Checks if the value of the field is `NORMAL`
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        **self == ST10_A::NORMAL
    }
    ///Checks if the value of the field is `RESET`
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        **self == ST10_A::RESET
    }
}
impl core::ops::Deref for ST10_R {
    type Target = crate::FieldReader<bool, ST10_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `ST10` writer - Stage 0
pub struct ST10_W<'a> {
    w: &'a mut W,
}
impl<'a> ST10_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: ST10_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Normal operation, waiting for HWVAD trigger event (stage 0).
    #[inline(always)]
    pub fn normal(self) -> &'a mut W {
        self.variant(ST10_A::NORMAL)
    }
    ///Reset internal interrupt flag by writing a '1' pulse.
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(ST10_A::RESET)
    }
    ///Sets the field bit
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    ///Clears the field bit
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    ///Writes raw bits to the field
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
impl R {
    ///Bit 0 - Stage 0
    #[inline(always)]
    pub fn st10(&self) -> ST10_R {
        ST10_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    ///Bit 0 - Stage 0
    #[inline(always)]
    pub fn st10(&mut self) -> ST10_W {
        ST10_W { w: self }
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///HWVAD control register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [hwvadst10](index.html) module
pub struct HWVADST10_SPEC;
impl crate::RegisterSpec for HWVADST10_SPEC {
    type Ux = u32;
}
///`read()` method returns [hwvadst10::R](R) reader structure
impl crate::Readable for HWVADST10_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [hwvadst10::W](W) writer structure
impl crate::Writable for HWVADST10_SPEC {
    type Writer = W;
}
///`reset()` method sets HWVADST10 to value 0
impl crate::Resettable for HWVADST10_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
