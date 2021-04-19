#[doc = "Register `CTRL` reader"]
pub struct R(crate::R<CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<CTRL_SPEC>> for R {
    fn from(reader: crate::R<CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTRL` writer"]
pub struct W(crate::W<CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTRL_SPEC>;
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
impl core::convert::From<crate::W<CTRL_SPEC>> for W {
    fn from(writer: crate::W<CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Mode of operation. May only change from 0 to another value. So, if 2 or 3, must be changed to 0 1st. Any attempt to go from non-0 to non-0 will result in 0 anyway.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum POLLMODE_A {
    #[doc = "0: None, inactive. Poll and time counters are turned off. Writing this will reset state and stop any collection in progress. Note: this has no effect on STATUS - those must be cleared manually."]
    NONE = 0,
    #[doc = "1: Poll now - forces a manual poll to be started immediately, using XPINSEL X pin(s) to activate in the integration loop (all pins set together). Self clears - clear is not indication it is done (see STATUS)."]
    POLL_NOW = 1,
    #[doc = "2: Normal polling using poll delay from POLL_TCNT register. This will start with the poll delay (which can be 0)."]
    NORMAL = 2,
    #[doc = "3: The CAPT block will operate in low-power mode. This means it will use GPIO as input, use combination touch measurements, and assume it is to wake the system. This will use the POLL_TCNT poll delay, and start with the delay."]
    LOW_POWER_MODE = 3,
}
impl From<POLLMODE_A> for u8 {
    #[inline(always)]
    fn from(variant: POLLMODE_A) -> Self {
        variant as _
    }
}
#[doc = "Field `POLLMODE` reader - Mode of operation. May only change from 0 to another value. So, if 2 or 3, must be changed to 0 1st. Any attempt to go from non-0 to non-0 will result in 0 anyway."]
pub struct POLLMODE_R(crate::FieldReader<u8, POLLMODE_A>);
impl POLLMODE_R {
    pub(crate) fn new(bits: u8) -> Self {
        POLLMODE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> POLLMODE_A {
        match self.bits {
            0 => POLLMODE_A::NONE,
            1 => POLLMODE_A::POLL_NOW,
            2 => POLLMODE_A::NORMAL,
            3 => POLLMODE_A::LOW_POWER_MODE,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        **self == POLLMODE_A::NONE
    }
    #[doc = "Checks if the value of the field is `POLL_NOW`"]
    #[inline(always)]
    pub fn is_poll_now(&self) -> bool {
        **self == POLLMODE_A::POLL_NOW
    }
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        **self == POLLMODE_A::NORMAL
    }
    #[doc = "Checks if the value of the field is `LOW_POWER_MODE`"]
    #[inline(always)]
    pub fn is_low_power_mode(&self) -> bool {
        **self == POLLMODE_A::LOW_POWER_MODE
    }
}
impl core::ops::Deref for POLLMODE_R {
    type Target = crate::FieldReader<u8, POLLMODE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `POLLMODE` writer - Mode of operation. May only change from 0 to another value. So, if 2 or 3, must be changed to 0 1st. Any attempt to go from non-0 to non-0 will result in 0 anyway."]
pub struct POLLMODE_W<'a> {
    w: &'a mut W,
}
impl<'a> POLLMODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: POLLMODE_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "None, inactive. Poll and time counters are turned off. Writing this will reset state and stop any collection in progress. Note: this has no effect on STATUS - those must be cleared manually."]
    #[inline(always)]
    pub fn none(self) -> &'a mut W {
        self.variant(POLLMODE_A::NONE)
    }
    #[doc = "Poll now - forces a manual poll to be started immediately, using XPINSEL X pin(s) to activate in the integration loop (all pins set together). Self clears - clear is not indication it is done (see STATUS)."]
    #[inline(always)]
    pub fn poll_now(self) -> &'a mut W {
        self.variant(POLLMODE_A::POLL_NOW)
    }
    #[doc = "Normal polling using poll delay from POLL_TCNT register. This will start with the poll delay (which can be 0)."]
    #[inline(always)]
    pub fn normal(self) -> &'a mut W {
        self.variant(POLLMODE_A::NORMAL)
    }
    #[doc = "The CAPT block will operate in low-power mode. This means it will use GPIO as input, use combination touch measurements, and assume it is to wake the system. This will use the POLL_TCNT poll delay, and start with the delay."]
    #[inline(always)]
    pub fn low_power_mode(self) -> &'a mut W {
        self.variant(POLLMODE_A::LOW_POWER_MODE)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | (value as u32 & 0x03);
        self.w
    }
}
#[doc = "Selects type of Touch arrangement to use and so how to handle XPINSEL bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum TYPE_A {
    #[doc = "0: Normal - all X elements are treated as normal, such as buttons and sliders."]
    TYPE_0 = 0,
    #[doc = "1: 3x3 grid using NXP Complementary measurements. The 1st 9 Xs are assumed to be the 3x3 grid. After that would be normal X elements. This will also allow 3x1 and 3x2 Note: Only possible if XMAX in STATUS is >=8"]
    TYPE_1 = 1,
    #[doc = "2: 5 Sensors interleaved to act as 3x3 touch area using NXP Complementary measurements. 1st 5 Xs used for this, all remaining are treated as normal. Note that if 16 X pins allowed, the 16th will not be usable when TYPE=1. (use TYPE=0 and select 1 smaller than 15 ( and any others from 1 smaller than 5 on up in XPINSEL)."]
    TYPE_2 = 2,
    #[doc = "3: 9 Sensors interleaved to act as 5x5 touch area using NXP Complementary measurements. 1st 9 Xs used for this, all remaining are treated as normal. Note: Only possible if XMAX in STATUS is >=8"]
    TYPE_3 = 3,
}
impl From<TYPE_A> for u8 {
    #[inline(always)]
    fn from(variant: TYPE_A) -> Self {
        variant as _
    }
}
#[doc = "Field `TYPE` reader - Selects type of Touch arrangement to use and so how to handle XPINSEL bits"]
pub struct TYPE_R(crate::FieldReader<u8, TYPE_A>);
impl TYPE_R {
    pub(crate) fn new(bits: u8) -> Self {
        TYPE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TYPE_A {
        match self.bits {
            0 => TYPE_A::TYPE_0,
            1 => TYPE_A::TYPE_1,
            2 => TYPE_A::TYPE_2,
            3 => TYPE_A::TYPE_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `TYPE_0`"]
    #[inline(always)]
    pub fn is_type_0(&self) -> bool {
        **self == TYPE_A::TYPE_0
    }
    #[doc = "Checks if the value of the field is `TYPE_1`"]
    #[inline(always)]
    pub fn is_type_1(&self) -> bool {
        **self == TYPE_A::TYPE_1
    }
    #[doc = "Checks if the value of the field is `TYPE_2`"]
    #[inline(always)]
    pub fn is_type_2(&self) -> bool {
        **self == TYPE_A::TYPE_2
    }
    #[doc = "Checks if the value of the field is `TYPE_3`"]
    #[inline(always)]
    pub fn is_type_3(&self) -> bool {
        **self == TYPE_A::TYPE_3
    }
}
impl core::ops::Deref for TYPE_R {
    type Target = crate::FieldReader<u8, TYPE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TYPE` writer - Selects type of Touch arrangement to use and so how to handle XPINSEL bits"]
pub struct TYPE_W<'a> {
    w: &'a mut W,
}
impl<'a> TYPE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TYPE_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Normal - all X elements are treated as normal, such as buttons and sliders."]
    #[inline(always)]
    pub fn type_0(self) -> &'a mut W {
        self.variant(TYPE_A::TYPE_0)
    }
    #[doc = "3x3 grid using NXP Complementary measurements. The 1st 9 Xs are assumed to be the 3x3 grid. After that would be normal X elements. This will also allow 3x1 and 3x2 Note: Only possible if XMAX in STATUS is >=8"]
    #[inline(always)]
    pub fn type_1(self) -> &'a mut W {
        self.variant(TYPE_A::TYPE_1)
    }
    #[doc = "5 Sensors interleaved to act as 3x3 touch area using NXP Complementary measurements. 1st 5 Xs used for this, all remaining are treated as normal. Note that if 16 X pins allowed, the 16th will not be usable when TYPE=1. (use TYPE=0 and select 1 smaller than 15 ( and any others from 1 smaller than 5 on up in XPINSEL)."]
    #[inline(always)]
    pub fn type_2(self) -> &'a mut W {
        self.variant(TYPE_A::TYPE_2)
    }
    #[doc = "9 Sensors interleaved to act as 5x5 touch area using NXP Complementary measurements. 1st 9 Xs used for this, all remaining are treated as normal. Note: Only possible if XMAX in STATUS is >=8"]
    #[inline(always)]
    pub fn type_3(self) -> &'a mut W {
        self.variant(TYPE_A::TYPE_3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | ((value as u32 & 0x03) << 2);
        self.w
    }
}
#[doc = "This selects what is being used as the trigger\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TRIGGER_A {
    #[doc = "0: Uses YH GPIO. This is not normally used except in Low-power mode. But, it can be used with POLLNOW to baseline that measurement."]
    USES_YH_GPIO = 0,
    #[doc = "1: ACMP (if fitted). This assumes the ACMP state is fed in asynchronously and it will sample."]
    ACMP = 1,
}
impl From<TRIGGER_A> for bool {
    #[inline(always)]
    fn from(variant: TRIGGER_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TRIGGER` reader - This selects what is being used as the trigger"]
pub struct TRIGGER_R(crate::FieldReader<bool, TRIGGER_A>);
impl TRIGGER_R {
    pub(crate) fn new(bits: bool) -> Self {
        TRIGGER_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TRIGGER_A {
        match self.bits {
            false => TRIGGER_A::USES_YH_GPIO,
            true => TRIGGER_A::ACMP,
        }
    }
    #[doc = "Checks if the value of the field is `USES_YH_GPIO`"]
    #[inline(always)]
    pub fn is_uses_yh_gpio(&self) -> bool {
        **self == TRIGGER_A::USES_YH_GPIO
    }
    #[doc = "Checks if the value of the field is `ACMP`"]
    #[inline(always)]
    pub fn is_acmp(&self) -> bool {
        **self == TRIGGER_A::ACMP
    }
}
impl core::ops::Deref for TRIGGER_R {
    type Target = crate::FieldReader<bool, TRIGGER_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TRIGGER` writer - This selects what is being used as the trigger"]
pub struct TRIGGER_W<'a> {
    w: &'a mut W,
}
impl<'a> TRIGGER_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TRIGGER_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Uses YH GPIO. This is not normally used except in Low-power mode. But, it can be used with POLLNOW to baseline that measurement."]
    #[inline(always)]
    pub fn uses_yh_gpio(self) -> &'a mut W {
        self.variant(TRIGGER_A::USES_YH_GPIO)
    }
    #[doc = "ACMP (if fitted). This assumes the ACMP state is fed in asynchronously and it will sample."]
    #[inline(always)]
    pub fn acmp(self) -> &'a mut W {
        self.variant(TRIGGER_A::ACMP)
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | ((value as u32 & 0x01) << 4);
        self.w
    }
}
#[doc = "Field `WAIT` reader - If 0, the block will continue its X based measurements, even if the TOUCH register has not been read (and so could OVERRUN). If 1, it will wait until read when a touch (TOUCH's ISTOUCH bit is set) before starting the next. This should not normally be needed."]
pub struct WAIT_R(crate::FieldReader<bool, bool>);
impl WAIT_R {
    pub(crate) fn new(bits: bool) -> Self {
        WAIT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WAIT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WAIT` writer - If 0, the block will continue its X based measurements, even if the TOUCH register has not been read (and so could OVERRUN). If 1, it will wait until read when a touch (TOUCH's ISTOUCH bit is set) before starting the next. This should not normally be needed."]
pub struct WAIT_W<'a> {
    w: &'a mut W,
}
impl<'a> WAIT_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | ((value as u32 & 0x01) << 5);
        self.w
    }
}
#[doc = "If not 0, will use the DMA to read out touch events from TOUCH register. The values are shown below. This may be changed while active.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum DMA_A {
    #[doc = "0: No DMA. Application will use ISRs to read out data"]
    DMA_0 = 0,
    #[doc = "1: Trigger DMA on Touch events"]
    DMA_1 = 1,
    #[doc = "2: Trigger DMA on both Touch and No-Touch events"]
    DMA_2 = 2,
    #[doc = "3: Trigger DMA on both plus Timeout."]
    DMA_3 = 3,
}
impl From<DMA_A> for u8 {
    #[inline(always)]
    fn from(variant: DMA_A) -> Self {
        variant as _
    }
}
#[doc = "Field `DMA` reader - If not 0, will use the DMA to read out touch events from TOUCH register. The values are shown below. This may be changed while active."]
pub struct DMA_R(crate::FieldReader<u8, DMA_A>);
impl DMA_R {
    pub(crate) fn new(bits: u8) -> Self {
        DMA_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DMA_A {
        match self.bits {
            0 => DMA_A::DMA_0,
            1 => DMA_A::DMA_1,
            2 => DMA_A::DMA_2,
            3 => DMA_A::DMA_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DMA_0`"]
    #[inline(always)]
    pub fn is_dma_0(&self) -> bool {
        **self == DMA_A::DMA_0
    }
    #[doc = "Checks if the value of the field is `DMA_1`"]
    #[inline(always)]
    pub fn is_dma_1(&self) -> bool {
        **self == DMA_A::DMA_1
    }
    #[doc = "Checks if the value of the field is `DMA_2`"]
    #[inline(always)]
    pub fn is_dma_2(&self) -> bool {
        **self == DMA_A::DMA_2
    }
    #[doc = "Checks if the value of the field is `DMA_3`"]
    #[inline(always)]
    pub fn is_dma_3(&self) -> bool {
        **self == DMA_A::DMA_3
    }
}
impl core::ops::Deref for DMA_R {
    type Target = crate::FieldReader<u8, DMA_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DMA` writer - If not 0, will use the DMA to read out touch events from TOUCH register. The values are shown below. This may be changed while active."]
pub struct DMA_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DMA_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "No DMA. Application will use ISRs to read out data"]
    #[inline(always)]
    pub fn dma_0(self) -> &'a mut W {
        self.variant(DMA_A::DMA_0)
    }
    #[doc = "Trigger DMA on Touch events"]
    #[inline(always)]
    pub fn dma_1(self) -> &'a mut W {
        self.variant(DMA_A::DMA_1)
    }
    #[doc = "Trigger DMA on both Touch and No-Touch events"]
    #[inline(always)]
    pub fn dma_2(self) -> &'a mut W {
        self.variant(DMA_A::DMA_2)
    }
    #[doc = "Trigger DMA on both plus Timeout."]
    #[inline(always)]
    pub fn dma_3(self) -> &'a mut W {
        self.variant(DMA_A::DMA_3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | ((value as u32 & 0x03) << 6);
        self.w
    }
}
#[doc = "Functional clock divider, or 0 if no divide. The term \"clocks\" in this spec then refer to divided clocks. For a 12MHz input (e.g. FRO 12MHz), this would normally be set to generate a 4MHz output (so, 2). For a 1MHz input, it should be 0. Note for internal use: this does not produce a 50/50 duty cycle when non even divide.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum FDIV_A {
    #[doc = "0: No divide"]
    FDIV_0 = 0,
    #[doc = "1: /2"]
    FDIV_1 = 1,
    #[doc = "2: /3"]
    FDIV_2 = 2,
    #[doc = "3: /4"]
    FDIV_3 = 3,
    #[doc = "4: /5"]
    FDIV_4 = 4,
    #[doc = "5: /6"]
    FDIV_5 = 5,
    #[doc = "7: /(FDIV+1)"]
    FDIV_7 = 7,
    #[doc = "8: /(FDIV+1)"]
    FDIV_8 = 8,
    #[doc = "9: /(FDIV+1)"]
    FDIV_9 = 9,
    #[doc = "10: /(FDIV+1)"]
    FDIV_10 = 10,
    #[doc = "11: /(FDIV+1)"]
    FDIV_11 = 11,
    #[doc = "12: /(FDIV+1)"]
    FDIV_12 = 12,
    #[doc = "13: /(FDIV+1)"]
    FDIV_13 = 13,
    #[doc = "14: /(FDIV+1)"]
    FDIV_14 = 14,
    #[doc = "15: /(FDIV+1)"]
    FDIV_15 = 15,
}
impl From<FDIV_A> for u8 {
    #[inline(always)]
    fn from(variant: FDIV_A) -> Self {
        variant as _
    }
}
#[doc = "Field `FDIV` reader - Functional clock divider, or 0 if no divide. The term \"clocks\" in this spec then refer to divided clocks. For a 12MHz input (e.g. FRO 12MHz), this would normally be set to generate a 4MHz output (so, 2). For a 1MHz input, it should be 0. Note for internal use: this does not produce a 50/50 duty cycle when non even divide."]
pub struct FDIV_R(crate::FieldReader<u8, FDIV_A>);
impl FDIV_R {
    pub(crate) fn new(bits: u8) -> Self {
        FDIV_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<FDIV_A> {
        match self.bits {
            0 => Some(FDIV_A::FDIV_0),
            1 => Some(FDIV_A::FDIV_1),
            2 => Some(FDIV_A::FDIV_2),
            3 => Some(FDIV_A::FDIV_3),
            4 => Some(FDIV_A::FDIV_4),
            5 => Some(FDIV_A::FDIV_5),
            7 => Some(FDIV_A::FDIV_7),
            8 => Some(FDIV_A::FDIV_8),
            9 => Some(FDIV_A::FDIV_9),
            10 => Some(FDIV_A::FDIV_10),
            11 => Some(FDIV_A::FDIV_11),
            12 => Some(FDIV_A::FDIV_12),
            13 => Some(FDIV_A::FDIV_13),
            14 => Some(FDIV_A::FDIV_14),
            15 => Some(FDIV_A::FDIV_15),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `FDIV_0`"]
    #[inline(always)]
    pub fn is_fdiv_0(&self) -> bool {
        **self == FDIV_A::FDIV_0
    }
    #[doc = "Checks if the value of the field is `FDIV_1`"]
    #[inline(always)]
    pub fn is_fdiv_1(&self) -> bool {
        **self == FDIV_A::FDIV_1
    }
    #[doc = "Checks if the value of the field is `FDIV_2`"]
    #[inline(always)]
    pub fn is_fdiv_2(&self) -> bool {
        **self == FDIV_A::FDIV_2
    }
    #[doc = "Checks if the value of the field is `FDIV_3`"]
    #[inline(always)]
    pub fn is_fdiv_3(&self) -> bool {
        **self == FDIV_A::FDIV_3
    }
    #[doc = "Checks if the value of the field is `FDIV_4`"]
    #[inline(always)]
    pub fn is_fdiv_4(&self) -> bool {
        **self == FDIV_A::FDIV_4
    }
    #[doc = "Checks if the value of the field is `FDIV_5`"]
    #[inline(always)]
    pub fn is_fdiv_5(&self) -> bool {
        **self == FDIV_A::FDIV_5
    }
    #[doc = "Checks if the value of the field is `FDIV_7`"]
    #[inline(always)]
    pub fn is_fdiv_7(&self) -> bool {
        **self == FDIV_A::FDIV_7
    }
    #[doc = "Checks if the value of the field is `FDIV_8`"]
    #[inline(always)]
    pub fn is_fdiv_8(&self) -> bool {
        **self == FDIV_A::FDIV_8
    }
    #[doc = "Checks if the value of the field is `FDIV_9`"]
    #[inline(always)]
    pub fn is_fdiv_9(&self) -> bool {
        **self == FDIV_A::FDIV_9
    }
    #[doc = "Checks if the value of the field is `FDIV_10`"]
    #[inline(always)]
    pub fn is_fdiv_10(&self) -> bool {
        **self == FDIV_A::FDIV_10
    }
    #[doc = "Checks if the value of the field is `FDIV_11`"]
    #[inline(always)]
    pub fn is_fdiv_11(&self) -> bool {
        **self == FDIV_A::FDIV_11
    }
    #[doc = "Checks if the value of the field is `FDIV_12`"]
    #[inline(always)]
    pub fn is_fdiv_12(&self) -> bool {
        **self == FDIV_A::FDIV_12
    }
    #[doc = "Checks if the value of the field is `FDIV_13`"]
    #[inline(always)]
    pub fn is_fdiv_13(&self) -> bool {
        **self == FDIV_A::FDIV_13
    }
    #[doc = "Checks if the value of the field is `FDIV_14`"]
    #[inline(always)]
    pub fn is_fdiv_14(&self) -> bool {
        **self == FDIV_A::FDIV_14
    }
    #[doc = "Checks if the value of the field is `FDIV_15`"]
    #[inline(always)]
    pub fn is_fdiv_15(&self) -> bool {
        **self == FDIV_A::FDIV_15
    }
}
impl core::ops::Deref for FDIV_R {
    type Target = crate::FieldReader<u8, FDIV_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FDIV` writer - Functional clock divider, or 0 if no divide. The term \"clocks\" in this spec then refer to divided clocks. For a 12MHz input (e.g. FRO 12MHz), this would normally be set to generate a 4MHz output (so, 2). For a 1MHz input, it should be 0. Note for internal use: this does not produce a 50/50 duty cycle when non even divide."]
pub struct FDIV_W<'a> {
    w: &'a mut W,
}
impl<'a> FDIV_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FDIV_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "No divide"]
    #[inline(always)]
    pub fn fdiv_0(self) -> &'a mut W {
        self.variant(FDIV_A::FDIV_0)
    }
    #[doc = "/2"]
    #[inline(always)]
    pub fn fdiv_1(self) -> &'a mut W {
        self.variant(FDIV_A::FDIV_1)
    }
    #[doc = "/3"]
    #[inline(always)]
    pub fn fdiv_2(self) -> &'a mut W {
        self.variant(FDIV_A::FDIV_2)
    }
    #[doc = "/4"]
    #[inline(always)]
    pub fn fdiv_3(self) -> &'a mut W {
        self.variant(FDIV_A::FDIV_3)
    }
    #[doc = "/5"]
    #[inline(always)]
    pub fn fdiv_4(self) -> &'a mut W {
        self.variant(FDIV_A::FDIV_4)
    }
    #[doc = "/6"]
    #[inline(always)]
    pub fn fdiv_5(self) -> &'a mut W {
        self.variant(FDIV_A::FDIV_5)
    }
    #[doc = "/(FDIV+1)"]
    #[inline(always)]
    pub fn fdiv_7(self) -> &'a mut W {
        self.variant(FDIV_A::FDIV_7)
    }
    #[doc = "/(FDIV+1)"]
    #[inline(always)]
    pub fn fdiv_8(self) -> &'a mut W {
        self.variant(FDIV_A::FDIV_8)
    }
    #[doc = "/(FDIV+1)"]
    #[inline(always)]
    pub fn fdiv_9(self) -> &'a mut W {
        self.variant(FDIV_A::FDIV_9)
    }
    #[doc = "/(FDIV+1)"]
    #[inline(always)]
    pub fn fdiv_10(self) -> &'a mut W {
        self.variant(FDIV_A::FDIV_10)
    }
    #[doc = "/(FDIV+1)"]
    #[inline(always)]
    pub fn fdiv_11(self) -> &'a mut W {
        self.variant(FDIV_A::FDIV_11)
    }
    #[doc = "/(FDIV+1)"]
    #[inline(always)]
    pub fn fdiv_12(self) -> &'a mut W {
        self.variant(FDIV_A::FDIV_12)
    }
    #[doc = "/(FDIV+1)"]
    #[inline(always)]
    pub fn fdiv_13(self) -> &'a mut W {
        self.variant(FDIV_A::FDIV_13)
    }
    #[doc = "/(FDIV+1)"]
    #[inline(always)]
    pub fn fdiv_14(self) -> &'a mut W {
        self.variant(FDIV_A::FDIV_14)
    }
    #[doc = "/(FDIV+1)"]
    #[inline(always)]
    pub fn fdiv_15(self) -> &'a mut W {
        self.variant(FDIV_A::FDIV_15)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | ((value as u32 & 0x0f) << 8);
        self.w
    }
}
#[doc = "Controls how X pins selected in XPINSEL are used when not active in the current polling round.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum XPINUSE_A {
    #[doc = "0: Normal mode. Each inactive X pin is Hi-Z."]
    NORMAL_MDOE = 0,
    #[doc = "1: Ground mode. Each inactive X pin is Low"]
    GROUND_MDOE = 1,
}
impl From<XPINUSE_A> for u8 {
    #[inline(always)]
    fn from(variant: XPINUSE_A) -> Self {
        variant as _
    }
}
#[doc = "Field `XPINUSE` reader - Controls how X pins selected in XPINSEL are used when not active in the current polling round."]
pub struct XPINUSE_R(crate::FieldReader<u8, XPINUSE_A>);
impl XPINUSE_R {
    pub(crate) fn new(bits: u8) -> Self {
        XPINUSE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<XPINUSE_A> {
        match self.bits {
            0 => Some(XPINUSE_A::NORMAL_MDOE),
            1 => Some(XPINUSE_A::GROUND_MDOE),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `NORMAL_MDOE`"]
    #[inline(always)]
    pub fn is_normal_mdoe(&self) -> bool {
        **self == XPINUSE_A::NORMAL_MDOE
    }
    #[doc = "Checks if the value of the field is `GROUND_MDOE`"]
    #[inline(always)]
    pub fn is_ground_mdoe(&self) -> bool {
        **self == XPINUSE_A::GROUND_MDOE
    }
}
impl core::ops::Deref for XPINUSE_R {
    type Target = crate::FieldReader<u8, XPINUSE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `XPINUSE` writer - Controls how X pins selected in XPINSEL are used when not active in the current polling round."]
pub struct XPINUSE_W<'a> {
    w: &'a mut W,
}
impl<'a> XPINUSE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: XPINUSE_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Normal mode. Each inactive X pin is Hi-Z."]
    #[inline(always)]
    pub fn normal_mdoe(self) -> &'a mut W {
        self.variant(XPINUSE_A::NORMAL_MDOE)
    }
    #[doc = "Ground mode. Each inactive X pin is Low"]
    #[inline(always)]
    pub fn ground_mdoe(self) -> &'a mut W {
        self.variant(XPINUSE_A::GROUND_MDOE)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 12)) | ((value as u32 & 0x03) << 12);
        self.w
    }
}
#[doc = "Field `INCHANGE` reader - If 1, do not attempt to write to this register again. This means the last change has not been propagated. This can only happen after changing POLLMODE and DMA. Worse case time would be based on divided FCLK."]
pub struct INCHANGE_R(crate::FieldReader<bool, bool>);
impl INCHANGE_R {
    pub(crate) fn new(bits: bool) -> Self {
        INCHANGE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INCHANGE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `XPINSEL` reader - Selects which of the X pins are to be used within the allowed pins - see XMAX in STATUS. The X pins are mapped via the IOCON (as are the YH and YL pins) to physical pads. So, this only selects which are to be used as the X half of the touch element. Note: when polling, these are \"walked\" (active) one at a time. When using POLLNOW, the 1 or more selected are used at the same time. Likewise, when in low-power mode, they are used at the same time (or small groups). X pads not selected by XPINSEL are kept at High-Z if they are connected to a pad. This allows using controlled sets for touch detection based on context."]
pub struct XPINSEL_R(crate::FieldReader<u16, u16>);
impl XPINSEL_R {
    pub(crate) fn new(bits: u16) -> Self {
        XPINSEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for XPINSEL_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `XPINSEL` writer - Selects which of the X pins are to be used within the allowed pins - see XMAX in STATUS. The X pins are mapped via the IOCON (as are the YH and YL pins) to physical pads. So, this only selects which are to be used as the X half of the touch element. Note: when polling, these are \"walked\" (active) one at a time. When using POLLNOW, the 1 or more selected are used at the same time. Likewise, when in low-power mode, they are used at the same time (or small groups). X pads not selected by XPINSEL are kept at High-Z if they are connected to a pad. This allows using controlled sets for touch detection based on context."]
pub struct XPINSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> XPINSEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | ((value as u32 & 0xffff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - Mode of operation. May only change from 0 to another value. So, if 2 or 3, must be changed to 0 1st. Any attempt to go from non-0 to non-0 will result in 0 anyway."]
    #[inline(always)]
    pub fn pollmode(&self) -> POLLMODE_R {
        POLLMODE_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 2:3 - Selects type of Touch arrangement to use and so how to handle XPINSEL bits"]
    #[inline(always)]
    pub fn type_(&self) -> TYPE_R {
        TYPE_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bit 4 - This selects what is being used as the trigger"]
    #[inline(always)]
    pub fn trigger(&self) -> TRIGGER_R {
        TRIGGER_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - If 0, the block will continue its X based measurements, even if the TOUCH register has not been read (and so could OVERRUN). If 1, it will wait until read when a touch (TOUCH's ISTOUCH bit is set) before starting the next. This should not normally be needed."]
    #[inline(always)]
    pub fn wait(&self) -> WAIT_R {
        WAIT_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bits 6:7 - If not 0, will use the DMA to read out touch events from TOUCH register. The values are shown below. This may be changed while active."]
    #[inline(always)]
    pub fn dma(&self) -> DMA_R {
        DMA_R::new(((self.bits >> 6) & 0x03) as u8)
    }
    #[doc = "Bits 8:11 - Functional clock divider, or 0 if no divide. The term \"clocks\" in this spec then refer to divided clocks. For a 12MHz input (e.g. FRO 12MHz), this would normally be set to generate a 4MHz output (so, 2). For a 1MHz input, it should be 0. Note for internal use: this does not produce a 50/50 duty cycle when non even divide."]
    #[inline(always)]
    pub fn fdiv(&self) -> FDIV_R {
        FDIV_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:13 - Controls how X pins selected in XPINSEL are used when not active in the current polling round."]
    #[inline(always)]
    pub fn xpinuse(&self) -> XPINUSE_R {
        XPINUSE_R::new(((self.bits >> 12) & 0x03) as u8)
    }
    #[doc = "Bit 15 - If 1, do not attempt to write to this register again. This means the last change has not been propagated. This can only happen after changing POLLMODE and DMA. Worse case time would be based on divided FCLK."]
    #[inline(always)]
    pub fn inchange(&self) -> INCHANGE_R {
        INCHANGE_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bits 16:31 - Selects which of the X pins are to be used within the allowed pins - see XMAX in STATUS. The X pins are mapped via the IOCON (as are the YH and YL pins) to physical pads. So, this only selects which are to be used as the X half of the touch element. Note: when polling, these are \"walked\" (active) one at a time. When using POLLNOW, the 1 or more selected are used at the same time. Likewise, when in low-power mode, they are used at the same time (or small groups). X pads not selected by XPINSEL are kept at High-Z if they are connected to a pad. This allows using controlled sets for touch detection based on context."]
    #[inline(always)]
    pub fn xpinsel(&self) -> XPINSEL_R {
        XPINSEL_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:1 - Mode of operation. May only change from 0 to another value. So, if 2 or 3, must be changed to 0 1st. Any attempt to go from non-0 to non-0 will result in 0 anyway."]
    #[inline(always)]
    pub fn pollmode(&mut self) -> POLLMODE_W {
        POLLMODE_W { w: self }
    }
    #[doc = "Bits 2:3 - Selects type of Touch arrangement to use and so how to handle XPINSEL bits"]
    #[inline(always)]
    pub fn type_(&mut self) -> TYPE_W {
        TYPE_W { w: self }
    }
    #[doc = "Bit 4 - This selects what is being used as the trigger"]
    #[inline(always)]
    pub fn trigger(&mut self) -> TRIGGER_W {
        TRIGGER_W { w: self }
    }
    #[doc = "Bit 5 - If 0, the block will continue its X based measurements, even if the TOUCH register has not been read (and so could OVERRUN). If 1, it will wait until read when a touch (TOUCH's ISTOUCH bit is set) before starting the next. This should not normally be needed."]
    #[inline(always)]
    pub fn wait(&mut self) -> WAIT_W {
        WAIT_W { w: self }
    }
    #[doc = "Bits 6:7 - If not 0, will use the DMA to read out touch events from TOUCH register. The values are shown below. This may be changed while active."]
    #[inline(always)]
    pub fn dma(&mut self) -> DMA_W {
        DMA_W { w: self }
    }
    #[doc = "Bits 8:11 - Functional clock divider, or 0 if no divide. The term \"clocks\" in this spec then refer to divided clocks. For a 12MHz input (e.g. FRO 12MHz), this would normally be set to generate a 4MHz output (so, 2). For a 1MHz input, it should be 0. Note for internal use: this does not produce a 50/50 duty cycle when non even divide."]
    #[inline(always)]
    pub fn fdiv(&mut self) -> FDIV_W {
        FDIV_W { w: self }
    }
    #[doc = "Bits 12:13 - Controls how X pins selected in XPINSEL are used when not active in the current polling round."]
    #[inline(always)]
    pub fn xpinuse(&mut self) -> XPINUSE_W {
        XPINUSE_W { w: self }
    }
    #[doc = "Bits 16:31 - Selects which of the X pins are to be used within the allowed pins - see XMAX in STATUS. The X pins are mapped via the IOCON (as are the YH and YL pins) to physical pads. So, this only selects which are to be used as the X half of the touch element. Note: when polling, these are \"walked\" (active) one at a time. When using POLLNOW, the 1 or more selected are used at the same time. Likewise, when in low-power mode, they are used at the same time (or small groups). X pads not selected by XPINSEL are kept at High-Z if they are connected to a pad. This allows using controlled sets for touch detection based on context."]
    #[inline(always)]
    pub fn xpinsel(&mut self) -> XPINSEL_W {
        XPINSEL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Configuration and control to setup the functional clock, the rules, and the pin selections and rules.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrl](index.html) module"]
pub struct CTRL_SPEC;
impl crate::RegisterSpec for CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ctrl::R](R) reader structure"]
impl crate::Readable for CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctrl::W](W) writer structure"]
impl crate::Writable for CTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CTRL to value 0"]
impl crate::Resettable for CTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
