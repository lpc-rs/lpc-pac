///Register `ASYNCAPBCLKCTRL` reader
pub struct R(crate::R<ASYNCAPBCLKCTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ASYNCAPBCLKCTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ASYNCAPBCLKCTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ASYNCAPBCLKCTRL_SPEC>) -> Self {
        R(reader)
    }
}
///Register `ASYNCAPBCLKCTRL` writer
pub struct W(crate::W<ASYNCAPBCLKCTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ASYNCAPBCLKCTRL_SPEC>;
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
impl From<crate::W<ASYNCAPBCLKCTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ASYNCAPBCLKCTRL_SPEC>) -> Self {
        W(writer)
    }
}
///Field `CTIMER3` reader - Controls the clock for CTIMER3. 0 = Disable; 1 = Enable.
pub struct CTIMER3_R(crate::FieldReader<bool, bool>);
impl CTIMER3_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CTIMER3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CTIMER3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `CTIMER3` writer - Controls the clock for CTIMER3. 0 = Disable; 1 = Enable.
pub struct CTIMER3_W<'a> {
    w: &'a mut W,
}
impl<'a> CTIMER3_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | ((value as u32 & 0x01) << 13);
        self.w
    }
}
///Field `CTIMER4` reader - Controls the clock for CTIMER4. 0 = Disable; 1 = Enable.
pub struct CTIMER4_R(crate::FieldReader<bool, bool>);
impl CTIMER4_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CTIMER4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CTIMER4_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `CTIMER4` writer - Controls the clock for CTIMER4. 0 = Disable; 1 = Enable.
pub struct CTIMER4_W<'a> {
    w: &'a mut W,
}
impl<'a> CTIMER4_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | ((value as u32 & 0x01) << 14);
        self.w
    }
}
impl R {
    ///Bit 13 - Controls the clock for CTIMER3. 0 = Disable; 1 = Enable.
    #[inline(always)]
    pub fn ctimer3(&self) -> CTIMER3_R {
        CTIMER3_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    ///Bit 14 - Controls the clock for CTIMER4. 0 = Disable; 1 = Enable.
    #[inline(always)]
    pub fn ctimer4(&self) -> CTIMER4_R {
        CTIMER4_R::new(((self.bits >> 14) & 0x01) != 0)
    }
}
impl W {
    ///Bit 13 - Controls the clock for CTIMER3. 0 = Disable; 1 = Enable.
    #[inline(always)]
    pub fn ctimer3(&mut self) -> CTIMER3_W {
        CTIMER3_W { w: self }
    }
    ///Bit 14 - Controls the clock for CTIMER4. 0 = Disable; 1 = Enable.
    #[inline(always)]
    pub fn ctimer4(&mut self) -> CTIMER4_W {
        CTIMER4_W { w: self }
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///Async peripheral clock control
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [asyncapbclkctrl](index.html) module
pub struct ASYNCAPBCLKCTRL_SPEC;
impl crate::RegisterSpec for ASYNCAPBCLKCTRL_SPEC {
    type Ux = u32;
}
///`read()` method returns [asyncapbclkctrl::R](R) reader structure
impl crate::Readable for ASYNCAPBCLKCTRL_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [asyncapbclkctrl::W](W) writer structure
impl crate::Writable for ASYNCAPBCLKCTRL_SPEC {
    type Writer = W;
}
///`reset()` method sets ASYNCAPBCLKCTRL to value 0
impl crate::Resettable for ASYNCAPBCLKCTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
