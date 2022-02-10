///DYNAMICCONFIG register accessor: an alias for `Reg<DYNAMICCONFIG_SPEC>`
pub type DYNAMICCONFIG = crate::Reg<dynamicconfig::DYNAMICCONFIG_SPEC>;
///Configuration information for EMC_DYCSx
pub mod dynamicconfig;
///DYNAMICRASCAS register accessor: an alias for `Reg<DYNAMICRASCAS_SPEC>`
pub type DYNAMICRASCAS = crate::Reg<dynamicrascas::DYNAMICRASCAS_SPEC>;
///RAS and CAS latencies for EMC_DYCSx
pub mod dynamicrascas;
