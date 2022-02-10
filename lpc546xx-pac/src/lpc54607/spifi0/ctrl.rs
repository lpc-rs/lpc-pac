///Register `CTRL` reader
pub struct R(crate::R<CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTRL_SPEC>) -> Self {
        R(reader)
    }
}
///Register `CTRL` writer
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
impl From<crate::W<CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTRL_SPEC>) -> Self {
        W(writer)
    }
}
///Field `TIMEOUT` reader - This field contains the number of serial clock periods without the processor reading data in memory mode, which will cause the SPIFI hardware to terminate the command by driving the CS pin high and negating the CMD bit in the Status register. (This allows the flash memory to enter a lower-power state.) If the processor reads data from the flash region after a time-out, the command in the Memory Command Register is issued again.
pub struct TIMEOUT_R(crate::FieldReader<u16, u16>);
impl TIMEOUT_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        TIMEOUT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TIMEOUT_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `TIMEOUT` writer - This field contains the number of serial clock periods without the processor reading data in memory mode, which will cause the SPIFI hardware to terminate the command by driving the CS pin high and negating the CMD bit in the Status register. (This allows the flash memory to enter a lower-power state.) If the processor reads data from the flash region after a time-out, the command in the Memory Command Register is issued again.
pub struct TIMEOUT_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMEOUT_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | (value as u32 & 0xffff);
        self.w
    }
}
///Field `CSHIGH` reader - This field controls the minimum CS high time, expressed as a number of serial clock periods minus one.
pub struct CSHIGH_R(crate::FieldReader<u8, u8>);
impl CSHIGH_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CSHIGH_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CSHIGH_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `CSHIGH` writer - This field controls the minimum CS high time, expressed as a number of serial clock periods minus one.
pub struct CSHIGH_W<'a> {
    w: &'a mut W,
}
impl<'a> CSHIGH_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 16)) | ((value as u32 & 0x0f) << 16);
        self.w
    }
}
///Field `D_PRFTCH_DIS` reader - This bit allows conditioning of memory mode prefetches based on the AHB HPROT (instruction/data) access information. A 1 in this register means that the SPIFI will not attempt a speculative prefetch when it encounters data accesses.
pub struct D_PRFTCH_DIS_R(crate::FieldReader<bool, bool>);
impl D_PRFTCH_DIS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        D_PRFTCH_DIS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for D_PRFTCH_DIS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `D_PRFTCH_DIS` writer - This bit allows conditioning of memory mode prefetches based on the AHB HPROT (instruction/data) access information. A 1 in this register means that the SPIFI will not attempt a speculative prefetch when it encounters data accesses.
pub struct D_PRFTCH_DIS_W<'a> {
    w: &'a mut W,
}
impl<'a> D_PRFTCH_DIS_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 21)) | ((value as u32 & 0x01) << 21);
        self.w
    }
}
///Field `INTEN` reader - If this bit is 1 when a command ends, the SPIFI will assert its interrupt request output. See INTRQ in the status register for further details.
pub struct INTEN_R(crate::FieldReader<bool, bool>);
impl INTEN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        INTEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INTEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `INTEN` writer - If this bit is 1 when a command ends, the SPIFI will assert its interrupt request output. See INTRQ in the status register for further details.
pub struct INTEN_W<'a> {
    w: &'a mut W,
}
impl<'a> INTEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 22)) | ((value as u32 & 0x01) << 22);
        self.w
    }
}
///SPI Mode 3 select.
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MODE3_A {
    ///0: SCK LOW. The SPIFI drives SCK low after the rising edge at which the last bit of each command is captured, and keeps it low while CS is HIGH.
    SCK_LOW = 0,
    ///1: SCK HIGH. the SPIFI keeps SCK high after the rising edge for the last bit of each command and while CS is HIGH, and drives it low after it drives CS LOW. (Known serial flash devices can handle either mode, but some devices may require a particular mode for proper operation.) MODE3, RFCLK, and FBCLK should not all be 1, because in this case there is no final falling edge on SCK on which to sample the last data bit of the frame.
    SCK_HIGH = 1,
}
impl From<MODE3_A> for bool {
    #[inline(always)]
    fn from(variant: MODE3_A) -> Self {
        variant as u8 != 0
    }
}
///Field `MODE3` reader - SPI Mode 3 select.
pub struct MODE3_R(crate::FieldReader<bool, MODE3_A>);
impl MODE3_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        MODE3_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> MODE3_A {
        match self.bits {
            false => MODE3_A::SCK_LOW,
            true => MODE3_A::SCK_HIGH,
        }
    }
    ///Checks if the value of the field is `SCK_LOW`
    #[inline(always)]
    pub fn is_sck_low(&self) -> bool {
        **self == MODE3_A::SCK_LOW
    }
    ///Checks if the value of the field is `SCK_HIGH`
    #[inline(always)]
    pub fn is_sck_high(&self) -> bool {
        **self == MODE3_A::SCK_HIGH
    }
}
impl core::ops::Deref for MODE3_R {
    type Target = crate::FieldReader<bool, MODE3_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `MODE3` writer - SPI Mode 3 select.
pub struct MODE3_W<'a> {
    w: &'a mut W,
}
impl<'a> MODE3_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: MODE3_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///SCK LOW. The SPIFI drives SCK low after the rising edge at which the last bit of each command is captured, and keeps it low while CS is HIGH.
    #[inline(always)]
    pub fn sck_low(self) -> &'a mut W {
        self.variant(MODE3_A::SCK_LOW)
    }
    ///SCK HIGH. the SPIFI keeps SCK high after the rising edge for the last bit of each command and while CS is HIGH, and drives it low after it drives CS LOW. (Known serial flash devices can handle either mode, but some devices may require a particular mode for proper operation.) MODE3, RFCLK, and FBCLK should not all be 1, because in this case there is no final falling edge on SCK on which to sample the last data bit of the frame.
    #[inline(always)]
    pub fn sck_high(self) -> &'a mut W {
        self.variant(MODE3_A::SCK_HIGH)
    }
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
        self.w.bits = (self.w.bits & !(0x01 << 23)) | ((value as u32 & 0x01) << 23);
        self.w
    }
}
///Cache prefetching enable. The SPIFI includes an internal cache. A 1 in this bit disables prefetching of cache lines.
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PRFTCH_DIS_A {
    ///0: Enable. Cache prefetching enabled.
    ENABLE = 0,
    ///1: Disable. Disables prefetching of cache lines.
    DISABLE = 1,
}
impl From<PRFTCH_DIS_A> for bool {
    #[inline(always)]
    fn from(variant: PRFTCH_DIS_A) -> Self {
        variant as u8 != 0
    }
}
///Field `PRFTCH_DIS` reader - Cache prefetching enable. The SPIFI includes an internal cache. A 1 in this bit disables prefetching of cache lines.
pub struct PRFTCH_DIS_R(crate::FieldReader<bool, PRFTCH_DIS_A>);
impl PRFTCH_DIS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PRFTCH_DIS_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> PRFTCH_DIS_A {
        match self.bits {
            false => PRFTCH_DIS_A::ENABLE,
            true => PRFTCH_DIS_A::DISABLE,
        }
    }
    ///Checks if the value of the field is `ENABLE`
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        **self == PRFTCH_DIS_A::ENABLE
    }
    ///Checks if the value of the field is `DISABLE`
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        **self == PRFTCH_DIS_A::DISABLE
    }
}
impl core::ops::Deref for PRFTCH_DIS_R {
    type Target = crate::FieldReader<bool, PRFTCH_DIS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `PRFTCH_DIS` writer - Cache prefetching enable. The SPIFI includes an internal cache. A 1 in this bit disables prefetching of cache lines.
pub struct PRFTCH_DIS_W<'a> {
    w: &'a mut W,
}
impl<'a> PRFTCH_DIS_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: PRFTCH_DIS_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Enable. Cache prefetching enabled.
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(PRFTCH_DIS_A::ENABLE)
    }
    ///Disable. Disables prefetching of cache lines.
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(PRFTCH_DIS_A::DISABLE)
    }
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
        self.w.bits = (self.w.bits & !(0x01 << 27)) | ((value as u32 & 0x01) << 27);
        self.w
    }
}
///Select dual protocol.
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DUAL_A {
    ///0: Quad protocol. This protocol uses IO3:0.
    QUAD = 0,
    ///1: Dual protocol. This protocol uses IO1:0.
    DUAL = 1,
}
impl From<DUAL_A> for bool {
    #[inline(always)]
    fn from(variant: DUAL_A) -> Self {
        variant as u8 != 0
    }
}
///Field `DUAL` reader - Select dual protocol.
pub struct DUAL_R(crate::FieldReader<bool, DUAL_A>);
impl DUAL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DUAL_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> DUAL_A {
        match self.bits {
            false => DUAL_A::QUAD,
            true => DUAL_A::DUAL,
        }
    }
    ///Checks if the value of the field is `QUAD`
    #[inline(always)]
    pub fn is_quad(&self) -> bool {
        **self == DUAL_A::QUAD
    }
    ///Checks if the value of the field is `DUAL`
    #[inline(always)]
    pub fn is_dual(&self) -> bool {
        **self == DUAL_A::DUAL
    }
}
impl core::ops::Deref for DUAL_R {
    type Target = crate::FieldReader<bool, DUAL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `DUAL` writer - Select dual protocol.
pub struct DUAL_W<'a> {
    w: &'a mut W,
}
impl<'a> DUAL_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: DUAL_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Quad protocol. This protocol uses IO3:0.
    #[inline(always)]
    pub fn quad(self) -> &'a mut W {
        self.variant(DUAL_A::QUAD)
    }
    ///Dual protocol. This protocol uses IO1:0.
    #[inline(always)]
    pub fn dual(self) -> &'a mut W {
        self.variant(DUAL_A::DUAL)
    }
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
        self.w.bits = (self.w.bits & !(0x01 << 28)) | ((value as u32 & 0x01) << 28);
        self.w
    }
}
///Select active clock edge for input data.
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RFCLK_A {
    ///0: Rising edge. Read data is sampled on rising edges on the clock, as in classic SPI operation.
    RISING_EDGE = 0,
    ///1: Falling edge. Read data is sampled on falling edges of the clock, allowing a full serial clock of of time in order to maximize the serial clock frequency. MODE3, RFCLK, and FBCLK should not all be 1, because in this case there is no final falling edge on SCK on which to sample the last data bit of the frame.
    FALLING_EDGE = 1,
}
impl From<RFCLK_A> for bool {
    #[inline(always)]
    fn from(variant: RFCLK_A) -> Self {
        variant as u8 != 0
    }
}
///Field `RFCLK` reader - Select active clock edge for input data.
pub struct RFCLK_R(crate::FieldReader<bool, RFCLK_A>);
impl RFCLK_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RFCLK_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> RFCLK_A {
        match self.bits {
            false => RFCLK_A::RISING_EDGE,
            true => RFCLK_A::FALLING_EDGE,
        }
    }
    ///Checks if the value of the field is `RISING_EDGE`
    #[inline(always)]
    pub fn is_rising_edge(&self) -> bool {
        **self == RFCLK_A::RISING_EDGE
    }
    ///Checks if the value of the field is `FALLING_EDGE`
    #[inline(always)]
    pub fn is_falling_edge(&self) -> bool {
        **self == RFCLK_A::FALLING_EDGE
    }
}
impl core::ops::Deref for RFCLK_R {
    type Target = crate::FieldReader<bool, RFCLK_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `RFCLK` writer - Select active clock edge for input data.
pub struct RFCLK_W<'a> {
    w: &'a mut W,
}
impl<'a> RFCLK_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: RFCLK_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Rising edge. Read data is sampled on rising edges on the clock, as in classic SPI operation.
    #[inline(always)]
    pub fn rising_edge(self) -> &'a mut W {
        self.variant(RFCLK_A::RISING_EDGE)
    }
    ///Falling edge. Read data is sampled on falling edges of the clock, allowing a full serial clock of of time in order to maximize the serial clock frequency. MODE3, RFCLK, and FBCLK should not all be 1, because in this case there is no final falling edge on SCK on which to sample the last data bit of the frame.
    #[inline(always)]
    pub fn falling_edge(self) -> &'a mut W {
        self.variant(RFCLK_A::FALLING_EDGE)
    }
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
        self.w.bits = (self.w.bits & !(0x01 << 29)) | ((value as u32 & 0x01) << 29);
        self.w
    }
}
///Feedback clock select.
///
///Value on reset: 1
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FBCLK_A {
    ///0: Internal clock. The SPIFI samples read data using an internal clock.
    INTERNAL_CLOCK = 0,
    ///1: Feedback clock. Read data is sampled using a feedback clock from the SCK pin. This allows slightly more time for each received bit. MODE3, RFCLK, and FBCLK should not all be 1, because in this case there is no final falling edge on SCK on which to sample the last data bit of the frame.
    FEEDBACK_CLOCK = 1,
}
impl From<FBCLK_A> for bool {
    #[inline(always)]
    fn from(variant: FBCLK_A) -> Self {
        variant as u8 != 0
    }
}
///Field `FBCLK` reader - Feedback clock select.
pub struct FBCLK_R(crate::FieldReader<bool, FBCLK_A>);
impl FBCLK_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FBCLK_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> FBCLK_A {
        match self.bits {
            false => FBCLK_A::INTERNAL_CLOCK,
            true => FBCLK_A::FEEDBACK_CLOCK,
        }
    }
    ///Checks if the value of the field is `INTERNAL_CLOCK`
    #[inline(always)]
    pub fn is_internal_clock(&self) -> bool {
        **self == FBCLK_A::INTERNAL_CLOCK
    }
    ///Checks if the value of the field is `FEEDBACK_CLOCK`
    #[inline(always)]
    pub fn is_feedback_clock(&self) -> bool {
        **self == FBCLK_A::FEEDBACK_CLOCK
    }
}
impl core::ops::Deref for FBCLK_R {
    type Target = crate::FieldReader<bool, FBCLK_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `FBCLK` writer - Feedback clock select.
pub struct FBCLK_W<'a> {
    w: &'a mut W,
}
impl<'a> FBCLK_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: FBCLK_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Internal clock. The SPIFI samples read data using an internal clock.
    #[inline(always)]
    pub fn internal_clock(self) -> &'a mut W {
        self.variant(FBCLK_A::INTERNAL_CLOCK)
    }
    ///Feedback clock. Read data is sampled using a feedback clock from the SCK pin. This allows slightly more time for each received bit. MODE3, RFCLK, and FBCLK should not all be 1, because in this case there is no final falling edge on SCK on which to sample the last data bit of the frame.
    #[inline(always)]
    pub fn feedback_clock(self) -> &'a mut W {
        self.variant(FBCLK_A::FEEDBACK_CLOCK)
    }
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
        self.w.bits = (self.w.bits & !(0x01 << 30)) | ((value as u32 & 0x01) << 30);
        self.w
    }
}
///Field `DMAEN` reader - A 1 in this bit enables the DMA Request output from the SPIFI. Set this bit only when a DMA channel is used to transfer data in peripheral mode. Do not set this bit when a DMA channel is used for memory-to-memory transfers from the SPIFI memory area. DMAEN should only be used in Command mode.
pub struct DMAEN_R(crate::FieldReader<bool, bool>);
impl DMAEN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DMAEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DMAEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `DMAEN` writer - A 1 in this bit enables the DMA Request output from the SPIFI. Set this bit only when a DMA channel is used to transfer data in peripheral mode. Do not set this bit when a DMA channel is used for memory-to-memory transfers from the SPIFI memory area. DMAEN should only be used in Command mode.
pub struct DMAEN_W<'a> {
    w: &'a mut W,
}
impl<'a> DMAEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | ((value as u32 & 0x01) << 31);
        self.w
    }
}
impl R {
    ///Bits 0:15 - This field contains the number of serial clock periods without the processor reading data in memory mode, which will cause the SPIFI hardware to terminate the command by driving the CS pin high and negating the CMD bit in the Status register. (This allows the flash memory to enter a lower-power state.) If the processor reads data from the flash region after a time-out, the command in the Memory Command Register is issued again.
    #[inline(always)]
    pub fn timeout(&self) -> TIMEOUT_R {
        TIMEOUT_R::new((self.bits & 0xffff) as u16)
    }
    ///Bits 16:19 - This field controls the minimum CS high time, expressed as a number of serial clock periods minus one.
    #[inline(always)]
    pub fn cshigh(&self) -> CSHIGH_R {
        CSHIGH_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    ///Bit 21 - This bit allows conditioning of memory mode prefetches based on the AHB HPROT (instruction/data) access information. A 1 in this register means that the SPIFI will not attempt a speculative prefetch when it encounters data accesses.
    #[inline(always)]
    pub fn d_prftch_dis(&self) -> D_PRFTCH_DIS_R {
        D_PRFTCH_DIS_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    ///Bit 22 - If this bit is 1 when a command ends, the SPIFI will assert its interrupt request output. See INTRQ in the status register for further details.
    #[inline(always)]
    pub fn inten(&self) -> INTEN_R {
        INTEN_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    ///Bit 23 - SPI Mode 3 select.
    #[inline(always)]
    pub fn mode3(&self) -> MODE3_R {
        MODE3_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    ///Bit 27 - Cache prefetching enable. The SPIFI includes an internal cache. A 1 in this bit disables prefetching of cache lines.
    #[inline(always)]
    pub fn prftch_dis(&self) -> PRFTCH_DIS_R {
        PRFTCH_DIS_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    ///Bit 28 - Select dual protocol.
    #[inline(always)]
    pub fn dual(&self) -> DUAL_R {
        DUAL_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    ///Bit 29 - Select active clock edge for input data.
    #[inline(always)]
    pub fn rfclk(&self) -> RFCLK_R {
        RFCLK_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    ///Bit 30 - Feedback clock select.
    #[inline(always)]
    pub fn fbclk(&self) -> FBCLK_R {
        FBCLK_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    ///Bit 31 - A 1 in this bit enables the DMA Request output from the SPIFI. Set this bit only when a DMA channel is used to transfer data in peripheral mode. Do not set this bit when a DMA channel is used for memory-to-memory transfers from the SPIFI memory area. DMAEN should only be used in Command mode.
    #[inline(always)]
    pub fn dmaen(&self) -> DMAEN_R {
        DMAEN_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    ///Bits 0:15 - This field contains the number of serial clock periods without the processor reading data in memory mode, which will cause the SPIFI hardware to terminate the command by driving the CS pin high and negating the CMD bit in the Status register. (This allows the flash memory to enter a lower-power state.) If the processor reads data from the flash region after a time-out, the command in the Memory Command Register is issued again.
    #[inline(always)]
    pub fn timeout(&mut self) -> TIMEOUT_W {
        TIMEOUT_W { w: self }
    }
    ///Bits 16:19 - This field controls the minimum CS high time, expressed as a number of serial clock periods minus one.
    #[inline(always)]
    pub fn cshigh(&mut self) -> CSHIGH_W {
        CSHIGH_W { w: self }
    }
    ///Bit 21 - This bit allows conditioning of memory mode prefetches based on the AHB HPROT (instruction/data) access information. A 1 in this register means that the SPIFI will not attempt a speculative prefetch when it encounters data accesses.
    #[inline(always)]
    pub fn d_prftch_dis(&mut self) -> D_PRFTCH_DIS_W {
        D_PRFTCH_DIS_W { w: self }
    }
    ///Bit 22 - If this bit is 1 when a command ends, the SPIFI will assert its interrupt request output. See INTRQ in the status register for further details.
    #[inline(always)]
    pub fn inten(&mut self) -> INTEN_W {
        INTEN_W { w: self }
    }
    ///Bit 23 - SPI Mode 3 select.
    #[inline(always)]
    pub fn mode3(&mut self) -> MODE3_W {
        MODE3_W { w: self }
    }
    ///Bit 27 - Cache prefetching enable. The SPIFI includes an internal cache. A 1 in this bit disables prefetching of cache lines.
    #[inline(always)]
    pub fn prftch_dis(&mut self) -> PRFTCH_DIS_W {
        PRFTCH_DIS_W { w: self }
    }
    ///Bit 28 - Select dual protocol.
    #[inline(always)]
    pub fn dual(&mut self) -> DUAL_W {
        DUAL_W { w: self }
    }
    ///Bit 29 - Select active clock edge for input data.
    #[inline(always)]
    pub fn rfclk(&mut self) -> RFCLK_W {
        RFCLK_W { w: self }
    }
    ///Bit 30 - Feedback clock select.
    #[inline(always)]
    pub fn fbclk(&mut self) -> FBCLK_W {
        FBCLK_W { w: self }
    }
    ///Bit 31 - A 1 in this bit enables the DMA Request output from the SPIFI. Set this bit only when a DMA channel is used to transfer data in peripheral mode. Do not set this bit when a DMA channel is used for memory-to-memory transfers from the SPIFI memory area. DMAEN should only be used in Command mode.
    #[inline(always)]
    pub fn dmaen(&mut self) -> DMAEN_W {
        DMAEN_W { w: self }
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///SPIFI control register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [ctrl](index.html) module
pub struct CTRL_SPEC;
impl crate::RegisterSpec for CTRL_SPEC {
    type Ux = u32;
}
///`read()` method returns [ctrl::R](R) reader structure
impl crate::Readable for CTRL_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [ctrl::W](W) writer structure
impl crate::Writable for CTRL_SPEC {
    type Writer = W;
}
///`reset()` method sets CTRL to value 0x400f_ffff
impl crate::Resettable for CTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x400f_ffff
    }
}
