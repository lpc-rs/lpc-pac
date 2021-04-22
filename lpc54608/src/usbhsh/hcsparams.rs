#[doc = "Register `HCSPARAMS` reader"]
pub struct R(crate::R<HCSPARAMS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HCSPARAMS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<HCSPARAMS_SPEC>> for R {
    fn from(reader: crate::R<HCSPARAMS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `N_PORTS` reader - This register specifies the number of physical downstream ports implemented on this host controller."]
pub struct N_PORTS_R(crate::FieldReader<u8, u8>);
impl N_PORTS_R {
    pub(crate) fn new(bits: u8) -> Self {
        N_PORTS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for N_PORTS_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PPC` reader - This field indicates whether the host controller implementation includes port power control."]
pub struct PPC_R(crate::FieldReader<bool, bool>);
impl PPC_R {
    pub(crate) fn new(bits: bool) -> Self {
        PPC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PPC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P_INDICATOR` reader - This bit indicates whether the ports support port indicator control."]
pub struct P_INDICATOR_R(crate::FieldReader<bool, bool>);
impl P_INDICATOR_R {
    pub(crate) fn new(bits: bool) -> Self {
        P_INDICATOR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P_INDICATOR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:3 - This register specifies the number of physical downstream ports implemented on this host controller."]
    #[inline(always)]
    pub fn n_ports(&self) -> N_PORTS_R {
        N_PORTS_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 4 - This field indicates whether the host controller implementation includes port power control."]
    #[inline(always)]
    pub fn ppc(&self) -> PPC_R {
        PPC_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 16 - This bit indicates whether the ports support port indicator control."]
    #[inline(always)]
    pub fn p_indicator(&self) -> P_INDICATOR_R {
        P_INDICATOR_R::new(((self.bits >> 16) & 0x01) != 0)
    }
}
#[doc = "Host Controller Structural Parameters\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hcsparams](index.html) module"]
pub struct HCSPARAMS_SPEC;
impl crate::RegisterSpec for HCSPARAMS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hcsparams::R](R) reader structure"]
impl crate::Readable for HCSPARAMS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets HCSPARAMS to value 0x0001_0011"]
impl crate::Resettable for HCSPARAMS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0001_0011
    }
}
