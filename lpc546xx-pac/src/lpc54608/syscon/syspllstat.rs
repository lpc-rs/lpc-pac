///Register `SYSPLLSTAT` reader
pub struct R(crate::R<SYSPLLSTAT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SYSPLLSTAT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SYSPLLSTAT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SYSPLLSTAT_SPEC>) -> Self {
        R(reader)
    }
}
///Register `SYSPLLSTAT` writer
pub struct W(crate::W<SYSPLLSTAT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SYSPLLSTAT_SPEC>;
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
impl From<crate::W<SYSPLLSTAT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SYSPLLSTAT_SPEC>) -> Self {
        W(writer)
    }
}
///Field `LOCK` reader - PLL lock indicator.
pub struct LOCK_R(crate::FieldReader<bool, bool>);
impl LOCK_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        LOCK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LOCK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `LOCK` writer - PLL lock indicator.
pub struct LOCK_W<'a> {
    w: &'a mut W,
}
impl<'a> LOCK_W<'a> {
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
    ///Bit 0 - PLL lock indicator.
    #[inline(always)]
    pub fn lock(&self) -> LOCK_R {
        LOCK_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    ///Bit 0 - PLL lock indicator.
    #[inline(always)]
    pub fn lock(&mut self) -> LOCK_W {
        LOCK_W { w: self }
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///PLL status
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [syspllstat](index.html) module
pub struct SYSPLLSTAT_SPEC;
impl crate::RegisterSpec for SYSPLLSTAT_SPEC {
    type Ux = u32;
}
///`read()` method returns [syspllstat::R](R) reader structure
impl crate::Readable for SYSPLLSTAT_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [syspllstat::W](W) writer structure
impl crate::Writable for SYSPLLSTAT_SPEC {
    type Writer = W;
}
///`reset()` method sets SYSPLLSTAT to value 0
impl crate::Resettable for SYSPLLSTAT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
