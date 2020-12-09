#[doc = "Reader of register IDR"]
pub type R = crate::R<u32, super::IDR>;
#[doc = "Reader of field `ImplementationRevision`"]
pub type IMPLEMENTATIONREVISION_R = crate::R<u8, u8>;
#[doc = "Reader of field `MinorETMarchitectureVersion`"]
pub type MINORETMARCHITECTUREVERSION_R = crate::R<u8, u8>;
#[doc = "Reader of field `MajorETMarchitectureVersion`"]
pub type MAJORETMARCHITECTUREVERSION_R = crate::R<u8, u8>;
#[doc = "Reader of field `ProcessorFamily`"]
pub type PROCESSORFAMILY_R = crate::R<u8, u8>;
#[doc = "Reader of field `LoadPCfirst`"]
pub type LOADPCFIRST_R = crate::R<bool, bool>;
#[doc = "32-bit Thumb instruction tracing. The value of this bit is 1, indicating that a 32-bit Thumb instruction is traced as a single instruction.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum THUMBINSTRUCTIONTRACING_A {
    #[doc = "0: A 32-bit Thumb instruction is traced as two instructions, and exceptions might occur between these two instructions."]
    THUMBINSTRUCTIONTRACING_0 = 0,
    #[doc = "1: A 32-bit Thimb instruction is traced as a single instruction."]
    THUMBINSTRUCTIONTRACING_1 = 1,
}
impl From<THUMBINSTRUCTIONTRACING_A> for bool {
    #[inline(always)]
    fn from(variant: THUMBINSTRUCTIONTRACING_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ThumbInstructionTracing`"]
pub type THUMBINSTRUCTIONTRACING_R = crate::R<bool, THUMBINSTRUCTIONTRACING_A>;
impl THUMBINSTRUCTIONTRACING_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> THUMBINSTRUCTIONTRACING_A {
        match self.bits {
            false => THUMBINSTRUCTIONTRACING_A::THUMBINSTRUCTIONTRACING_0,
            true => THUMBINSTRUCTIONTRACING_A::THUMBINSTRUCTIONTRACING_1,
        }
    }
    #[doc = "Checks if the value of the field is `THUMBINSTRUCTIONTRACING_0`"]
    #[inline(always)]
    pub fn is_thumb_instruction_tracing_0(&self) -> bool {
        *self == THUMBINSTRUCTIONTRACING_A::THUMBINSTRUCTIONTRACING_0
    }
    #[doc = "Checks if the value of the field is `THUMBINSTRUCTIONTRACING_1`"]
    #[inline(always)]
    pub fn is_thumb_instruction_tracing_1(&self) -> bool {
        *self == THUMBINSTRUCTIONTRACING_A::THUMBINSTRUCTIONTRACING_1
    }
}
#[doc = "Security Extensions support. The value of this bit is 0, indicating that the ETM behaves as if the processor is in Secure state at all times.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SECURITYEXTENSIONSUPPORT_A {
    #[doc = "0: The ETM behaves as if the processor is in Secure state at all times."]
    SECURITYEXTENSIONSUPPORT_0 = 0,
    #[doc = "1: The ARM architecture Security Extensions are implemented by the processor."]
    SECURITYEXTENSIONSUPPORT_1 = 1,
}
impl From<SECURITYEXTENSIONSUPPORT_A> for bool {
    #[inline(always)]
    fn from(variant: SECURITYEXTENSIONSUPPORT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SecurityExtensionSupport`"]
pub type SECURITYEXTENSIONSUPPORT_R = crate::R<bool, SECURITYEXTENSIONSUPPORT_A>;
impl SECURITYEXTENSIONSUPPORT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SECURITYEXTENSIONSUPPORT_A {
        match self.bits {
            false => SECURITYEXTENSIONSUPPORT_A::SECURITYEXTENSIONSUPPORT_0,
            true => SECURITYEXTENSIONSUPPORT_A::SECURITYEXTENSIONSUPPORT_1,
        }
    }
    #[doc = "Checks if the value of the field is `SECURITYEXTENSIONSUPPORT_0`"]
    #[inline(always)]
    pub fn is_security_extension_support_0(&self) -> bool {
        *self == SECURITYEXTENSIONSUPPORT_A::SECURITYEXTENSIONSUPPORT_0
    }
    #[doc = "Checks if the value of the field is `SECURITYEXTENSIONSUPPORT_1`"]
    #[inline(always)]
    pub fn is_security_extension_support_1(&self) -> bool {
        *self == SECURITYEXTENSIONSUPPORT_A::SECURITYEXTENSIONSUPPORT_1
    }
}
#[doc = "Branch packet encoding. The value of this bit is 1, indicating that alternative branch packet encoding is implemented.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BRANCHPACKETENCODING_A {
    #[doc = "0: The ETM implements the original branch packet encoding."]
    BRANCHPACKETENCODING_0 = 0,
    #[doc = "1: The ETM implements the alternative branch packet encoding."]
    BRANCHPACKETENCODING_1 = 1,
}
impl From<BRANCHPACKETENCODING_A> for bool {
    #[inline(always)]
    fn from(variant: BRANCHPACKETENCODING_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `BranchPacketEncoding`"]
pub type BRANCHPACKETENCODING_R = crate::R<bool, BRANCHPACKETENCODING_A>;
impl BRANCHPACKETENCODING_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BRANCHPACKETENCODING_A {
        match self.bits {
            false => BRANCHPACKETENCODING_A::BRANCHPACKETENCODING_0,
            true => BRANCHPACKETENCODING_A::BRANCHPACKETENCODING_1,
        }
    }
    #[doc = "Checks if the value of the field is `BRANCHPACKETENCODING_0`"]
    #[inline(always)]
    pub fn is_branch_packet_encoding_0(&self) -> bool {
        *self == BRANCHPACKETENCODING_A::BRANCHPACKETENCODING_0
    }
    #[doc = "Checks if the value of the field is `BRANCHPACKETENCODING_1`"]
    #[inline(always)]
    pub fn is_branch_packet_encoding_1(&self) -> bool {
        *self == BRANCHPACKETENCODING_A::BRANCHPACKETENCODING_1
    }
}
#[doc = "Reader of field `ImplementorCode`"]
pub type IMPLEMENTORCODE_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:3 - Implementation revision. The value of these bits is b0000, indicating implementation revision, 0."]
    #[inline(always)]
    pub fn implementation_revision(&self) -> IMPLEMENTATIONREVISION_R {
        IMPLEMENTATIONREVISION_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Minor ETM architecture version. The value of these bits is 0b0101, indicating minor architecture version number 5."]
    #[inline(always)]
    pub fn minor_etmarchitecture_version(&self) -> MINORETMARCHITECTUREVERSION_R {
        MINORETMARCHITECTUREVERSION_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - Major ETM architecture version. The value of these bits is 0b0010, indicating major architecture version number 3, ETMv3."]
    #[inline(always)]
    pub fn major_etmarchitecture_version(&self) -> MAJORETMARCHITECTUREVERSION_R {
        MAJORETMARCHITECTUREVERSION_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - Processor family. The value of these bits is 0b1111, indicating that the processor family is not identified in this register."]
    #[inline(always)]
    pub fn processor_family(&self) -> PROCESSORFAMILY_R {
        PROCESSORFAMILY_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bit 16 - Load PC first. The value of this bit is 0, indicating that data tracing is not supported."]
    #[inline(always)]
    pub fn load_pcfirst(&self) -> LOADPCFIRST_R {
        LOADPCFIRST_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 18 - 32-bit Thumb instruction tracing. The value of this bit is 1, indicating that a 32-bit Thumb instruction is traced as a single instruction."]
    #[inline(always)]
    pub fn thumb_instruction_tracing(&self) -> THUMBINSTRUCTIONTRACING_R {
        THUMBINSTRUCTIONTRACING_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Security Extensions support. The value of this bit is 0, indicating that the ETM behaves as if the processor is in Secure state at all times."]
    #[inline(always)]
    pub fn security_extension_support(&self) -> SECURITYEXTENSIONSUPPORT_R {
        SECURITYEXTENSIONSUPPORT_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Branch packet encoding. The value of this bit is 1, indicating that alternative branch packet encoding is implemented."]
    #[inline(always)]
    pub fn branch_packet_encoding(&self) -> BRANCHPACKETENCODING_R {
        BRANCHPACKETENCODING_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bits 24:31 - Implementor code. These bits identify ARM as the implementor of the processor. The value of these bits is 01000001."]
    #[inline(always)]
    pub fn implementor_code(&self) -> IMPLEMENTORCODE_R {
        IMPLEMENTORCODE_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
