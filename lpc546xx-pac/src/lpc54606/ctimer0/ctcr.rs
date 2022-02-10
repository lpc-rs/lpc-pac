///Register `CTCR` reader
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
///Register `CTCR` writer
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
///Counter/Timer Mode This field selects which rising APB bus clock edges can increment Timer's Prescale Counter (PC), or clear PC and increment Timer Counter (TC). Timer Mode: the TC is incremented when the Prescale Counter matches the Prescale Register.
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CTMODE_A {
    ///0: Timer Mode. Incremented every rising APB bus clock edge.
    TIMER = 0,
    ///1: Counter Mode rising edge. TC is incremented on rising edges on the CAP input selected by bits 3:2.
    COUNTER_RISING_EDGE = 1,
    ///2: Counter Mode falling edge. TC is incremented on falling edges on the CAP input selected by bits 3:2.
    COUNTER_FALLING_EDGE = 2,
    ///3: Counter Mode dual edge. TC is incremented on both edges on the CAP input selected by bits 3:2.
    COUNTER_DUAL_EDGE = 3,
}
impl From<CTMODE_A> for u8 {
    #[inline(always)]
    fn from(variant: CTMODE_A) -> Self {
        variant as _
    }
}
///Field `CTMODE` reader - Counter/Timer Mode This field selects which rising APB bus clock edges can increment Timer's Prescale Counter (PC), or clear PC and increment Timer Counter (TC). Timer Mode: the TC is incremented when the Prescale Counter matches the Prescale Register.
pub struct CTMODE_R(crate::FieldReader<u8, CTMODE_A>);
impl CTMODE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CTMODE_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> CTMODE_A {
        match self.bits {
            0 => CTMODE_A::TIMER,
            1 => CTMODE_A::COUNTER_RISING_EDGE,
            2 => CTMODE_A::COUNTER_FALLING_EDGE,
            3 => CTMODE_A::COUNTER_DUAL_EDGE,
            _ => unreachable!(),
        }
    }
    ///Checks if the value of the field is `TIMER`
    #[inline(always)]
    pub fn is_timer(&self) -> bool {
        **self == CTMODE_A::TIMER
    }
    ///Checks if the value of the field is `COUNTER_RISING_EDGE`
    #[inline(always)]
    pub fn is_counter_rising_edge(&self) -> bool {
        **self == CTMODE_A::COUNTER_RISING_EDGE
    }
    ///Checks if the value of the field is `COUNTER_FALLING_EDGE`
    #[inline(always)]
    pub fn is_counter_falling_edge(&self) -> bool {
        **self == CTMODE_A::COUNTER_FALLING_EDGE
    }
    ///Checks if the value of the field is `COUNTER_DUAL_EDGE`
    #[inline(always)]
    pub fn is_counter_dual_edge(&self) -> bool {
        **self == CTMODE_A::COUNTER_DUAL_EDGE
    }
}
impl core::ops::Deref for CTMODE_R {
    type Target = crate::FieldReader<u8, CTMODE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `CTMODE` writer - Counter/Timer Mode This field selects which rising APB bus clock edges can increment Timer's Prescale Counter (PC), or clear PC and increment Timer Counter (TC). Timer Mode: the TC is incremented when the Prescale Counter matches the Prescale Register.
pub struct CTMODE_W<'a> {
    w: &'a mut W,
}
impl<'a> CTMODE_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: CTMODE_A) -> &'a mut W {
        self.bits(variant.into())
    }
    ///Timer Mode. Incremented every rising APB bus clock edge.
    #[inline(always)]
    pub fn timer(self) -> &'a mut W {
        self.variant(CTMODE_A::TIMER)
    }
    ///Counter Mode rising edge. TC is incremented on rising edges on the CAP input selected by bits 3:2.
    #[inline(always)]
    pub fn counter_rising_edge(self) -> &'a mut W {
        self.variant(CTMODE_A::COUNTER_RISING_EDGE)
    }
    ///Counter Mode falling edge. TC is incremented on falling edges on the CAP input selected by bits 3:2.
    #[inline(always)]
    pub fn counter_falling_edge(self) -> &'a mut W {
        self.variant(CTMODE_A::COUNTER_FALLING_EDGE)
    }
    ///Counter Mode dual edge. TC is incremented on both edges on the CAP input selected by bits 3:2.
    #[inline(always)]
    pub fn counter_dual_edge(self) -> &'a mut W {
        self.variant(CTMODE_A::COUNTER_DUAL_EDGE)
    }
    ///Writes raw bits to the field
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | (value as u32 & 0x03);
        self.w
    }
}
///Count Input Select When bits 1:0 in this register are not 00, these bits select which CAP pin is sampled for clocking. Note: If Counter mode is selected for a particular CAPn input in the CTCR, the 3 bits for that input in the Capture Control Register (CCR) must be programmed as 000. However, capture and/or interrupt can be selected for the other 3 CAPn inputs in the same timer.
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CINSEL_A {
    ///0: Channel 0. CAPn.0 for CTIMERn
    CHANNEL_0 = 0,
    ///1: Channel 1. CAPn.1 for CTIMERn
    CHANNEL_1 = 1,
    ///2: Channel 2. CAPn.2 for CTIMERn
    CHANNEL_2 = 2,
    ///3: Channel 3. CAPn.3 for CTIMERn
    CHANNEL_3 = 3,
}
impl From<CINSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: CINSEL_A) -> Self {
        variant as _
    }
}
///Field `CINSEL` reader - Count Input Select When bits 1:0 in this register are not 00, these bits select which CAP pin is sampled for clocking. Note: If Counter mode is selected for a particular CAPn input in the CTCR, the 3 bits for that input in the Capture Control Register (CCR) must be programmed as 000. However, capture and/or interrupt can be selected for the other 3 CAPn inputs in the same timer.
pub struct CINSEL_R(crate::FieldReader<u8, CINSEL_A>);
impl CINSEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CINSEL_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> CINSEL_A {
        match self.bits {
            0 => CINSEL_A::CHANNEL_0,
            1 => CINSEL_A::CHANNEL_1,
            2 => CINSEL_A::CHANNEL_2,
            3 => CINSEL_A::CHANNEL_3,
            _ => unreachable!(),
        }
    }
    ///Checks if the value of the field is `CHANNEL_0`
    #[inline(always)]
    pub fn is_channel_0(&self) -> bool {
        **self == CINSEL_A::CHANNEL_0
    }
    ///Checks if the value of the field is `CHANNEL_1`
    #[inline(always)]
    pub fn is_channel_1(&self) -> bool {
        **self == CINSEL_A::CHANNEL_1
    }
    ///Checks if the value of the field is `CHANNEL_2`
    #[inline(always)]
    pub fn is_channel_2(&self) -> bool {
        **self == CINSEL_A::CHANNEL_2
    }
    ///Checks if the value of the field is `CHANNEL_3`
    #[inline(always)]
    pub fn is_channel_3(&self) -> bool {
        **self == CINSEL_A::CHANNEL_3
    }
}
impl core::ops::Deref for CINSEL_R {
    type Target = crate::FieldReader<u8, CINSEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `CINSEL` writer - Count Input Select When bits 1:0 in this register are not 00, these bits select which CAP pin is sampled for clocking. Note: If Counter mode is selected for a particular CAPn input in the CTCR, the 3 bits for that input in the Capture Control Register (CCR) must be programmed as 000. However, capture and/or interrupt can be selected for the other 3 CAPn inputs in the same timer.
pub struct CINSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> CINSEL_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: CINSEL_A) -> &'a mut W {
        self.bits(variant.into())
    }
    ///Channel 0. CAPn.0 for CTIMERn
    #[inline(always)]
    pub fn channel_0(self) -> &'a mut W {
        self.variant(CINSEL_A::CHANNEL_0)
    }
    ///Channel 1. CAPn.1 for CTIMERn
    #[inline(always)]
    pub fn channel_1(self) -> &'a mut W {
        self.variant(CINSEL_A::CHANNEL_1)
    }
    ///Channel 2. CAPn.2 for CTIMERn
    #[inline(always)]
    pub fn channel_2(self) -> &'a mut W {
        self.variant(CINSEL_A::CHANNEL_2)
    }
    ///Channel 3. CAPn.3 for CTIMERn
    #[inline(always)]
    pub fn channel_3(self) -> &'a mut W {
        self.variant(CINSEL_A::CHANNEL_3)
    }
    ///Writes raw bits to the field
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | ((value as u32 & 0x03) << 2);
        self.w
    }
}
///Field `ENCC` reader - Setting this bit to 1 enables clearing of the timer and the prescaler when the capture-edge event specified in bits 7:5 occurs.
pub struct ENCC_R(crate::FieldReader<bool, bool>);
impl ENCC_R {
    #[inline(always)]
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
///Field `ENCC` writer - Setting this bit to 1 enables clearing of the timer and the prescaler when the capture-edge event specified in bits 7:5 occurs.
pub struct ENCC_W<'a> {
    w: &'a mut W,
}
impl<'a> ENCC_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | ((value as u32 & 0x01) << 4);
        self.w
    }
}
///Edge select. When bit 4 is 1, these bits select which capture input edge will cause the timer and prescaler to be cleared. These bits have no effect when bit 4 is low. Values 0x2 to 0x3 and 0x6 to 0x7 are reserved.
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SELCC_A {
    ///0: Channel 0 Rising Edge. Rising edge of the signal on capture channel 0 clears the timer (if bit 4 is set).
    CHANNEL_0_RISING = 0,
    ///1: Channel 0 Falling Edge. Falling edge of the signal on capture channel 0 clears the timer (if bit 4 is set).
    CHANNEL_0_FALLING = 1,
    ///2: Channel 1 Rising Edge. Rising edge of the signal on capture channel 1 clears the timer (if bit 4 is set).
    CHANNEL_1_RISING = 2,
    ///3: Channel 1 Falling Edge. Falling edge of the signal on capture channel 1 clears the timer (if bit 4 is set).
    CHANNEL_1_FALLING = 3,
    ///4: Channel 2 Rising Edge. Rising edge of the signal on capture channel 2 clears the timer (if bit 4 is set).
    CHANNEL_2_RISING = 4,
    ///5: Channel 2 Falling Edge. Falling edge of the signal on capture channel 2 clears the timer (if bit 4 is set).
    CHANNEL_2_FALLING = 5,
}
impl From<SELCC_A> for u8 {
    #[inline(always)]
    fn from(variant: SELCC_A) -> Self {
        variant as _
    }
}
///Field `SELCC` reader - Edge select. When bit 4 is 1, these bits select which capture input edge will cause the timer and prescaler to be cleared. These bits have no effect when bit 4 is low. Values 0x2 to 0x3 and 0x6 to 0x7 are reserved.
pub struct SELCC_R(crate::FieldReader<u8, SELCC_A>);
impl SELCC_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        SELCC_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> Option<SELCC_A> {
        match self.bits {
            0 => Some(SELCC_A::CHANNEL_0_RISING),
            1 => Some(SELCC_A::CHANNEL_0_FALLING),
            2 => Some(SELCC_A::CHANNEL_1_RISING),
            3 => Some(SELCC_A::CHANNEL_1_FALLING),
            4 => Some(SELCC_A::CHANNEL_2_RISING),
            5 => Some(SELCC_A::CHANNEL_2_FALLING),
            _ => None,
        }
    }
    ///Checks if the value of the field is `CHANNEL_0_RISING`
    #[inline(always)]
    pub fn is_channel_0_rising(&self) -> bool {
        **self == SELCC_A::CHANNEL_0_RISING
    }
    ///Checks if the value of the field is `CHANNEL_0_FALLING`
    #[inline(always)]
    pub fn is_channel_0_falling(&self) -> bool {
        **self == SELCC_A::CHANNEL_0_FALLING
    }
    ///Checks if the value of the field is `CHANNEL_1_RISING`
    #[inline(always)]
    pub fn is_channel_1_rising(&self) -> bool {
        **self == SELCC_A::CHANNEL_1_RISING
    }
    ///Checks if the value of the field is `CHANNEL_1_FALLING`
    #[inline(always)]
    pub fn is_channel_1_falling(&self) -> bool {
        **self == SELCC_A::CHANNEL_1_FALLING
    }
    ///Checks if the value of the field is `CHANNEL_2_RISING`
    #[inline(always)]
    pub fn is_channel_2_rising(&self) -> bool {
        **self == SELCC_A::CHANNEL_2_RISING
    }
    ///Checks if the value of the field is `CHANNEL_2_FALLING`
    #[inline(always)]
    pub fn is_channel_2_falling(&self) -> bool {
        **self == SELCC_A::CHANNEL_2_FALLING
    }
}
impl core::ops::Deref for SELCC_R {
    type Target = crate::FieldReader<u8, SELCC_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `SELCC` writer - Edge select. When bit 4 is 1, these bits select which capture input edge will cause the timer and prescaler to be cleared. These bits have no effect when bit 4 is low. Values 0x2 to 0x3 and 0x6 to 0x7 are reserved.
pub struct SELCC_W<'a> {
    w: &'a mut W,
}
impl<'a> SELCC_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: SELCC_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    ///Channel 0 Rising Edge. Rising edge of the signal on capture channel 0 clears the timer (if bit 4 is set).
    #[inline(always)]
    pub fn channel_0_rising(self) -> &'a mut W {
        self.variant(SELCC_A::CHANNEL_0_RISING)
    }
    ///Channel 0 Falling Edge. Falling edge of the signal on capture channel 0 clears the timer (if bit 4 is set).
    #[inline(always)]
    pub fn channel_0_falling(self) -> &'a mut W {
        self.variant(SELCC_A::CHANNEL_0_FALLING)
    }
    ///Channel 1 Rising Edge. Rising edge of the signal on capture channel 1 clears the timer (if bit 4 is set).
    #[inline(always)]
    pub fn channel_1_rising(self) -> &'a mut W {
        self.variant(SELCC_A::CHANNEL_1_RISING)
    }
    ///Channel 1 Falling Edge. Falling edge of the signal on capture channel 1 clears the timer (if bit 4 is set).
    #[inline(always)]
    pub fn channel_1_falling(self) -> &'a mut W {
        self.variant(SELCC_A::CHANNEL_1_FALLING)
    }
    ///Channel 2 Rising Edge. Rising edge of the signal on capture channel 2 clears the timer (if bit 4 is set).
    #[inline(always)]
    pub fn channel_2_rising(self) -> &'a mut W {
        self.variant(SELCC_A::CHANNEL_2_RISING)
    }
    ///Channel 2 Falling Edge. Falling edge of the signal on capture channel 2 clears the timer (if bit 4 is set).
    #[inline(always)]
    pub fn channel_2_falling(self) -> &'a mut W {
        self.variant(SELCC_A::CHANNEL_2_FALLING)
    }
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 5)) | ((value as u32 & 0x07) << 5);
        self.w
    }
}
impl R {
    ///Bits 0:1 - Counter/Timer Mode This field selects which rising APB bus clock edges can increment Timer's Prescale Counter (PC), or clear PC and increment Timer Counter (TC). Timer Mode: the TC is incremented when the Prescale Counter matches the Prescale Register.
    #[inline(always)]
    pub fn ctmode(&self) -> CTMODE_R {
        CTMODE_R::new((self.bits & 0x03) as u8)
    }
    ///Bits 2:3 - Count Input Select When bits 1:0 in this register are not 00, these bits select which CAP pin is sampled for clocking. Note: If Counter mode is selected for a particular CAPn input in the CTCR, the 3 bits for that input in the Capture Control Register (CCR) must be programmed as 000. However, capture and/or interrupt can be selected for the other 3 CAPn inputs in the same timer.
    #[inline(always)]
    pub fn cinsel(&self) -> CINSEL_R {
        CINSEL_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    ///Bit 4 - Setting this bit to 1 enables clearing of the timer and the prescaler when the capture-edge event specified in bits 7:5 occurs.
    #[inline(always)]
    pub fn encc(&self) -> ENCC_R {
        ENCC_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    ///Bits 5:7 - Edge select. When bit 4 is 1, these bits select which capture input edge will cause the timer and prescaler to be cleared. These bits have no effect when bit 4 is low. Values 0x2 to 0x3 and 0x6 to 0x7 are reserved.
    #[inline(always)]
    pub fn selcc(&self) -> SELCC_R {
        SELCC_R::new(((self.bits >> 5) & 0x07) as u8)
    }
}
impl W {
    ///Bits 0:1 - Counter/Timer Mode This field selects which rising APB bus clock edges can increment Timer's Prescale Counter (PC), or clear PC and increment Timer Counter (TC). Timer Mode: the TC is incremented when the Prescale Counter matches the Prescale Register.
    #[inline(always)]
    pub fn ctmode(&mut self) -> CTMODE_W {
        CTMODE_W { w: self }
    }
    ///Bits 2:3 - Count Input Select When bits 1:0 in this register are not 00, these bits select which CAP pin is sampled for clocking. Note: If Counter mode is selected for a particular CAPn input in the CTCR, the 3 bits for that input in the Capture Control Register (CCR) must be programmed as 000. However, capture and/or interrupt can be selected for the other 3 CAPn inputs in the same timer.
    #[inline(always)]
    pub fn cinsel(&mut self) -> CINSEL_W {
        CINSEL_W { w: self }
    }
    ///Bit 4 - Setting this bit to 1 enables clearing of the timer and the prescaler when the capture-edge event specified in bits 7:5 occurs.
    #[inline(always)]
    pub fn encc(&mut self) -> ENCC_W {
        ENCC_W { w: self }
    }
    ///Bits 5:7 - Edge select. When bit 4 is 1, these bits select which capture input edge will cause the timer and prescaler to be cleared. These bits have no effect when bit 4 is low. Values 0x2 to 0x3 and 0x6 to 0x7 are reserved.
    #[inline(always)]
    pub fn selcc(&mut self) -> SELCC_W {
        SELCC_W { w: self }
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///Count Control Register. The CTCR selects between Timer and Counter mode, and in Counter mode selects the signal and edge(s) for counting.
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [ctcr](index.html) module
pub struct CTCR_SPEC;
impl crate::RegisterSpec for CTCR_SPEC {
    type Ux = u32;
}
///`read()` method returns [ctcr::R](R) reader structure
impl crate::Readable for CTCR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [ctcr::W](W) writer structure
impl crate::Writable for CTCR_SPEC {
    type Writer = W;
}
///`reset()` method sets CTCR to value 0
impl crate::Resettable for CTCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
