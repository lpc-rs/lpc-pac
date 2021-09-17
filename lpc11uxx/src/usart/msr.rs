#[doc = "Register `MSR` reader"]
pub struct R(crate::R<MSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Delta CTS. Set upon state change of input CTS. Cleared on an MSR read.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DCTS_A {
    #[doc = "0: No change detected on modem input, CTS."]
    NO_CHANGE_DETECTED_O = 0,
    #[doc = "1: State change detected on modem input, CTS."]
    STATE_CHANGE_DETECTE = 1,
}
impl From<DCTS_A> for bool {
    #[inline(always)]
    fn from(variant: DCTS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DCTS` reader - Delta CTS. Set upon state change of input CTS. Cleared on an MSR read."]
pub struct DCTS_R(crate::FieldReader<bool, DCTS_A>);
impl DCTS_R {
    pub(crate) fn new(bits: bool) -> Self {
        DCTS_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DCTS_A {
        match self.bits {
            false => DCTS_A::NO_CHANGE_DETECTED_O,
            true => DCTS_A::STATE_CHANGE_DETECTE,
        }
    }
    #[doc = "Checks if the value of the field is `NO_CHANGE_DETECTED_O`"]
    #[inline(always)]
    pub fn is_no_change_detected_o(&self) -> bool {
        **self == DCTS_A::NO_CHANGE_DETECTED_O
    }
    #[doc = "Checks if the value of the field is `STATE_CHANGE_DETECTE`"]
    #[inline(always)]
    pub fn is_state_change_detecte(&self) -> bool {
        **self == DCTS_A::STATE_CHANGE_DETECTE
    }
}
impl core::ops::Deref for DCTS_R {
    type Target = crate::FieldReader<bool, DCTS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Delta DSR. Set upon state change of input DSR. Cleared on an MSR read.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DDSR_A {
    #[doc = "0: No change detected on modem input, DSR."]
    NO_CHANGE_DETECTED_O = 0,
    #[doc = "1: State change detected on modem input, DSR."]
    STATE_CHANGE_DETECTE = 1,
}
impl From<DDSR_A> for bool {
    #[inline(always)]
    fn from(variant: DDSR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DDSR` reader - Delta DSR. Set upon state change of input DSR. Cleared on an MSR read."]
pub struct DDSR_R(crate::FieldReader<bool, DDSR_A>);
impl DDSR_R {
    pub(crate) fn new(bits: bool) -> Self {
        DDSR_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DDSR_A {
        match self.bits {
            false => DDSR_A::NO_CHANGE_DETECTED_O,
            true => DDSR_A::STATE_CHANGE_DETECTE,
        }
    }
    #[doc = "Checks if the value of the field is `NO_CHANGE_DETECTED_O`"]
    #[inline(always)]
    pub fn is_no_change_detected_o(&self) -> bool {
        **self == DDSR_A::NO_CHANGE_DETECTED_O
    }
    #[doc = "Checks if the value of the field is `STATE_CHANGE_DETECTE`"]
    #[inline(always)]
    pub fn is_state_change_detecte(&self) -> bool {
        **self == DDSR_A::STATE_CHANGE_DETECTE
    }
}
impl core::ops::Deref for DDSR_R {
    type Target = crate::FieldReader<bool, DDSR_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Trailing Edge RI. Set upon low to high transition of input RI. Cleared on an MSR read.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TERI_A {
    #[doc = "0: No change detected on modem input, RI."]
    NO_CHANGE_DETECTED_O = 0,
    #[doc = "1: Low-to-high transition detected on RI."]
    LOW_TO_HIGH_TRANSITI = 1,
}
impl From<TERI_A> for bool {
    #[inline(always)]
    fn from(variant: TERI_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TERI` reader - Trailing Edge RI. Set upon low to high transition of input RI. Cleared on an MSR read."]
pub struct TERI_R(crate::FieldReader<bool, TERI_A>);
impl TERI_R {
    pub(crate) fn new(bits: bool) -> Self {
        TERI_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TERI_A {
        match self.bits {
            false => TERI_A::NO_CHANGE_DETECTED_O,
            true => TERI_A::LOW_TO_HIGH_TRANSITI,
        }
    }
    #[doc = "Checks if the value of the field is `NO_CHANGE_DETECTED_O`"]
    #[inline(always)]
    pub fn is_no_change_detected_o(&self) -> bool {
        **self == TERI_A::NO_CHANGE_DETECTED_O
    }
    #[doc = "Checks if the value of the field is `LOW_TO_HIGH_TRANSITI`"]
    #[inline(always)]
    pub fn is_low_to_high_transiti(&self) -> bool {
        **self == TERI_A::LOW_TO_HIGH_TRANSITI
    }
}
impl core::ops::Deref for TERI_R {
    type Target = crate::FieldReader<bool, TERI_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Delta DCD. Set upon state change of input DCD. Cleared on an MSR read.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DDCD_A {
    #[doc = "0: No change detected on modem input, DCD."]
    NO_CHANGE_DETECTED_O = 0,
    #[doc = "1: State change detected on modem input, DCD."]
    STATE_CHANGE_DETECTE = 1,
}
impl From<DDCD_A> for bool {
    #[inline(always)]
    fn from(variant: DDCD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DDCD` reader - Delta DCD. Set upon state change of input DCD. Cleared on an MSR read."]
pub struct DDCD_R(crate::FieldReader<bool, DDCD_A>);
impl DDCD_R {
    pub(crate) fn new(bits: bool) -> Self {
        DDCD_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DDCD_A {
        match self.bits {
            false => DDCD_A::NO_CHANGE_DETECTED_O,
            true => DDCD_A::STATE_CHANGE_DETECTE,
        }
    }
    #[doc = "Checks if the value of the field is `NO_CHANGE_DETECTED_O`"]
    #[inline(always)]
    pub fn is_no_change_detected_o(&self) -> bool {
        **self == DDCD_A::NO_CHANGE_DETECTED_O
    }
    #[doc = "Checks if the value of the field is `STATE_CHANGE_DETECTE`"]
    #[inline(always)]
    pub fn is_state_change_detecte(&self) -> bool {
        **self == DDCD_A::STATE_CHANGE_DETECTE
    }
}
impl core::ops::Deref for DDCD_R {
    type Target = crate::FieldReader<bool, DDCD_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CTS` reader - Clear To Send State. Complement of input signal CTS. This bit is connected to MCR\\[1\\]
in modem loopback mode."]
pub struct CTS_R(crate::FieldReader<bool, bool>);
impl CTS_R {
    pub(crate) fn new(bits: bool) -> Self {
        CTS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CTS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DSR` reader - Data Set Ready State. Complement of input signal DSR. This bit is connected to MCR\\[0\\]
in modem loopback mode."]
pub struct DSR_R(crate::FieldReader<bool, bool>);
impl DSR_R {
    pub(crate) fn new(bits: bool) -> Self {
        DSR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DSR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RI` reader - Ring Indicator State. Complement of input RI. This bit is connected to MCR\\[2\\]
in modem loopback mode."]
pub struct RI_R(crate::FieldReader<bool, bool>);
impl RI_R {
    pub(crate) fn new(bits: bool) -> Self {
        RI_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RI_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DCD` reader - Data Carrier Detect State. Complement of input DCD. This bit is connected to MCR\\[3\\]
in modem loopback mode."]
pub struct DCD_R(crate::FieldReader<bool, bool>);
impl DCD_R {
    pub(crate) fn new(bits: bool) -> Self {
        DCD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DCD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - Delta CTS. Set upon state change of input CTS. Cleared on an MSR read."]
    #[inline(always)]
    pub fn dcts(&self) -> DCTS_R {
        DCTS_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Delta DSR. Set upon state change of input DSR. Cleared on an MSR read."]
    #[inline(always)]
    pub fn ddsr(&self) -> DDSR_R {
        DDSR_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Trailing Edge RI. Set upon low to high transition of input RI. Cleared on an MSR read."]
    #[inline(always)]
    pub fn teri(&self) -> TERI_R {
        TERI_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Delta DCD. Set upon state change of input DCD. Cleared on an MSR read."]
    #[inline(always)]
    pub fn ddcd(&self) -> DDCD_R {
        DDCD_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Clear To Send State. Complement of input signal CTS. This bit is connected to MCR\\[1\\]
in modem loopback mode."]
    #[inline(always)]
    pub fn cts(&self) -> CTS_R {
        CTS_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Data Set Ready State. Complement of input signal DSR. This bit is connected to MCR\\[0\\]
in modem loopback mode."]
    #[inline(always)]
    pub fn dsr(&self) -> DSR_R {
        DSR_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Ring Indicator State. Complement of input RI. This bit is connected to MCR\\[2\\]
in modem loopback mode."]
    #[inline(always)]
    pub fn ri(&self) -> RI_R {
        RI_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Data Carrier Detect State. Complement of input DCD. This bit is connected to MCR\\[3\\]
in modem loopback mode."]
    #[inline(always)]
    pub fn dcd(&self) -> DCD_R {
        DCD_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
#[doc = "Modem Status Register.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [msr](index.html) module"]
pub struct MSR_SPEC;
impl crate::RegisterSpec for MSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [msr::R](R) reader structure"]
impl crate::Readable for MSR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets MSR to value 0"]
impl crate::Resettable for MSR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
