///Register block
#[repr(C)]
pub struct RegisterBlock {
    ///0x00 - BCD representation of the version of the HCI specification that is implemented by the Host Controller (HC)
    pub hcrevision: crate::Reg<hcrevision::HCREVISION_SPEC>,
    ///0x04 - Defines the operating modes of the HC
    pub hccontrol: crate::Reg<hccontrol::HCCONTROL_SPEC>,
    ///0x08 - This register is used to receive the commands from the Host Controller Driver (HCD)
    pub hccommandstatus: crate::Reg<hccommandstatus::HCCOMMANDSTATUS_SPEC>,
    ///0x0c - Indicates the status on various events that cause hardware interrupts by setting the appropriate bits
    pub hcinterruptstatus: crate::Reg<hcinterruptstatus::HCINTERRUPTSTATUS_SPEC>,
    ///0x10 - Controls the bits in the HcInterruptStatus register and indicates which events will generate a hardware interrupt
    pub hcinterruptenable: crate::Reg<hcinterruptenable::HCINTERRUPTENABLE_SPEC>,
    ///0x14 - The bits in this register are used to disable corresponding bits in the HCInterruptStatus register and in turn disable that event leading to hardware interrupt
    pub hcinterruptdisable: crate::Reg<hcinterruptdisable::HCINTERRUPTDISABLE_SPEC>,
    ///0x18 - Contains the physical address of the host controller communication area
    pub hchcca: crate::Reg<hchcca::HCHCCA_SPEC>,
    ///0x1c - Contains the physical address of the current isochronous or interrupt endpoint descriptor
    pub hcperiodcurrented: crate::Reg<hcperiodcurrented::HCPERIODCURRENTED_SPEC>,
    ///0x20 - Contains the physical address of the first endpoint descriptor of the control list
    pub hccontrolheaded: crate::Reg<hccontrolheaded::HCCONTROLHEADED_SPEC>,
    ///0x24 - Contains the physical address of the current endpoint descriptor of the control list
    pub hccontrolcurrented: crate::Reg<hccontrolcurrented::HCCONTROLCURRENTED_SPEC>,
    ///0x28 - Contains the physical address of the first endpoint descriptor of the bulk list
    pub hcbulkheaded: crate::Reg<hcbulkheaded::HCBULKHEADED_SPEC>,
    ///0x2c - Contains the physical address of the current endpoint descriptor of the bulk list
    pub hcbulkcurrented: crate::Reg<hcbulkcurrented::HCBULKCURRENTED_SPEC>,
    ///0x30 - Contains the physical address of the last transfer descriptor added to the 'Done' queue
    pub hcdonehead: crate::Reg<hcdonehead::HCDONEHEAD_SPEC>,
    ///0x34 - Defines the bit time interval in a frame and the full speed maximum packet size which would not cause an overrun
    pub hcfminterval: crate::Reg<hcfminterval::HCFMINTERVAL_SPEC>,
    ///0x38 - A 14-bit counter showing the bit time remaining in the current frame
    pub hcfmremaining: crate::Reg<hcfmremaining::HCFMREMAINING_SPEC>,
    ///0x3c - Contains a 16-bit counter and provides the timing reference among events happening in the HC and the HCD
    pub hcfmnumber: crate::Reg<hcfmnumber::HCFMNUMBER_SPEC>,
    ///0x40 - Contains a programmable 14-bit value which determines the earliest time HC should start processing a periodic list
    pub hcperiodicstart: crate::Reg<hcperiodicstart::HCPERIODICSTART_SPEC>,
    ///0x44 - Contains 11-bit value which is used by the HC to determine whether to commit to transfer a maximum of 8-byte LS packet before EOF
    pub hclsthreshold: crate::Reg<hclsthreshold::HCLSTHRESHOLD_SPEC>,
    ///0x48 - First of the two registers which describes the characteristics of the root hub
    pub hcrhdescriptora: crate::Reg<hcrhdescriptora::HCRHDESCRIPTORA_SPEC>,
    ///0x4c - Second of the two registers which describes the characteristics of the Root Hub
    pub hcrhdescriptorb: crate::Reg<hcrhdescriptorb::HCRHDESCRIPTORB_SPEC>,
    ///0x50 - This register is divided into two parts
    pub hcrhstatus: crate::Reg<hcrhstatus::HCRHSTATUS_SPEC>,
    ///0x54 - Controls and reports the port events on a per-port basis
    pub hcrhportstatus: crate::Reg<hcrhportstatus::HCRHPORTSTATUS_SPEC>,
    _reserved22: [u8; 0x04],
    ///0x5c - Controls the port if it is attached to the host block or the device block
    pub portmode: crate::Reg<portmode::PORTMODE_SPEC>,
}
///HCREVISION register accessor: an alias for `Reg<HCREVISION_SPEC>`
pub type HCREVISION = crate::Reg<hcrevision::HCREVISION_SPEC>;
///BCD representation of the version of the HCI specification that is implemented by the Host Controller (HC)
pub mod hcrevision;
///HCCONTROL register accessor: an alias for `Reg<HCCONTROL_SPEC>`
pub type HCCONTROL = crate::Reg<hccontrol::HCCONTROL_SPEC>;
///Defines the operating modes of the HC
pub mod hccontrol;
///HCCOMMANDSTATUS register accessor: an alias for `Reg<HCCOMMANDSTATUS_SPEC>`
pub type HCCOMMANDSTATUS = crate::Reg<hccommandstatus::HCCOMMANDSTATUS_SPEC>;
///This register is used to receive the commands from the Host Controller Driver (HCD)
pub mod hccommandstatus;
///HCINTERRUPTSTATUS register accessor: an alias for `Reg<HCINTERRUPTSTATUS_SPEC>`
pub type HCINTERRUPTSTATUS = crate::Reg<hcinterruptstatus::HCINTERRUPTSTATUS_SPEC>;
///Indicates the status on various events that cause hardware interrupts by setting the appropriate bits
pub mod hcinterruptstatus;
///HCINTERRUPTENABLE register accessor: an alias for `Reg<HCINTERRUPTENABLE_SPEC>`
pub type HCINTERRUPTENABLE = crate::Reg<hcinterruptenable::HCINTERRUPTENABLE_SPEC>;
///Controls the bits in the HcInterruptStatus register and indicates which events will generate a hardware interrupt
pub mod hcinterruptenable;
///HCINTERRUPTDISABLE register accessor: an alias for `Reg<HCINTERRUPTDISABLE_SPEC>`
pub type HCINTERRUPTDISABLE = crate::Reg<hcinterruptdisable::HCINTERRUPTDISABLE_SPEC>;
///The bits in this register are used to disable corresponding bits in the HCInterruptStatus register and in turn disable that event leading to hardware interrupt
pub mod hcinterruptdisable;
///HCHCCA register accessor: an alias for `Reg<HCHCCA_SPEC>`
pub type HCHCCA = crate::Reg<hchcca::HCHCCA_SPEC>;
///Contains the physical address of the host controller communication area
pub mod hchcca;
///HCPERIODCURRENTED register accessor: an alias for `Reg<HCPERIODCURRENTED_SPEC>`
pub type HCPERIODCURRENTED = crate::Reg<hcperiodcurrented::HCPERIODCURRENTED_SPEC>;
///Contains the physical address of the current isochronous or interrupt endpoint descriptor
pub mod hcperiodcurrented;
///HCCONTROLHEADED register accessor: an alias for `Reg<HCCONTROLHEADED_SPEC>`
pub type HCCONTROLHEADED = crate::Reg<hccontrolheaded::HCCONTROLHEADED_SPEC>;
///Contains the physical address of the first endpoint descriptor of the control list
pub mod hccontrolheaded;
///HCCONTROLCURRENTED register accessor: an alias for `Reg<HCCONTROLCURRENTED_SPEC>`
pub type HCCONTROLCURRENTED = crate::Reg<hccontrolcurrented::HCCONTROLCURRENTED_SPEC>;
///Contains the physical address of the current endpoint descriptor of the control list
pub mod hccontrolcurrented;
///HCBULKHEADED register accessor: an alias for `Reg<HCBULKHEADED_SPEC>`
pub type HCBULKHEADED = crate::Reg<hcbulkheaded::HCBULKHEADED_SPEC>;
///Contains the physical address of the first endpoint descriptor of the bulk list
pub mod hcbulkheaded;
///HCBULKCURRENTED register accessor: an alias for `Reg<HCBULKCURRENTED_SPEC>`
pub type HCBULKCURRENTED = crate::Reg<hcbulkcurrented::HCBULKCURRENTED_SPEC>;
///Contains the physical address of the current endpoint descriptor of the bulk list
pub mod hcbulkcurrented;
///HCDONEHEAD register accessor: an alias for `Reg<HCDONEHEAD_SPEC>`
pub type HCDONEHEAD = crate::Reg<hcdonehead::HCDONEHEAD_SPEC>;
///Contains the physical address of the last transfer descriptor added to the 'Done' queue
pub mod hcdonehead;
///HCFMINTERVAL register accessor: an alias for `Reg<HCFMINTERVAL_SPEC>`
pub type HCFMINTERVAL = crate::Reg<hcfminterval::HCFMINTERVAL_SPEC>;
///Defines the bit time interval in a frame and the full speed maximum packet size which would not cause an overrun
pub mod hcfminterval;
///HCFMREMAINING register accessor: an alias for `Reg<HCFMREMAINING_SPEC>`
pub type HCFMREMAINING = crate::Reg<hcfmremaining::HCFMREMAINING_SPEC>;
///A 14-bit counter showing the bit time remaining in the current frame
pub mod hcfmremaining;
///HCFMNUMBER register accessor: an alias for `Reg<HCFMNUMBER_SPEC>`
pub type HCFMNUMBER = crate::Reg<hcfmnumber::HCFMNUMBER_SPEC>;
///Contains a 16-bit counter and provides the timing reference among events happening in the HC and the HCD
pub mod hcfmnumber;
///HCPERIODICSTART register accessor: an alias for `Reg<HCPERIODICSTART_SPEC>`
pub type HCPERIODICSTART = crate::Reg<hcperiodicstart::HCPERIODICSTART_SPEC>;
///Contains a programmable 14-bit value which determines the earliest time HC should start processing a periodic list
pub mod hcperiodicstart;
///HCLSTHRESHOLD register accessor: an alias for `Reg<HCLSTHRESHOLD_SPEC>`
pub type HCLSTHRESHOLD = crate::Reg<hclsthreshold::HCLSTHRESHOLD_SPEC>;
///Contains 11-bit value which is used by the HC to determine whether to commit to transfer a maximum of 8-byte LS packet before EOF
pub mod hclsthreshold;
///HCRHDESCRIPTORA register accessor: an alias for `Reg<HCRHDESCRIPTORA_SPEC>`
pub type HCRHDESCRIPTORA = crate::Reg<hcrhdescriptora::HCRHDESCRIPTORA_SPEC>;
///First of the two registers which describes the characteristics of the root hub
pub mod hcrhdescriptora;
///HCRHDESCRIPTORB register accessor: an alias for `Reg<HCRHDESCRIPTORB_SPEC>`
pub type HCRHDESCRIPTORB = crate::Reg<hcrhdescriptorb::HCRHDESCRIPTORB_SPEC>;
///Second of the two registers which describes the characteristics of the Root Hub
pub mod hcrhdescriptorb;
///HCRHSTATUS register accessor: an alias for `Reg<HCRHSTATUS_SPEC>`
pub type HCRHSTATUS = crate::Reg<hcrhstatus::HCRHSTATUS_SPEC>;
///This register is divided into two parts
pub mod hcrhstatus;
///HCRHPORTSTATUS register accessor: an alias for `Reg<HCRHPORTSTATUS_SPEC>`
pub type HCRHPORTSTATUS = crate::Reg<hcrhportstatus::HCRHPORTSTATUS_SPEC>;
///Controls and reports the port events on a per-port basis
pub mod hcrhportstatus;
///PORTMODE register accessor: an alias for `Reg<PORTMODE_SPEC>`
pub type PORTMODE = crate::Reg<portmode::PORTMODE_SPEC>;
///Controls the port if it is attached to the host block or the device block
pub mod portmode;
