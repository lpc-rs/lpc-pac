#[doc = "Reader of register CCER"]
pub type R = crate::R<u32, super::CCER>;
#[doc = "Reader of field `ExtendedExternalInputSelectors`"]
pub type EXTENDEDEXTERNALINPUTSELECTORS_R = crate::R<u8, u8>;
#[doc = "Reader of field `ExtendedExternalInputBus`"]
pub type EXTENDEDEXTERNALINPUTBUS_R = crate::R<u8, u8>;
#[doc = "Reader of field `ReadableRegisters`"]
pub type READABLEREGISTERS_R = crate::R<bool, bool>;
#[doc = "Reader of field `DataAddressComparisons`"]
pub type DATAADDRESSCOMPARISONS_R = crate::R<bool, bool>;
#[doc = "Reader of field `InstrumentationResources`"]
pub type INSTRUMENTATIONRESOURCES_R = crate::R<u8, u8>;
#[doc = "Reader of field `EmbeddedICEwatchpointInputs`"]
pub type EMBEDDEDICEWATCHPOINTINPUTS_R = crate::R<u8, u8>;
#[doc = "Reader of field `TraceStartStopBlockUsesEmbeddedICEwatchpointInputs`"]
pub type TRACESTARTSTOPBLOCKUSESEMBEDDEDICEWATCHPOINTINPUTS_R = crate::R<bool, bool>;
#[doc = "Reader of field `EmbeddedICEbehaviorControlImplemented`"]
pub type EMBEDDEDICEBEHAVIORCONTROLIMPLEMENTED_R = crate::R<bool, bool>;
#[doc = "Reader of field `TimestampingImplemented`"]
pub type TIMESTAMPINGIMPLEMENTED_R = crate::R<bool, bool>;
#[doc = "Reader of field `ReducedFunctionCounter`"]
pub type REDUCEDFUNCTIONCOUNTER_R = crate::R<bool, bool>;
#[doc = "Reader of field `TimestampEncoding`"]
pub type TIMESTAMPENCODING_R = crate::R<bool, bool>;
#[doc = "Reader of field `TimestampSize`"]
pub type TIMESTAMPSIZE_R = crate::R<bool, bool>;
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
