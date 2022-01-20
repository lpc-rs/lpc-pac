#[doc = "Register `CFG1` reader"]
pub struct R(crate::R<CFG1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CFG1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CFG1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CFG1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CFG1` writer"]
pub struct W(crate::W<CFG1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CFG1_SPEC>;
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
impl From<crate::W<CFG1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CFG1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Main enable for I 2S function in this Flexcomm\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MAINENABLE_A {
    #[doc = "0: All I 2S channel pairs in this Flexcomm are disabled and the internal state machines, counters, and flags are reset. No other channel pairs can be enabled."]
    DISABLED = 0,
    #[doc = "1: This I 2S channel pair is enabled. Other channel pairs in this Flexcomm may be enabled in their individual PAIRENABLE bits."]
    ENABLED = 1,
}
impl From<MAINENABLE_A> for bool {
    #[inline(always)]
    fn from(variant: MAINENABLE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MAINENABLE` reader - Main enable for I 2S function in this Flexcomm"]
pub struct MAINENABLE_R(crate::FieldReader<bool, MAINENABLE_A>);
impl MAINENABLE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        MAINENABLE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MAINENABLE_A {
        match self.bits {
            false => MAINENABLE_A::DISABLED,
            true => MAINENABLE_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == MAINENABLE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == MAINENABLE_A::ENABLED
    }
}
impl core::ops::Deref for MAINENABLE_R {
    type Target = crate::FieldReader<bool, MAINENABLE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MAINENABLE` writer - Main enable for I 2S function in this Flexcomm"]
pub struct MAINENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> MAINENABLE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MAINENABLE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "All I 2S channel pairs in this Flexcomm are disabled and the internal state machines, counters, and flags are reset. No other channel pairs can be enabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(MAINENABLE_A::DISABLED)
    }
    #[doc = "This I 2S channel pair is enabled. Other channel pairs in this Flexcomm may be enabled in their individual PAIRENABLE bits."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(MAINENABLE_A::ENABLED)
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
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
#[doc = "Data flow Pause. Allows pausing data flow between the I2S serializer/deserializer and the FIFO. This could be done in order to change streams, or while restarting after a data underflow or overflow. When paused, FIFO operations can be done without corrupting data that is in the process of being sent or received. Once a data pause has been requested, the interface may need to complete sending data that was in progress before interrupting the flow of data. Software must check that the pause is actually in effect before taking action. This is done by monitoring the DATAPAUSED flag in the STAT register. When DATAPAUSE is cleared, data transfer will resume at the beginning of the next frame.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DATAPAUSE_A {
    #[doc = "0: Normal operation, or resuming normal operation at the next frame if the I2S has already been paused."]
    NORMAL = 0,
    #[doc = "1: A pause in the data flow is being requested. It is in effect when DATAPAUSED in STAT = 1."]
    PAUSE = 1,
}
impl From<DATAPAUSE_A> for bool {
    #[inline(always)]
    fn from(variant: DATAPAUSE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DATAPAUSE` reader - Data flow Pause. Allows pausing data flow between the I2S serializer/deserializer and the FIFO. This could be done in order to change streams, or while restarting after a data underflow or overflow. When paused, FIFO operations can be done without corrupting data that is in the process of being sent or received. Once a data pause has been requested, the interface may need to complete sending data that was in progress before interrupting the flow of data. Software must check that the pause is actually in effect before taking action. This is done by monitoring the DATAPAUSED flag in the STAT register. When DATAPAUSE is cleared, data transfer will resume at the beginning of the next frame."]
pub struct DATAPAUSE_R(crate::FieldReader<bool, DATAPAUSE_A>);
impl DATAPAUSE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DATAPAUSE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DATAPAUSE_A {
        match self.bits {
            false => DATAPAUSE_A::NORMAL,
            true => DATAPAUSE_A::PAUSE,
        }
    }
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        **self == DATAPAUSE_A::NORMAL
    }
    #[doc = "Checks if the value of the field is `PAUSE`"]
    #[inline(always)]
    pub fn is_pause(&self) -> bool {
        **self == DATAPAUSE_A::PAUSE
    }
}
impl core::ops::Deref for DATAPAUSE_R {
    type Target = crate::FieldReader<bool, DATAPAUSE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DATAPAUSE` writer - Data flow Pause. Allows pausing data flow between the I2S serializer/deserializer and the FIFO. This could be done in order to change streams, or while restarting after a data underflow or overflow. When paused, FIFO operations can be done without corrupting data that is in the process of being sent or received. Once a data pause has been requested, the interface may need to complete sending data that was in progress before interrupting the flow of data. Software must check that the pause is actually in effect before taking action. This is done by monitoring the DATAPAUSED flag in the STAT register. When DATAPAUSE is cleared, data transfer will resume at the beginning of the next frame."]
pub struct DATAPAUSE_W<'a> {
    w: &'a mut W,
}
impl<'a> DATAPAUSE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DATAPAUSE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Normal operation, or resuming normal operation at the next frame if the I2S has already been paused."]
    #[inline(always)]
    pub fn normal(self) -> &'a mut W {
        self.variant(DATAPAUSE_A::NORMAL)
    }
    #[doc = "A pause in the data flow is being requested. It is in effect when DATAPAUSED in STAT = 1."]
    #[inline(always)]
    pub fn pause(self) -> &'a mut W {
        self.variant(DATAPAUSE_A::PAUSE)
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u32 & 0x01) << 1);
        self.w
    }
}
#[doc = "Provides the number of I2S channel pairs in this Flexcomm This is a read-only field whose value may be different in other Flexcomms. 00 = there is 1 I2S channel pair in this Flexcomm. 01 = there are 2 I2S channel pairs in this Flexcomm. 10 = there are 3 I2S channel pairs in this Flexcomm. 11 = there are 4 I2S channel pairs in this Flexcomm.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PAIRCOUNT_A {
    #[doc = "0: 1 I2S channel pairs in this flexcomm"]
    PAIRS_1 = 0,
    #[doc = "1: 2 I2S channel pairs in this flexcomm"]
    PAIRS_2 = 1,
    #[doc = "2: 3 I2S channel pairs in this flexcomm"]
    PAIRS_3 = 2,
    #[doc = "3: 4 I2S channel pairs in this flexcomm"]
    PAIRS_4 = 3,
}
impl From<PAIRCOUNT_A> for u8 {
    #[inline(always)]
    fn from(variant: PAIRCOUNT_A) -> Self {
        variant as _
    }
}
#[doc = "Field `PAIRCOUNT` reader - Provides the number of I2S channel pairs in this Flexcomm This is a read-only field whose value may be different in other Flexcomms. 00 = there is 1 I2S channel pair in this Flexcomm. 01 = there are 2 I2S channel pairs in this Flexcomm. 10 = there are 3 I2S channel pairs in this Flexcomm. 11 = there are 4 I2S channel pairs in this Flexcomm."]
pub struct PAIRCOUNT_R(crate::FieldReader<u8, PAIRCOUNT_A>);
impl PAIRCOUNT_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PAIRCOUNT_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PAIRCOUNT_A {
        match self.bits {
            0 => PAIRCOUNT_A::PAIRS_1,
            1 => PAIRCOUNT_A::PAIRS_2,
            2 => PAIRCOUNT_A::PAIRS_3,
            3 => PAIRCOUNT_A::PAIRS_4,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PAIRS_1`"]
    #[inline(always)]
    pub fn is_pairs_1(&self) -> bool {
        **self == PAIRCOUNT_A::PAIRS_1
    }
    #[doc = "Checks if the value of the field is `PAIRS_2`"]
    #[inline(always)]
    pub fn is_pairs_2(&self) -> bool {
        **self == PAIRCOUNT_A::PAIRS_2
    }
    #[doc = "Checks if the value of the field is `PAIRS_3`"]
    #[inline(always)]
    pub fn is_pairs_3(&self) -> bool {
        **self == PAIRCOUNT_A::PAIRS_3
    }
    #[doc = "Checks if the value of the field is `PAIRS_4`"]
    #[inline(always)]
    pub fn is_pairs_4(&self) -> bool {
        **self == PAIRCOUNT_A::PAIRS_4
    }
}
impl core::ops::Deref for PAIRCOUNT_R {
    type Target = crate::FieldReader<u8, PAIRCOUNT_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PAIRCOUNT` writer - Provides the number of I2S channel pairs in this Flexcomm This is a read-only field whose value may be different in other Flexcomms. 00 = there is 1 I2S channel pair in this Flexcomm. 01 = there are 2 I2S channel pairs in this Flexcomm. 10 = there are 3 I2S channel pairs in this Flexcomm. 11 = there are 4 I2S channel pairs in this Flexcomm."]
pub struct PAIRCOUNT_W<'a> {
    w: &'a mut W,
}
impl<'a> PAIRCOUNT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAIRCOUNT_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "1 I2S channel pairs in this flexcomm"]
    #[inline(always)]
    pub fn pairs_1(self) -> &'a mut W {
        self.variant(PAIRCOUNT_A::PAIRS_1)
    }
    #[doc = "2 I2S channel pairs in this flexcomm"]
    #[inline(always)]
    pub fn pairs_2(self) -> &'a mut W {
        self.variant(PAIRCOUNT_A::PAIRS_2)
    }
    #[doc = "3 I2S channel pairs in this flexcomm"]
    #[inline(always)]
    pub fn pairs_3(self) -> &'a mut W {
        self.variant(PAIRCOUNT_A::PAIRS_3)
    }
    #[doc = "4 I2S channel pairs in this flexcomm"]
    #[inline(always)]
    pub fn pairs_4(self) -> &'a mut W {
        self.variant(PAIRCOUNT_A::PAIRS_4)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | ((value as u32 & 0x03) << 2);
        self.w
    }
}
#[doc = "Master / slave configuration selection, determining how SCK and WS are used by all channel pairs in this Flexcomm.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum MSTSLVCFG_A {
    #[doc = "0: Normal slave mode, the default mode. SCK and WS are received from a master and used to transmit or receive data."]
    NORMAL_SLAVE_MODE = 0,
    #[doc = "1: WS synchronized master. WS is received from another master and used to synchronize the generation of SCK, when divided from the Flexcomm function clock."]
    WS_SYNC_MASTER = 1,
    #[doc = "2: Master using an existing SCK. SCK is received and used directly to generate WS, as well as transmitting or receiving data."]
    MASTER_USING_SCK = 2,
    #[doc = "3: Normal master mode. SCK and WS are generated so they can be sent to one or more slave devices."]
    NORMAL_MASTER = 3,
}
impl From<MSTSLVCFG_A> for u8 {
    #[inline(always)]
    fn from(variant: MSTSLVCFG_A) -> Self {
        variant as _
    }
}
#[doc = "Field `MSTSLVCFG` reader - Master / slave configuration selection, determining how SCK and WS are used by all channel pairs in this Flexcomm."]
pub struct MSTSLVCFG_R(crate::FieldReader<u8, MSTSLVCFG_A>);
impl MSTSLVCFG_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        MSTSLVCFG_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MSTSLVCFG_A {
        match self.bits {
            0 => MSTSLVCFG_A::NORMAL_SLAVE_MODE,
            1 => MSTSLVCFG_A::WS_SYNC_MASTER,
            2 => MSTSLVCFG_A::MASTER_USING_SCK,
            3 => MSTSLVCFG_A::NORMAL_MASTER,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NORMAL_SLAVE_MODE`"]
    #[inline(always)]
    pub fn is_normal_slave_mode(&self) -> bool {
        **self == MSTSLVCFG_A::NORMAL_SLAVE_MODE
    }
    #[doc = "Checks if the value of the field is `WS_SYNC_MASTER`"]
    #[inline(always)]
    pub fn is_ws_sync_master(&self) -> bool {
        **self == MSTSLVCFG_A::WS_SYNC_MASTER
    }
    #[doc = "Checks if the value of the field is `MASTER_USING_SCK`"]
    #[inline(always)]
    pub fn is_master_using_sck(&self) -> bool {
        **self == MSTSLVCFG_A::MASTER_USING_SCK
    }
    #[doc = "Checks if the value of the field is `NORMAL_MASTER`"]
    #[inline(always)]
    pub fn is_normal_master(&self) -> bool {
        **self == MSTSLVCFG_A::NORMAL_MASTER
    }
}
impl core::ops::Deref for MSTSLVCFG_R {
    type Target = crate::FieldReader<u8, MSTSLVCFG_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MSTSLVCFG` writer - Master / slave configuration selection, determining how SCK and WS are used by all channel pairs in this Flexcomm."]
pub struct MSTSLVCFG_W<'a> {
    w: &'a mut W,
}
impl<'a> MSTSLVCFG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MSTSLVCFG_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Normal slave mode, the default mode. SCK and WS are received from a master and used to transmit or receive data."]
    #[inline(always)]
    pub fn normal_slave_mode(self) -> &'a mut W {
        self.variant(MSTSLVCFG_A::NORMAL_SLAVE_MODE)
    }
    #[doc = "WS synchronized master. WS is received from another master and used to synchronize the generation of SCK, when divided from the Flexcomm function clock."]
    #[inline(always)]
    pub fn ws_sync_master(self) -> &'a mut W {
        self.variant(MSTSLVCFG_A::WS_SYNC_MASTER)
    }
    #[doc = "Master using an existing SCK. SCK is received and used directly to generate WS, as well as transmitting or receiving data."]
    #[inline(always)]
    pub fn master_using_sck(self) -> &'a mut W {
        self.variant(MSTSLVCFG_A::MASTER_USING_SCK)
    }
    #[doc = "Normal master mode. SCK and WS are generated so they can be sent to one or more slave devices."]
    #[inline(always)]
    pub fn normal_master(self) -> &'a mut W {
        self.variant(MSTSLVCFG_A::NORMAL_MASTER)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | ((value as u32 & 0x03) << 4);
        self.w
    }
}
#[doc = "Selects the basic I2S operating mode. Other configurations modify this to obtain all supported cases. See Formats and modes for examples.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum MODE_A {
    #[doc = "0: I2S mode a.k.a. 'classic' mode. WS has a 50% duty cycle, with (for each enabled channel pair) one piece of left channel data occurring during the first phase, and one pieces of right channel data occurring during the second phase. In this mode, the data region begins one clock after the leading WS edge for the frame. For a 50% WS duty cycle, FRAMELEN must define an even number of I2S clocks for the frame. If FRAMELEN defines an odd number of clocks per frame, the extra clock will occur on the right."]
    CLASSIC_MODE = 0,
    #[doc = "1: DSP mode where WS has a 50% duty cycle. See remark for mode 0."]
    DSP_MODE_WS_50_DUTYCYCLE = 1,
    #[doc = "2: DSP mode where WS has a one clock long pulse at the beginning of each data frame."]
    DSP_MODE_WS_1_CLOCK = 2,
    #[doc = "3: DSP mode where WS has a one data slot long pulse at the beginning of each data frame."]
    DSP_MODE_WS_1_DATA = 3,
}
impl From<MODE_A> for u8 {
    #[inline(always)]
    fn from(variant: MODE_A) -> Self {
        variant as _
    }
}
#[doc = "Field `MODE` reader - Selects the basic I2S operating mode. Other configurations modify this to obtain all supported cases. See Formats and modes for examples."]
pub struct MODE_R(crate::FieldReader<u8, MODE_A>);
impl MODE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        MODE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MODE_A {
        match self.bits {
            0 => MODE_A::CLASSIC_MODE,
            1 => MODE_A::DSP_MODE_WS_50_DUTYCYCLE,
            2 => MODE_A::DSP_MODE_WS_1_CLOCK,
            3 => MODE_A::DSP_MODE_WS_1_DATA,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `CLASSIC_MODE`"]
    #[inline(always)]
    pub fn is_classic_mode(&self) -> bool {
        **self == MODE_A::CLASSIC_MODE
    }
    #[doc = "Checks if the value of the field is `DSP_MODE_WS_50_DUTYCYCLE`"]
    #[inline(always)]
    pub fn is_dsp_mode_ws_50_dutycycle(&self) -> bool {
        **self == MODE_A::DSP_MODE_WS_50_DUTYCYCLE
    }
    #[doc = "Checks if the value of the field is `DSP_MODE_WS_1_CLOCK`"]
    #[inline(always)]
    pub fn is_dsp_mode_ws_1_clock(&self) -> bool {
        **self == MODE_A::DSP_MODE_WS_1_CLOCK
    }
    #[doc = "Checks if the value of the field is `DSP_MODE_WS_1_DATA`"]
    #[inline(always)]
    pub fn is_dsp_mode_ws_1_data(&self) -> bool {
        **self == MODE_A::DSP_MODE_WS_1_DATA
    }
}
impl core::ops::Deref for MODE_R {
    type Target = crate::FieldReader<u8, MODE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MODE` writer - Selects the basic I2S operating mode. Other configurations modify this to obtain all supported cases. See Formats and modes for examples."]
pub struct MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> MODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MODE_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "I2S mode a.k.a. 'classic' mode. WS has a 50% duty cycle, with (for each enabled channel pair) one piece of left channel data occurring during the first phase, and one pieces of right channel data occurring during the second phase. In this mode, the data region begins one clock after the leading WS edge for the frame. For a 50% WS duty cycle, FRAMELEN must define an even number of I2S clocks for the frame. If FRAMELEN defines an odd number of clocks per frame, the extra clock will occur on the right."]
    #[inline(always)]
    pub fn classic_mode(self) -> &'a mut W {
        self.variant(MODE_A::CLASSIC_MODE)
    }
    #[doc = "DSP mode where WS has a 50% duty cycle. See remark for mode 0."]
    #[inline(always)]
    pub fn dsp_mode_ws_50_dutycycle(self) -> &'a mut W {
        self.variant(MODE_A::DSP_MODE_WS_50_DUTYCYCLE)
    }
    #[doc = "DSP mode where WS has a one clock long pulse at the beginning of each data frame."]
    #[inline(always)]
    pub fn dsp_mode_ws_1_clock(self) -> &'a mut W {
        self.variant(MODE_A::DSP_MODE_WS_1_CLOCK)
    }
    #[doc = "DSP mode where WS has a one data slot long pulse at the beginning of each data frame."]
    #[inline(always)]
    pub fn dsp_mode_ws_1_data(self) -> &'a mut W {
        self.variant(MODE_A::DSP_MODE_WS_1_DATA)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | ((value as u32 & 0x03) << 6);
        self.w
    }
}
#[doc = "Right channel data is in the Low portion of FIFO data. Essentially, this swaps left and right channel data as it is transferred to or from the FIFO. This bit is not used if the data width is greater than 24 bits or if PDMDATA = 1. Note that if the ONECHANNEL field (bit 10 of this register) = 1, the one channel to be used is the nominally the left channel. POSITION can still place that data in the frame where right channel data is normally located. if all enabled channel pairs have ONECHANNEL = 1, then RIGHTLOW = 1 is not allowed.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RIGHTLOW_A {
    #[doc = "0: The right channel is taken from the high part of the FIFO data. For example, when data is 16 bits, FIFO bits 31:16 are used for the right channel."]
    RIGHT_HIGH = 0,
    #[doc = "1: The right channel is taken from the low part of the FIFO data. For example, when data is 16 bits, FIFO bits 15:0 are used for the right channel."]
    RIGHT_LOW = 1,
}
impl From<RIGHTLOW_A> for bool {
    #[inline(always)]
    fn from(variant: RIGHTLOW_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RIGHTLOW` reader - Right channel data is in the Low portion of FIFO data. Essentially, this swaps left and right channel data as it is transferred to or from the FIFO. This bit is not used if the data width is greater than 24 bits or if PDMDATA = 1. Note that if the ONECHANNEL field (bit 10 of this register) = 1, the one channel to be used is the nominally the left channel. POSITION can still place that data in the frame where right channel data is normally located. if all enabled channel pairs have ONECHANNEL = 1, then RIGHTLOW = 1 is not allowed."]
pub struct RIGHTLOW_R(crate::FieldReader<bool, RIGHTLOW_A>);
impl RIGHTLOW_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RIGHTLOW_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RIGHTLOW_A {
        match self.bits {
            false => RIGHTLOW_A::RIGHT_HIGH,
            true => RIGHTLOW_A::RIGHT_LOW,
        }
    }
    #[doc = "Checks if the value of the field is `RIGHT_HIGH`"]
    #[inline(always)]
    pub fn is_right_high(&self) -> bool {
        **self == RIGHTLOW_A::RIGHT_HIGH
    }
    #[doc = "Checks if the value of the field is `RIGHT_LOW`"]
    #[inline(always)]
    pub fn is_right_low(&self) -> bool {
        **self == RIGHTLOW_A::RIGHT_LOW
    }
}
impl core::ops::Deref for RIGHTLOW_R {
    type Target = crate::FieldReader<bool, RIGHTLOW_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RIGHTLOW` writer - Right channel data is in the Low portion of FIFO data. Essentially, this swaps left and right channel data as it is transferred to or from the FIFO. This bit is not used if the data width is greater than 24 bits or if PDMDATA = 1. Note that if the ONECHANNEL field (bit 10 of this register) = 1, the one channel to be used is the nominally the left channel. POSITION can still place that data in the frame where right channel data is normally located. if all enabled channel pairs have ONECHANNEL = 1, then RIGHTLOW = 1 is not allowed."]
pub struct RIGHTLOW_W<'a> {
    w: &'a mut W,
}
impl<'a> RIGHTLOW_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RIGHTLOW_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The right channel is taken from the high part of the FIFO data. For example, when data is 16 bits, FIFO bits 31:16 are used for the right channel."]
    #[inline(always)]
    pub fn right_high(self) -> &'a mut W {
        self.variant(RIGHTLOW_A::RIGHT_HIGH)
    }
    #[doc = "The right channel is taken from the low part of the FIFO data. For example, when data is 16 bits, FIFO bits 15:0 are used for the right channel."]
    #[inline(always)]
    pub fn right_low(self) -> &'a mut W {
        self.variant(RIGHTLOW_A::RIGHT_LOW)
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | ((value as u32 & 0x01) << 8);
        self.w
    }
}
#[doc = "Left Justify data.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LEFTJUST_A {
    #[doc = "0: Data is transferred between the FIFO and the I2S serializer/deserializer right justified, i.e. starting from bit 0 and continuing to the position defined by DATALEN. This would correspond to right justified data in the stream on the data bus."]
    RIGHT_JUSTIFIED = 0,
    #[doc = "1: Data is transferred between the FIFO and the I2S serializer/deserializer left justified, i.e. starting from the MSB of the FIFO entry and continuing for the number of bits defined by DATALEN. This would correspond to left justified data in the stream on the data bus."]
    LEFT_JUSTIFIED = 1,
}
impl From<LEFTJUST_A> for bool {
    #[inline(always)]
    fn from(variant: LEFTJUST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LEFTJUST` reader - Left Justify data."]
pub struct LEFTJUST_R(crate::FieldReader<bool, LEFTJUST_A>);
impl LEFTJUST_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        LEFTJUST_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LEFTJUST_A {
        match self.bits {
            false => LEFTJUST_A::RIGHT_JUSTIFIED,
            true => LEFTJUST_A::LEFT_JUSTIFIED,
        }
    }
    #[doc = "Checks if the value of the field is `RIGHT_JUSTIFIED`"]
    #[inline(always)]
    pub fn is_right_justified(&self) -> bool {
        **self == LEFTJUST_A::RIGHT_JUSTIFIED
    }
    #[doc = "Checks if the value of the field is `LEFT_JUSTIFIED`"]
    #[inline(always)]
    pub fn is_left_justified(&self) -> bool {
        **self == LEFTJUST_A::LEFT_JUSTIFIED
    }
}
impl core::ops::Deref for LEFTJUST_R {
    type Target = crate::FieldReader<bool, LEFTJUST_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LEFTJUST` writer - Left Justify data."]
pub struct LEFTJUST_W<'a> {
    w: &'a mut W,
}
impl<'a> LEFTJUST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LEFTJUST_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Data is transferred between the FIFO and the I2S serializer/deserializer right justified, i.e. starting from bit 0 and continuing to the position defined by DATALEN. This would correspond to right justified data in the stream on the data bus."]
    #[inline(always)]
    pub fn right_justified(self) -> &'a mut W {
        self.variant(LEFTJUST_A::RIGHT_JUSTIFIED)
    }
    #[doc = "Data is transferred between the FIFO and the I2S serializer/deserializer left justified, i.e. starting from the MSB of the FIFO entry and continuing for the number of bits defined by DATALEN. This would correspond to left justified data in the stream on the data bus."]
    #[inline(always)]
    pub fn left_justified(self) -> &'a mut W {
        self.variant(LEFTJUST_A::LEFT_JUSTIFIED)
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | ((value as u32 & 0x01) << 9);
        self.w
    }
}
#[doc = "Single channel mode. Applies to both transmit and receive. This configuration bit applies only to the first I2S channel pair. Other channel pairs may select this mode independently in their separate CFG1 registers.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ONECHANNEL_A {
    #[doc = "0: I2S data for this channel pair is treated as left and right channels."]
    DUAL_CHANNEL = 0,
    #[doc = "1: I2S data for this channel pair is treated as a single channel, functionally the left channel for this pair. In mode 0 only, the right side of the frame begins at POSITION = 0x100. This is because mode 0 makes a clear distinction between the left and right sides of the frame. When ONECHANNEL = 1, the single channel of data may be placed on the right by setting POSITION to 0x100 + the data position within the right side (e.g. 0x108 would place data starting at the 8th clock after the middle of the frame). In other modes, data for the single channel of data is placed at the clock defined by POSITION."]
    SINGLE_CHANNEL = 1,
}
impl From<ONECHANNEL_A> for bool {
    #[inline(always)]
    fn from(variant: ONECHANNEL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ONECHANNEL` reader - Single channel mode. Applies to both transmit and receive. This configuration bit applies only to the first I2S channel pair. Other channel pairs may select this mode independently in their separate CFG1 registers."]
pub struct ONECHANNEL_R(crate::FieldReader<bool, ONECHANNEL_A>);
impl ONECHANNEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ONECHANNEL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ONECHANNEL_A {
        match self.bits {
            false => ONECHANNEL_A::DUAL_CHANNEL,
            true => ONECHANNEL_A::SINGLE_CHANNEL,
        }
    }
    #[doc = "Checks if the value of the field is `DUAL_CHANNEL`"]
    #[inline(always)]
    pub fn is_dual_channel(&self) -> bool {
        **self == ONECHANNEL_A::DUAL_CHANNEL
    }
    #[doc = "Checks if the value of the field is `SINGLE_CHANNEL`"]
    #[inline(always)]
    pub fn is_single_channel(&self) -> bool {
        **self == ONECHANNEL_A::SINGLE_CHANNEL
    }
}
impl core::ops::Deref for ONECHANNEL_R {
    type Target = crate::FieldReader<bool, ONECHANNEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ONECHANNEL` writer - Single channel mode. Applies to both transmit and receive. This configuration bit applies only to the first I2S channel pair. Other channel pairs may select this mode independently in their separate CFG1 registers."]
pub struct ONECHANNEL_W<'a> {
    w: &'a mut W,
}
impl<'a> ONECHANNEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ONECHANNEL_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "I2S data for this channel pair is treated as left and right channels."]
    #[inline(always)]
    pub fn dual_channel(self) -> &'a mut W {
        self.variant(ONECHANNEL_A::DUAL_CHANNEL)
    }
    #[doc = "I2S data for this channel pair is treated as a single channel, functionally the left channel for this pair. In mode 0 only, the right side of the frame begins at POSITION = 0x100. This is because mode 0 makes a clear distinction between the left and right sides of the frame. When ONECHANNEL = 1, the single channel of data may be placed on the right by setting POSITION to 0x100 + the data position within the right side (e.g. 0x108 would place data starting at the 8th clock after the middle of the frame). In other modes, data for the single channel of data is placed at the clock defined by POSITION."]
    #[inline(always)]
    pub fn single_channel(self) -> &'a mut W {
        self.variant(ONECHANNEL_A::SINGLE_CHANNEL)
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | ((value as u32 & 0x01) << 10);
        self.w
    }
}
#[doc = "PDM Data selection. This bit controls the data source for I2S transmit, and cannot be set in Rx mode. This bit only has an effect if the device the Flexcomm resides in includes a D-Mic subsystem. For the LPC5411x, this bit applies only to Flexcomm 7.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PDMDATA_A {
    #[doc = "0: Normal operation, data is transferred to or from the Flexcomm FIFO."]
    NORMAL = 0,
    #[doc = "1: The data source is the D-Mic subsystem. When PDMDATA = 1, only the primary channel pair can be used in this Flexcomm. If ONECHANNEL = 1, only the PDM left data is used. the WS rate must match the Fs (sample rate) of the D-Mic decimator. A rate mismatch will at some point cause the I2S to overrun or underrun."]
    DMIC_SUBSYSTEM = 1,
}
impl From<PDMDATA_A> for bool {
    #[inline(always)]
    fn from(variant: PDMDATA_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PDMDATA` reader - PDM Data selection. This bit controls the data source for I2S transmit, and cannot be set in Rx mode. This bit only has an effect if the device the Flexcomm resides in includes a D-Mic subsystem. For the LPC5411x, this bit applies only to Flexcomm 7."]
pub struct PDMDATA_R(crate::FieldReader<bool, PDMDATA_A>);
impl PDMDATA_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PDMDATA_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PDMDATA_A {
        match self.bits {
            false => PDMDATA_A::NORMAL,
            true => PDMDATA_A::DMIC_SUBSYSTEM,
        }
    }
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        **self == PDMDATA_A::NORMAL
    }
    #[doc = "Checks if the value of the field is `DMIC_SUBSYSTEM`"]
    #[inline(always)]
    pub fn is_dmic_subsystem(&self) -> bool {
        **self == PDMDATA_A::DMIC_SUBSYSTEM
    }
}
impl core::ops::Deref for PDMDATA_R {
    type Target = crate::FieldReader<bool, PDMDATA_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PDMDATA` writer - PDM Data selection. This bit controls the data source for I2S transmit, and cannot be set in Rx mode. This bit only has an effect if the device the Flexcomm resides in includes a D-Mic subsystem. For the LPC5411x, this bit applies only to Flexcomm 7."]
pub struct PDMDATA_W<'a> {
    w: &'a mut W,
}
impl<'a> PDMDATA_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PDMDATA_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Normal operation, data is transferred to or from the Flexcomm FIFO."]
    #[inline(always)]
    pub fn normal(self) -> &'a mut W {
        self.variant(PDMDATA_A::NORMAL)
    }
    #[doc = "The data source is the D-Mic subsystem. When PDMDATA = 1, only the primary channel pair can be used in this Flexcomm. If ONECHANNEL = 1, only the PDM left data is used. the WS rate must match the Fs (sample rate) of the D-Mic decimator. A rate mismatch will at some point cause the I2S to overrun or underrun."]
    #[inline(always)]
    pub fn dmic_subsystem(self) -> &'a mut W {
        self.variant(PDMDATA_A::DMIC_SUBSYSTEM)
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
#[doc = "SCK polarity.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SCK_POL_A {
    #[doc = "0: Data is launched on SCK falling edges and sampled on SCK rising edges (standard for I2S)."]
    FALLING_EDGE = 0,
    #[doc = "1: Data is launched on SCK rising edges and sampled on SCK falling edges."]
    RISING_EDGE = 1,
}
impl From<SCK_POL_A> for bool {
    #[inline(always)]
    fn from(variant: SCK_POL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SCK_POL` reader - SCK polarity."]
pub struct SCK_POL_R(crate::FieldReader<bool, SCK_POL_A>);
impl SCK_POL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SCK_POL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SCK_POL_A {
        match self.bits {
            false => SCK_POL_A::FALLING_EDGE,
            true => SCK_POL_A::RISING_EDGE,
        }
    }
    #[doc = "Checks if the value of the field is `FALLING_EDGE`"]
    #[inline(always)]
    pub fn is_falling_edge(&self) -> bool {
        **self == SCK_POL_A::FALLING_EDGE
    }
    #[doc = "Checks if the value of the field is `RISING_EDGE`"]
    #[inline(always)]
    pub fn is_rising_edge(&self) -> bool {
        **self == SCK_POL_A::RISING_EDGE
    }
}
impl core::ops::Deref for SCK_POL_R {
    type Target = crate::FieldReader<bool, SCK_POL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SCK_POL` writer - SCK polarity."]
pub struct SCK_POL_W<'a> {
    w: &'a mut W,
}
impl<'a> SCK_POL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SCK_POL_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Data is launched on SCK falling edges and sampled on SCK rising edges (standard for I2S)."]
    #[inline(always)]
    pub fn falling_edge(self) -> &'a mut W {
        self.variant(SCK_POL_A::FALLING_EDGE)
    }
    #[doc = "Data is launched on SCK rising edges and sampled on SCK falling edges."]
    #[inline(always)]
    pub fn rising_edge(self) -> &'a mut W {
        self.variant(SCK_POL_A::RISING_EDGE)
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | ((value as u32 & 0x01) << 12);
        self.w
    }
}
#[doc = "WS polarity.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WS_POL_A {
    #[doc = "0: Data frames begin at a falling edge of WS (standard for classic I2S)."]
    NOT_INVERTED = 0,
    #[doc = "1: WS is inverted, resulting in a data frame beginning at a rising edge of WS (standard for most 'non-classic' variations of I2S)."]
    INVERTED = 1,
}
impl From<WS_POL_A> for bool {
    #[inline(always)]
    fn from(variant: WS_POL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WS_POL` reader - WS polarity."]
pub struct WS_POL_R(crate::FieldReader<bool, WS_POL_A>);
impl WS_POL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        WS_POL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WS_POL_A {
        match self.bits {
            false => WS_POL_A::NOT_INVERTED,
            true => WS_POL_A::INVERTED,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_INVERTED`"]
    #[inline(always)]
    pub fn is_not_inverted(&self) -> bool {
        **self == WS_POL_A::NOT_INVERTED
    }
    #[doc = "Checks if the value of the field is `INVERTED`"]
    #[inline(always)]
    pub fn is_inverted(&self) -> bool {
        **self == WS_POL_A::INVERTED
    }
}
impl core::ops::Deref for WS_POL_R {
    type Target = crate::FieldReader<bool, WS_POL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WS_POL` writer - WS polarity."]
pub struct WS_POL_W<'a> {
    w: &'a mut W,
}
impl<'a> WS_POL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WS_POL_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Data frames begin at a falling edge of WS (standard for classic I2S)."]
    #[inline(always)]
    pub fn not_inverted(self) -> &'a mut W {
        self.variant(WS_POL_A::NOT_INVERTED)
    }
    #[doc = "WS is inverted, resulting in a data frame beginning at a rising edge of WS (standard for most 'non-classic' variations of I2S)."]
    #[inline(always)]
    pub fn inverted(self) -> &'a mut W {
        self.variant(WS_POL_A::INVERTED)
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | ((value as u32 & 0x01) << 13);
        self.w
    }
}
#[doc = "Field `DATALEN` reader - Data Length, minus 1 encoded, defines the number of data bits to be transmitted or received for all I2S channel pairs in this Flexcomm. Note that data is only driven to or received from SDA for the number of bits defined by DATALEN. DATALEN is also used in these ways by the I2S: Determines the size of data transfers between the FIFO and the I2S serializer/deserializer. See FIFO buffer configurations and usage In mode 1, 2, and 3, determines the location of right data following left data in the frame. In mode 3 (where WS has a one data slot long pulse at the beginning of each data frame) determines the duration of the WS pulse. Values: 0x00 to 0x02 = not supported 0x03 = data is 4 bits in length 0x04 = data is 5 bits in length 0x1F = data is 32 bits in length"]
pub struct DATALEN_R(crate::FieldReader<u8, u8>);
impl DATALEN_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        DATALEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DATALEN_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DATALEN` writer - Data Length, minus 1 encoded, defines the number of data bits to be transmitted or received for all I2S channel pairs in this Flexcomm. Note that data is only driven to or received from SDA for the number of bits defined by DATALEN. DATALEN is also used in these ways by the I2S: Determines the size of data transfers between the FIFO and the I2S serializer/deserializer. See FIFO buffer configurations and usage In mode 1, 2, and 3, determines the location of right data following left data in the frame. In mode 3 (where WS has a one data slot long pulse at the beginning of each data frame) determines the duration of the WS pulse. Values: 0x00 to 0x02 = not supported 0x03 = data is 4 bits in length 0x04 = data is 5 bits in length 0x1F = data is 32 bits in length"]
pub struct DATALEN_W<'a> {
    w: &'a mut W,
}
impl<'a> DATALEN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 16)) | ((value as u32 & 0x1f) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Main enable for I 2S function in this Flexcomm"]
    #[inline(always)]
    pub fn mainenable(&self) -> MAINENABLE_R {
        MAINENABLE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Data flow Pause. Allows pausing data flow between the I2S serializer/deserializer and the FIFO. This could be done in order to change streams, or while restarting after a data underflow or overflow. When paused, FIFO operations can be done without corrupting data that is in the process of being sent or received. Once a data pause has been requested, the interface may need to complete sending data that was in progress before interrupting the flow of data. Software must check that the pause is actually in effect before taking action. This is done by monitoring the DATAPAUSED flag in the STAT register. When DATAPAUSE is cleared, data transfer will resume at the beginning of the next frame."]
    #[inline(always)]
    pub fn datapause(&self) -> DATAPAUSE_R {
        DATAPAUSE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bits 2:3 - Provides the number of I2S channel pairs in this Flexcomm This is a read-only field whose value may be different in other Flexcomms. 00 = there is 1 I2S channel pair in this Flexcomm. 01 = there are 2 I2S channel pairs in this Flexcomm. 10 = there are 3 I2S channel pairs in this Flexcomm. 11 = there are 4 I2S channel pairs in this Flexcomm."]
    #[inline(always)]
    pub fn paircount(&self) -> PAIRCOUNT_R {
        PAIRCOUNT_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bits 4:5 - Master / slave configuration selection, determining how SCK and WS are used by all channel pairs in this Flexcomm."]
    #[inline(always)]
    pub fn mstslvcfg(&self) -> MSTSLVCFG_R {
        MSTSLVCFG_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bits 6:7 - Selects the basic I2S operating mode. Other configurations modify this to obtain all supported cases. See Formats and modes for examples."]
    #[inline(always)]
    pub fn mode(&self) -> MODE_R {
        MODE_R::new(((self.bits >> 6) & 0x03) as u8)
    }
    #[doc = "Bit 8 - Right channel data is in the Low portion of FIFO data. Essentially, this swaps left and right channel data as it is transferred to or from the FIFO. This bit is not used if the data width is greater than 24 bits or if PDMDATA = 1. Note that if the ONECHANNEL field (bit 10 of this register) = 1, the one channel to be used is the nominally the left channel. POSITION can still place that data in the frame where right channel data is normally located. if all enabled channel pairs have ONECHANNEL = 1, then RIGHTLOW = 1 is not allowed."]
    #[inline(always)]
    pub fn rightlow(&self) -> RIGHTLOW_R {
        RIGHTLOW_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Left Justify data."]
    #[inline(always)]
    pub fn leftjust(&self) -> LEFTJUST_R {
        LEFTJUST_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Single channel mode. Applies to both transmit and receive. This configuration bit applies only to the first I2S channel pair. Other channel pairs may select this mode independently in their separate CFG1 registers."]
    #[inline(always)]
    pub fn onechannel(&self) -> ONECHANNEL_R {
        ONECHANNEL_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - PDM Data selection. This bit controls the data source for I2S transmit, and cannot be set in Rx mode. This bit only has an effect if the device the Flexcomm resides in includes a D-Mic subsystem. For the LPC5411x, this bit applies only to Flexcomm 7."]
    #[inline(always)]
    pub fn pdmdata(&self) -> PDMDATA_R {
        PDMDATA_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - SCK polarity."]
    #[inline(always)]
    pub fn sck_pol(&self) -> SCK_POL_R {
        SCK_POL_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - WS polarity."]
    #[inline(always)]
    pub fn ws_pol(&self) -> WS_POL_R {
        WS_POL_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bits 16:20 - Data Length, minus 1 encoded, defines the number of data bits to be transmitted or received for all I2S channel pairs in this Flexcomm. Note that data is only driven to or received from SDA for the number of bits defined by DATALEN. DATALEN is also used in these ways by the I2S: Determines the size of data transfers between the FIFO and the I2S serializer/deserializer. See FIFO buffer configurations and usage In mode 1, 2, and 3, determines the location of right data following left data in the frame. In mode 3 (where WS has a one data slot long pulse at the beginning of each data frame) determines the duration of the WS pulse. Values: 0x00 to 0x02 = not supported 0x03 = data is 4 bits in length 0x04 = data is 5 bits in length 0x1F = data is 32 bits in length"]
    #[inline(always)]
    pub fn datalen(&self) -> DATALEN_R {
        DATALEN_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Main enable for I 2S function in this Flexcomm"]
    #[inline(always)]
    pub fn mainenable(&mut self) -> MAINENABLE_W {
        MAINENABLE_W { w: self }
    }
    #[doc = "Bit 1 - Data flow Pause. Allows pausing data flow between the I2S serializer/deserializer and the FIFO. This could be done in order to change streams, or while restarting after a data underflow or overflow. When paused, FIFO operations can be done without corrupting data that is in the process of being sent or received. Once a data pause has been requested, the interface may need to complete sending data that was in progress before interrupting the flow of data. Software must check that the pause is actually in effect before taking action. This is done by monitoring the DATAPAUSED flag in the STAT register. When DATAPAUSE is cleared, data transfer will resume at the beginning of the next frame."]
    #[inline(always)]
    pub fn datapause(&mut self) -> DATAPAUSE_W {
        DATAPAUSE_W { w: self }
    }
    #[doc = "Bits 2:3 - Provides the number of I2S channel pairs in this Flexcomm This is a read-only field whose value may be different in other Flexcomms. 00 = there is 1 I2S channel pair in this Flexcomm. 01 = there are 2 I2S channel pairs in this Flexcomm. 10 = there are 3 I2S channel pairs in this Flexcomm. 11 = there are 4 I2S channel pairs in this Flexcomm."]
    #[inline(always)]
    pub fn paircount(&mut self) -> PAIRCOUNT_W {
        PAIRCOUNT_W { w: self }
    }
    #[doc = "Bits 4:5 - Master / slave configuration selection, determining how SCK and WS are used by all channel pairs in this Flexcomm."]
    #[inline(always)]
    pub fn mstslvcfg(&mut self) -> MSTSLVCFG_W {
        MSTSLVCFG_W { w: self }
    }
    #[doc = "Bits 6:7 - Selects the basic I2S operating mode. Other configurations modify this to obtain all supported cases. See Formats and modes for examples."]
    #[inline(always)]
    pub fn mode(&mut self) -> MODE_W {
        MODE_W { w: self }
    }
    #[doc = "Bit 8 - Right channel data is in the Low portion of FIFO data. Essentially, this swaps left and right channel data as it is transferred to or from the FIFO. This bit is not used if the data width is greater than 24 bits or if PDMDATA = 1. Note that if the ONECHANNEL field (bit 10 of this register) = 1, the one channel to be used is the nominally the left channel. POSITION can still place that data in the frame where right channel data is normally located. if all enabled channel pairs have ONECHANNEL = 1, then RIGHTLOW = 1 is not allowed."]
    #[inline(always)]
    pub fn rightlow(&mut self) -> RIGHTLOW_W {
        RIGHTLOW_W { w: self }
    }
    #[doc = "Bit 9 - Left Justify data."]
    #[inline(always)]
    pub fn leftjust(&mut self) -> LEFTJUST_W {
        LEFTJUST_W { w: self }
    }
    #[doc = "Bit 10 - Single channel mode. Applies to both transmit and receive. This configuration bit applies only to the first I2S channel pair. Other channel pairs may select this mode independently in their separate CFG1 registers."]
    #[inline(always)]
    pub fn onechannel(&mut self) -> ONECHANNEL_W {
        ONECHANNEL_W { w: self }
    }
    #[doc = "Bit 11 - PDM Data selection. This bit controls the data source for I2S transmit, and cannot be set in Rx mode. This bit only has an effect if the device the Flexcomm resides in includes a D-Mic subsystem. For the LPC5411x, this bit applies only to Flexcomm 7."]
    #[inline(always)]
    pub fn pdmdata(&mut self) -> PDMDATA_W {
        PDMDATA_W { w: self }
    }
    #[doc = "Bit 12 - SCK polarity."]
    #[inline(always)]
    pub fn sck_pol(&mut self) -> SCK_POL_W {
        SCK_POL_W { w: self }
    }
    #[doc = "Bit 13 - WS polarity."]
    #[inline(always)]
    pub fn ws_pol(&mut self) -> WS_POL_W {
        WS_POL_W { w: self }
    }
    #[doc = "Bits 16:20 - Data Length, minus 1 encoded, defines the number of data bits to be transmitted or received for all I2S channel pairs in this Flexcomm. Note that data is only driven to or received from SDA for the number of bits defined by DATALEN. DATALEN is also used in these ways by the I2S: Determines the size of data transfers between the FIFO and the I2S serializer/deserializer. See FIFO buffer configurations and usage In mode 1, 2, and 3, determines the location of right data following left data in the frame. In mode 3 (where WS has a one data slot long pulse at the beginning of each data frame) determines the duration of the WS pulse. Values: 0x00 to 0x02 = not supported 0x03 = data is 4 bits in length 0x04 = data is 5 bits in length 0x1F = data is 32 bits in length"]
    #[inline(always)]
    pub fn datalen(&mut self) -> DATALEN_W {
        DATALEN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Configuration register 1 for the primary channel pair.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cfg1](index.html) module"]
pub struct CFG1_SPEC;
impl crate::RegisterSpec for CFG1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cfg1::R](R) reader structure"]
impl crate::Readable for CFG1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cfg1::W](W) writer structure"]
impl crate::Writable for CFG1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CFG1 to value 0"]
impl crate::Resettable for CFG1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
