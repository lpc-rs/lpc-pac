///Register `MAC_SYS_TIME_SCND_UPD` reader
pub struct R(crate::R<MAC_SYS_TIME_SCND_UPD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MAC_SYS_TIME_SCND_UPD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MAC_SYS_TIME_SCND_UPD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MAC_SYS_TIME_SCND_UPD_SPEC>) -> Self {
        R(reader)
    }
}
///Register `MAC_SYS_TIME_SCND_UPD` writer
pub struct W(crate::W<MAC_SYS_TIME_SCND_UPD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MAC_SYS_TIME_SCND_UPD_SPEC>;
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
impl From<crate::W<MAC_SYS_TIME_SCND_UPD_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MAC_SYS_TIME_SCND_UPD_SPEC>) -> Self {
        W(writer)
    }
}
///Field `TSS` reader - Time stamp second The value in this field indicates the time, in seconds, to be initialized or added to the system time.
pub struct TSS_R(crate::FieldReader<u32, u32>);
impl TSS_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        TSS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TSS_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `TSS` writer - Time stamp second The value in this field indicates the time, in seconds, to be initialized or added to the system time.
pub struct TSS_W<'a> {
    w: &'a mut W,
}
impl<'a> TSS_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = value;
        self.w
    }
}
impl R {
    ///Bits 0:31 - Time stamp second The value in this field indicates the time, in seconds, to be initialized or added to the system time.
    #[inline(always)]
    pub fn tss(&self) -> TSS_R {
        TSS_R::new(self.bits)
    }
}
impl W {
    ///Bits 0:31 - Time stamp second The value in this field indicates the time, in seconds, to be initialized or added to the system time.
    #[inline(always)]
    pub fn tss(&mut self) -> TSS_W {
        TSS_W { w: self }
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
///For information about available fields see [mac_sys_time_scnd_upd](index.html) module
pub struct MAC_SYS_TIME_SCND_UPD_SPEC;
impl crate::RegisterSpec for MAC_SYS_TIME_SCND_UPD_SPEC {
    type Ux = u32;
}
///`read()` method returns [mac_sys_time_scnd_upd::R](R) reader structure
impl crate::Readable for MAC_SYS_TIME_SCND_UPD_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [mac_sys_time_scnd_upd::W](W) writer structure
impl crate::Writable for MAC_SYS_TIME_SCND_UPD_SPEC {
    type Writer = W;
}
///`reset()` method sets MAC_SYS_TIME_SCND_UPD to value 0
impl crate::Resettable for MAC_SYS_TIME_SCND_UPD_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
