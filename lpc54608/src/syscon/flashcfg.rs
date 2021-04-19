#[doc = "Register `FLASHCFG` reader"]
pub struct R(crate::R<FLASHCFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FLASHCFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<FLASHCFG_SPEC>> for R {
    fn from(reader: crate::R<FLASHCFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FLASHCFG` writer"]
pub struct W(crate::W<FLASHCFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FLASHCFG_SPEC>;
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
impl core::convert::From<crate::W<FLASHCFG_SPEC>> for W {
    fn from(writer: crate::W<FLASHCFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Instruction fetch configuration. This field determines how flash accelerator buffers are used for instruction fetches.\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum FETCHCFG_A {
    #[doc = "0: Instruction fetches from flash are not buffered. Every fetch request from the CPU results in a read of the flash memory. This setting may use significantly more power than when buffering is enabled."]
    NO_BUFFER = 0,
    #[doc = "1: One buffer is used for all instruction fetches."]
    ONE_BUFFER = 1,
    #[doc = "2: All buffers may be used for instruction fetches."]
    ALL_BUFFERS = 2,
}
impl From<FETCHCFG_A> for u8 {
    #[inline(always)]
    fn from(variant: FETCHCFG_A) -> Self {
        variant as _
    }
}
#[doc = "Field `FETCHCFG` reader - Instruction fetch configuration. This field determines how flash accelerator buffers are used for instruction fetches."]
pub struct FETCHCFG_R(crate::FieldReader<u8, FETCHCFG_A>);
impl FETCHCFG_R {
    pub(crate) fn new(bits: u8) -> Self {
        FETCHCFG_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<FETCHCFG_A> {
        match self.bits {
            0 => Some(FETCHCFG_A::NO_BUFFER),
            1 => Some(FETCHCFG_A::ONE_BUFFER),
            2 => Some(FETCHCFG_A::ALL_BUFFERS),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `NO_BUFFER`"]
    #[inline(always)]
    pub fn is_no_buffer(&self) -> bool {
        **self == FETCHCFG_A::NO_BUFFER
    }
    #[doc = "Checks if the value of the field is `ONE_BUFFER`"]
    #[inline(always)]
    pub fn is_one_buffer(&self) -> bool {
        **self == FETCHCFG_A::ONE_BUFFER
    }
    #[doc = "Checks if the value of the field is `ALL_BUFFERS`"]
    #[inline(always)]
    pub fn is_all_buffers(&self) -> bool {
        **self == FETCHCFG_A::ALL_BUFFERS
    }
}
impl core::ops::Deref for FETCHCFG_R {
    type Target = crate::FieldReader<u8, FETCHCFG_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FETCHCFG` writer - Instruction fetch configuration. This field determines how flash accelerator buffers are used for instruction fetches."]
pub struct FETCHCFG_W<'a> {
    w: &'a mut W,
}
impl<'a> FETCHCFG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FETCHCFG_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Instruction fetches from flash are not buffered. Every fetch request from the CPU results in a read of the flash memory. This setting may use significantly more power than when buffering is enabled."]
    #[inline(always)]
    pub fn no_buffer(self) -> &'a mut W {
        self.variant(FETCHCFG_A::NO_BUFFER)
    }
    #[doc = "One buffer is used for all instruction fetches."]
    #[inline(always)]
    pub fn one_buffer(self) -> &'a mut W {
        self.variant(FETCHCFG_A::ONE_BUFFER)
    }
    #[doc = "All buffers may be used for instruction fetches."]
    #[inline(always)]
    pub fn all_buffers(self) -> &'a mut W {
        self.variant(FETCHCFG_A::ALL_BUFFERS)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | (value as u32 & 0x03);
        self.w
    }
}
#[doc = "Data read configuration. This field determines how flash accelerator buffers are used for data accesses.\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum DATACFG_A {
    #[doc = "0: Data accesses from flash are not buffered. Every data access from the CPU results in a read of the flash memory."]
    NOT_BUFFERED = 0,
    #[doc = "1: One buffer is used for all data accesses."]
    ONE_BUFFER = 1,
    #[doc = "2: All buffers may be used for data accesses."]
    ALL_BUFFERS = 2,
}
impl From<DATACFG_A> for u8 {
    #[inline(always)]
    fn from(variant: DATACFG_A) -> Self {
        variant as _
    }
}
#[doc = "Field `DATACFG` reader - Data read configuration. This field determines how flash accelerator buffers are used for data accesses."]
pub struct DATACFG_R(crate::FieldReader<u8, DATACFG_A>);
impl DATACFG_R {
    pub(crate) fn new(bits: u8) -> Self {
        DATACFG_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<DATACFG_A> {
        match self.bits {
            0 => Some(DATACFG_A::NOT_BUFFERED),
            1 => Some(DATACFG_A::ONE_BUFFER),
            2 => Some(DATACFG_A::ALL_BUFFERS),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_BUFFERED`"]
    #[inline(always)]
    pub fn is_not_buffered(&self) -> bool {
        **self == DATACFG_A::NOT_BUFFERED
    }
    #[doc = "Checks if the value of the field is `ONE_BUFFER`"]
    #[inline(always)]
    pub fn is_one_buffer(&self) -> bool {
        **self == DATACFG_A::ONE_BUFFER
    }
    #[doc = "Checks if the value of the field is `ALL_BUFFERS`"]
    #[inline(always)]
    pub fn is_all_buffers(&self) -> bool {
        **self == DATACFG_A::ALL_BUFFERS
    }
}
impl core::ops::Deref for DATACFG_R {
    type Target = crate::FieldReader<u8, DATACFG_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DATACFG` writer - Data read configuration. This field determines how flash accelerator buffers are used for data accesses."]
pub struct DATACFG_W<'a> {
    w: &'a mut W,
}
impl<'a> DATACFG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DATACFG_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Data accesses from flash are not buffered. Every data access from the CPU results in a read of the flash memory."]
    #[inline(always)]
    pub fn not_buffered(self) -> &'a mut W {
        self.variant(DATACFG_A::NOT_BUFFERED)
    }
    #[doc = "One buffer is used for all data accesses."]
    #[inline(always)]
    pub fn one_buffer(self) -> &'a mut W {
        self.variant(DATACFG_A::ONE_BUFFER)
    }
    #[doc = "All buffers may be used for data accesses."]
    #[inline(always)]
    pub fn all_buffers(self) -> &'a mut W {
        self.variant(DATACFG_A::ALL_BUFFERS)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | ((value as u32 & 0x03) << 2);
        self.w
    }
}
#[doc = "Acceleration enable.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ACCEL_A {
    #[doc = "0: Flash acceleration is disabled. Every flash read (including those fulfilled from a buffer) takes FLASHTIM + 1 system clocks. This allows more determinism at a cost of performance."]
    DISABLED = 0,
    #[doc = "1: Flash acceleration is enabled. Performance is enhanced, dependent on other FLASHCFG settings."]
    ENABLED = 1,
}
impl From<ACCEL_A> for bool {
    #[inline(always)]
    fn from(variant: ACCEL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ACCEL` reader - Acceleration enable."]
pub struct ACCEL_R(crate::FieldReader<bool, ACCEL_A>);
impl ACCEL_R {
    pub(crate) fn new(bits: bool) -> Self {
        ACCEL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ACCEL_A {
        match self.bits {
            false => ACCEL_A::DISABLED,
            true => ACCEL_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == ACCEL_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == ACCEL_A::ENABLED
    }
}
impl core::ops::Deref for ACCEL_R {
    type Target = crate::FieldReader<bool, ACCEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ACCEL` writer - Acceleration enable."]
pub struct ACCEL_W<'a> {
    w: &'a mut W,
}
impl<'a> ACCEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ACCEL_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Flash acceleration is disabled. Every flash read (including those fulfilled from a buffer) takes FLASHTIM + 1 system clocks. This allows more determinism at a cost of performance."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ACCEL_A::DISABLED)
    }
    #[doc = "Flash acceleration is enabled. Performance is enhanced, dependent on other FLASHCFG settings."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ACCEL_A::ENABLED)
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
#[doc = "Prefetch enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PREFEN_A {
    #[doc = "0: No instruction prefetch is performed."]
    NO_PREFETCH = 0,
    #[doc = "1: If the FETCHCFG field is not 0, the next flash line following the current execution address is automatically prefetched if it is not already buffered."]
    PREFETCH = 1,
}
impl From<PREFEN_A> for bool {
    #[inline(always)]
    fn from(variant: PREFEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PREFEN` reader - Prefetch enable."]
pub struct PREFEN_R(crate::FieldReader<bool, PREFEN_A>);
impl PREFEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        PREFEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PREFEN_A {
        match self.bits {
            false => PREFEN_A::NO_PREFETCH,
            true => PREFEN_A::PREFETCH,
        }
    }
    #[doc = "Checks if the value of the field is `NO_PREFETCH`"]
    #[inline(always)]
    pub fn is_no_prefetch(&self) -> bool {
        **self == PREFEN_A::NO_PREFETCH
    }
    #[doc = "Checks if the value of the field is `PREFETCH`"]
    #[inline(always)]
    pub fn is_prefetch(&self) -> bool {
        **self == PREFEN_A::PREFETCH
    }
}
impl core::ops::Deref for PREFEN_R {
    type Target = crate::FieldReader<bool, PREFEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PREFEN` writer - Prefetch enable."]
pub struct PREFEN_W<'a> {
    w: &'a mut W,
}
impl<'a> PREFEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PREFEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No instruction prefetch is performed."]
    #[inline(always)]
    pub fn no_prefetch(self) -> &'a mut W {
        self.variant(PREFEN_A::NO_PREFETCH)
    }
    #[doc = "If the FETCHCFG field is not 0, the next flash line following the current execution address is automatically prefetched if it is not already buffered."]
    #[inline(always)]
    pub fn prefetch(self) -> &'a mut W {
        self.variant(PREFEN_A::PREFETCH)
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | ((value as u32 & 0x01) << 5);
        self.w
    }
}
#[doc = "Prefetch override. This bit only applies when PREFEN = 1 and a buffered instruction is completing for which the next flash line is not already buffered or being prefetched.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PREFOVR_A {
    #[doc = "0: Any previously initiated prefetch will be completed."]
    PREFETCH_COMPLETED = 0,
    #[doc = "1: Any previously initiated prefetch will be aborted, and the next flash line following the current execution address will be prefetched if not already buffered."]
    PREFETCH_ABORT = 1,
}
impl From<PREFOVR_A> for bool {
    #[inline(always)]
    fn from(variant: PREFOVR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PREFOVR` reader - Prefetch override. This bit only applies when PREFEN = 1 and a buffered instruction is completing for which the next flash line is not already buffered or being prefetched."]
pub struct PREFOVR_R(crate::FieldReader<bool, PREFOVR_A>);
impl PREFOVR_R {
    pub(crate) fn new(bits: bool) -> Self {
        PREFOVR_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PREFOVR_A {
        match self.bits {
            false => PREFOVR_A::PREFETCH_COMPLETED,
            true => PREFOVR_A::PREFETCH_ABORT,
        }
    }
    #[doc = "Checks if the value of the field is `PREFETCH_COMPLETED`"]
    #[inline(always)]
    pub fn is_prefetch_completed(&self) -> bool {
        **self == PREFOVR_A::PREFETCH_COMPLETED
    }
    #[doc = "Checks if the value of the field is `PREFETCH_ABORT`"]
    #[inline(always)]
    pub fn is_prefetch_abort(&self) -> bool {
        **self == PREFOVR_A::PREFETCH_ABORT
    }
}
impl core::ops::Deref for PREFOVR_R {
    type Target = crate::FieldReader<bool, PREFOVR_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PREFOVR` writer - Prefetch override. This bit only applies when PREFEN = 1 and a buffered instruction is completing for which the next flash line is not already buffered or being prefetched."]
pub struct PREFOVR_W<'a> {
    w: &'a mut W,
}
impl<'a> PREFOVR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PREFOVR_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Any previously initiated prefetch will be completed."]
    #[inline(always)]
    pub fn prefetch_completed(self) -> &'a mut W {
        self.variant(PREFOVR_A::PREFETCH_COMPLETED)
    }
    #[doc = "Any previously initiated prefetch will be aborted, and the next flash line following the current execution address will be prefetched if not already buffered."]
    #[inline(always)]
    pub fn prefetch_abort(self) -> &'a mut W {
        self.variant(PREFOVR_A::PREFETCH_ABORT)
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | ((value as u32 & 0x01) << 6);
        self.w
    }
}
#[doc = "Flash memory access time. The number of system clocks used for flash accesses is equal to FLASHTIM +1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum FLASHTIM_A {
    #[doc = "0: 1 system clock flash access time (for system clock rates up to 12 MHz)."]
    N_1_CLOCK_CYCLE = 0,
    #[doc = "1: 2 system clocks flash access time (for system clock rates up to 30 MHz)."]
    N_2_CLOCK_CYCLES = 1,
    #[doc = "2: 3 system clocks flash access time (for system clock rates up to 60 MHz)."]
    N_3_CLOCK_CYCLES = 2,
    #[doc = "3: 4 system clocks flash access time (for system clock rates up to 85 MHz)."]
    N_4_CLOCK_CYCLES = 3,
    #[doc = "4: 5 system clocks flash access time (for system clock rates up to 100 MHz)."]
    N_5_CLOCK_CYCLES = 4,
}
impl From<FLASHTIM_A> for u8 {
    #[inline(always)]
    fn from(variant: FLASHTIM_A) -> Self {
        variant as _
    }
}
#[doc = "Field `FLASHTIM` reader - Flash memory access time. The number of system clocks used for flash accesses is equal to FLASHTIM +1."]
pub struct FLASHTIM_R(crate::FieldReader<u8, FLASHTIM_A>);
impl FLASHTIM_R {
    pub(crate) fn new(bits: u8) -> Self {
        FLASHTIM_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<FLASHTIM_A> {
        match self.bits {
            0 => Some(FLASHTIM_A::N_1_CLOCK_CYCLE),
            1 => Some(FLASHTIM_A::N_2_CLOCK_CYCLES),
            2 => Some(FLASHTIM_A::N_3_CLOCK_CYCLES),
            3 => Some(FLASHTIM_A::N_4_CLOCK_CYCLES),
            4 => Some(FLASHTIM_A::N_5_CLOCK_CYCLES),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `N_1_CLOCK_CYCLE`"]
    #[inline(always)]
    pub fn is_n_1_clock_cycle(&self) -> bool {
        **self == FLASHTIM_A::N_1_CLOCK_CYCLE
    }
    #[doc = "Checks if the value of the field is `N_2_CLOCK_CYCLES`"]
    #[inline(always)]
    pub fn is_n_2_clock_cycles(&self) -> bool {
        **self == FLASHTIM_A::N_2_CLOCK_CYCLES
    }
    #[doc = "Checks if the value of the field is `N_3_CLOCK_CYCLES`"]
    #[inline(always)]
    pub fn is_n_3_clock_cycles(&self) -> bool {
        **self == FLASHTIM_A::N_3_CLOCK_CYCLES
    }
    #[doc = "Checks if the value of the field is `N_4_CLOCK_CYCLES`"]
    #[inline(always)]
    pub fn is_n_4_clock_cycles(&self) -> bool {
        **self == FLASHTIM_A::N_4_CLOCK_CYCLES
    }
    #[doc = "Checks if the value of the field is `N_5_CLOCK_CYCLES`"]
    #[inline(always)]
    pub fn is_n_5_clock_cycles(&self) -> bool {
        **self == FLASHTIM_A::N_5_CLOCK_CYCLES
    }
}
impl core::ops::Deref for FLASHTIM_R {
    type Target = crate::FieldReader<u8, FLASHTIM_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FLASHTIM` writer - Flash memory access time. The number of system clocks used for flash accesses is equal to FLASHTIM +1."]
pub struct FLASHTIM_W<'a> {
    w: &'a mut W,
}
impl<'a> FLASHTIM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FLASHTIM_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "1 system clock flash access time (for system clock rates up to 12 MHz)."]
    #[inline(always)]
    pub fn n_1_clock_cycle(self) -> &'a mut W {
        self.variant(FLASHTIM_A::N_1_CLOCK_CYCLE)
    }
    #[doc = "2 system clocks flash access time (for system clock rates up to 30 MHz)."]
    #[inline(always)]
    pub fn n_2_clock_cycles(self) -> &'a mut W {
        self.variant(FLASHTIM_A::N_2_CLOCK_CYCLES)
    }
    #[doc = "3 system clocks flash access time (for system clock rates up to 60 MHz)."]
    #[inline(always)]
    pub fn n_3_clock_cycles(self) -> &'a mut W {
        self.variant(FLASHTIM_A::N_3_CLOCK_CYCLES)
    }
    #[doc = "4 system clocks flash access time (for system clock rates up to 85 MHz)."]
    #[inline(always)]
    pub fn n_4_clock_cycles(self) -> &'a mut W {
        self.variant(FLASHTIM_A::N_4_CLOCK_CYCLES)
    }
    #[doc = "5 system clocks flash access time (for system clock rates up to 100 MHz)."]
    #[inline(always)]
    pub fn n_5_clock_cycles(self) -> &'a mut W {
        self.variant(FLASHTIM_A::N_5_CLOCK_CYCLES)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 12)) | ((value as u32 & 0x0f) << 12);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - Instruction fetch configuration. This field determines how flash accelerator buffers are used for instruction fetches."]
    #[inline(always)]
    pub fn fetchcfg(&self) -> FETCHCFG_R {
        FETCHCFG_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 2:3 - Data read configuration. This field determines how flash accelerator buffers are used for data accesses."]
    #[inline(always)]
    pub fn datacfg(&self) -> DATACFG_R {
        DATACFG_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bit 4 - Acceleration enable."]
    #[inline(always)]
    pub fn accel(&self) -> ACCEL_R {
        ACCEL_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Prefetch enable."]
    #[inline(always)]
    pub fn prefen(&self) -> PREFEN_R {
        PREFEN_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Prefetch override. This bit only applies when PREFEN = 1 and a buffered instruction is completing for which the next flash line is not already buffered or being prefetched."]
    #[inline(always)]
    pub fn prefovr(&self) -> PREFOVR_R {
        PREFOVR_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bits 12:15 - Flash memory access time. The number of system clocks used for flash accesses is equal to FLASHTIM +1."]
    #[inline(always)]
    pub fn flashtim(&self) -> FLASHTIM_R {
        FLASHTIM_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Instruction fetch configuration. This field determines how flash accelerator buffers are used for instruction fetches."]
    #[inline(always)]
    pub fn fetchcfg(&mut self) -> FETCHCFG_W {
        FETCHCFG_W { w: self }
    }
    #[doc = "Bits 2:3 - Data read configuration. This field determines how flash accelerator buffers are used for data accesses."]
    #[inline(always)]
    pub fn datacfg(&mut self) -> DATACFG_W {
        DATACFG_W { w: self }
    }
    #[doc = "Bit 4 - Acceleration enable."]
    #[inline(always)]
    pub fn accel(&mut self) -> ACCEL_W {
        ACCEL_W { w: self }
    }
    #[doc = "Bit 5 - Prefetch enable."]
    #[inline(always)]
    pub fn prefen(&mut self) -> PREFEN_W {
        PREFEN_W { w: self }
    }
    #[doc = "Bit 6 - Prefetch override. This bit only applies when PREFEN = 1 and a buffered instruction is completing for which the next flash line is not already buffered or being prefetched."]
    #[inline(always)]
    pub fn prefovr(&mut self) -> PREFOVR_W {
        PREFOVR_W { w: self }
    }
    #[doc = "Bits 12:15 - Flash memory access time. The number of system clocks used for flash accesses is equal to FLASHTIM +1."]
    #[inline(always)]
    pub fn flashtim(&mut self) -> FLASHTIM_W {
        FLASHTIM_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Flash wait states configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [flashcfg](index.html) module"]
pub struct FLASHCFG_SPEC;
impl crate::RegisterSpec for FLASHCFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [flashcfg::R](R) reader structure"]
impl crate::Readable for FLASHCFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [flashcfg::W](W) writer structure"]
impl crate::Writable for FLASHCFG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FLASHCFG to value 0x0d1a"]
impl crate::Resettable for FLASHCFG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0d1a
    }
}
