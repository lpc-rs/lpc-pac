#[doc = "Register `CCER` reader"]
pub struct R(crate::R<CCER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CCER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<CCER_SPEC>> for R {
    fn from(reader: crate::R<CCER_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `ExtendedExternalInputSelectors` reader - Extended external input selectors. The value of these bits is 0, indicating that extended external input selectors are not implemented."]
pub struct EXTENDEDEXTERNALINPUTSELECTORS_R(crate::FieldReader<u8, u8>);
impl EXTENDEDEXTERNALINPUTSELECTORS_R {
    pub(crate) fn new(bits: u8) -> Self {
        EXTENDEDEXTERNALINPUTSELECTORS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EXTENDEDEXTERNALINPUTSELECTORS_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ExtendedExternalInputBus` reader - Extended external input bus. The value of these bits is 0, indicating that the extended external input bus is not implemented."]
pub struct EXTENDEDEXTERNALINPUTBUS_R(crate::FieldReader<u8, u8>);
impl EXTENDEDEXTERNALINPUTBUS_R {
    pub(crate) fn new(bits: u8) -> Self {
        EXTENDEDEXTERNALINPUTBUS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EXTENDEDEXTERNALINPUTBUS_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ReadableRegisters` reader - Readable registers. The value of this bit is 1, indicating that all registers are readable."]
pub struct READABLEREGISTERS_R(crate::FieldReader<bool, bool>);
impl READABLEREGISTERS_R {
    pub(crate) fn new(bits: bool) -> Self {
        READABLEREGISTERS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for READABLEREGISTERS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DataAddressComparisons` reader - Data address comparisons. The value of this bit is 1, indicating that data address comparisons are not supported."]
pub struct DATAADDRESSCOMPARISONS_R(crate::FieldReader<bool, bool>);
impl DATAADDRESSCOMPARISONS_R {
    pub(crate) fn new(bits: bool) -> Self {
        DATAADDRESSCOMPARISONS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DATAADDRESSCOMPARISONS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `InstrumentationResources` reader - Instrumentation resources. The value of these bits is 0b000, indicating that no Instrumentation resources are supported."]
pub struct INSTRUMENTATIONRESOURCES_R(crate::FieldReader<u8, u8>);
impl INSTRUMENTATIONRESOURCES_R {
    pub(crate) fn new(bits: u8) -> Self {
        INSTRUMENTATIONRESOURCES_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INSTRUMENTATIONRESOURCES_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EmbeddedICEwatchpointInputs` reader - EmbeddedICE watchpoint inputs. The value of these bits is 0b0100, indicating that the number of EmbeddedICE watchpoint inputs implemented is four. These inputs come from the DWT."]
pub struct EMBEDDEDICEWATCHPOINTINPUTS_R(crate::FieldReader<u8, u8>);
impl EMBEDDEDICEWATCHPOINTINPUTS_R {
    pub(crate) fn new(bits: u8) -> Self {
        EMBEDDEDICEWATCHPOINTINPUTS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EMBEDDEDICEWATCHPOINTINPUTS_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TraceStartStopBlockUsesEmbeddedICEwatchpointInputs` reader - Trace Start/Stop block uses EmbeddedICE watchpoint inputs. The value of this bit is 1, indicating that the Trace Start/Stop block uses the EmbeddedICE watchpoint inputs."]
pub struct TRACESTARTSTOPBLOCKUSESEMBEDDEDICEWATCHPOINTINPUTS_R(crate::FieldReader<bool, bool>);
impl TRACESTARTSTOPBLOCKUSESEMBEDDEDICEWATCHPOINTINPUTS_R {
    pub(crate) fn new(bits: bool) -> Self {
        TRACESTARTSTOPBLOCKUSESEMBEDDEDICEWATCHPOINTINPUTS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TRACESTARTSTOPBLOCKUSESEMBEDDEDICEWATCHPOINTINPUTS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EmbeddedICEbehaviorControlImplemented` reader - EmbeddedICE behavior control implemented. The value of this bit is 0, indicating that the ETMEIBCR is not implemented."]
pub struct EMBEDDEDICEBEHAVIORCONTROLIMPLEMENTED_R(crate::FieldReader<bool, bool>);
impl EMBEDDEDICEBEHAVIORCONTROLIMPLEMENTED_R {
    pub(crate) fn new(bits: bool) -> Self {
        EMBEDDEDICEBEHAVIORCONTROLIMPLEMENTED_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EMBEDDEDICEBEHAVIORCONTROLIMPLEMENTED_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TimestampingImplemented` reader - Timestamping implemented. This bit is set to 1, indicating that timestamping is implemented."]
pub struct TIMESTAMPINGIMPLEMENTED_R(crate::FieldReader<bool, bool>);
impl TIMESTAMPINGIMPLEMENTED_R {
    pub(crate) fn new(bits: bool) -> Self {
        TIMESTAMPINGIMPLEMENTED_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TIMESTAMPINGIMPLEMENTED_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ReducedFunctionCounter` reader - Reduced function counter. Set to 1 to indicate that Counter 1 is a reduced function counter."]
pub struct REDUCEDFUNCTIONCOUNTER_R(crate::FieldReader<bool, bool>);
impl REDUCEDFUNCTIONCOUNTER_R {
    pub(crate) fn new(bits: bool) -> Self {
        REDUCEDFUNCTIONCOUNTER_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for REDUCEDFUNCTIONCOUNTER_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TimestampEncoding` reader - Timestamp encoding. Set to 1 to indicate that the timestamp is encoded as a natural binary number."]
pub struct TIMESTAMPENCODING_R(crate::FieldReader<bool, bool>);
impl TIMESTAMPENCODING_R {
    pub(crate) fn new(bits: bool) -> Self {
        TIMESTAMPENCODING_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TIMESTAMPENCODING_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TimestampSize` reader - Timestamp size. Set to 0 to indicate a size of 48 bits."]
pub struct TIMESTAMPSIZE_R(crate::FieldReader<bool, bool>);
impl TIMESTAMPSIZE_R {
    pub(crate) fn new(bits: bool) -> Self {
        TIMESTAMPSIZE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TIMESTAMPSIZE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:2 - Extended external input selectors. The value of these bits is 0, indicating that extended external input selectors are not implemented."]
    #[inline(always)]
    pub fn extended_external_input_selectors(&self) -> EXTENDEDEXTERNALINPUTSELECTORS_R {
        EXTENDEDEXTERNALINPUTSELECTORS_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bits 3:10 - Extended external input bus. The value of these bits is 0, indicating that the extended external input bus is not implemented."]
    #[inline(always)]
    pub fn extended_external_input_bus(&self) -> EXTENDEDEXTERNALINPUTBUS_R {
        EXTENDEDEXTERNALINPUTBUS_R::new(((self.bits >> 3) & 0xff) as u8)
    }
    #[doc = "Bit 11 - Readable registers. The value of this bit is 1, indicating that all registers are readable."]
    #[inline(always)]
    pub fn readable_registers(&self) -> READABLEREGISTERS_R {
        READABLEREGISTERS_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Data address comparisons. The value of this bit is 1, indicating that data address comparisons are not supported."]
    #[inline(always)]
    pub fn data_address_comparisons(&self) -> DATAADDRESSCOMPARISONS_R {
        DATAADDRESSCOMPARISONS_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bits 13:15 - Instrumentation resources. The value of these bits is 0b000, indicating that no Instrumentation resources are supported."]
    #[inline(always)]
    pub fn instrumentation_resources(&self) -> INSTRUMENTATIONRESOURCES_R {
        INSTRUMENTATIONRESOURCES_R::new(((self.bits >> 13) & 0x07) as u8)
    }
    #[doc = "Bits 16:19 - EmbeddedICE watchpoint inputs. The value of these bits is 0b0100, indicating that the number of EmbeddedICE watchpoint inputs implemented is four. These inputs come from the DWT."]
    #[inline(always)]
    pub fn embedded_icewatchpoint_inputs(&self) -> EMBEDDEDICEWATCHPOINTINPUTS_R {
        EMBEDDEDICEWATCHPOINTINPUTS_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bit 20 - Trace Start/Stop block uses EmbeddedICE watchpoint inputs. The value of this bit is 1, indicating that the Trace Start/Stop block uses the EmbeddedICE watchpoint inputs."]
    #[inline(always)]
    pub fn trace_start_stop_block_uses_embedded_icewatchpoint_inputs(
        &self,
    ) -> TRACESTARTSTOPBLOCKUSESEMBEDDEDICEWATCHPOINTINPUTS_R {
        TRACESTARTSTOPBLOCKUSESEMBEDDEDICEWATCHPOINTINPUTS_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - EmbeddedICE behavior control implemented. The value of this bit is 0, indicating that the ETMEIBCR is not implemented."]
    #[inline(always)]
    pub fn embedded_icebehavior_control_implemented(
        &self,
    ) -> EMBEDDEDICEBEHAVIORCONTROLIMPLEMENTED_R {
        EMBEDDEDICEBEHAVIORCONTROLIMPLEMENTED_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - Timestamping implemented. This bit is set to 1, indicating that timestamping is implemented."]
    #[inline(always)]
    pub fn timestamping_implemented(&self) -> TIMESTAMPINGIMPLEMENTED_R {
        TIMESTAMPINGIMPLEMENTED_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 27 - Reduced function counter. Set to 1 to indicate that Counter 1 is a reduced function counter."]
    #[inline(always)]
    pub fn reduced_function_counter(&self) -> REDUCEDFUNCTIONCOUNTER_R {
        REDUCEDFUNCTIONCOUNTER_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - Timestamp encoding. Set to 1 to indicate that the timestamp is encoded as a natural binary number."]
    #[inline(always)]
    pub fn timestamp_encoding(&self) -> TIMESTAMPENCODING_R {
        TIMESTAMPENCODING_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - Timestamp size. Set to 0 to indicate a size of 48 bits."]
    #[inline(always)]
    pub fn timestamp_size(&self) -> TIMESTAMPSIZE_R {
        TIMESTAMPSIZE_R::new(((self.bits >> 29) & 0x01) != 0)
    }
}
#[doc = "Configuration Code Extension Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ccer](index.html) module"]
pub struct CCER_SPEC;
impl crate::RegisterSpec for CCER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ccer::R](R) reader structure"]
impl crate::Readable for CCER_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets CCER to value 0x1854_1800"]
impl crate::Resettable for CCER_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x1854_1800
    }
}
