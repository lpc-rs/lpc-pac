#[doc = "Register `CR` reader"]
pub struct R(crate::R<CR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<CR_SPEC>> for R {
    fn from(reader: crate::R<CR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CR` writer"]
pub struct W(crate::W<CR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CR_SPEC>;
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
impl core::convert::From<crate::W<CR_SPEC>> for W {
    fn from(writer: crate::W<CR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ETMPD` reader - ETM power down. This bit can be used by an implementation to control if the ETM is in a low power state. This bit must be cleared by the trace software tools at the beginning of a debug session. When this bit is set to 1, writes to some registers and fields might be ignored."]
pub struct ETMPD_R(crate::FieldReader<bool, bool>);
impl ETMPD_R {
    pub(crate) fn new(bits: bool) -> Self {
        ETMPD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ETMPD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ETMPD` writer - ETM power down. This bit can be used by an implementation to control if the ETM is in a low power state. This bit must be cleared by the trace software tools at the beginning of a debug session. When this bit is set to 1, writes to some registers and fields might be ignored."]
pub struct ETMPD_W<'a> {
    w: &'a mut W,
}
impl<'a> ETMPD_W<'a> {
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
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
#[doc = "Field `PS` reader - Port size. The ETM-M4 has no influence over the external pins used for trace. These bits are implemented but not used. On an ETM reset these bits reset to 0b001."]
pub struct PS_R(crate::FieldReader<u8, u8>);
impl PS_R {
    pub(crate) fn new(bits: u8) -> Self {
        PS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PS_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PS` writer - Port size. The ETM-M4 has no influence over the external pins used for trace. These bits are implemented but not used. On an ETM reset these bits reset to 0b001."]
pub struct PS_W<'a> {
    w: &'a mut W,
}
impl<'a> PS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 4)) | ((value as u32 & 0x07) << 4);
        self.w
    }
}
#[doc = "Field `SP` reader - Stall processor. The FIFOFULL output can be used to stall the processor to prevent overflow. The FIFOFULL output is only enabled when the stall processor bit is set to 1. When the bit is 0 the FIFOFULL output remains LOW at all times and the FIFO overflows if there are too many trace packets. Trace resumes without corruption once the FIFO has drained, if overflow does occur. An ETM reset sets this bit to 0."]
pub struct SP_R(crate::FieldReader<bool, bool>);
impl SP_R {
    pub(crate) fn new(bits: bool) -> Self {
        SP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SP` writer - Stall processor. The FIFOFULL output can be used to stall the processor to prevent overflow. The FIFOFULL output is only enabled when the stall processor bit is set to 1. When the bit is 0 the FIFOFULL output remains LOW at all times and the FIFO overflows if there are too many trace packets. Trace resumes without corruption once the FIFO has drained, if overflow does occur. An ETM reset sets this bit to 0."]
pub struct SP_W<'a> {
    w: &'a mut W,
}
impl<'a> SP_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | ((value as u32 & 0x01) << 7);
        self.w
    }
}
#[doc = "Field `BO` reader - Branch output. When set to 1 all branch addresses are output, even if the branch was because of a direct branch instruction. Setting this bit enables reconstruction of the program flow without having access to the memory image of the code being executed. When this bit is set to 1, more trace data is generated, and this may affect the performance of the trace system. Information about the execution of a branch is traced regardless of the state of this bit. An ETM reset sets this bit to 0."]
pub struct BO_R(crate::FieldReader<bool, bool>);
impl BO_R {
    pub(crate) fn new(bits: bool) -> Self {
        BO_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BO_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BO` writer - Branch output. When set to 1 all branch addresses are output, even if the branch was because of a direct branch instruction. Setting this bit enables reconstruction of the program flow without having access to the memory image of the code being executed. When this bit is set to 1, more trace data is generated, and this may affect the performance of the trace system. Information about the execution of a branch is traced regardless of the state of this bit. An ETM reset sets this bit to 0."]
pub struct BO_W<'a> {
    w: &'a mut W,
}
impl<'a> BO_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | ((value as u32 & 0x01) << 8);
        self.w
    }
}
#[doc = "Field `DRC` reader - Debug request control. When set to 1 and the trigger event occurs, the DBGRQ output is asserted until DBGACK is observed. This enables the ARM processor to be forced into Debug state. An ETM reset sets this bit to 0."]
pub struct DRC_R(crate::FieldReader<bool, bool>);
impl DRC_R {
    pub(crate) fn new(bits: bool) -> Self {
        DRC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DRC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DRC` writer - Debug request control. When set to 1 and the trigger event occurs, the DBGRQ output is asserted until DBGACK is observed. This enables the ARM processor to be forced into Debug state. An ETM reset sets this bit to 0."]
pub struct DRC_W<'a> {
    w: &'a mut W,
}
impl<'a> DRC_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | ((value as u32 & 0x01) << 9);
        self.w
    }
}
#[doc = "Field `ETMP` reader - ETM programming. This bit must be set to 1 at the start of the ETM programming sequence. Tracing is prevented while this bit is set to 1. On an ETM reset this bit is set to b1."]
pub struct ETMP_R(crate::FieldReader<bool, bool>);
impl ETMP_R {
    pub(crate) fn new(bits: bool) -> Self {
        ETMP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ETMP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ETMP` writer - ETM programming. This bit must be set to 1 at the start of the ETM programming sequence. Tracing is prevented while this bit is set to 1. On an ETM reset this bit is set to b1."]
pub struct ETMP_W<'a> {
    w: &'a mut W,
}
impl<'a> ETMP_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | ((value as u32 & 0x01) << 10);
        self.w
    }
}
#[doc = "ETM port selection. This bit can be used to control other trace components in an implementation. This bit must be set by the trace software tools to ensure that trace output is enabled from this ETM. An ETM reset sets this bit to 0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ETMPS_A {
    #[doc = "0: ETMEN is LOW."]
    ETMPS_0 = 0,
    #[doc = "1: ETMEN is HIGH."]
    ETMPS_1 = 1,
}
impl From<ETMPS_A> for bool {
    #[inline(always)]
    fn from(variant: ETMPS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ETMPS` reader - ETM port selection. This bit can be used to control other trace components in an implementation. This bit must be set by the trace software tools to ensure that trace output is enabled from this ETM. An ETM reset sets this bit to 0."]
pub struct ETMPS_R(crate::FieldReader<bool, ETMPS_A>);
impl ETMPS_R {
    pub(crate) fn new(bits: bool) -> Self {
        ETMPS_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ETMPS_A {
        match self.bits {
            false => ETMPS_A::ETMPS_0,
            true => ETMPS_A::ETMPS_1,
        }
    }
    #[doc = "Checks if the value of the field is `ETMPS_0`"]
    #[inline(always)]
    pub fn is_etmps_0(&self) -> bool {
        **self == ETMPS_A::ETMPS_0
    }
    #[doc = "Checks if the value of the field is `ETMPS_1`"]
    #[inline(always)]
    pub fn is_etmps_1(&self) -> bool {
        **self == ETMPS_A::ETMPS_1
    }
}
impl core::ops::Deref for ETMPS_R {
    type Target = crate::FieldReader<bool, ETMPS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ETMPS` writer - ETM port selection. This bit can be used to control other trace components in an implementation. This bit must be set by the trace software tools to ensure that trace output is enabled from this ETM. An ETM reset sets this bit to 0."]
pub struct ETMPS_W<'a> {
    w: &'a mut W,
}
impl<'a> ETMPS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ETMPS_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "ETMEN is LOW."]
    #[inline(always)]
    pub fn etmps_0(self) -> &'a mut W {
        self.variant(ETMPS_A::ETMPS_0)
    }
    #[doc = "ETMEN is HIGH."]
    #[inline(always)]
    pub fn etmps_1(self) -> &'a mut W {
        self.variant(ETMPS_A::ETMPS_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | ((value as u32 & 0x01) << 11);
        self.w
    }
}
#[doc = "Field `PM2` reader - This bit is implemented but has no function. An ETM reset sets this bit to 0."]
pub struct PM2_R(crate::FieldReader<bool, bool>);
impl PM2_R {
    pub(crate) fn new(bits: bool) -> Self {
        PM2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PM2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PM2` writer - This bit is implemented but has no function. An ETM reset sets this bit to 0."]
pub struct PM2_W<'a> {
    w: &'a mut W,
}
impl<'a> PM2_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | ((value as u32 & 0x01) << 13);
        self.w
    }
}
#[doc = "Field `PM` reader - These bits are implemented but have no function. An ETM reset sets these bits to 0."]
pub struct PM_R(crate::FieldReader<u8, u8>);
impl PM_R {
    pub(crate) fn new(bits: u8) -> Self {
        PM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PM_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PM` writer - These bits are implemented but have no function. An ETM reset sets these bits to 0."]
pub struct PM_W<'a> {
    w: &'a mut W,
}
impl<'a> PM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 16)) | ((value as u32 & 0x03) << 16);
        self.w
    }
}
#[doc = "Field `PS3` reader - This bit is implemented but has no function. An ETM reset sets this bit to 0."]
pub struct PS3_R(crate::FieldReader<bool, bool>);
impl PS3_R {
    pub(crate) fn new(bits: bool) -> Self {
        PS3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PS3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PS3` writer - This bit is implemented but has no function. An ETM reset sets this bit to 0."]
pub struct PS3_W<'a> {
    w: &'a mut W,
}
impl<'a> PS3_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 21)) | ((value as u32 & 0x01) << 21);
        self.w
    }
}
#[doc = "Field `TE` reader - When set, this bit enables timestamping. An ETM reset sets this bit to 0."]
pub struct TE_R(crate::FieldReader<bool, bool>);
impl TE_R {
    pub(crate) fn new(bits: bool) -> Self {
        TE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TE` writer - When set, this bit enables timestamping. An ETM reset sets this bit to 0."]
pub struct TE_W<'a> {
    w: &'a mut W,
}
impl<'a> TE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 28)) | ((value as u32 & 0x01) << 28);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - ETM power down. This bit can be used by an implementation to control if the ETM is in a low power state. This bit must be cleared by the trace software tools at the beginning of a debug session. When this bit is set to 1, writes to some registers and fields might be ignored."]
    #[inline(always)]
    pub fn etmpd(&self) -> ETMPD_R {
        ETMPD_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bits 4:6 - Port size. The ETM-M4 has no influence over the external pins used for trace. These bits are implemented but not used. On an ETM reset these bits reset to 0b001."]
    #[inline(always)]
    pub fn ps(&self) -> PS_R {
        PS_R::new(((self.bits >> 4) & 0x07) as u8)
    }
    #[doc = "Bit 7 - Stall processor. The FIFOFULL output can be used to stall the processor to prevent overflow. The FIFOFULL output is only enabled when the stall processor bit is set to 1. When the bit is 0 the FIFOFULL output remains LOW at all times and the FIFO overflows if there are too many trace packets. Trace resumes without corruption once the FIFO has drained, if overflow does occur. An ETM reset sets this bit to 0."]
    #[inline(always)]
    pub fn sp(&self) -> SP_R {
        SP_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Branch output. When set to 1 all branch addresses are output, even if the branch was because of a direct branch instruction. Setting this bit enables reconstruction of the program flow without having access to the memory image of the code being executed. When this bit is set to 1, more trace data is generated, and this may affect the performance of the trace system. Information about the execution of a branch is traced regardless of the state of this bit. An ETM reset sets this bit to 0."]
    #[inline(always)]
    pub fn bo(&self) -> BO_R {
        BO_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Debug request control. When set to 1 and the trigger event occurs, the DBGRQ output is asserted until DBGACK is observed. This enables the ARM processor to be forced into Debug state. An ETM reset sets this bit to 0."]
    #[inline(always)]
    pub fn drc(&self) -> DRC_R {
        DRC_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - ETM programming. This bit must be set to 1 at the start of the ETM programming sequence. Tracing is prevented while this bit is set to 1. On an ETM reset this bit is set to b1."]
    #[inline(always)]
    pub fn etmp(&self) -> ETMP_R {
        ETMP_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - ETM port selection. This bit can be used to control other trace components in an implementation. This bit must be set by the trace software tools to ensure that trace output is enabled from this ETM. An ETM reset sets this bit to 0."]
    #[inline(always)]
    pub fn etmps(&self) -> ETMPS_R {
        ETMPS_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 13 - This bit is implemented but has no function. An ETM reset sets this bit to 0."]
    #[inline(always)]
    pub fn pm2(&self) -> PM2_R {
        PM2_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bits 16:17 - These bits are implemented but have no function. An ETM reset sets these bits to 0."]
    #[inline(always)]
    pub fn pm(&self) -> PM_R {
        PM_R::new(((self.bits >> 16) & 0x03) as u8)
    }
    #[doc = "Bit 21 - This bit is implemented but has no function. An ETM reset sets this bit to 0."]
    #[inline(always)]
    pub fn ps3(&self) -> PS3_R {
        PS3_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 28 - When set, this bit enables timestamping. An ETM reset sets this bit to 0."]
    #[inline(always)]
    pub fn te(&self) -> TE_R {
        TE_R::new(((self.bits >> 28) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - ETM power down. This bit can be used by an implementation to control if the ETM is in a low power state. This bit must be cleared by the trace software tools at the beginning of a debug session. When this bit is set to 1, writes to some registers and fields might be ignored."]
    #[inline(always)]
    pub fn etmpd(&mut self) -> ETMPD_W {
        ETMPD_W { w: self }
    }
    #[doc = "Bits 4:6 - Port size. The ETM-M4 has no influence over the external pins used for trace. These bits are implemented but not used. On an ETM reset these bits reset to 0b001."]
    #[inline(always)]
    pub fn ps(&mut self) -> PS_W {
        PS_W { w: self }
    }
    #[doc = "Bit 7 - Stall processor. The FIFOFULL output can be used to stall the processor to prevent overflow. The FIFOFULL output is only enabled when the stall processor bit is set to 1. When the bit is 0 the FIFOFULL output remains LOW at all times and the FIFO overflows if there are too many trace packets. Trace resumes without corruption once the FIFO has drained, if overflow does occur. An ETM reset sets this bit to 0."]
    #[inline(always)]
    pub fn sp(&mut self) -> SP_W {
        SP_W { w: self }
    }
    #[doc = "Bit 8 - Branch output. When set to 1 all branch addresses are output, even if the branch was because of a direct branch instruction. Setting this bit enables reconstruction of the program flow without having access to the memory image of the code being executed. When this bit is set to 1, more trace data is generated, and this may affect the performance of the trace system. Information about the execution of a branch is traced regardless of the state of this bit. An ETM reset sets this bit to 0."]
    #[inline(always)]
    pub fn bo(&mut self) -> BO_W {
        BO_W { w: self }
    }
    #[doc = "Bit 9 - Debug request control. When set to 1 and the trigger event occurs, the DBGRQ output is asserted until DBGACK is observed. This enables the ARM processor to be forced into Debug state. An ETM reset sets this bit to 0."]
    #[inline(always)]
    pub fn drc(&mut self) -> DRC_W {
        DRC_W { w: self }
    }
    #[doc = "Bit 10 - ETM programming. This bit must be set to 1 at the start of the ETM programming sequence. Tracing is prevented while this bit is set to 1. On an ETM reset this bit is set to b1."]
    #[inline(always)]
    pub fn etmp(&mut self) -> ETMP_W {
        ETMP_W { w: self }
    }
    #[doc = "Bit 11 - ETM port selection. This bit can be used to control other trace components in an implementation. This bit must be set by the trace software tools to ensure that trace output is enabled from this ETM. An ETM reset sets this bit to 0."]
    #[inline(always)]
    pub fn etmps(&mut self) -> ETMPS_W {
        ETMPS_W { w: self }
    }
    #[doc = "Bit 13 - This bit is implemented but has no function. An ETM reset sets this bit to 0."]
    #[inline(always)]
    pub fn pm2(&mut self) -> PM2_W {
        PM2_W { w: self }
    }
    #[doc = "Bits 16:17 - These bits are implemented but have no function. An ETM reset sets these bits to 0."]
    #[inline(always)]
    pub fn pm(&mut self) -> PM_W {
        PM_W { w: self }
    }
    #[doc = "Bit 21 - This bit is implemented but has no function. An ETM reset sets this bit to 0."]
    #[inline(always)]
    pub fn ps3(&mut self) -> PS3_W {
        PS3_W { w: self }
    }
    #[doc = "Bit 28 - When set, this bit enables timestamping. An ETM reset sets this bit to 0."]
    #[inline(always)]
    pub fn te(&mut self) -> TE_W {
        TE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Main Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cr](index.html) module"]
pub struct CR_SPEC;
impl crate::RegisterSpec for CR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cr::R](R) reader structure"]
impl crate::Readable for CR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cr::W](W) writer structure"]
impl crate::Writable for CR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CR to value 0x0411"]
impl crate::Resettable for CR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0411
    }
}
