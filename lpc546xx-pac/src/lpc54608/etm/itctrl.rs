///Register `ITCTRL` reader
pub struct R(crate::R<ITCTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ITCTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ITCTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ITCTRL_SPEC>) -> Self {
        R(reader)
    }
}
///Register `ITCTRL` writer
pub struct W(crate::W<ITCTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ITCTRL_SPEC>;
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
impl From<crate::W<ITCTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ITCTRL_SPEC>) -> Self {
        W(writer)
    }
}
///Field `Mode` reader - Enable integration mode. When this bit is set to 1, the device enters integration mode to enable Topology Detection or Integration Testing to be checked. On an ETM reset this bit is cleared to 0.
pub struct MODE_R(crate::FieldReader<bool, bool>);
impl MODE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        MODE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MODE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `Mode` writer - Enable integration mode. When this bit is set to 1, the device enters integration mode to enable Topology Detection or Integration Testing to be checked. On an ETM reset this bit is cleared to 0.
pub struct MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> MODE_W<'a> {
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
    ///Bit 0 - Enable integration mode. When this bit is set to 1, the device enters integration mode to enable Topology Detection or Integration Testing to be checked. On an ETM reset this bit is cleared to 0.
    #[inline(always)]
    pub fn mode(&self) -> MODE_R {
        MODE_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    ///Bit 0 - Enable integration mode. When this bit is set to 1, the device enters integration mode to enable Topology Detection or Integration Testing to be checked. On an ETM reset this bit is cleared to 0.
    #[inline(always)]
    pub fn mode(&mut self) -> MODE_W {
        MODE_W { w: self }
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///Integration Mode Control Register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [itctrl](index.html) module
pub struct ITCTRL_SPEC;
impl crate::RegisterSpec for ITCTRL_SPEC {
    type Ux = u32;
}
///`read()` method returns [itctrl::R](R) reader structure
impl crate::Readable for ITCTRL_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [itctrl::W](W) writer structure
impl crate::Writable for ITCTRL_SPEC {
    type Writer = W;
}
///`reset()` method sets ITCTRL to value 0
impl crate::Resettable for ITCTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
