#[doc = "Register `INTSTAT` reader"]
pub struct R(crate::R<INTSTAT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTSTAT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<INTSTAT_SPEC>> for R {
    fn from(reader: crate::R<INTSTAT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `MSTPENDING` reader - Master Pending."]
pub struct MSTPENDING_R(crate::FieldReader<bool, bool>);
impl MSTPENDING_R {
    pub(crate) fn new(bits: bool) -> Self {
        MSTPENDING_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MSTPENDING_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MSTARBLOSS` reader - Master Arbitration Loss flag."]
pub struct MSTARBLOSS_R(crate::FieldReader<bool, bool>);
impl MSTARBLOSS_R {
    pub(crate) fn new(bits: bool) -> Self {
        MSTARBLOSS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MSTARBLOSS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MSTSTSTPERR` reader - Master Start/Stop Error flag."]
pub struct MSTSTSTPERR_R(crate::FieldReader<bool, bool>);
impl MSTSTSTPERR_R {
    pub(crate) fn new(bits: bool) -> Self {
        MSTSTSTPERR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MSTSTSTPERR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SLVPENDING` reader - Slave Pending."]
pub struct SLVPENDING_R(crate::FieldReader<bool, bool>);
impl SLVPENDING_R {
    pub(crate) fn new(bits: bool) -> Self {
        SLVPENDING_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SLVPENDING_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SLVNOTSTR` reader - Slave Not Stretching status."]
pub struct SLVNOTSTR_R(crate::FieldReader<bool, bool>);
impl SLVNOTSTR_R {
    pub(crate) fn new(bits: bool) -> Self {
        SLVNOTSTR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SLVNOTSTR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SLVDESEL` reader - Slave Deselected flag."]
pub struct SLVDESEL_R(crate::FieldReader<bool, bool>);
impl SLVDESEL_R {
    pub(crate) fn new(bits: bool) -> Self {
        SLVDESEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SLVDESEL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MONRDY` reader - Monitor Ready."]
pub struct MONRDY_R(crate::FieldReader<bool, bool>);
impl MONRDY_R {
    pub(crate) fn new(bits: bool) -> Self {
        MONRDY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MONRDY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MONOV` reader - Monitor Overflow flag."]
pub struct MONOV_R(crate::FieldReader<bool, bool>);
impl MONOV_R {
    pub(crate) fn new(bits: bool) -> Self {
        MONOV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MONOV_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MONIDLE` reader - Monitor Idle flag."]
pub struct MONIDLE_R(crate::FieldReader<bool, bool>);
impl MONIDLE_R {
    pub(crate) fn new(bits: bool) -> Self {
        MONIDLE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MONIDLE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EVENTTIMEOUT` reader - Event time-out Interrupt flag."]
pub struct EVENTTIMEOUT_R(crate::FieldReader<bool, bool>);
impl EVENTTIMEOUT_R {
    pub(crate) fn new(bits: bool) -> Self {
        EVENTTIMEOUT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EVENTTIMEOUT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SCLTIMEOUT` reader - SCL time-out Interrupt flag."]
pub struct SCLTIMEOUT_R(crate::FieldReader<bool, bool>);
impl SCLTIMEOUT_R {
    pub(crate) fn new(bits: bool) -> Self {
        SCLTIMEOUT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SCLTIMEOUT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - Master Pending."]
    #[inline(always)]
    pub fn mstpending(&self) -> MSTPENDING_R {
        MSTPENDING_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 4 - Master Arbitration Loss flag."]
    #[inline(always)]
    pub fn mstarbloss(&self) -> MSTARBLOSS_R {
        MSTARBLOSS_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Master Start/Stop Error flag."]
    #[inline(always)]
    pub fn mstststperr(&self) -> MSTSTSTPERR_R {
        MSTSTSTPERR_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Slave Pending."]
    #[inline(always)]
    pub fn slvpending(&self) -> SLVPENDING_R {
        SLVPENDING_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Slave Not Stretching status."]
    #[inline(always)]
    pub fn slvnotstr(&self) -> SLVNOTSTR_R {
        SLVNOTSTR_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Slave Deselected flag."]
    #[inline(always)]
    pub fn slvdesel(&self) -> SLVDESEL_R {
        SLVDESEL_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Monitor Ready."]
    #[inline(always)]
    pub fn monrdy(&self) -> MONRDY_R {
        MONRDY_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Monitor Overflow flag."]
    #[inline(always)]
    pub fn monov(&self) -> MONOV_R {
        MONOV_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Monitor Idle flag."]
    #[inline(always)]
    pub fn monidle(&self) -> MONIDLE_R {
        MONIDLE_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Event time-out Interrupt flag."]
    #[inline(always)]
    pub fn eventtimeout(&self) -> EVENTTIMEOUT_R {
        EVENTTIMEOUT_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - SCL time-out Interrupt flag."]
    #[inline(always)]
    pub fn scltimeout(&self) -> SCLTIMEOUT_R {
        SCLTIMEOUT_R::new(((self.bits >> 25) & 0x01) != 0)
    }
}
#[doc = "Interrupt Status register for Master, Slave, and Monitor functions.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intstat](index.html) module"]
pub struct INTSTAT_SPEC;
impl crate::RegisterSpec for INTSTAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [intstat::R](R) reader structure"]
impl crate::Readable for INTSTAT_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets INTSTAT to value 0x0801"]
impl crate::Resettable for INTSTAT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0801
    }
}
