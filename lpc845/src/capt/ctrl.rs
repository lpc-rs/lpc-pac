#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CTRL {
    #[doc = r" Modifies the contents of the register"]
    #[inline]
    pub fn modify<F>(&self, f: F)
    where
        for<'w> F: FnOnce(&R, &'w mut W) -> &'w mut W,
    {
        let bits = self.register.get();
        let r = R { bits: bits };
        let mut w = W { bits: bits };
        f(&r, &mut w);
        self.register.set(w.bits);
    }
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
    #[doc = r" Writes to the register"]
    #[inline]
    pub fn write<F>(&self, f: F)
    where
        F: FnOnce(&mut W) -> &mut W,
    {
        let mut w = W::reset_value();
        f(&mut w);
        self.register.set(w.bits);
    }
    #[doc = r" Writes the reset value to the register"]
    #[inline]
    pub fn reset(&self) {
        self.write(|w| w)
    }
}
#[doc = "Possible values of the field `POLLMODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum POLLMODER {
    #[doc = "None, inactive. Poll and time counters are turned off. Writing this will reset state and stop any collection in progress. Note: this has no effect on STATUS - those must be cleared manually."]
    NONE,
    #[doc = "Poll now - forces a manual poll to be started immediately, using XPINSEL X pin(s) to activate in the integration loop (all pins set together). Self clears - clear is not indication it is done (see STATUS)."]
    POLL_NOW,
    #[doc = "Normal polling using poll delay from POLL_TCNT register. This will start with the poll delay (which can be 0)."]
    NORMAL,
    #[doc = "The CAPT block will operate in low-power mode. This means it will use GPIO as input, use combination touch measurements, and assume it is to wake the system. This will use the POLL_TCNT poll delay, and start with the delay."]
    LOW_POWER_MODE,
}
impl POLLMODER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            POLLMODER::NONE => 0,
            POLLMODER::POLL_NOW => 1,
            POLLMODER::NORMAL => 2,
            POLLMODER::LOW_POWER_MODE => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> POLLMODER {
        match value {
            0 => POLLMODER::NONE,
            1 => POLLMODER::POLL_NOW,
            2 => POLLMODER::NORMAL,
            3 => POLLMODER::LOW_POWER_MODE,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline]
    pub fn is_none(&self) -> bool {
        *self == POLLMODER::NONE
    }
    #[doc = "Checks if the value of the field is `POLL_NOW`"]
    #[inline]
    pub fn is_poll_now(&self) -> bool {
        *self == POLLMODER::POLL_NOW
    }
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline]
    pub fn is_normal(&self) -> bool {
        *self == POLLMODER::NORMAL
    }
    #[doc = "Checks if the value of the field is `LOW_POWER_MODE`"]
    #[inline]
    pub fn is_low_power_mode(&self) -> bool {
        *self == POLLMODER::LOW_POWER_MODE
    }
}
#[doc = "Possible values of the field `TYPE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TYPER {
    #[doc = "Normal - all X elements are treated as normal, such as buttons and sliders."]
    TYPE_0,
    #[doc = "3x3 grid using NXP Complementary measurements. The 1st 9 Xs are assumed to be the 3x3 grid. After that would be normal X elements. This will also allow 3x1 and 3x2 Note: Only possible if XMAX in STATUS is >=8"]
    TYPE_1,
    #[doc = "5 Sensors interleaved to act as 3x3 touch area using NXP Complementary measurements. 1st 5 Xs used for this, all remaining are treated as normal. Note that if 16 X pins allowed, the 16th will not be usable when TYPE=1. (use TYPE=0 and select 1 smaller than 15 ( and any others from 1 smaller than 5 on up in XPINSEL)."]
    TYPE_2,
    #[doc = "9 Sensors interleaved to act as 5x5 touch area using NXP Complementary measurements. 1st 9 Xs used for this, all remaining are treated as normal. Note: Only possible if XMAX in STATUS is >=8"]
    TYPE_3,
}
impl TYPER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            TYPER::TYPE_0 => 0,
            TYPER::TYPE_1 => 1,
            TYPER::TYPE_2 => 2,
            TYPER::TYPE_3 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> TYPER {
        match value {
            0 => TYPER::TYPE_0,
            1 => TYPER::TYPE_1,
            2 => TYPER::TYPE_2,
            3 => TYPER::TYPE_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `TYPE_0`"]
    #[inline]
    pub fn is_type_0(&self) -> bool {
        *self == TYPER::TYPE_0
    }
    #[doc = "Checks if the value of the field is `TYPE_1`"]
    #[inline]
    pub fn is_type_1(&self) -> bool {
        *self == TYPER::TYPE_1
    }
    #[doc = "Checks if the value of the field is `TYPE_2`"]
    #[inline]
    pub fn is_type_2(&self) -> bool {
        *self == TYPER::TYPE_2
    }
    #[doc = "Checks if the value of the field is `TYPE_3`"]
    #[inline]
    pub fn is_type_3(&self) -> bool {
        *self == TYPER::TYPE_3
    }
}
#[doc = "Possible values of the field `TRIGGER`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TRIGGERR {
    #[doc = "Uses YH GPIO. This is not normally used except in Low-power mode. But, it can be used with POLLNOW to baseline that measurement."]
    USES_YH_GPIO,
    #[doc = "ACMP (if fitted). This assumes the ACMP state is fed in asynchronously and it will sample."]
    ACMP,
}
impl TRIGGERR {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            TRIGGERR::USES_YH_GPIO => false,
            TRIGGERR::ACMP => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TRIGGERR {
        match value {
            false => TRIGGERR::USES_YH_GPIO,
            true => TRIGGERR::ACMP,
        }
    }
    #[doc = "Checks if the value of the field is `USES_YH_GPIO`"]
    #[inline]
    pub fn is_uses_yh_gpio(&self) -> bool {
        *self == TRIGGERR::USES_YH_GPIO
    }
    #[doc = "Checks if the value of the field is `ACMP`"]
    #[inline]
    pub fn is_acmp(&self) -> bool {
        *self == TRIGGERR::ACMP
    }
}
#[doc = r" Value of the field"]
pub struct WAITR {
    bits: bool,
}
impl WAITR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
#[doc = "Possible values of the field `DMA`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DMAR {
    #[doc = "No DMA. Application will use ISRs to read out data"]
    DMA_0,
    #[doc = "Trigger DMA on Touch events"]
    DMA_1,
    #[doc = "Trigger DMA on both Touch and No-Touch events"]
    DMA_2,
    #[doc = "Trigger DMA on both plus Timeout."]
    DMA_3,
}
impl DMAR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            DMAR::DMA_0 => 0,
            DMAR::DMA_1 => 1,
            DMAR::DMA_2 => 2,
            DMAR::DMA_3 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> DMAR {
        match value {
            0 => DMAR::DMA_0,
            1 => DMAR::DMA_1,
            2 => DMAR::DMA_2,
            3 => DMAR::DMA_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DMA_0`"]
    #[inline]
    pub fn is_dma_0(&self) -> bool {
        *self == DMAR::DMA_0
    }
    #[doc = "Checks if the value of the field is `DMA_1`"]
    #[inline]
    pub fn is_dma_1(&self) -> bool {
        *self == DMAR::DMA_1
    }
    #[doc = "Checks if the value of the field is `DMA_2`"]
    #[inline]
    pub fn is_dma_2(&self) -> bool {
        *self == DMAR::DMA_2
    }
    #[doc = "Checks if the value of the field is `DMA_3`"]
    #[inline]
    pub fn is_dma_3(&self) -> bool {
        *self == DMAR::DMA_3
    }
}
#[doc = "Possible values of the field `FDIV`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FDIVR {
    #[doc = "No divide"]
    FDIV_0,
    #[doc = "/2"]
    FDIV_1,
    #[doc = "/3"]
    FDIV_2,
    #[doc = "/4"]
    FDIV_3,
    #[doc = "/5"]
    FDIV_4,
    #[doc = "/6"]
    FDIV_5,
    #[doc = "/(FDIV+1)"]
    FDIV_7,
    #[doc = "/(FDIV+1)"]
    FDIV_8,
    #[doc = "/(FDIV+1)"]
    FDIV_9,
    #[doc = "/(FDIV+1)"]
    FDIV_10,
    #[doc = "/(FDIV+1)"]
    FDIV_11,
    #[doc = "/(FDIV+1)"]
    FDIV_12,
    #[doc = "/(FDIV+1)"]
    FDIV_13,
    #[doc = "/(FDIV+1)"]
    FDIV_14,
    #[doc = "/(FDIV+1)"]
    FDIV_15,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl FDIVR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            FDIVR::FDIV_0 => 0,
            FDIVR::FDIV_1 => 1,
            FDIVR::FDIV_2 => 2,
            FDIVR::FDIV_3 => 3,
            FDIVR::FDIV_4 => 4,
            FDIVR::FDIV_5 => 5,
            FDIVR::FDIV_7 => 7,
            FDIVR::FDIV_8 => 8,
            FDIVR::FDIV_9 => 9,
            FDIVR::FDIV_10 => 10,
            FDIVR::FDIV_11 => 11,
            FDIVR::FDIV_12 => 12,
            FDIVR::FDIV_13 => 13,
            FDIVR::FDIV_14 => 14,
            FDIVR::FDIV_15 => 15,
            FDIVR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> FDIVR {
        match value {
            0 => FDIVR::FDIV_0,
            1 => FDIVR::FDIV_1,
            2 => FDIVR::FDIV_2,
            3 => FDIVR::FDIV_3,
            4 => FDIVR::FDIV_4,
            5 => FDIVR::FDIV_5,
            7 => FDIVR::FDIV_7,
            8 => FDIVR::FDIV_8,
            9 => FDIVR::FDIV_9,
            10 => FDIVR::FDIV_10,
            11 => FDIVR::FDIV_11,
            12 => FDIVR::FDIV_12,
            13 => FDIVR::FDIV_13,
            14 => FDIVR::FDIV_14,
            15 => FDIVR::FDIV_15,
            i => FDIVR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `FDIV_0`"]
    #[inline]
    pub fn is_fdiv_0(&self) -> bool {
        *self == FDIVR::FDIV_0
    }
    #[doc = "Checks if the value of the field is `FDIV_1`"]
    #[inline]
    pub fn is_fdiv_1(&self) -> bool {
        *self == FDIVR::FDIV_1
    }
    #[doc = "Checks if the value of the field is `FDIV_2`"]
    #[inline]
    pub fn is_fdiv_2(&self) -> bool {
        *self == FDIVR::FDIV_2
    }
    #[doc = "Checks if the value of the field is `FDIV_3`"]
    #[inline]
    pub fn is_fdiv_3(&self) -> bool {
        *self == FDIVR::FDIV_3
    }
    #[doc = "Checks if the value of the field is `FDIV_4`"]
    #[inline]
    pub fn is_fdiv_4(&self) -> bool {
        *self == FDIVR::FDIV_4
    }
    #[doc = "Checks if the value of the field is `FDIV_5`"]
    #[inline]
    pub fn is_fdiv_5(&self) -> bool {
        *self == FDIVR::FDIV_5
    }
    #[doc = "Checks if the value of the field is `FDIV_7`"]
    #[inline]
    pub fn is_fdiv_7(&self) -> bool {
        *self == FDIVR::FDIV_7
    }
    #[doc = "Checks if the value of the field is `FDIV_8`"]
    #[inline]
    pub fn is_fdiv_8(&self) -> bool {
        *self == FDIVR::FDIV_8
    }
    #[doc = "Checks if the value of the field is `FDIV_9`"]
    #[inline]
    pub fn is_fdiv_9(&self) -> bool {
        *self == FDIVR::FDIV_9
    }
    #[doc = "Checks if the value of the field is `FDIV_10`"]
    #[inline]
    pub fn is_fdiv_10(&self) -> bool {
        *self == FDIVR::FDIV_10
    }
    #[doc = "Checks if the value of the field is `FDIV_11`"]
    #[inline]
    pub fn is_fdiv_11(&self) -> bool {
        *self == FDIVR::FDIV_11
    }
    #[doc = "Checks if the value of the field is `FDIV_12`"]
    #[inline]
    pub fn is_fdiv_12(&self) -> bool {
        *self == FDIVR::FDIV_12
    }
    #[doc = "Checks if the value of the field is `FDIV_13`"]
    #[inline]
    pub fn is_fdiv_13(&self) -> bool {
        *self == FDIVR::FDIV_13
    }
    #[doc = "Checks if the value of the field is `FDIV_14`"]
    #[inline]
    pub fn is_fdiv_14(&self) -> bool {
        *self == FDIVR::FDIV_14
    }
    #[doc = "Checks if the value of the field is `FDIV_15`"]
    #[inline]
    pub fn is_fdiv_15(&self) -> bool {
        *self == FDIVR::FDIV_15
    }
}
#[doc = "Possible values of the field `XPINUSE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum XPINUSER {
    #[doc = "Normal mode. Each inactive X pin is Hi-Z."]
    NORMAL_MDOE,
    #[doc = "Ground mode. Each inactive X pin is Low"]
    GROUND_MDOE,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl XPINUSER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            XPINUSER::NORMAL_MDOE => 0,
            XPINUSER::GROUND_MDOE => 1,
            XPINUSER::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> XPINUSER {
        match value {
            0 => XPINUSER::NORMAL_MDOE,
            1 => XPINUSER::GROUND_MDOE,
            i => XPINUSER::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `NORMAL_MDOE`"]
    #[inline]
    pub fn is_normal_mdoe(&self) -> bool {
        *self == XPINUSER::NORMAL_MDOE
    }
    #[doc = "Checks if the value of the field is `GROUND_MDOE`"]
    #[inline]
    pub fn is_ground_mdoe(&self) -> bool {
        *self == XPINUSER::GROUND_MDOE
    }
}
#[doc = r" Value of the field"]
pub struct INCHANGER {
    bits: bool,
}
impl INCHANGER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
#[doc = r" Value of the field"]
pub struct XPINSELR {
    bits: u16,
}
impl XPINSELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
}
#[doc = "Values that can be written to the field `POLLMODE`"]
pub enum POLLMODEW {
    #[doc = "None, inactive. Poll and time counters are turned off. Writing this will reset state and stop any collection in progress. Note: this has no effect on STATUS - those must be cleared manually."]
    NONE,
    #[doc = "Poll now - forces a manual poll to be started immediately, using XPINSEL X pin(s) to activate in the integration loop (all pins set together). Self clears - clear is not indication it is done (see STATUS)."]
    POLL_NOW,
    #[doc = "Normal polling using poll delay from POLL_TCNT register. This will start with the poll delay (which can be 0)."]
    NORMAL,
    #[doc = "The CAPT block will operate in low-power mode. This means it will use GPIO as input, use combination touch measurements, and assume it is to wake the system. This will use the POLL_TCNT poll delay, and start with the delay."]
    LOW_POWER_MODE,
}
impl POLLMODEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            POLLMODEW::NONE => 0,
            POLLMODEW::POLL_NOW => 1,
            POLLMODEW::NORMAL => 2,
            POLLMODEW::LOW_POWER_MODE => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _POLLMODEW<'a> {
    w: &'a mut W,
}
impl<'a> _POLLMODEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: POLLMODEW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "None, inactive. Poll and time counters are turned off. Writing this will reset state and stop any collection in progress. Note: this has no effect on STATUS - those must be cleared manually."]
    #[inline]
    pub fn none(self) -> &'a mut W {
        self.variant(POLLMODEW::NONE)
    }
    #[doc = "Poll now - forces a manual poll to be started immediately, using XPINSEL X pin(s) to activate in the integration loop (all pins set together). Self clears - clear is not indication it is done (see STATUS)."]
    #[inline]
    pub fn poll_now(self) -> &'a mut W {
        self.variant(POLLMODEW::POLL_NOW)
    }
    #[doc = "Normal polling using poll delay from POLL_TCNT register. This will start with the poll delay (which can be 0)."]
    #[inline]
    pub fn normal(self) -> &'a mut W {
        self.variant(POLLMODEW::NORMAL)
    }
    #[doc = "The CAPT block will operate in low-power mode. This means it will use GPIO as input, use combination touch measurements, and assume it is to wake the system. This will use the POLL_TCNT poll delay, and start with the delay."]
    #[inline]
    pub fn low_power_mode(self) -> &'a mut W {
        self.variant(POLLMODEW::LOW_POWER_MODE)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `TYPE`"]
pub enum TYPEW {
    #[doc = "Normal - all X elements are treated as normal, such as buttons and sliders."]
    TYPE_0,
    #[doc = "3x3 grid using NXP Complementary measurements. The 1st 9 Xs are assumed to be the 3x3 grid. After that would be normal X elements. This will also allow 3x1 and 3x2 Note: Only possible if XMAX in STATUS is >=8"]
    TYPE_1,
    #[doc = "5 Sensors interleaved to act as 3x3 touch area using NXP Complementary measurements. 1st 5 Xs used for this, all remaining are treated as normal. Note that if 16 X pins allowed, the 16th will not be usable when TYPE=1. (use TYPE=0 and select 1 smaller than 15 ( and any others from 1 smaller than 5 on up in XPINSEL)."]
    TYPE_2,
    #[doc = "9 Sensors interleaved to act as 5x5 touch area using NXP Complementary measurements. 1st 9 Xs used for this, all remaining are treated as normal. Note: Only possible if XMAX in STATUS is >=8"]
    TYPE_3,
}
impl TYPEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            TYPEW::TYPE_0 => 0,
            TYPEW::TYPE_1 => 1,
            TYPEW::TYPE_2 => 2,
            TYPEW::TYPE_3 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TYPEW<'a> {
    w: &'a mut W,
}
impl<'a> _TYPEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TYPEW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Normal - all X elements are treated as normal, such as buttons and sliders."]
    #[inline]
    pub fn type_0(self) -> &'a mut W {
        self.variant(TYPEW::TYPE_0)
    }
    #[doc = "3x3 grid using NXP Complementary measurements. The 1st 9 Xs are assumed to be the 3x3 grid. After that would be normal X elements. This will also allow 3x1 and 3x2 Note: Only possible if XMAX in STATUS is >=8"]
    #[inline]
    pub fn type_1(self) -> &'a mut W {
        self.variant(TYPEW::TYPE_1)
    }
    #[doc = "5 Sensors interleaved to act as 3x3 touch area using NXP Complementary measurements. 1st 5 Xs used for this, all remaining are treated as normal. Note that if 16 X pins allowed, the 16th will not be usable when TYPE=1. (use TYPE=0 and select 1 smaller than 15 ( and any others from 1 smaller than 5 on up in XPINSEL)."]
    #[inline]
    pub fn type_2(self) -> &'a mut W {
        self.variant(TYPEW::TYPE_2)
    }
    #[doc = "9 Sensors interleaved to act as 5x5 touch area using NXP Complementary measurements. 1st 9 Xs used for this, all remaining are treated as normal. Note: Only possible if XMAX in STATUS is >=8"]
    #[inline]
    pub fn type_3(self) -> &'a mut W {
        self.variant(TYPEW::TYPE_3)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 2;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `TRIGGER`"]
pub enum TRIGGERW {
    #[doc = "Uses YH GPIO. This is not normally used except in Low-power mode. But, it can be used with POLLNOW to baseline that measurement."]
    USES_YH_GPIO,
    #[doc = "ACMP (if fitted). This assumes the ACMP state is fed in asynchronously and it will sample."]
    ACMP,
}
impl TRIGGERW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TRIGGERW::USES_YH_GPIO => false,
            TRIGGERW::ACMP => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TRIGGERW<'a> {
    w: &'a mut W,
}
impl<'a> _TRIGGERW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TRIGGERW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Uses YH GPIO. This is not normally used except in Low-power mode. But, it can be used with POLLNOW to baseline that measurement."]
    #[inline]
    pub fn uses_yh_gpio(self) -> &'a mut W {
        self.variant(TRIGGERW::USES_YH_GPIO)
    }
    #[doc = "ACMP (if fitted). This assumes the ACMP state is fed in asynchronously and it will sample."]
    #[inline]
    pub fn acmp(self) -> &'a mut W {
        self.variant(TRIGGERW::ACMP)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _WAITW<'a> {
    w: &'a mut W,
}
impl<'a> _WAITW<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 5;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `DMA`"]
pub enum DMAW {
    #[doc = "No DMA. Application will use ISRs to read out data"]
    DMA_0,
    #[doc = "Trigger DMA on Touch events"]
    DMA_1,
    #[doc = "Trigger DMA on both Touch and No-Touch events"]
    DMA_2,
    #[doc = "Trigger DMA on both plus Timeout."]
    DMA_3,
}
impl DMAW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            DMAW::DMA_0 => 0,
            DMAW::DMA_1 => 1,
            DMAW::DMA_2 => 2,
            DMAW::DMA_3 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DMAW<'a> {
    w: &'a mut W,
}
impl<'a> _DMAW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DMAW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "No DMA. Application will use ISRs to read out data"]
    #[inline]
    pub fn dma_0(self) -> &'a mut W {
        self.variant(DMAW::DMA_0)
    }
    #[doc = "Trigger DMA on Touch events"]
    #[inline]
    pub fn dma_1(self) -> &'a mut W {
        self.variant(DMAW::DMA_1)
    }
    #[doc = "Trigger DMA on both Touch and No-Touch events"]
    #[inline]
    pub fn dma_2(self) -> &'a mut W {
        self.variant(DMAW::DMA_2)
    }
    #[doc = "Trigger DMA on both plus Timeout."]
    #[inline]
    pub fn dma_3(self) -> &'a mut W {
        self.variant(DMAW::DMA_3)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 6;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `FDIV`"]
pub enum FDIVW {
    #[doc = "No divide"]
    FDIV_0,
    #[doc = "/2"]
    FDIV_1,
    #[doc = "/3"]
    FDIV_2,
    #[doc = "/4"]
    FDIV_3,
    #[doc = "/5"]
    FDIV_4,
    #[doc = "/6"]
    FDIV_5,
    #[doc = "/(FDIV+1)"]
    FDIV_7,
    #[doc = "/(FDIV+1)"]
    FDIV_8,
    #[doc = "/(FDIV+1)"]
    FDIV_9,
    #[doc = "/(FDIV+1)"]
    FDIV_10,
    #[doc = "/(FDIV+1)"]
    FDIV_11,
    #[doc = "/(FDIV+1)"]
    FDIV_12,
    #[doc = "/(FDIV+1)"]
    FDIV_13,
    #[doc = "/(FDIV+1)"]
    FDIV_14,
    #[doc = "/(FDIV+1)"]
    FDIV_15,
}
impl FDIVW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            FDIVW::FDIV_0 => 0,
            FDIVW::FDIV_1 => 1,
            FDIVW::FDIV_2 => 2,
            FDIVW::FDIV_3 => 3,
            FDIVW::FDIV_4 => 4,
            FDIVW::FDIV_5 => 5,
            FDIVW::FDIV_7 => 7,
            FDIVW::FDIV_8 => 8,
            FDIVW::FDIV_9 => 9,
            FDIVW::FDIV_10 => 10,
            FDIVW::FDIV_11 => 11,
            FDIVW::FDIV_12 => 12,
            FDIVW::FDIV_13 => 13,
            FDIVW::FDIV_14 => 14,
            FDIVW::FDIV_15 => 15,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FDIVW<'a> {
    w: &'a mut W,
}
impl<'a> _FDIVW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FDIVW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "No divide"]
    #[inline]
    pub fn fdiv_0(self) -> &'a mut W {
        self.variant(FDIVW::FDIV_0)
    }
    #[doc = "/2"]
    #[inline]
    pub fn fdiv_1(self) -> &'a mut W {
        self.variant(FDIVW::FDIV_1)
    }
    #[doc = "/3"]
    #[inline]
    pub fn fdiv_2(self) -> &'a mut W {
        self.variant(FDIVW::FDIV_2)
    }
    #[doc = "/4"]
    #[inline]
    pub fn fdiv_3(self) -> &'a mut W {
        self.variant(FDIVW::FDIV_3)
    }
    #[doc = "/5"]
    #[inline]
    pub fn fdiv_4(self) -> &'a mut W {
        self.variant(FDIVW::FDIV_4)
    }
    #[doc = "/6"]
    #[inline]
    pub fn fdiv_5(self) -> &'a mut W {
        self.variant(FDIVW::FDIV_5)
    }
    #[doc = "/(FDIV+1)"]
    #[inline]
    pub fn fdiv_7(self) -> &'a mut W {
        self.variant(FDIVW::FDIV_7)
    }
    #[doc = "/(FDIV+1)"]
    #[inline]
    pub fn fdiv_8(self) -> &'a mut W {
        self.variant(FDIVW::FDIV_8)
    }
    #[doc = "/(FDIV+1)"]
    #[inline]
    pub fn fdiv_9(self) -> &'a mut W {
        self.variant(FDIVW::FDIV_9)
    }
    #[doc = "/(FDIV+1)"]
    #[inline]
    pub fn fdiv_10(self) -> &'a mut W {
        self.variant(FDIVW::FDIV_10)
    }
    #[doc = "/(FDIV+1)"]
    #[inline]
    pub fn fdiv_11(self) -> &'a mut W {
        self.variant(FDIVW::FDIV_11)
    }
    #[doc = "/(FDIV+1)"]
    #[inline]
    pub fn fdiv_12(self) -> &'a mut W {
        self.variant(FDIVW::FDIV_12)
    }
    #[doc = "/(FDIV+1)"]
    #[inline]
    pub fn fdiv_13(self) -> &'a mut W {
        self.variant(FDIVW::FDIV_13)
    }
    #[doc = "/(FDIV+1)"]
    #[inline]
    pub fn fdiv_14(self) -> &'a mut W {
        self.variant(FDIVW::FDIV_14)
    }
    #[doc = "/(FDIV+1)"]
    #[inline]
    pub fn fdiv_15(self) -> &'a mut W {
        self.variant(FDIVW::FDIV_15)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `XPINUSE`"]
pub enum XPINUSEW {
    #[doc = "Normal mode. Each inactive X pin is Hi-Z."]
    NORMAL_MDOE,
    #[doc = "Ground mode. Each inactive X pin is Low"]
    GROUND_MDOE,
}
impl XPINUSEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            XPINUSEW::NORMAL_MDOE => 0,
            XPINUSEW::GROUND_MDOE => 1,
        }
    }
}
#[doc = r" Proxy"]
pub struct _XPINUSEW<'a> {
    w: &'a mut W,
}
impl<'a> _XPINUSEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: XPINUSEW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Normal mode. Each inactive X pin is Hi-Z."]
    #[inline]
    pub fn normal_mdoe(self) -> &'a mut W {
        self.variant(XPINUSEW::NORMAL_MDOE)
    }
    #[doc = "Ground mode. Each inactive X pin is Low"]
    #[inline]
    pub fn ground_mdoe(self) -> &'a mut W {
        self.variant(XPINUSEW::GROUND_MDOE)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 12;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _XPINSELW<'a> {
    w: &'a mut W,
}
impl<'a> _XPINSELW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        const MASK: u16 = 65535;
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:1 - Mode of operation. May only change from 0 to another value. So, if 2 or 3, must be changed to 0 1st. Any attempt to go from non-0 to non-0 will result in 0 anyway."]
    #[inline]
    pub fn pollmode(&self) -> POLLMODER {
        POLLMODER::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 2:3 - Selects type of Touch arrangement to use and so how to handle XPINSEL bits"]
    #[inline]
    pub fn type_(&self) -> TYPER {
        TYPER::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 4 - This selects what is being used as the trigger"]
    #[inline]
    pub fn trigger(&self) -> TRIGGERR {
        TRIGGERR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 5 - If 0, the block will continue its X based measurements, even if the TOUCH register has not been read (and so could OVERRUN). If 1, it will wait until read when a touch (TOUCH's ISTOUCH bit is set) before starting the next. This should not normally be needed."]
    #[inline]
    pub fn wait(&self) -> WAITR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        WAITR { bits }
    }
    #[doc = "Bits 6:7 - If not 0, will use the DMA to read out touch events from TOUCH register. The values are shown below. This may be changed while active."]
    #[inline]
    pub fn dma(&self) -> DMAR {
        DMAR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 8:11 - Functional clock divider, or 0 if no divide. The term \"clocks\" in this spec then refer to divided clocks. For a 12MHz input (e.g. FRO 12MHz), this would normally be set to generate a 4MHz output (so, 2). For a 1MHz input, it should be 0. Note for internal use: this does not produce a 50/50 duty cycle when non even divide."]
    #[inline]
    pub fn fdiv(&self) -> FDIVR {
        FDIVR::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 12:13 - Controls how X pins selected in XPINSEL are used when not active in the current polling round."]
    #[inline]
    pub fn xpinuse(&self) -> XPINUSER {
        XPINUSER::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 15 - If 1, do not attempt to write to this register again. This means the last change has not been propagated. This can only happen after changing POLLMODE and DMA. Worse case time would be based on divided FCLK."]
    #[inline]
    pub fn inchange(&self) -> INCHANGER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        INCHANGER { bits }
    }
    #[doc = "Bits 16:31 - Selects which of the X pins are to be used within the allowed pins - see XMAX in STATUS. The X pins are mapped via the IOCON (as are the YH and YL pins) to physical pads. So, this only selects which are to be used as the X half of the touch element. Note: when polling, these are \"walked\" (active) one at a time. When using POLLNOW, the 1 or more selected are used at the same time. Likewise, when in low-power mode, they are used at the same time (or small groups). X pads not selected by XPINSEL are kept at High-Z if they are connected to a pad. This allows using controlled sets for touch detection based on context."]
    #[inline]
    pub fn xpinsel(&self) -> XPINSELR {
        let bits = {
            const MASK: u16 = 65535;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        };
        XPINSELR { bits }
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 0 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:1 - Mode of operation. May only change from 0 to another value. So, if 2 or 3, must be changed to 0 1st. Any attempt to go from non-0 to non-0 will result in 0 anyway."]
    #[inline]
    pub fn pollmode(&mut self) -> _POLLMODEW {
        _POLLMODEW { w: self }
    }
    #[doc = "Bits 2:3 - Selects type of Touch arrangement to use and so how to handle XPINSEL bits"]
    #[inline]
    pub fn type_(&mut self) -> _TYPEW {
        _TYPEW { w: self }
    }
    #[doc = "Bit 4 - This selects what is being used as the trigger"]
    #[inline]
    pub fn trigger(&mut self) -> _TRIGGERW {
        _TRIGGERW { w: self }
    }
    #[doc = "Bit 5 - If 0, the block will continue its X based measurements, even if the TOUCH register has not been read (and so could OVERRUN). If 1, it will wait until read when a touch (TOUCH's ISTOUCH bit is set) before starting the next. This should not normally be needed."]
    #[inline]
    pub fn wait(&mut self) -> _WAITW {
        _WAITW { w: self }
    }
    #[doc = "Bits 6:7 - If not 0, will use the DMA to read out touch events from TOUCH register. The values are shown below. This may be changed while active."]
    #[inline]
    pub fn dma(&mut self) -> _DMAW {
        _DMAW { w: self }
    }
    #[doc = "Bits 8:11 - Functional clock divider, or 0 if no divide. The term \"clocks\" in this spec then refer to divided clocks. For a 12MHz input (e.g. FRO 12MHz), this would normally be set to generate a 4MHz output (so, 2). For a 1MHz input, it should be 0. Note for internal use: this does not produce a 50/50 duty cycle when non even divide."]
    #[inline]
    pub fn fdiv(&mut self) -> _FDIVW {
        _FDIVW { w: self }
    }
    #[doc = "Bits 12:13 - Controls how X pins selected in XPINSEL are used when not active in the current polling round."]
    #[inline]
    pub fn xpinuse(&mut self) -> _XPINUSEW {
        _XPINUSEW { w: self }
    }
    #[doc = "Bits 16:31 - Selects which of the X pins are to be used within the allowed pins - see XMAX in STATUS. The X pins are mapped via the IOCON (as are the YH and YL pins) to physical pads. So, this only selects which are to be used as the X half of the touch element. Note: when polling, these are \"walked\" (active) one at a time. When using POLLNOW, the 1 or more selected are used at the same time. Likewise, when in low-power mode, they are used at the same time (or small groups). X pads not selected by XPINSEL are kept at High-Z if they are connected to a pad. This allows using controlled sets for touch detection based on context."]
    #[inline]
    pub fn xpinsel(&mut self) -> _XPINSELW {
        _XPINSELW { w: self }
    }
}
