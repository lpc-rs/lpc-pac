#[doc = "Register `CTCR` reader"]
pub struct R(crate::R<CTCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTCR` writer"]
pub struct W(crate::W<CTCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTCR_SPEC>;
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
impl From<crate::W<CTCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Counter/Timer Mode. This field selects which rising PCLK edges can increment Timer's Prescale Counter (PC), or clear PC and increment Timer Counter (TC). If Counter mode is selected in the CTCR, bits 2:0 in the Capture Control Register (CCR) must be programmed as 000.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CTM_A {
    #[doc = "0: Timer Mode: every rising PCLK edge"]
    TIMER_MODE_EVERY_RI = 0,
    #[doc = "1: Counter Mode: TC is incremented on rising edges on the CAP input selected by bits 3:2."]
    RISING = 1,
    #[doc = "2: Counter Mode: TC is incremented on falling edges on the CAP input selected by bits 3:2."]
    FALLING = 2,
    #[doc = "3: Counter Mode: TC is incremented on both edges on the CAP input selected by bits 3:2."]
    BOTH = 3,
}
impl From<CTM_A> for u8 {
    #[inline(always)]
    fn from(variant: CTM_A) -> Self {
        variant as _
    }
}
#[doc = "Field `CTM` reader - Counter/Timer Mode. This field selects which rising PCLK edges can increment Timer's Prescale Counter (PC), or clear PC and increment Timer Counter (TC). If Counter mode is selected in the CTCR, bits 2:0 in the Capture Control Register (CCR) must be programmed as 000."]
pub struct CTM_R(crate::FieldReader<u8, CTM_A>);
impl CTM_R {
    pub(crate) fn new(bits: u8) -> Self {
        CTM_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CTM_A {
        match self.bits {
            0 => CTM_A::TIMER_MODE_EVERY_RI,
            1 => CTM_A::RISING,
            2 => CTM_A::FALLING,
            3 => CTM_A::BOTH,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `TIMER_MODE_EVERY_RI`"]
    #[inline(always)]
    pub fn is_timer_mode_every_ri(&self) -> bool {
        **self == CTM_A::TIMER_MODE_EVERY_RI
    }
    #[doc = "Checks if the value of the field is `RISING`"]
    #[inline(always)]
    pub fn is_rising(&self) -> bool {
        **self == CTM_A::RISING
    }
    #[doc = "Checks if the value of the field is `FALLING`"]
    #[inline(always)]
    pub fn is_falling(&self) -> bool {
        **self == CTM_A::FALLING
    }
    #[doc = "Checks if the value of the field is `BOTH`"]
    #[inline(always)]
    pub fn is_both(&self) -> bool {
        **self == CTM_A::BOTH
    }
}
impl core::ops::Deref for CTM_R {
    type Target = crate::FieldReader<u8, CTM_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CTM` writer - Counter/Timer Mode. This field selects which rising PCLK edges can increment Timer's Prescale Counter (PC), or clear PC and increment Timer Counter (TC). If Counter mode is selected in the CTCR, bits 2:0 in the Capture Control Register (CCR) must be programmed as 000."]
pub struct CTM_W<'a> {
    w: &'a mut W,
}
impl<'a> CTM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CTM_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Timer Mode: every rising PCLK edge"]
    #[inline(always)]
    pub fn timer_mode_every_ri(self) -> &'a mut W {
        self.variant(CTM_A::TIMER_MODE_EVERY_RI)
    }
    #[doc = "Counter Mode: TC is incremented on rising edges on the CAP input selected by bits 3:2."]
    #[inline(always)]
    pub fn rising(self) -> &'a mut W {
        self.variant(CTM_A::RISING)
    }
    #[doc = "Counter Mode: TC is incremented on falling edges on the CAP input selected by bits 3:2."]
    #[inline(always)]
    pub fn falling(self) -> &'a mut W {
        self.variant(CTM_A::FALLING)
    }
    #[doc = "Counter Mode: TC is incremented on both edges on the CAP input selected by bits 3:2."]
    #[inline(always)]
    pub fn both(self) -> &'a mut W {
        self.variant(CTM_A::BOTH)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | (value as u32 & 0x03);
        self.w
    }
}
#[doc = "Count Input Select. In counter mode (when bits 1:0 in this register are not 00), these bits select which CAP pin or comparator output is sampled for clocking. If Counter mode is selected in the CTCR, the 3 bits for that input in the Capture Control Register (CCR) must be programmed as 000. Values 0x1 and0x3 are reserved.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CIS_A {
    #[doc = "0: CT32B0_CAP0"]
    CT32B0_CAP0 = 0,
    #[doc = "1: Reserved."]
    RESERVED_1 = 1,
    #[doc = "2: CT32B0_CAP1"]
    CT32B0_CAP1 = 2,
}
impl From<CIS_A> for u8 {
    #[inline(always)]
    fn from(variant: CIS_A) -> Self {
        variant as _
    }
}
#[doc = "Field `CIS` reader - Count Input Select. In counter mode (when bits 1:0 in this register are not 00), these bits select which CAP pin or comparator output is sampled for clocking. If Counter mode is selected in the CTCR, the 3 bits for that input in the Capture Control Register (CCR) must be programmed as 000. Values 0x1 and0x3 are reserved."]
pub struct CIS_R(crate::FieldReader<u8, CIS_A>);
impl CIS_R {
    pub(crate) fn new(bits: u8) -> Self {
        CIS_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CIS_A> {
        match self.bits {
            0 => Some(CIS_A::CT32B0_CAP0),
            1 => Some(CIS_A::RESERVED_1),
            2 => Some(CIS_A::CT32B0_CAP1),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `CT32B0_CAP0`"]
    #[inline(always)]
    pub fn is_ct32b0_cap0(&self) -> bool {
        **self == CIS_A::CT32B0_CAP0
    }
    #[doc = "Checks if the value of the field is `RESERVED_1`"]
    #[inline(always)]
    pub fn is_reserved_1(&self) -> bool {
        **self == CIS_A::RESERVED_1
    }
    #[doc = "Checks if the value of the field is `CT32B0_CAP1`"]
    #[inline(always)]
    pub fn is_ct32b0_cap1(&self) -> bool {
        **self == CIS_A::CT32B0_CAP1
    }
}
impl core::ops::Deref for CIS_R {
    type Target = crate::FieldReader<u8, CIS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CIS` writer - Count Input Select. In counter mode (when bits 1:0 in this register are not 00), these bits select which CAP pin or comparator output is sampled for clocking. If Counter mode is selected in the CTCR, the 3 bits for that input in the Capture Control Register (CCR) must be programmed as 000. Values 0x1 and0x3 are reserved."]
pub struct CIS_W<'a> {
    w: &'a mut W,
}
impl<'a> CIS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CIS_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "CT32B0_CAP0"]
    #[inline(always)]
    pub fn ct32b0_cap0(self) -> &'a mut W {
        self.variant(CIS_A::CT32B0_CAP0)
    }
    #[doc = "Reserved."]
    #[inline(always)]
    pub fn reserved_1(self) -> &'a mut W {
        self.variant(CIS_A::RESERVED_1)
    }
    #[doc = "CT32B0_CAP1"]
    #[inline(always)]
    pub fn ct32b0_cap1(self) -> &'a mut W {
        self.variant(CIS_A::CT32B0_CAP1)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | ((value as u32 & 0x03) << 2);
        self.w
    }
}
#[doc = "Field `ENCC` reader - Setting this bit to 1 enables clearing of the timer and the prescaler when the capture-edge event specified in bits 7:5 occurs."]
pub struct ENCC_R(crate::FieldReader<bool, bool>);
impl ENCC_R {
    pub(crate) fn new(bits: bool) -> Self {
        ENCC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ENCC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ENCC` writer - Setting this bit to 1 enables clearing of the timer and the prescaler when the capture-edge event specified in bits 7:5 occurs."]
pub struct ENCC_W<'a> {
    w: &'a mut W,
}
impl<'a> ENCC_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | ((value as u32 & 0x01) << 4);
        self.w
    }
}
#[doc = "When bit 4 is a 1, these bits select which capture input edge will cause the timer and prescaler to be cleared. These bits have no effect when bit 4 is low. Values 0x2 to 0x3 and 0x6 to 0x7 are reserved.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SELCC_A {
    #[doc = "0: Rising Edge of CT32B0_CAP0 clears the timer (if bit 4 is set)"]
    RISING_EDGE_OF_CT32B_CAP0 = 0,
    #[doc = "1: Falling Edge of CT32B0_CAP0 clears the timer (if bit 4 is set)"]
    FALLING_EDGE_OF_CT32_CAP0 = 1,
    #[doc = "2: Reserved,"]
    RESERVED_2 = 2,
    #[doc = "3: Reserved."]
    RESERVED_3 = 3,
    #[doc = "4: Rising Edge of CT32B0_CAP1 clears the timer (if bit 4 is set)"]
    RISING_EDGE_OF_CT32B_CAP1 = 4,
    #[doc = "5: Falling Edge of CT32B0_CAP1 clears the timer (if bit 4 is set)"]
    FALLING_EDGE_OF_CT32_CAP1 = 5,
}
impl From<SELCC_A> for u8 {
    #[inline(always)]
    fn from(variant: SELCC_A) -> Self {
        variant as _
    }
}
#[doc = "Field `SElCC` reader - When bit 4 is a 1, these bits select which capture input edge will cause the timer and prescaler to be cleared. These bits have no effect when bit 4 is low. Values 0x2 to 0x3 and 0x6 to 0x7 are reserved."]
pub struct SELCC_R(crate::FieldReader<u8, SELCC_A>);
impl SELCC_R {
    pub(crate) fn new(bits: u8) -> Self {
        SELCC_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SELCC_A> {
        match self.bits {
            0 => Some(SELCC_A::RISING_EDGE_OF_CT32B_CAP0),
            1 => Some(SELCC_A::FALLING_EDGE_OF_CT32_CAP0),
            2 => Some(SELCC_A::RESERVED_2),
            3 => Some(SELCC_A::RESERVED_3),
            4 => Some(SELCC_A::RISING_EDGE_OF_CT32B_CAP1),
            5 => Some(SELCC_A::FALLING_EDGE_OF_CT32_CAP1),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `RISING_EDGE_OF_CT32B_CAP0`"]
    #[inline(always)]
    pub fn is_rising_edge_of_ct32b_cap0(&self) -> bool {
        **self == SELCC_A::RISING_EDGE_OF_CT32B_CAP0
    }
    #[doc = "Checks if the value of the field is `FALLING_EDGE_OF_CT32_CAP0`"]
    #[inline(always)]
    pub fn is_falling_edge_of_ct32_cap0(&self) -> bool {
        **self == SELCC_A::FALLING_EDGE_OF_CT32_CAP0
    }
    #[doc = "Checks if the value of the field is `RESERVED_2`"]
    #[inline(always)]
    pub fn is_reserved_2(&self) -> bool {
        **self == SELCC_A::RESERVED_2
    }
    #[doc = "Checks if the value of the field is `RESERVED_3`"]
    #[inline(always)]
    pub fn is_reserved_3(&self) -> bool {
        **self == SELCC_A::RESERVED_3
    }
    #[doc = "Checks if the value of the field is `RISING_EDGE_OF_CT32B_CAP1`"]
    #[inline(always)]
    pub fn is_rising_edge_of_ct32b_cap1(&self) -> bool {
        **self == SELCC_A::RISING_EDGE_OF_CT32B_CAP1
    }
    #[doc = "Checks if the value of the field is `FALLING_EDGE_OF_CT32_CAP1`"]
    #[inline(always)]
    pub fn is_falling_edge_of_ct32_cap1(&self) -> bool {
        **self == SELCC_A::FALLING_EDGE_OF_CT32_CAP1
    }
}
impl core::ops::Deref for SELCC_R {
    type Target = crate::FieldReader<u8, SELCC_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SElCC` writer - When bit 4 is a 1, these bits select which capture input edge will cause the timer and prescaler to be cleared. These bits have no effect when bit 4 is low. Values 0x2 to 0x3 and 0x6 to 0x7 are reserved."]
pub struct SELCC_W<'a> {
    w: &'a mut W,
}
impl<'a> SELCC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SELCC_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Rising Edge of CT32B0_CAP0 clears the timer (if bit 4 is set)"]
    #[inline(always)]
    pub fn rising_edge_of_ct32b_cap0(self) -> &'a mut W {
        self.variant(SELCC_A::RISING_EDGE_OF_CT32B_CAP0)
    }
    #[doc = "Falling Edge of CT32B0_CAP0 clears the timer (if bit 4 is set)"]
    #[inline(always)]
    pub fn falling_edge_of_ct32_cap0(self) -> &'a mut W {
        self.variant(SELCC_A::FALLING_EDGE_OF_CT32_CAP0)
    }
    #[doc = "Reserved,"]
    #[inline(always)]
    pub fn reserved_2(self) -> &'a mut W {
        self.variant(SELCC_A::RESERVED_2)
    }
    #[doc = "Reserved."]
    #[inline(always)]
    pub fn reserved_3(self) -> &'a mut W {
        self.variant(SELCC_A::RESERVED_3)
    }
    #[doc = "Rising Edge of CT32B0_CAP1 clears the timer (if bit 4 is set)"]
    #[inline(always)]
    pub fn rising_edge_of_ct32b_cap1(self) -> &'a mut W {
        self.variant(SELCC_A::RISING_EDGE_OF_CT32B_CAP1)
    }
    #[doc = "Falling Edge of CT32B0_CAP1 clears the timer (if bit 4 is set)"]
    #[inline(always)]
    pub fn falling_edge_of_ct32_cap1(self) -> &'a mut W {
        self.variant(SELCC_A::FALLING_EDGE_OF_CT32_CAP1)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 5)) | ((value as u32 & 0x07) << 5);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - Counter/Timer Mode. This field selects which rising PCLK edges can increment Timer's Prescale Counter (PC), or clear PC and increment Timer Counter (TC). If Counter mode is selected in the CTCR, bits 2:0 in the Capture Control Register (CCR) must be programmed as 000."]
    #[inline(always)]
    pub fn ctm(&self) -> CTM_R {
        CTM_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 2:3 - Count Input Select. In counter mode (when bits 1:0 in this register are not 00), these bits select which CAP pin or comparator output is sampled for clocking. If Counter mode is selected in the CTCR, the 3 bits for that input in the Capture Control Register (CCR) must be programmed as 000. Values 0x1 and0x3 are reserved."]
    #[inline(always)]
    pub fn cis(&self) -> CIS_R {
        CIS_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bit 4 - Setting this bit to 1 enables clearing of the timer and the prescaler when the capture-edge event specified in bits 7:5 occurs."]
    #[inline(always)]
    pub fn encc(&self) -> ENCC_R {
        ENCC_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bits 5:7 - When bit 4 is a 1, these bits select which capture input edge will cause the timer and prescaler to be cleared. These bits have no effect when bit 4 is low. Values 0x2 to 0x3 and 0x6 to 0x7 are reserved."]
    #[inline(always)]
    pub fn sel_cc(&self) -> SELCC_R {
        SELCC_R::new(((self.bits >> 5) & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Counter/Timer Mode. This field selects which rising PCLK edges can increment Timer's Prescale Counter (PC), or clear PC and increment Timer Counter (TC). If Counter mode is selected in the CTCR, bits 2:0 in the Capture Control Register (CCR) must be programmed as 000."]
    #[inline(always)]
    pub fn ctm(&mut self) -> CTM_W {
        CTM_W { w: self }
    }
    #[doc = "Bits 2:3 - Count Input Select. In counter mode (when bits 1:0 in this register are not 00), these bits select which CAP pin or comparator output is sampled for clocking. If Counter mode is selected in the CTCR, the 3 bits for that input in the Capture Control Register (CCR) must be programmed as 000. Values 0x1 and0x3 are reserved."]
    #[inline(always)]
    pub fn cis(&mut self) -> CIS_W {
        CIS_W { w: self }
    }
    #[doc = "Bit 4 - Setting this bit to 1 enables clearing of the timer and the prescaler when the capture-edge event specified in bits 7:5 occurs."]
    #[inline(always)]
    pub fn encc(&mut self) -> ENCC_W {
        ENCC_W { w: self }
    }
    #[doc = "Bits 5:7 - When bit 4 is a 1, these bits select which capture input edge will cause the timer and prescaler to be cleared. These bits have no effect when bit 4 is low. Values 0x2 to 0x3 and 0x6 to 0x7 are reserved."]
    #[inline(always)]
    pub fn sel_cc(&mut self) -> SELCC_W {
        SELCC_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Count Control Register. The CTCR selects between Timer and Counter mode, and in Counter mode selects the signal and edge(s) for counting.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctcr](index.html) module"]
pub struct CTCR_SPEC;
impl crate::RegisterSpec for CTCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ctcr::R](R) reader structure"]
impl crate::Readable for CTCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctcr::W](W) writer structure"]
impl crate::Writable for CTCR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CTCR to value 0"]
impl crate::Resettable for CTCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
