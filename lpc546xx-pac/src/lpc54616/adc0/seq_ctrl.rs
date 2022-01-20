#[doc = "Register `SEQ_CTRL%s` reader"]
pub struct R(crate::R<SEQ_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SEQ_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SEQ_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SEQ_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SEQ_CTRL%s` writer"]
pub struct W(crate::W<SEQ_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SEQ_CTRL_SPEC>;
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
impl From<crate::W<SEQ_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SEQ_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CHANNELS` reader - Selects which one or more of the ADC channels will be sampled and converted when this sequence is launched. A 1 in any bit of this field will cause the corresponding channel to be included in the conversion sequence, where bit 0 corresponds to channel 0, bit 1 to channel 1 and so forth. When this conversion sequence is triggered, either by a hardware trigger or via software command, ADC conversions will be performed on each enabled channel, in sequence, beginning with the lowest-ordered channel. This field can ONLY be changed while SEQA_ENA (bit 31) is LOW. It is allowed to change this field and set bit 31 in the same write."]
pub struct CHANNELS_R(crate::FieldReader<u16, u16>);
impl CHANNELS_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        CHANNELS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CHANNELS_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CHANNELS` writer - Selects which one or more of the ADC channels will be sampled and converted when this sequence is launched. A 1 in any bit of this field will cause the corresponding channel to be included in the conversion sequence, where bit 0 corresponds to channel 0, bit 1 to channel 1 and so forth. When this conversion sequence is triggered, either by a hardware trigger or via software command, ADC conversions will be performed on each enabled channel, in sequence, beginning with the lowest-ordered channel. This field can ONLY be changed while SEQA_ENA (bit 31) is LOW. It is allowed to change this field and set bit 31 in the same write."]
pub struct CHANNELS_W<'a> {
    w: &'a mut W,
}
impl<'a> CHANNELS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0fff) | (value as u32 & 0x0fff);
        self.w
    }
}
#[doc = "Field `TRIGGER` reader - Selects which of the available hardware trigger sources will cause this conversion sequence to be initiated. Program the trigger input number in this field. See Table 476. In order to avoid generating a spurious trigger, it is recommended writing to this field only when SEQA_ENA (bit 31) is low. It is safe to change this field and set bit 31 in the same write."]
pub struct TRIGGER_R(crate::FieldReader<u8, u8>);
impl TRIGGER_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        TRIGGER_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TRIGGER_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TRIGGER` writer - Selects which of the available hardware trigger sources will cause this conversion sequence to be initiated. Program the trigger input number in this field. See Table 476. In order to avoid generating a spurious trigger, it is recommended writing to this field only when SEQA_ENA (bit 31) is low. It is safe to change this field and set bit 31 in the same write."]
pub struct TRIGGER_W<'a> {
    w: &'a mut W,
}
impl<'a> TRIGGER_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 12)) | ((value as u32 & 0x3f) << 12);
        self.w
    }
}
#[doc = "Select the polarity of the selected input trigger for this conversion sequence. In order to avoid generating a spurious trigger, it is recommended writing to this field only when SEQA_ENA (bit 31) is low. It is safe to change this field and set bit 31 in the same write.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TRIGPOL_A {
    #[doc = "0: Negative edge. A negative edge launches the conversion sequence on the selected trigger input."]
    NEGATIVE_EDGE = 0,
    #[doc = "1: Positive edge. A positive edge launches the conversion sequence on the selected trigger input."]
    POSITIVE_EDGE = 1,
}
impl From<TRIGPOL_A> for bool {
    #[inline(always)]
    fn from(variant: TRIGPOL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TRIGPOL` reader - Select the polarity of the selected input trigger for this conversion sequence. In order to avoid generating a spurious trigger, it is recommended writing to this field only when SEQA_ENA (bit 31) is low. It is safe to change this field and set bit 31 in the same write."]
pub struct TRIGPOL_R(crate::FieldReader<bool, TRIGPOL_A>);
impl TRIGPOL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TRIGPOL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TRIGPOL_A {
        match self.bits {
            false => TRIGPOL_A::NEGATIVE_EDGE,
            true => TRIGPOL_A::POSITIVE_EDGE,
        }
    }
    #[doc = "Checks if the value of the field is `NEGATIVE_EDGE`"]
    #[inline(always)]
    pub fn is_negative_edge(&self) -> bool {
        **self == TRIGPOL_A::NEGATIVE_EDGE
    }
    #[doc = "Checks if the value of the field is `POSITIVE_EDGE`"]
    #[inline(always)]
    pub fn is_positive_edge(&self) -> bool {
        **self == TRIGPOL_A::POSITIVE_EDGE
    }
}
impl core::ops::Deref for TRIGPOL_R {
    type Target = crate::FieldReader<bool, TRIGPOL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TRIGPOL` writer - Select the polarity of the selected input trigger for this conversion sequence. In order to avoid generating a spurious trigger, it is recommended writing to this field only when SEQA_ENA (bit 31) is low. It is safe to change this field and set bit 31 in the same write."]
pub struct TRIGPOL_W<'a> {
    w: &'a mut W,
}
impl<'a> TRIGPOL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TRIGPOL_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Negative edge. A negative edge launches the conversion sequence on the selected trigger input."]
    #[inline(always)]
    pub fn negative_edge(self) -> &'a mut W {
        self.variant(TRIGPOL_A::NEGATIVE_EDGE)
    }
    #[doc = "Positive edge. A positive edge launches the conversion sequence on the selected trigger input."]
    #[inline(always)]
    pub fn positive_edge(self) -> &'a mut W {
        self.variant(TRIGPOL_A::POSITIVE_EDGE)
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | ((value as u32 & 0x01) << 18);
        self.w
    }
}
#[doc = "Setting this bit allows the hardware trigger input to bypass synchronization flip-flop stages and therefore shorten the time between the trigger input signal and the start of a conversion. There are slightly different criteria for whether or not this bit can be set depending on the clock operating mode: Synchronous mode (the ASYNMODE in the CTRL register = 0): Synchronization may be bypassed (this bit may be set) if the selected trigger source is already synchronous with the main system clock (eg. coming from an on-chip, system-clock-based timer). Whether this bit is set or not, a trigger pulse must be maintained for at least one system clock period. Asynchronous mode (the ASYNMODE in the CTRL register = 1): Synchronization may be bypassed (this bit may be set) if it is certain that the duration of a trigger input pulse will be at least one cycle of the ADC clock (regardless of whether the trigger comes from and on-chip or off-chip source). If this bit is NOT set, the trigger pulse must at least be maintained for one system clock period.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SYNCBYPASS_A {
    #[doc = "0: Enable trigger synchronization. The hardware trigger bypass is not enabled."]
    ENABLE_TRIGGER_SYNCH = 0,
    #[doc = "1: Bypass trigger synchronization. The hardware trigger bypass is enabled."]
    BYPASS_TRIGGER_SYNCH = 1,
}
impl From<SYNCBYPASS_A> for bool {
    #[inline(always)]
    fn from(variant: SYNCBYPASS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SYNCBYPASS` reader - Setting this bit allows the hardware trigger input to bypass synchronization flip-flop stages and therefore shorten the time between the trigger input signal and the start of a conversion. There are slightly different criteria for whether or not this bit can be set depending on the clock operating mode: Synchronous mode (the ASYNMODE in the CTRL register = 0): Synchronization may be bypassed (this bit may be set) if the selected trigger source is already synchronous with the main system clock (eg. coming from an on-chip, system-clock-based timer). Whether this bit is set or not, a trigger pulse must be maintained for at least one system clock period. Asynchronous mode (the ASYNMODE in the CTRL register = 1): Synchronization may be bypassed (this bit may be set) if it is certain that the duration of a trigger input pulse will be at least one cycle of the ADC clock (regardless of whether the trigger comes from and on-chip or off-chip source). If this bit is NOT set, the trigger pulse must at least be maintained for one system clock period."]
pub struct SYNCBYPASS_R(crate::FieldReader<bool, SYNCBYPASS_A>);
impl SYNCBYPASS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SYNCBYPASS_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SYNCBYPASS_A {
        match self.bits {
            false => SYNCBYPASS_A::ENABLE_TRIGGER_SYNCH,
            true => SYNCBYPASS_A::BYPASS_TRIGGER_SYNCH,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLE_TRIGGER_SYNCH`"]
    #[inline(always)]
    pub fn is_enable_trigger_synch(&self) -> bool {
        **self == SYNCBYPASS_A::ENABLE_TRIGGER_SYNCH
    }
    #[doc = "Checks if the value of the field is `BYPASS_TRIGGER_SYNCH`"]
    #[inline(always)]
    pub fn is_bypass_trigger_synch(&self) -> bool {
        **self == SYNCBYPASS_A::BYPASS_TRIGGER_SYNCH
    }
}
impl core::ops::Deref for SYNCBYPASS_R {
    type Target = crate::FieldReader<bool, SYNCBYPASS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SYNCBYPASS` writer - Setting this bit allows the hardware trigger input to bypass synchronization flip-flop stages and therefore shorten the time between the trigger input signal and the start of a conversion. There are slightly different criteria for whether or not this bit can be set depending on the clock operating mode: Synchronous mode (the ASYNMODE in the CTRL register = 0): Synchronization may be bypassed (this bit may be set) if the selected trigger source is already synchronous with the main system clock (eg. coming from an on-chip, system-clock-based timer). Whether this bit is set or not, a trigger pulse must be maintained for at least one system clock period. Asynchronous mode (the ASYNMODE in the CTRL register = 1): Synchronization may be bypassed (this bit may be set) if it is certain that the duration of a trigger input pulse will be at least one cycle of the ADC clock (regardless of whether the trigger comes from and on-chip or off-chip source). If this bit is NOT set, the trigger pulse must at least be maintained for one system clock period."]
pub struct SYNCBYPASS_W<'a> {
    w: &'a mut W,
}
impl<'a> SYNCBYPASS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SYNCBYPASS_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Enable trigger synchronization. The hardware trigger bypass is not enabled."]
    #[inline(always)]
    pub fn enable_trigger_synch(self) -> &'a mut W {
        self.variant(SYNCBYPASS_A::ENABLE_TRIGGER_SYNCH)
    }
    #[doc = "Bypass trigger synchronization. The hardware trigger bypass is enabled."]
    #[inline(always)]
    pub fn bypass_trigger_synch(self) -> &'a mut W {
        self.variant(SYNCBYPASS_A::BYPASS_TRIGGER_SYNCH)
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
        self.w.bits = (self.w.bits & !(0x01 << 19)) | ((value as u32 & 0x01) << 19);
        self.w
    }
}
#[doc = "Field `START` reader - Writing a 1 to this field will launch one pass through this conversion sequence. The behavior will be identical to a sequence triggered by a hardware trigger. Do not write 1 to this bit if the BURST bit is set. This bit is only set to a 1 momentarily when written to launch a conversion sequence. It will consequently always read back as a zero."]
pub struct START_R(crate::FieldReader<bool, bool>);
impl START_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        START_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for START_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `START` writer - Writing a 1 to this field will launch one pass through this conversion sequence. The behavior will be identical to a sequence triggered by a hardware trigger. Do not write 1 to this bit if the BURST bit is set. This bit is only set to a 1 momentarily when written to launch a conversion sequence. It will consequently always read back as a zero."]
pub struct START_W<'a> {
    w: &'a mut W,
}
impl<'a> START_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 26)) | ((value as u32 & 0x01) << 26);
        self.w
    }
}
#[doc = "Field `BURST` reader - Writing a 1 to this bit will cause this conversion sequence to be continuously cycled through. Other sequence A triggers will be ignored while this bit is set. Repeated conversions can be halted by clearing this bit. The sequence currently in progress will be completed before conversions are terminated. Note that a new sequence could begin just before BURST is cleared."]
pub struct BURST_R(crate::FieldReader<bool, bool>);
impl BURST_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        BURST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BURST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BURST` writer - Writing a 1 to this bit will cause this conversion sequence to be continuously cycled through. Other sequence A triggers will be ignored while this bit is set. Repeated conversions can be halted by clearing this bit. The sequence currently in progress will be completed before conversions are terminated. Note that a new sequence could begin just before BURST is cleared."]
pub struct BURST_W<'a> {
    w: &'a mut W,
}
impl<'a> BURST_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 27)) | ((value as u32 & 0x01) << 27);
        self.w
    }
}
#[doc = "Field `SINGLESTEP` reader - When this bit is set, a hardware trigger or a write to the START bit will launch a single conversion on the next channel in the sequence instead of the default response of launching an entire sequence of conversions. Once all of the channels comprising a sequence have been converted, a subsequent trigger will repeat the sequence beginning with the first enabled channel. Interrupt generation will still occur either after each individual conversion or at the end of the entire sequence, depending on the state of the MODE bit."]
pub struct SINGLESTEP_R(crate::FieldReader<bool, bool>);
impl SINGLESTEP_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SINGLESTEP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SINGLESTEP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SINGLESTEP` writer - When this bit is set, a hardware trigger or a write to the START bit will launch a single conversion on the next channel in the sequence instead of the default response of launching an entire sequence of conversions. Once all of the channels comprising a sequence have been converted, a subsequent trigger will repeat the sequence beginning with the first enabled channel. Interrupt generation will still occur either after each individual conversion or at the end of the entire sequence, depending on the state of the MODE bit."]
pub struct SINGLESTEP_W<'a> {
    w: &'a mut W,
}
impl<'a> SINGLESTEP_W<'a> {
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
#[doc = "Set priority for sequence A.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LOWPRIO_A {
    #[doc = "0: Low priority. Any B trigger which occurs while an A conversion sequence is active will be ignored and lost."]
    LOW_PRIORITY = 0,
    #[doc = "1: High priority. Setting this bit to a 1 will permit any enabled B sequence trigger (including a B sequence software start) to immediately interrupt sequence A and launch a B sequence in it's place. The conversion currently in progress will be terminated. The A sequence that was interrupted will automatically resume after the B sequence completes. The channel whose conversion was terminated will be re-sampled and the conversion sequence will resume from that point."]
    HIGH_PRIORITY = 1,
}
impl From<LOWPRIO_A> for bool {
    #[inline(always)]
    fn from(variant: LOWPRIO_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LOWPRIO` reader - Set priority for sequence A."]
pub struct LOWPRIO_R(crate::FieldReader<bool, LOWPRIO_A>);
impl LOWPRIO_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        LOWPRIO_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LOWPRIO_A {
        match self.bits {
            false => LOWPRIO_A::LOW_PRIORITY,
            true => LOWPRIO_A::HIGH_PRIORITY,
        }
    }
    #[doc = "Checks if the value of the field is `LOW_PRIORITY`"]
    #[inline(always)]
    pub fn is_low_priority(&self) -> bool {
        **self == LOWPRIO_A::LOW_PRIORITY
    }
    #[doc = "Checks if the value of the field is `HIGH_PRIORITY`"]
    #[inline(always)]
    pub fn is_high_priority(&self) -> bool {
        **self == LOWPRIO_A::HIGH_PRIORITY
    }
}
impl core::ops::Deref for LOWPRIO_R {
    type Target = crate::FieldReader<bool, LOWPRIO_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LOWPRIO` writer - Set priority for sequence A."]
pub struct LOWPRIO_W<'a> {
    w: &'a mut W,
}
impl<'a> LOWPRIO_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LOWPRIO_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Low priority. Any B trigger which occurs while an A conversion sequence is active will be ignored and lost."]
    #[inline(always)]
    pub fn low_priority(self) -> &'a mut W {
        self.variant(LOWPRIO_A::LOW_PRIORITY)
    }
    #[doc = "High priority. Setting this bit to a 1 will permit any enabled B sequence trigger (including a B sequence software start) to immediately interrupt sequence A and launch a B sequence in it's place. The conversion currently in progress will be terminated. The A sequence that was interrupted will automatically resume after the B sequence completes. The channel whose conversion was terminated will be re-sampled and the conversion sequence will resume from that point."]
    #[inline(always)]
    pub fn high_priority(self) -> &'a mut W {
        self.variant(LOWPRIO_A::HIGH_PRIORITY)
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
        self.w.bits = (self.w.bits & !(0x01 << 29)) | ((value as u32 & 0x01) << 29);
        self.w
    }
}
#[doc = "Indicates whether the primary method for retrieving conversion results for this sequence will be accomplished via reading the global data register (SEQA_GDAT) at the end of each conversion, or the individual channel result registers at the end of the entire sequence. Impacts when conversion-complete interrupt/DMA trigger for sequence-A will be generated and which overrun conditions contribute to an overrun interrupt as described below.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MODE_A {
    #[doc = "0: End of conversion. The sequence A interrupt/DMA trigger will be set at the end of each individual ADC conversion performed under sequence A. This flag will mirror the DATAVALID bit in the SEQA_GDAT register. The OVERRUN bit in the SEQA_GDAT register will contribute to generation of an overrun interrupt/DMA trigger if enabled."]
    END_OF_CONVERSION = 0,
    #[doc = "1: End of sequence. The sequence A interrupt/DMA trigger will be set when the entire set of sequence-A conversions completes. This flag will need to be explicitly cleared by software or by the DMA-clear signal in this mode. The OVERRUN bit in the SEQA_GDAT register will NOT contribute to generation of an overrun interrupt/DMA trigger since it is assumed this register may not be utilized in this mode."]
    END_OF_SEQUENCE = 1,
}
impl From<MODE_A> for bool {
    #[inline(always)]
    fn from(variant: MODE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MODE` reader - Indicates whether the primary method for retrieving conversion results for this sequence will be accomplished via reading the global data register (SEQA_GDAT) at the end of each conversion, or the individual channel result registers at the end of the entire sequence. Impacts when conversion-complete interrupt/DMA trigger for sequence-A will be generated and which overrun conditions contribute to an overrun interrupt as described below."]
pub struct MODE_R(crate::FieldReader<bool, MODE_A>);
impl MODE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        MODE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MODE_A {
        match self.bits {
            false => MODE_A::END_OF_CONVERSION,
            true => MODE_A::END_OF_SEQUENCE,
        }
    }
    #[doc = "Checks if the value of the field is `END_OF_CONVERSION`"]
    #[inline(always)]
    pub fn is_end_of_conversion(&self) -> bool {
        **self == MODE_A::END_OF_CONVERSION
    }
    #[doc = "Checks if the value of the field is `END_OF_SEQUENCE`"]
    #[inline(always)]
    pub fn is_end_of_sequence(&self) -> bool {
        **self == MODE_A::END_OF_SEQUENCE
    }
}
impl core::ops::Deref for MODE_R {
    type Target = crate::FieldReader<bool, MODE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MODE` writer - Indicates whether the primary method for retrieving conversion results for this sequence will be accomplished via reading the global data register (SEQA_GDAT) at the end of each conversion, or the individual channel result registers at the end of the entire sequence. Impacts when conversion-complete interrupt/DMA trigger for sequence-A will be generated and which overrun conditions contribute to an overrun interrupt as described below."]
pub struct MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> MODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MODE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "End of conversion. The sequence A interrupt/DMA trigger will be set at the end of each individual ADC conversion performed under sequence A. This flag will mirror the DATAVALID bit in the SEQA_GDAT register. The OVERRUN bit in the SEQA_GDAT register will contribute to generation of an overrun interrupt/DMA trigger if enabled."]
    #[inline(always)]
    pub fn end_of_conversion(self) -> &'a mut W {
        self.variant(MODE_A::END_OF_CONVERSION)
    }
    #[doc = "End of sequence. The sequence A interrupt/DMA trigger will be set when the entire set of sequence-A conversions completes. This flag will need to be explicitly cleared by software or by the DMA-clear signal in this mode. The OVERRUN bit in the SEQA_GDAT register will NOT contribute to generation of an overrun interrupt/DMA trigger since it is assumed this register may not be utilized in this mode."]
    #[inline(always)]
    pub fn end_of_sequence(self) -> &'a mut W {
        self.variant(MODE_A::END_OF_SEQUENCE)
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
        self.w.bits = (self.w.bits & !(0x01 << 30)) | ((value as u32 & 0x01) << 30);
        self.w
    }
}
#[doc = "Sequence Enable. In order to avoid spuriously triggering the sequence, care should be taken to only set the SEQn_ENA bit when the selected trigger input is in its INACTIVE state (as defined by the TRIGPOL bit). If this condition is not met, the sequence will be triggered immediately upon being enabled. In order to avoid spuriously triggering the sequence, care should be taken to only set the SEQn_ENA bit when the selected trigger input is in its INACTIVE state (as defined by the TRIGPOL bit). If this condition is not met, the sequence will be triggered immediately upon being enabled.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SEQ_ENA_A {
    #[doc = "0: Disabled. Sequence n is disabled. Sequence n triggers are ignored. If this bit is cleared while sequence n is in progress, the sequence will be halted at the end of the current conversion. After the sequence is re-enabled, a new trigger will be required to restart the sequence beginning with the next enabled channel."]
    DISABLED = 0,
    #[doc = "1: Enabled. Sequence n is enabled."]
    ENABLED = 1,
}
impl From<SEQ_ENA_A> for bool {
    #[inline(always)]
    fn from(variant: SEQ_ENA_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SEQ_ENA` reader - Sequence Enable. In order to avoid spuriously triggering the sequence, care should be taken to only set the SEQn_ENA bit when the selected trigger input is in its INACTIVE state (as defined by the TRIGPOL bit). If this condition is not met, the sequence will be triggered immediately upon being enabled. In order to avoid spuriously triggering the sequence, care should be taken to only set the SEQn_ENA bit when the selected trigger input is in its INACTIVE state (as defined by the TRIGPOL bit). If this condition is not met, the sequence will be triggered immediately upon being enabled."]
pub struct SEQ_ENA_R(crate::FieldReader<bool, SEQ_ENA_A>);
impl SEQ_ENA_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SEQ_ENA_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SEQ_ENA_A {
        match self.bits {
            false => SEQ_ENA_A::DISABLED,
            true => SEQ_ENA_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == SEQ_ENA_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == SEQ_ENA_A::ENABLED
    }
}
impl core::ops::Deref for SEQ_ENA_R {
    type Target = crate::FieldReader<bool, SEQ_ENA_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SEQ_ENA` writer - Sequence Enable. In order to avoid spuriously triggering the sequence, care should be taken to only set the SEQn_ENA bit when the selected trigger input is in its INACTIVE state (as defined by the TRIGPOL bit). If this condition is not met, the sequence will be triggered immediately upon being enabled. In order to avoid spuriously triggering the sequence, care should be taken to only set the SEQn_ENA bit when the selected trigger input is in its INACTIVE state (as defined by the TRIGPOL bit). If this condition is not met, the sequence will be triggered immediately upon being enabled."]
pub struct SEQ_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> SEQ_ENA_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SEQ_ENA_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disabled. Sequence n is disabled. Sequence n triggers are ignored. If this bit is cleared while sequence n is in progress, the sequence will be halted at the end of the current conversion. After the sequence is re-enabled, a new trigger will be required to restart the sequence beginning with the next enabled channel."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(SEQ_ENA_A::DISABLED)
    }
    #[doc = "Enabled. Sequence n is enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(SEQ_ENA_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | ((value as u32 & 0x01) << 31);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:11 - Selects which one or more of the ADC channels will be sampled and converted when this sequence is launched. A 1 in any bit of this field will cause the corresponding channel to be included in the conversion sequence, where bit 0 corresponds to channel 0, bit 1 to channel 1 and so forth. When this conversion sequence is triggered, either by a hardware trigger or via software command, ADC conversions will be performed on each enabled channel, in sequence, beginning with the lowest-ordered channel. This field can ONLY be changed while SEQA_ENA (bit 31) is LOW. It is allowed to change this field and set bit 31 in the same write."]
    #[inline(always)]
    pub fn channels(&self) -> CHANNELS_R {
        CHANNELS_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 12:17 - Selects which of the available hardware trigger sources will cause this conversion sequence to be initiated. Program the trigger input number in this field. See Table 476. In order to avoid generating a spurious trigger, it is recommended writing to this field only when SEQA_ENA (bit 31) is low. It is safe to change this field and set bit 31 in the same write."]
    #[inline(always)]
    pub fn trigger(&self) -> TRIGGER_R {
        TRIGGER_R::new(((self.bits >> 12) & 0x3f) as u8)
    }
    #[doc = "Bit 18 - Select the polarity of the selected input trigger for this conversion sequence. In order to avoid generating a spurious trigger, it is recommended writing to this field only when SEQA_ENA (bit 31) is low. It is safe to change this field and set bit 31 in the same write."]
    #[inline(always)]
    pub fn trigpol(&self) -> TRIGPOL_R {
        TRIGPOL_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Setting this bit allows the hardware trigger input to bypass synchronization flip-flop stages and therefore shorten the time between the trigger input signal and the start of a conversion. There are slightly different criteria for whether or not this bit can be set depending on the clock operating mode: Synchronous mode (the ASYNMODE in the CTRL register = 0): Synchronization may be bypassed (this bit may be set) if the selected trigger source is already synchronous with the main system clock (eg. coming from an on-chip, system-clock-based timer). Whether this bit is set or not, a trigger pulse must be maintained for at least one system clock period. Asynchronous mode (the ASYNMODE in the CTRL register = 1): Synchronization may be bypassed (this bit may be set) if it is certain that the duration of a trigger input pulse will be at least one cycle of the ADC clock (regardless of whether the trigger comes from and on-chip or off-chip source). If this bit is NOT set, the trigger pulse must at least be maintained for one system clock period."]
    #[inline(always)]
    pub fn syncbypass(&self) -> SYNCBYPASS_R {
        SYNCBYPASS_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 26 - Writing a 1 to this field will launch one pass through this conversion sequence. The behavior will be identical to a sequence triggered by a hardware trigger. Do not write 1 to this bit if the BURST bit is set. This bit is only set to a 1 momentarily when written to launch a conversion sequence. It will consequently always read back as a zero."]
    #[inline(always)]
    pub fn start(&self) -> START_R {
        START_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - Writing a 1 to this bit will cause this conversion sequence to be continuously cycled through. Other sequence A triggers will be ignored while this bit is set. Repeated conversions can be halted by clearing this bit. The sequence currently in progress will be completed before conversions are terminated. Note that a new sequence could begin just before BURST is cleared."]
    #[inline(always)]
    pub fn burst(&self) -> BURST_R {
        BURST_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - When this bit is set, a hardware trigger or a write to the START bit will launch a single conversion on the next channel in the sequence instead of the default response of launching an entire sequence of conversions. Once all of the channels comprising a sequence have been converted, a subsequent trigger will repeat the sequence beginning with the first enabled channel. Interrupt generation will still occur either after each individual conversion or at the end of the entire sequence, depending on the state of the MODE bit."]
    #[inline(always)]
    pub fn singlestep(&self) -> SINGLESTEP_R {
        SINGLESTEP_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - Set priority for sequence A."]
    #[inline(always)]
    pub fn lowprio(&self) -> LOWPRIO_R {
        LOWPRIO_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - Indicates whether the primary method for retrieving conversion results for this sequence will be accomplished via reading the global data register (SEQA_GDAT) at the end of each conversion, or the individual channel result registers at the end of the entire sequence. Impacts when conversion-complete interrupt/DMA trigger for sequence-A will be generated and which overrun conditions contribute to an overrun interrupt as described below."]
    #[inline(always)]
    pub fn mode(&self) -> MODE_R {
        MODE_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - Sequence Enable. In order to avoid spuriously triggering the sequence, care should be taken to only set the SEQn_ENA bit when the selected trigger input is in its INACTIVE state (as defined by the TRIGPOL bit). If this condition is not met, the sequence will be triggered immediately upon being enabled. In order to avoid spuriously triggering the sequence, care should be taken to only set the SEQn_ENA bit when the selected trigger input is in its INACTIVE state (as defined by the TRIGPOL bit). If this condition is not met, the sequence will be triggered immediately upon being enabled."]
    #[inline(always)]
    pub fn seq_ena(&self) -> SEQ_ENA_R {
        SEQ_ENA_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:11 - Selects which one or more of the ADC channels will be sampled and converted when this sequence is launched. A 1 in any bit of this field will cause the corresponding channel to be included in the conversion sequence, where bit 0 corresponds to channel 0, bit 1 to channel 1 and so forth. When this conversion sequence is triggered, either by a hardware trigger or via software command, ADC conversions will be performed on each enabled channel, in sequence, beginning with the lowest-ordered channel. This field can ONLY be changed while SEQA_ENA (bit 31) is LOW. It is allowed to change this field and set bit 31 in the same write."]
    #[inline(always)]
    pub fn channels(&mut self) -> CHANNELS_W {
        CHANNELS_W { w: self }
    }
    #[doc = "Bits 12:17 - Selects which of the available hardware trigger sources will cause this conversion sequence to be initiated. Program the trigger input number in this field. See Table 476. In order to avoid generating a spurious trigger, it is recommended writing to this field only when SEQA_ENA (bit 31) is low. It is safe to change this field and set bit 31 in the same write."]
    #[inline(always)]
    pub fn trigger(&mut self) -> TRIGGER_W {
        TRIGGER_W { w: self }
    }
    #[doc = "Bit 18 - Select the polarity of the selected input trigger for this conversion sequence. In order to avoid generating a spurious trigger, it is recommended writing to this field only when SEQA_ENA (bit 31) is low. It is safe to change this field and set bit 31 in the same write."]
    #[inline(always)]
    pub fn trigpol(&mut self) -> TRIGPOL_W {
        TRIGPOL_W { w: self }
    }
    #[doc = "Bit 19 - Setting this bit allows the hardware trigger input to bypass synchronization flip-flop stages and therefore shorten the time between the trigger input signal and the start of a conversion. There are slightly different criteria for whether or not this bit can be set depending on the clock operating mode: Synchronous mode (the ASYNMODE in the CTRL register = 0): Synchronization may be bypassed (this bit may be set) if the selected trigger source is already synchronous with the main system clock (eg. coming from an on-chip, system-clock-based timer). Whether this bit is set or not, a trigger pulse must be maintained for at least one system clock period. Asynchronous mode (the ASYNMODE in the CTRL register = 1): Synchronization may be bypassed (this bit may be set) if it is certain that the duration of a trigger input pulse will be at least one cycle of the ADC clock (regardless of whether the trigger comes from and on-chip or off-chip source). If this bit is NOT set, the trigger pulse must at least be maintained for one system clock period."]
    #[inline(always)]
    pub fn syncbypass(&mut self) -> SYNCBYPASS_W {
        SYNCBYPASS_W { w: self }
    }
    #[doc = "Bit 26 - Writing a 1 to this field will launch one pass through this conversion sequence. The behavior will be identical to a sequence triggered by a hardware trigger. Do not write 1 to this bit if the BURST bit is set. This bit is only set to a 1 momentarily when written to launch a conversion sequence. It will consequently always read back as a zero."]
    #[inline(always)]
    pub fn start(&mut self) -> START_W {
        START_W { w: self }
    }
    #[doc = "Bit 27 - Writing a 1 to this bit will cause this conversion sequence to be continuously cycled through. Other sequence A triggers will be ignored while this bit is set. Repeated conversions can be halted by clearing this bit. The sequence currently in progress will be completed before conversions are terminated. Note that a new sequence could begin just before BURST is cleared."]
    #[inline(always)]
    pub fn burst(&mut self) -> BURST_W {
        BURST_W { w: self }
    }
    #[doc = "Bit 28 - When this bit is set, a hardware trigger or a write to the START bit will launch a single conversion on the next channel in the sequence instead of the default response of launching an entire sequence of conversions. Once all of the channels comprising a sequence have been converted, a subsequent trigger will repeat the sequence beginning with the first enabled channel. Interrupt generation will still occur either after each individual conversion or at the end of the entire sequence, depending on the state of the MODE bit."]
    #[inline(always)]
    pub fn singlestep(&mut self) -> SINGLESTEP_W {
        SINGLESTEP_W { w: self }
    }
    #[doc = "Bit 29 - Set priority for sequence A."]
    #[inline(always)]
    pub fn lowprio(&mut self) -> LOWPRIO_W {
        LOWPRIO_W { w: self }
    }
    #[doc = "Bit 30 - Indicates whether the primary method for retrieving conversion results for this sequence will be accomplished via reading the global data register (SEQA_GDAT) at the end of each conversion, or the individual channel result registers at the end of the entire sequence. Impacts when conversion-complete interrupt/DMA trigger for sequence-A will be generated and which overrun conditions contribute to an overrun interrupt as described below."]
    #[inline(always)]
    pub fn mode(&mut self) -> MODE_W {
        MODE_W { w: self }
    }
    #[doc = "Bit 31 - Sequence Enable. In order to avoid spuriously triggering the sequence, care should be taken to only set the SEQn_ENA bit when the selected trigger input is in its INACTIVE state (as defined by the TRIGPOL bit). If this condition is not met, the sequence will be triggered immediately upon being enabled. In order to avoid spuriously triggering the sequence, care should be taken to only set the SEQn_ENA bit when the selected trigger input is in its INACTIVE state (as defined by the TRIGPOL bit). If this condition is not met, the sequence will be triggered immediately upon being enabled."]
    #[inline(always)]
    pub fn seq_ena(&mut self) -> SEQ_ENA_W {
        SEQ_ENA_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ADC Conversion Sequence-n control register: Controls triggering and channel selection for conversion sequence-n. Also specifies interrupt mode for sequence-n.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [seq_ctrl](index.html) module"]
pub struct SEQ_CTRL_SPEC;
impl crate::RegisterSpec for SEQ_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [seq_ctrl::R](R) reader structure"]
impl crate::Readable for SEQ_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [seq_ctrl::W](W) writer structure"]
impl crate::Writable for SEQ_CTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SEQ_CTRL%s to value 0"]
impl crate::Resettable for SEQ_CTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
