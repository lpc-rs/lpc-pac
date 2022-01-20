#[doc = "Register `IDR` reader"]
pub struct R(crate::R<IDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `ImplementationRevision` reader - Implementation revision. The value of these bits is b0000, indicating implementation revision, 0."]
pub struct IMPLEMENTATIONREVISION_R(crate::FieldReader<u8, u8>);
impl IMPLEMENTATIONREVISION_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        IMPLEMENTATIONREVISION_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IMPLEMENTATIONREVISION_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MinorETMarchitectureVersion` reader - Minor ETM architecture version. The value of these bits is 0b0101, indicating minor architecture version number 5."]
pub struct MINORETMARCHITECTUREVERSION_R(crate::FieldReader<u8, u8>);
impl MINORETMARCHITECTUREVERSION_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        MINORETMARCHITECTUREVERSION_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MINORETMARCHITECTUREVERSION_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MajorETMarchitectureVersion` reader - Major ETM architecture version. The value of these bits is 0b0010, indicating major architecture version number 3, ETMv3."]
pub struct MAJORETMARCHITECTUREVERSION_R(crate::FieldReader<u8, u8>);
impl MAJORETMARCHITECTUREVERSION_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        MAJORETMARCHITECTUREVERSION_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MAJORETMARCHITECTUREVERSION_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ProcessorFamily` reader - Processor family. The value of these bits is 0b1111, indicating that the processor family is not identified in this register."]
pub struct PROCESSORFAMILY_R(crate::FieldReader<u8, u8>);
impl PROCESSORFAMILY_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PROCESSORFAMILY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PROCESSORFAMILY_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LoadPCfirst` reader - Load PC first. The value of this bit is 0, indicating that data tracing is not supported."]
pub struct LOADPCFIRST_R(crate::FieldReader<bool, bool>);
impl LOADPCFIRST_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        LOADPCFIRST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LOADPCFIRST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
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
#[doc = "Field `ThumbInstructionTracing` reader - 32-bit Thumb instruction tracing. The value of this bit is 1, indicating that a 32-bit Thumb instruction is traced as a single instruction."]
pub struct THUMBINSTRUCTIONTRACING_R(crate::FieldReader<bool, THUMBINSTRUCTIONTRACING_A>);
impl THUMBINSTRUCTIONTRACING_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        THUMBINSTRUCTIONTRACING_R(crate::FieldReader::new(bits))
    }
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
        **self == THUMBINSTRUCTIONTRACING_A::THUMBINSTRUCTIONTRACING_0
    }
    #[doc = "Checks if the value of the field is `THUMBINSTRUCTIONTRACING_1`"]
    #[inline(always)]
    pub fn is_thumb_instruction_tracing_1(&self) -> bool {
        **self == THUMBINSTRUCTIONTRACING_A::THUMBINSTRUCTIONTRACING_1
    }
}
impl core::ops::Deref for THUMBINSTRUCTIONTRACING_R {
    type Target = crate::FieldReader<bool, THUMBINSTRUCTIONTRACING_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
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
#[doc = "Field `SecurityExtensionSupport` reader - Security Extensions support. The value of this bit is 0, indicating that the ETM behaves as if the processor is in Secure state at all times."]
pub struct SECURITYEXTENSIONSUPPORT_R(crate::FieldReader<bool, SECURITYEXTENSIONSUPPORT_A>);
impl SECURITYEXTENSIONSUPPORT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SECURITYEXTENSIONSUPPORT_R(crate::FieldReader::new(bits))
    }
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
        **self == SECURITYEXTENSIONSUPPORT_A::SECURITYEXTENSIONSUPPORT_0
    }
    #[doc = "Checks if the value of the field is `SECURITYEXTENSIONSUPPORT_1`"]
    #[inline(always)]
    pub fn is_security_extension_support_1(&self) -> bool {
        **self == SECURITYEXTENSIONSUPPORT_A::SECURITYEXTENSIONSUPPORT_1
    }
}
impl core::ops::Deref for SECURITYEXTENSIONSUPPORT_R {
    type Target = crate::FieldReader<bool, SECURITYEXTENSIONSUPPORT_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
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
#[doc = "Field `BranchPacketEncoding` reader - Branch packet encoding. The value of this bit is 1, indicating that alternative branch packet encoding is implemented."]
pub struct BRANCHPACKETENCODING_R(crate::FieldReader<bool, BRANCHPACKETENCODING_A>);
impl BRANCHPACKETENCODING_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        BRANCHPACKETENCODING_R(crate::FieldReader::new(bits))
    }
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
        **self == BRANCHPACKETENCODING_A::BRANCHPACKETENCODING_0
    }
    #[doc = "Checks if the value of the field is `BRANCHPACKETENCODING_1`"]
    #[inline(always)]
    pub fn is_branch_packet_encoding_1(&self) -> bool {
        **self == BRANCHPACKETENCODING_A::BRANCHPACKETENCODING_1
    }
}
impl core::ops::Deref for BRANCHPACKETENCODING_R {
    type Target = crate::FieldReader<bool, BRANCHPACKETENCODING_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ImplementorCode` reader - Implementor code. These bits identify ARM as the implementor of the processor. The value of these bits is 01000001."]
pub struct IMPLEMENTORCODE_R(crate::FieldReader<u8, u8>);
impl IMPLEMENTORCODE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        IMPLEMENTORCODE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IMPLEMENTORCODE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
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
#[doc = "ID Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [idr](index.html) module"]
pub struct IDR_SPEC;
impl crate::RegisterSpec for IDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [idr::R](R) reader structure"]
impl crate::Readable for IDR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets IDR to value 0x4114_f250"]
impl crate::Resettable for IDR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x4114_f250
    }
}
