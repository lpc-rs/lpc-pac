///Register `ETHSBDCTRL` reader
pub struct R(crate::R<ETHSBDCTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ETHSBDCTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ETHSBDCTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ETHSBDCTRL_SPEC>) -> Self {
        R(reader)
    }
}
///Register `ETHSBDCTRL` writer
pub struct W(crate::W<ETHSBDCTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ETHSBDCTRL_SPEC>;
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
impl From<crate::W<ETHSBDCTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ETHSBDCTRL_SPEC>) -> Self {
        W(writer)
    }
}
///Field `SBD_CTRL` reader - Sideband Flow Control.
pub struct SBD_CTRL_R(crate::FieldReader<u8, u8>);
impl SBD_CTRL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        SBD_CTRL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SBD_CTRL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `SBD_CTRL` writer - Sideband Flow Control.
pub struct SBD_CTRL_W<'a> {
    w: &'a mut W,
}
impl<'a> SBD_CTRL_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | (value as u32 & 0x03);
        self.w
    }
}
impl R {
    ///Bits 0:1 - Sideband Flow Control.
    #[inline(always)]
    pub fn sbd_ctrl(&self) -> SBD_CTRL_R {
        SBD_CTRL_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    ///Bits 0:1 - Sideband Flow Control.
    #[inline(always)]
    pub fn sbd_ctrl(&mut self) -> SBD_CTRL_W {
        SBD_CTRL_W { w: self }
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///Ethernet SBD flow control
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [ethsbdctrl](index.html) module
pub struct ETHSBDCTRL_SPEC;
impl crate::RegisterSpec for ETHSBDCTRL_SPEC {
    type Ux = u32;
}
///`read()` method returns [ethsbdctrl::R](R) reader structure
impl crate::Readable for ETHSBDCTRL_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [ethsbdctrl::W](W) writer structure
impl crate::Writable for ETHSBDCTRL_SPEC {
    type Writer = W;
}
///`reset()` method sets ETHSBDCTRL to value 0
impl crate::Resettable for ETHSBDCTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
