///Register `MAC_SYS_TIME_HWORD_SCND` reader
pub struct R(crate::R<MAC_SYS_TIME_HWORD_SCND_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MAC_SYS_TIME_HWORD_SCND_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MAC_SYS_TIME_HWORD_SCND_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MAC_SYS_TIME_HWORD_SCND_SPEC>) -> Self {
        R(reader)
    }
}
///Register `MAC_SYS_TIME_HWORD_SCND` writer
pub struct W(crate::W<MAC_SYS_TIME_HWORD_SCND_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MAC_SYS_TIME_HWORD_SCND_SPEC>;
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
impl From<crate::W<MAC_SYS_TIME_HWORD_SCND_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MAC_SYS_TIME_HWORD_SCND_SPEC>) -> Self {
        W(writer)
    }
}
///Field `TSHWR` reader - Time stamp higher word Contains the most significant 16-bits of the Time stamp seconds value.
pub struct TSHWR_R(crate::FieldReader<u16, u16>);
impl TSHWR_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        TSHWR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TSHWR_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `TSHWR` writer - Time stamp higher word Contains the most significant 16-bits of the Time stamp seconds value.
pub struct TSHWR_W<'a> {
    w: &'a mut W,
}
impl<'a> TSHWR_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | (value as u32 & 0xffff);
        self.w
    }
}
impl R {
    ///Bits 0:15 - Time stamp higher word Contains the most significant 16-bits of the Time stamp seconds value.
    #[inline(always)]
    pub fn tshwr(&self) -> TSHWR_R {
        TSHWR_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    ///Bits 0:15 - Time stamp higher word Contains the most significant 16-bits of the Time stamp seconds value.
    #[inline(always)]
    pub fn tshwr(&mut self) -> TSHWR_W {
        TSHWR_W { w: self }
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///no description available
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [mac_sys_time_hword_scnd](index.html) module
pub struct MAC_SYS_TIME_HWORD_SCND_SPEC;
impl crate::RegisterSpec for MAC_SYS_TIME_HWORD_SCND_SPEC {
    type Ux = u32;
}
///`read()` method returns [mac_sys_time_hword_scnd::R](R) reader structure
impl crate::Readable for MAC_SYS_TIME_HWORD_SCND_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [mac_sys_time_hword_scnd::W](W) writer structure
impl crate::Writable for MAC_SYS_TIME_HWORD_SCND_SPEC {
    type Writer = W;
}
///`reset()` method sets MAC_SYS_TIME_HWORD_SCND to value 0
impl crate::Resettable for MAC_SYS_TIME_HWORD_SCND_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
