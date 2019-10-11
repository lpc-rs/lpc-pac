#[doc = "Reader of register CONTROL%s"]
pub type R = crate::R<u32, super::CONTROL>;
#[doc = "Writer for register CONTROL%s"]
pub type W = crate::W<u32, super::CONTROL>;
#[doc = "Register CONTROL%s `reset()`'s with value 0"]
impl crate::ResetValue for super::CONTROL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `TRANSFERSIZE`"]
pub type TRANSFERSIZE_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `TRANSFERSIZE`"]
pub struct TRANSFERSIZE_W<'a> {
    w: &'a mut W,
}
impl<'a> TRANSFERSIZE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0fff) | ((value as u32) & 0x0fff);
        self.w
    }
}
#[doc = "Reader of field `SBSIZE`"]
pub type SBSIZE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SBSIZE`"]
pub struct SBSIZE_W<'a> {
    w: &'a mut W,
}
impl<'a> SBSIZE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 12)) | (((value as u32) & 0x07) << 12);
        self.w
    }
}
#[doc = "Reader of field `DBSIZE`"]
pub type DBSIZE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DBSIZE`"]
pub struct DBSIZE_W<'a> {
    w: &'a mut W,
}
impl<'a> DBSIZE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 15)) | (((value as u32) & 0x07) << 15);
        self.w
    }
}
#[doc = "Reader of field `SWIDTH`"]
pub type SWIDTH_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SWIDTH`"]
pub struct SWIDTH_W<'a> {
    w: &'a mut W,
}
impl<'a> SWIDTH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 18)) | (((value as u32) & 0x07) << 18);
        self.w
    }
}
#[doc = "Reader of field `DWIDTH`"]
pub type DWIDTH_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DWIDTH`"]
pub struct DWIDTH_W<'a> {
    w: &'a mut W,
}
impl<'a> DWIDTH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 21)) | (((value as u32) & 0x07) << 21);
        self.w
    }
}
#[doc = "Reader of field `SI`"]
pub type SI_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SI`"]
pub struct SI_W<'a> {
    w: &'a mut W,
}
impl<'a> SI_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 26)) | (((value as u32) & 0x01) << 26);
        self.w
    }
}
#[doc = "Reader of field `DI`"]
pub type DI_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DI`"]
pub struct DI_W<'a> {
    w: &'a mut W,
}
impl<'a> DI_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 27)) | (((value as u32) & 0x01) << 27);
        self.w
    }
}
#[doc = "Reader of field `PROT1`"]
pub type PROT1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PROT1`"]
pub struct PROT1_W<'a> {
    w: &'a mut W,
}
impl<'a> PROT1_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 28)) | (((value as u32) & 0x01) << 28);
        self.w
    }
}
#[doc = "Reader of field `PROT2`"]
pub type PROT2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PROT2`"]
pub struct PROT2_W<'a> {
    w: &'a mut W,
}
impl<'a> PROT2_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 29)) | (((value as u32) & 0x01) << 29);
        self.w
    }
}
#[doc = "Reader of field `PROT3`"]
pub type PROT3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PROT3`"]
pub struct PROT3_W<'a> {
    w: &'a mut W,
}
impl<'a> PROT3_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 30)) | (((value as u32) & 0x01) << 30);
        self.w
    }
}
#[doc = "Reader of field `I`"]
pub type I_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `I`"]
pub struct I_W<'a> {
    w: &'a mut W,
}
impl<'a> I_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | (((value as u32) & 0x01) << 31);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:11 - Transfer size. This field sets the size of the transfer when the DMA controller is the flow controller, in which case the value must be set before the channel is enabled. Transfer size is updated as data transfers are completed. A read from this field indicates the number of transfers completed on the destination bus. Reading the register when the channel is active does not give useful information because by the time that the software has processed the value read, the channel might have progressed. It is intended to be used only when a channel is enabled and then disabled. The transfer size value is not used if a peripheral is the flow controller."]
    #[inline(always)]
    pub fn transfersize(&self) -> TRANSFERSIZE_R {
        TRANSFERSIZE_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 12:14 - Source burst size. Indicates the number of transfers that make up a source burst. This value must be set to the burst size of the source peripheral, or if the source is memory, to the memory boundary size. The burst size is the amount of data that is transferred when the DMACBREQ signal goes active in the source peripheral. 000 - 1 001 - 4 010 - 8 011 - 16 100 - 32 101 - 64 110 - 128 111 - 256"]
    #[inline(always)]
    pub fn sbsize(&self) -> SBSIZE_R {
        SBSIZE_R::new(((self.bits >> 12) & 0x07) as u8)
    }
    #[doc = "Bits 15:17 - Destination burst size. Indicates the number of transfers that make up a destination burst transfer request. This value must be set to the burst size of the destination peripheral or, if the destination is memory, to the memory boundary size. The burst size is the amount of data that is transferred when the DMACBREQ signal goes active in the destination peripheral. 000 - 1 001 - 4 010 - 8 011 - 16 100 - 32 101 - 64 110 - 128 111 - 256"]
    #[inline(always)]
    pub fn dbsize(&self) -> DBSIZE_R {
        DBSIZE_R::new(((self.bits >> 15) & 0x07) as u8)
    }
    #[doc = "Bits 18:20 - Source transfer width. The source and destination widths can be different from each other. The hardware automatically packs and unpacks the data as required. 000 - Byte (8-bit) 001 - Halfword (16-bit) 010 - Word (32-bit) 011 to 111 - Reserved"]
    #[inline(always)]
    pub fn swidth(&self) -> SWIDTH_R {
        SWIDTH_R::new(((self.bits >> 18) & 0x07) as u8)
    }
    #[doc = "Bits 21:23 - Destination transfer width. The source and destination widths can be different from each other. The hardware automatically packs and unpacks the data as required. 000 - Byte (8-bit) 001 - Halfword (16-bit) 010 - Word (32-bit) 011 to 111 - Reserved"]
    #[inline(always)]
    pub fn dwidth(&self) -> DWIDTH_R {
        DWIDTH_R::new(((self.bits >> 21) & 0x07) as u8)
    }
    #[doc = "Bit 26 - Source increment: 0 - the source address is not incremented after each transfer. 1 - the source address is incremented after each transfer."]
    #[inline(always)]
    pub fn si(&self) -> SI_R {
        SI_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - Destination increment: 0 - the destination address is not incremented after each transfer. 1 - the destination address is incremented after each transfer."]
    #[inline(always)]
    pub fn di(&self) -> DI_R {
        DI_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - This is provided to the peripheral during a DMA bus access and indicates that the access is in user mode or privileged mode. This information is not used in the LPC178x/177x. 0 - access is in user mode. 1 - access is in privileged mode."]
    #[inline(always)]
    pub fn prot1(&self) -> PROT1_R {
        PROT1_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - This is provided to the peripheral during a DMA bus access and indicates to the peripheral that the access is bufferable or not bufferable. This information is not used in the LPC178x/177x. 0 - access is not bufferable. 1 - access is bufferable."]
    #[inline(always)]
    pub fn prot2(&self) -> PROT2_R {
        PROT2_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - This is provided to the peripheral during a DMA bus access and indicates to the peripheral that the access is cacheable or not cacheable. This information is not used in the LPC178x/177x. 0 - access is not cacheable. 1 - access is cacheable."]
    #[inline(always)]
    pub fn prot3(&self) -> PROT3_R {
        PROT3_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - Terminal count interrupt enable bit. 0 - the terminal count interrupt is disabled. 1 - the terminal count interrupt is enabled."]
    #[inline(always)]
    pub fn i(&self) -> I_R {
        I_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:11 - Transfer size. This field sets the size of the transfer when the DMA controller is the flow controller, in which case the value must be set before the channel is enabled. Transfer size is updated as data transfers are completed. A read from this field indicates the number of transfers completed on the destination bus. Reading the register when the channel is active does not give useful information because by the time that the software has processed the value read, the channel might have progressed. It is intended to be used only when a channel is enabled and then disabled. The transfer size value is not used if a peripheral is the flow controller."]
    #[inline(always)]
    pub fn transfersize(&mut self) -> TRANSFERSIZE_W {
        TRANSFERSIZE_W { w: self }
    }
    #[doc = "Bits 12:14 - Source burst size. Indicates the number of transfers that make up a source burst. This value must be set to the burst size of the source peripheral, or if the source is memory, to the memory boundary size. The burst size is the amount of data that is transferred when the DMACBREQ signal goes active in the source peripheral. 000 - 1 001 - 4 010 - 8 011 - 16 100 - 32 101 - 64 110 - 128 111 - 256"]
    #[inline(always)]
    pub fn sbsize(&mut self) -> SBSIZE_W {
        SBSIZE_W { w: self }
    }
    #[doc = "Bits 15:17 - Destination burst size. Indicates the number of transfers that make up a destination burst transfer request. This value must be set to the burst size of the destination peripheral or, if the destination is memory, to the memory boundary size. The burst size is the amount of data that is transferred when the DMACBREQ signal goes active in the destination peripheral. 000 - 1 001 - 4 010 - 8 011 - 16 100 - 32 101 - 64 110 - 128 111 - 256"]
    #[inline(always)]
    pub fn dbsize(&mut self) -> DBSIZE_W {
        DBSIZE_W { w: self }
    }
    #[doc = "Bits 18:20 - Source transfer width. The source and destination widths can be different from each other. The hardware automatically packs and unpacks the data as required. 000 - Byte (8-bit) 001 - Halfword (16-bit) 010 - Word (32-bit) 011 to 111 - Reserved"]
    #[inline(always)]
    pub fn swidth(&mut self) -> SWIDTH_W {
        SWIDTH_W { w: self }
    }
    #[doc = "Bits 21:23 - Destination transfer width. The source and destination widths can be different from each other. The hardware automatically packs and unpacks the data as required. 000 - Byte (8-bit) 001 - Halfword (16-bit) 010 - Word (32-bit) 011 to 111 - Reserved"]
    #[inline(always)]
    pub fn dwidth(&mut self) -> DWIDTH_W {
        DWIDTH_W { w: self }
    }
    #[doc = "Bit 26 - Source increment: 0 - the source address is not incremented after each transfer. 1 - the source address is incremented after each transfer."]
    #[inline(always)]
    pub fn si(&mut self) -> SI_W {
        SI_W { w: self }
    }
    #[doc = "Bit 27 - Destination increment: 0 - the destination address is not incremented after each transfer. 1 - the destination address is incremented after each transfer."]
    #[inline(always)]
    pub fn di(&mut self) -> DI_W {
        DI_W { w: self }
    }
    #[doc = "Bit 28 - This is provided to the peripheral during a DMA bus access and indicates that the access is in user mode or privileged mode. This information is not used in the LPC178x/177x. 0 - access is in user mode. 1 - access is in privileged mode."]
    #[inline(always)]
    pub fn prot1(&mut self) -> PROT1_W {
        PROT1_W { w: self }
    }
    #[doc = "Bit 29 - This is provided to the peripheral during a DMA bus access and indicates to the peripheral that the access is bufferable or not bufferable. This information is not used in the LPC178x/177x. 0 - access is not bufferable. 1 - access is bufferable."]
    #[inline(always)]
    pub fn prot2(&mut self) -> PROT2_W {
        PROT2_W { w: self }
    }
    #[doc = "Bit 30 - This is provided to the peripheral during a DMA bus access and indicates to the peripheral that the access is cacheable or not cacheable. This information is not used in the LPC178x/177x. 0 - access is not cacheable. 1 - access is cacheable."]
    #[inline(always)]
    pub fn prot3(&mut self) -> PROT3_W {
        PROT3_W { w: self }
    }
    #[doc = "Bit 31 - Terminal count interrupt enable bit. 0 - the terminal count interrupt is disabled. 1 - the terminal count interrupt is enabled."]
    #[inline(always)]
    pub fn i(&mut self) -> I_W {
        I_W { w: self }
    }
}
