#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - BCD representation of the version of the HCI specification that is implemented by the Host Controller (HC)"]
    pub hcrevision: HCREVISION,
    #[doc = "0x04 - Defines the operating modes of the HC"]
    pub hccontrol: HCCONTROL,
    #[doc = "0x08 - This register is used to receive the commands from the Host Controller Driver (HCD)"]
    pub hccommandstatus: HCCOMMANDSTATUS,
    #[doc = "0x0c - Indicates the status on various events that cause hardware interrupts by setting the appropriate bits"]
    pub hcinterruptstatus: HCINTERRUPTSTATUS,
    #[doc = "0x10 - Controls the bits in the HcInterruptStatus register and indicates which events will generate a hardware interrupt"]
    pub hcinterruptenable: HCINTERRUPTENABLE,
    #[doc = "0x14 - The bits in this register are used to disable corresponding bits in the HCInterruptStatus register and in turn disable that event leading to hardware interrupt"]
    pub hcinterruptdisable: HCINTERRUPTDISABLE,
    #[doc = "0x18 - Contains the physical address of the host controller communication area"]
    pub hchcca: HCHCCA,
    #[doc = "0x1c - Contains the physical address of the current isochronous or interrupt endpoint descriptor"]
    pub hcperiodcurrented: HCPERIODCURRENTED,
    #[doc = "0x20 - Contains the physical address of the first endpoint descriptor of the control list"]
    pub hccontrolheaded: HCCONTROLHEADED,
    #[doc = "0x24 - Contains the physical address of the current endpoint descriptor of the control list"]
    pub hccontrolcurrented: HCCONTROLCURRENTED,
    #[doc = "0x28 - Contains the physical address of the first endpoint descriptor of the bulk list"]
    pub hcbulkheaded: HCBULKHEADED,
    #[doc = "0x2c - Contains the physical address of the current endpoint descriptor of the bulk list"]
    pub hcbulkcurrented: HCBULKCURRENTED,
    #[doc = "0x30 - Contains the physical address of the last transfer descriptor added to the 'Done' queue"]
    pub hcdonehead: HCDONEHEAD,
    #[doc = "0x34 - Defines the bit time interval in a frame and the full speed maximum packet size which would not cause an overrun"]
    pub hcfminterval: HCFMINTERVAL,
    #[doc = "0x38 - A 14-bit counter showing the bit time remaining in the current frame"]
    pub hcfmremaining: HCFMREMAINING,
    #[doc = "0x3c - Contains a 16-bit counter and provides the timing reference among events happening in the HC and the HCD"]
    pub hcfmnumber: HCFMNUMBER,
    #[doc = "0x40 - Contains a programmable 14-bit value which determines the earliest time HC should start processing a periodic list"]
    pub hcperiodicstart: HCPERIODICSTART,
    #[doc = "0x44 - Contains 11-bit value which is used by the HC to determine whether to commit to transfer a maximum of 8-byte LS packet before EOF"]
    pub hclsthreshold: HCLSTHRESHOLD,
    #[doc = "0x48 - First of the two registers which describes the characteristics of the root hub"]
    pub hcrhdescriptora: HCRHDESCRIPTORA,
    #[doc = "0x4c - Second of the two registers which describes the characteristics of the Root Hub"]
    pub hcrhdescriptorb: HCRHDESCRIPTORB,
    #[doc = "0x50 - This register is divided into two parts"]
    pub hcrhstatus: HCRHSTATUS,
    #[doc = "0x54 - Controls and reports the port events on a per-port basis"]
    pub hcrhportstatus: HCRHPORTSTATUS,
    _reserved22: [u8; 4usize],
    #[doc = "0x5c - Controls the port if it is attached to the host block or the device block"]
    pub portmode: PORTMODE,
}
#[doc = "BCD representation of the version of the HCI specification that is implemented by the Host Controller (HC)\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hcrevision](hcrevision) module"]
pub type HCREVISION = crate::Reg<u32, _HCREVISION>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HCREVISION;
#[doc = "`read()` method returns [hcrevision::R](hcrevision::R) reader structure"]
impl crate::Readable for HCREVISION {}
#[doc = "BCD representation of the version of the HCI specification that is implemented by the Host Controller (HC)"]
pub mod hcrevision;
#[doc = "Defines the operating modes of the HC\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hccontrol](hccontrol) module"]
pub type HCCONTROL = crate::Reg<u32, _HCCONTROL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HCCONTROL;
#[doc = "`read()` method returns [hccontrol::R](hccontrol::R) reader structure"]
impl crate::Readable for HCCONTROL {}
#[doc = "`write(|w| ..)` method takes [hccontrol::W](hccontrol::W) writer structure"]
impl crate::Writable for HCCONTROL {}
#[doc = "Defines the operating modes of the HC"]
pub mod hccontrol;
#[doc = "This register is used to receive the commands from the Host Controller Driver (HCD)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hccommandstatus](hccommandstatus) module"]
pub type HCCOMMANDSTATUS = crate::Reg<u32, _HCCOMMANDSTATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HCCOMMANDSTATUS;
#[doc = "`read()` method returns [hccommandstatus::R](hccommandstatus::R) reader structure"]
impl crate::Readable for HCCOMMANDSTATUS {}
#[doc = "`write(|w| ..)` method takes [hccommandstatus::W](hccommandstatus::W) writer structure"]
impl crate::Writable for HCCOMMANDSTATUS {}
#[doc = "This register is used to receive the commands from the Host Controller Driver (HCD)"]
pub mod hccommandstatus;
#[doc = "Indicates the status on various events that cause hardware interrupts by setting the appropriate bits\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hcinterruptstatus](hcinterruptstatus) module"]
pub type HCINTERRUPTSTATUS = crate::Reg<u32, _HCINTERRUPTSTATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HCINTERRUPTSTATUS;
#[doc = "`read()` method returns [hcinterruptstatus::R](hcinterruptstatus::R) reader structure"]
impl crate::Readable for HCINTERRUPTSTATUS {}
#[doc = "`write(|w| ..)` method takes [hcinterruptstatus::W](hcinterruptstatus::W) writer structure"]
impl crate::Writable for HCINTERRUPTSTATUS {}
#[doc = "Indicates the status on various events that cause hardware interrupts by setting the appropriate bits"]
pub mod hcinterruptstatus;
#[doc = "Controls the bits in the HcInterruptStatus register and indicates which events will generate a hardware interrupt\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hcinterruptenable](hcinterruptenable) module"]
pub type HCINTERRUPTENABLE = crate::Reg<u32, _HCINTERRUPTENABLE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HCINTERRUPTENABLE;
#[doc = "`read()` method returns [hcinterruptenable::R](hcinterruptenable::R) reader structure"]
impl crate::Readable for HCINTERRUPTENABLE {}
#[doc = "`write(|w| ..)` method takes [hcinterruptenable::W](hcinterruptenable::W) writer structure"]
impl crate::Writable for HCINTERRUPTENABLE {}
#[doc = "Controls the bits in the HcInterruptStatus register and indicates which events will generate a hardware interrupt"]
pub mod hcinterruptenable;
#[doc = "The bits in this register are used to disable corresponding bits in the HCInterruptStatus register and in turn disable that event leading to hardware interrupt\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hcinterruptdisable](hcinterruptdisable) module"]
pub type HCINTERRUPTDISABLE = crate::Reg<u32, _HCINTERRUPTDISABLE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HCINTERRUPTDISABLE;
#[doc = "`read()` method returns [hcinterruptdisable::R](hcinterruptdisable::R) reader structure"]
impl crate::Readable for HCINTERRUPTDISABLE {}
#[doc = "`write(|w| ..)` method takes [hcinterruptdisable::W](hcinterruptdisable::W) writer structure"]
impl crate::Writable for HCINTERRUPTDISABLE {}
#[doc = "The bits in this register are used to disable corresponding bits in the HCInterruptStatus register and in turn disable that event leading to hardware interrupt"]
pub mod hcinterruptdisable;
#[doc = "Contains the physical address of the host controller communication area\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hchcca](hchcca) module"]
pub type HCHCCA = crate::Reg<u32, _HCHCCA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HCHCCA;
#[doc = "`read()` method returns [hchcca::R](hchcca::R) reader structure"]
impl crate::Readable for HCHCCA {}
#[doc = "`write(|w| ..)` method takes [hchcca::W](hchcca::W) writer structure"]
impl crate::Writable for HCHCCA {}
#[doc = "Contains the physical address of the host controller communication area"]
pub mod hchcca;
#[doc = "Contains the physical address of the current isochronous or interrupt endpoint descriptor\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hcperiodcurrented](hcperiodcurrented) module"]
pub type HCPERIODCURRENTED = crate::Reg<u32, _HCPERIODCURRENTED>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HCPERIODCURRENTED;
#[doc = "`read()` method returns [hcperiodcurrented::R](hcperiodcurrented::R) reader structure"]
impl crate::Readable for HCPERIODCURRENTED {}
#[doc = "`write(|w| ..)` method takes [hcperiodcurrented::W](hcperiodcurrented::W) writer structure"]
impl crate::Writable for HCPERIODCURRENTED {}
#[doc = "Contains the physical address of the current isochronous or interrupt endpoint descriptor"]
pub mod hcperiodcurrented;
#[doc = "Contains the physical address of the first endpoint descriptor of the control list\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hccontrolheaded](hccontrolheaded) module"]
pub type HCCONTROLHEADED = crate::Reg<u32, _HCCONTROLHEADED>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HCCONTROLHEADED;
#[doc = "`read()` method returns [hccontrolheaded::R](hccontrolheaded::R) reader structure"]
impl crate::Readable for HCCONTROLHEADED {}
#[doc = "`write(|w| ..)` method takes [hccontrolheaded::W](hccontrolheaded::W) writer structure"]
impl crate::Writable for HCCONTROLHEADED {}
#[doc = "Contains the physical address of the first endpoint descriptor of the control list"]
pub mod hccontrolheaded;
#[doc = "Contains the physical address of the current endpoint descriptor of the control list\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hccontrolcurrented](hccontrolcurrented) module"]
pub type HCCONTROLCURRENTED = crate::Reg<u32, _HCCONTROLCURRENTED>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HCCONTROLCURRENTED;
#[doc = "`read()` method returns [hccontrolcurrented::R](hccontrolcurrented::R) reader structure"]
impl crate::Readable for HCCONTROLCURRENTED {}
#[doc = "`write(|w| ..)` method takes [hccontrolcurrented::W](hccontrolcurrented::W) writer structure"]
impl crate::Writable for HCCONTROLCURRENTED {}
#[doc = "Contains the physical address of the current endpoint descriptor of the control list"]
pub mod hccontrolcurrented;
#[doc = "Contains the physical address of the first endpoint descriptor of the bulk list\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hcbulkheaded](hcbulkheaded) module"]
pub type HCBULKHEADED = crate::Reg<u32, _HCBULKHEADED>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HCBULKHEADED;
#[doc = "`read()` method returns [hcbulkheaded::R](hcbulkheaded::R) reader structure"]
impl crate::Readable for HCBULKHEADED {}
#[doc = "`write(|w| ..)` method takes [hcbulkheaded::W](hcbulkheaded::W) writer structure"]
impl crate::Writable for HCBULKHEADED {}
#[doc = "Contains the physical address of the first endpoint descriptor of the bulk list"]
pub mod hcbulkheaded;
#[doc = "Contains the physical address of the current endpoint descriptor of the bulk list\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hcbulkcurrented](hcbulkcurrented) module"]
pub type HCBULKCURRENTED = crate::Reg<u32, _HCBULKCURRENTED>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HCBULKCURRENTED;
#[doc = "`read()` method returns [hcbulkcurrented::R](hcbulkcurrented::R) reader structure"]
impl crate::Readable for HCBULKCURRENTED {}
#[doc = "`write(|w| ..)` method takes [hcbulkcurrented::W](hcbulkcurrented::W) writer structure"]
impl crate::Writable for HCBULKCURRENTED {}
#[doc = "Contains the physical address of the current endpoint descriptor of the bulk list"]
pub mod hcbulkcurrented;
#[doc = "Contains the physical address of the last transfer descriptor added to the 'Done' queue\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hcdonehead](hcdonehead) module"]
pub type HCDONEHEAD = crate::Reg<u32, _HCDONEHEAD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HCDONEHEAD;
#[doc = "`read()` method returns [hcdonehead::R](hcdonehead::R) reader structure"]
impl crate::Readable for HCDONEHEAD {}
#[doc = "`write(|w| ..)` method takes [hcdonehead::W](hcdonehead::W) writer structure"]
impl crate::Writable for HCDONEHEAD {}
#[doc = "Contains the physical address of the last transfer descriptor added to the 'Done' queue"]
pub mod hcdonehead;
#[doc = "Defines the bit time interval in a frame and the full speed maximum packet size which would not cause an overrun\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hcfminterval](hcfminterval) module"]
pub type HCFMINTERVAL = crate::Reg<u32, _HCFMINTERVAL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HCFMINTERVAL;
#[doc = "`read()` method returns [hcfminterval::R](hcfminterval::R) reader structure"]
impl crate::Readable for HCFMINTERVAL {}
#[doc = "`write(|w| ..)` method takes [hcfminterval::W](hcfminterval::W) writer structure"]
impl crate::Writable for HCFMINTERVAL {}
#[doc = "Defines the bit time interval in a frame and the full speed maximum packet size which would not cause an overrun"]
pub mod hcfminterval;
#[doc = "A 14-bit counter showing the bit time remaining in the current frame\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hcfmremaining](hcfmremaining) module"]
pub type HCFMREMAINING = crate::Reg<u32, _HCFMREMAINING>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HCFMREMAINING;
#[doc = "`read()` method returns [hcfmremaining::R](hcfmremaining::R) reader structure"]
impl crate::Readable for HCFMREMAINING {}
#[doc = "`write(|w| ..)` method takes [hcfmremaining::W](hcfmremaining::W) writer structure"]
impl crate::Writable for HCFMREMAINING {}
#[doc = "A 14-bit counter showing the bit time remaining in the current frame"]
pub mod hcfmremaining;
#[doc = "Contains a 16-bit counter and provides the timing reference among events happening in the HC and the HCD\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hcfmnumber](hcfmnumber) module"]
pub type HCFMNUMBER = crate::Reg<u32, _HCFMNUMBER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HCFMNUMBER;
#[doc = "`read()` method returns [hcfmnumber::R](hcfmnumber::R) reader structure"]
impl crate::Readable for HCFMNUMBER {}
#[doc = "`write(|w| ..)` method takes [hcfmnumber::W](hcfmnumber::W) writer structure"]
impl crate::Writable for HCFMNUMBER {}
#[doc = "Contains a 16-bit counter and provides the timing reference among events happening in the HC and the HCD"]
pub mod hcfmnumber;
#[doc = "Contains a programmable 14-bit value which determines the earliest time HC should start processing a periodic list\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hcperiodicstart](hcperiodicstart) module"]
pub type HCPERIODICSTART = crate::Reg<u32, _HCPERIODICSTART>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HCPERIODICSTART;
#[doc = "`read()` method returns [hcperiodicstart::R](hcperiodicstart::R) reader structure"]
impl crate::Readable for HCPERIODICSTART {}
#[doc = "`write(|w| ..)` method takes [hcperiodicstart::W](hcperiodicstart::W) writer structure"]
impl crate::Writable for HCPERIODICSTART {}
#[doc = "Contains a programmable 14-bit value which determines the earliest time HC should start processing a periodic list"]
pub mod hcperiodicstart;
#[doc = "Contains 11-bit value which is used by the HC to determine whether to commit to transfer a maximum of 8-byte LS packet before EOF\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hclsthreshold](hclsthreshold) module"]
pub type HCLSTHRESHOLD = crate::Reg<u32, _HCLSTHRESHOLD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HCLSTHRESHOLD;
#[doc = "`read()` method returns [hclsthreshold::R](hclsthreshold::R) reader structure"]
impl crate::Readable for HCLSTHRESHOLD {}
#[doc = "`write(|w| ..)` method takes [hclsthreshold::W](hclsthreshold::W) writer structure"]
impl crate::Writable for HCLSTHRESHOLD {}
#[doc = "Contains 11-bit value which is used by the HC to determine whether to commit to transfer a maximum of 8-byte LS packet before EOF"]
pub mod hclsthreshold;
#[doc = "First of the two registers which describes the characteristics of the root hub\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hcrhdescriptora](hcrhdescriptora) module"]
pub type HCRHDESCRIPTORA = crate::Reg<u32, _HCRHDESCRIPTORA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HCRHDESCRIPTORA;
#[doc = "`read()` method returns [hcrhdescriptora::R](hcrhdescriptora::R) reader structure"]
impl crate::Readable for HCRHDESCRIPTORA {}
#[doc = "`write(|w| ..)` method takes [hcrhdescriptora::W](hcrhdescriptora::W) writer structure"]
impl crate::Writable for HCRHDESCRIPTORA {}
#[doc = "First of the two registers which describes the characteristics of the root hub"]
pub mod hcrhdescriptora;
#[doc = "Second of the two registers which describes the characteristics of the Root Hub\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hcrhdescriptorb](hcrhdescriptorb) module"]
pub type HCRHDESCRIPTORB = crate::Reg<u32, _HCRHDESCRIPTORB>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HCRHDESCRIPTORB;
#[doc = "`read()` method returns [hcrhdescriptorb::R](hcrhdescriptorb::R) reader structure"]
impl crate::Readable for HCRHDESCRIPTORB {}
#[doc = "`write(|w| ..)` method takes [hcrhdescriptorb::W](hcrhdescriptorb::W) writer structure"]
impl crate::Writable for HCRHDESCRIPTORB {}
#[doc = "Second of the two registers which describes the characteristics of the Root Hub"]
pub mod hcrhdescriptorb;
#[doc = "This register is divided into two parts\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hcrhstatus](hcrhstatus) module"]
pub type HCRHSTATUS = crate::Reg<u32, _HCRHSTATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HCRHSTATUS;
#[doc = "`read()` method returns [hcrhstatus::R](hcrhstatus::R) reader structure"]
impl crate::Readable for HCRHSTATUS {}
#[doc = "`write(|w| ..)` method takes [hcrhstatus::W](hcrhstatus::W) writer structure"]
impl crate::Writable for HCRHSTATUS {}
#[doc = "This register is divided into two parts"]
pub mod hcrhstatus;
#[doc = "Controls and reports the port events on a per-port basis\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hcrhportstatus](hcrhportstatus) module"]
pub type HCRHPORTSTATUS = crate::Reg<u32, _HCRHPORTSTATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HCRHPORTSTATUS;
#[doc = "`read()` method returns [hcrhportstatus::R](hcrhportstatus::R) reader structure"]
impl crate::Readable for HCRHPORTSTATUS {}
#[doc = "`write(|w| ..)` method takes [hcrhportstatus::W](hcrhportstatus::W) writer structure"]
impl crate::Writable for HCRHPORTSTATUS {}
#[doc = "Controls and reports the port events on a per-port basis"]
pub mod hcrhportstatus;
#[doc = "Controls the port if it is attached to the host block or the device block\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [portmode](portmode) module"]
pub type PORTMODE = crate::Reg<u32, _PORTMODE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PORTMODE;
#[doc = "`read()` method returns [portmode::R](portmode::R) reader structure"]
impl crate::Readable for PORTMODE {}
#[doc = "`write(|w| ..)` method takes [portmode::W](portmode::W) writer structure"]
impl crate::Writable for PORTMODE {}
#[doc = "Controls the port if it is attached to the host block or the device block"]
pub mod portmode;
