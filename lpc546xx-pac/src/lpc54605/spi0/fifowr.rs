///Register `FIFOWR` reader
pub struct R(crate::R<FIFOWR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FIFOWR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FIFOWR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FIFOWR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `FIFOWR` writer
pub struct W(crate::W<FIFOWR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FIFOWR_SPEC>;
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
impl From<crate::W<FIFOWR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FIFOWR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `TXDATA` reader - Transmit data to the FIFO.
pub struct TXDATA_R(crate::FieldReader<u16, u16>);
impl TXDATA_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        TXDATA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TXDATA_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `TXDATA` writer - Transmit data to the FIFO.
pub struct TXDATA_W<'a> {
    w: &'a mut W,
}
impl<'a> TXDATA_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | (value as u32 & 0xffff);
        self.w
    }
}
///Transmit slave select. This field asserts SSEL0 in master mode. The output on the pin is active LOW by default.
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXSSEL0_N_AW {
    ///0: SSEL0 asserted.
    ASSERTED = 0,
    ///1: SSEL0 not asserted.
    NOT_ASSERTED = 1,
}
impl From<TXSSEL0_N_AW> for bool {
    #[inline(always)]
    fn from(variant: TXSSEL0_N_AW) -> Self {
        variant as u8 != 0
    }
}
///Field `TXSSEL0_N` writer - Transmit slave select. This field asserts SSEL0 in master mode. The output on the pin is active LOW by default.
pub struct TXSSEL0_N_W<'a> {
    w: &'a mut W,
}
impl<'a> TXSSEL0_N_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: TXSSEL0_N_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    ///SSEL0 asserted.
    #[inline(always)]
    pub fn asserted(self) -> &'a mut W {
        self.variant(TXSSEL0_N_AW::ASSERTED)
    }
    ///SSEL0 not asserted.
    #[inline(always)]
    pub fn not_asserted(self) -> &'a mut W {
        self.variant(TXSSEL0_N_AW::NOT_ASSERTED)
    }
    ///Sets the field bit
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    ///Clears the field bit
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    ///Writes raw bits to the field
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 16)) | ((value as u32 & 0x01) << 16);
        self.w
    }
}
///Transmit slave select. This field asserts SSEL1 in master mode. The output on the pin is active LOW by default.
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXSSEL1_N_AW {
    ///0: SSEL1 asserted.
    ASSERTED = 0,
    ///1: SSEL1 not asserted.
    NOT_ASSERTED = 1,
}
impl From<TXSSEL1_N_AW> for bool {
    #[inline(always)]
    fn from(variant: TXSSEL1_N_AW) -> Self {
        variant as u8 != 0
    }
}
///Field `TXSSEL1_N` writer - Transmit slave select. This field asserts SSEL1 in master mode. The output on the pin is active LOW by default.
pub struct TXSSEL1_N_W<'a> {
    w: &'a mut W,
}
impl<'a> TXSSEL1_N_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: TXSSEL1_N_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    ///SSEL1 asserted.
    #[inline(always)]
    pub fn asserted(self) -> &'a mut W {
        self.variant(TXSSEL1_N_AW::ASSERTED)
    }
    ///SSEL1 not asserted.
    #[inline(always)]
    pub fn not_asserted(self) -> &'a mut W {
        self.variant(TXSSEL1_N_AW::NOT_ASSERTED)
    }
    ///Sets the field bit
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    ///Clears the field bit
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    ///Writes raw bits to the field
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 17)) | ((value as u32 & 0x01) << 17);
        self.w
    }
}
///Transmit slave select. This field asserts SSEL2 in master mode. The output on the pin is active LOW by default.
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXSSEL2_N_AW {
    ///0: SSEL2 asserted.
    ASSERTED = 0,
    ///1: SSEL2 not asserted.
    NOT_ASSERTED = 1,
}
impl From<TXSSEL2_N_AW> for bool {
    #[inline(always)]
    fn from(variant: TXSSEL2_N_AW) -> Self {
        variant as u8 != 0
    }
}
///Field `TXSSEL2_N` writer - Transmit slave select. This field asserts SSEL2 in master mode. The output on the pin is active LOW by default.
pub struct TXSSEL2_N_W<'a> {
    w: &'a mut W,
}
impl<'a> TXSSEL2_N_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: TXSSEL2_N_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    ///SSEL2 asserted.
    #[inline(always)]
    pub fn asserted(self) -> &'a mut W {
        self.variant(TXSSEL2_N_AW::ASSERTED)
    }
    ///SSEL2 not asserted.
    #[inline(always)]
    pub fn not_asserted(self) -> &'a mut W {
        self.variant(TXSSEL2_N_AW::NOT_ASSERTED)
    }
    ///Sets the field bit
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    ///Clears the field bit
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    ///Writes raw bits to the field
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 18)) | ((value as u32 & 0x01) << 18);
        self.w
    }
}
///Transmit slave select. This field asserts SSEL3 in master mode. The output on the pin is active LOW by default.
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXSSEL3_N_AW {
    ///0: SSEL3 asserted.
    ASSERTED = 0,
    ///1: SSEL3 not asserted.
    NOT_ASSERTED = 1,
}
impl From<TXSSEL3_N_AW> for bool {
    #[inline(always)]
    fn from(variant: TXSSEL3_N_AW) -> Self {
        variant as u8 != 0
    }
}
///Field `TXSSEL3_N` writer - Transmit slave select. This field asserts SSEL3 in master mode. The output on the pin is active LOW by default.
pub struct TXSSEL3_N_W<'a> {
    w: &'a mut W,
}
impl<'a> TXSSEL3_N_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: TXSSEL3_N_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    ///SSEL3 asserted.
    #[inline(always)]
    pub fn asserted(self) -> &'a mut W {
        self.variant(TXSSEL3_N_AW::ASSERTED)
    }
    ///SSEL3 not asserted.
    #[inline(always)]
    pub fn not_asserted(self) -> &'a mut W {
        self.variant(TXSSEL3_N_AW::NOT_ASSERTED)
    }
    ///Sets the field bit
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    ///Clears the field bit
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    ///Writes raw bits to the field
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 19)) | ((value as u32 & 0x01) << 19);
        self.w
    }
}
///End of transfer. The asserted SSEL will be deasserted at the end of a transfer and remain so far at least the time specified by the Transfer_delay value in the DLY register.
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EOT_AW {
    ///0: SSEL not deasserted. This piece of data is not treated as the end of a transfer. SSEL will not be deasserted at the end of this data.
    NOT_DEASSERTED = 0,
    ///1: SSEL deasserted. This piece of data is treated as the end of a transfer. SSEL will be deasserted at the end of this piece of data.
    DEASSERTED = 1,
}
impl From<EOT_AW> for bool {
    #[inline(always)]
    fn from(variant: EOT_AW) -> Self {
        variant as u8 != 0
    }
}
///Field `EOT` writer - End of transfer. The asserted SSEL will be deasserted at the end of a transfer and remain so far at least the time specified by the Transfer_delay value in the DLY register.
pub struct EOT_W<'a> {
    w: &'a mut W,
}
impl<'a> EOT_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: EOT_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    ///SSEL not deasserted. This piece of data is not treated as the end of a transfer. SSEL will not be deasserted at the end of this data.
    #[inline(always)]
    pub fn not_deasserted(self) -> &'a mut W {
        self.variant(EOT_AW::NOT_DEASSERTED)
    }
    ///SSEL deasserted. This piece of data is treated as the end of a transfer. SSEL will be deasserted at the end of this piece of data.
    #[inline(always)]
    pub fn deasserted(self) -> &'a mut W {
        self.variant(EOT_AW::DEASSERTED)
    }
    ///Sets the field bit
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    ///Clears the field bit
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    ///Writes raw bits to the field
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 20)) | ((value as u32 & 0x01) << 20);
        self.w
    }
}
///End of frame. Between frames, a delay may be inserted, as defined by the Frame_delay value in the DLY register. The end of a frame may not be particularly meaningful if the Frame_delay value = 0. This control can be used as part of the support for frame lengths greater than 16 bits.
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EOF_AW {
    ///0: Data not EOF. This piece of data transmitted is not treated as the end of a frame.
    NOT_EOF = 0,
    ///1: Data EOF. This piece of data is treated as the end of a frame, causing the Frame_delay time to be inserted before subsequent data is transmitted.
    EOF = 1,
}
impl From<EOF_AW> for bool {
    #[inline(always)]
    fn from(variant: EOF_AW) -> Self {
        variant as u8 != 0
    }
}
///Field `EOF` writer - End of frame. Between frames, a delay may be inserted, as defined by the Frame_delay value in the DLY register. The end of a frame may not be particularly meaningful if the Frame_delay value = 0. This control can be used as part of the support for frame lengths greater than 16 bits.
pub struct EOF_W<'a> {
    w: &'a mut W,
}
impl<'a> EOF_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: EOF_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Data not EOF. This piece of data transmitted is not treated as the end of a frame.
    #[inline(always)]
    pub fn not_eof(self) -> &'a mut W {
        self.variant(EOF_AW::NOT_EOF)
    }
    ///Data EOF. This piece of data is treated as the end of a frame, causing the Frame_delay time to be inserted before subsequent data is transmitted.
    #[inline(always)]
    pub fn eof(self) -> &'a mut W {
        self.variant(EOF_AW::EOF)
    }
    ///Sets the field bit
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    ///Clears the field bit
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    ///Writes raw bits to the field
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 21)) | ((value as u32 & 0x01) << 21);
        self.w
    }
}
///Receive Ignore. This allows data to be transmitted using the SPI without the need to read unneeded data from the receiver. Setting this bit simplifies the transmit process and can be used with the DMA.
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXIGNORE_AW {
    ///0: Read received data. Received data must be read in order to allow transmission to progress. SPI transmit will halt when the receive data FIFO is full. In slave mode, an overrun error will occur if received data is not read before new data is received.
    READ = 0,
    ///1: Ignore received data. Received data is ignored, allowing transmission without reading unneeded received data. No receiver flags are generated.
    IGNORE = 1,
}
impl From<RXIGNORE_AW> for bool {
    #[inline(always)]
    fn from(variant: RXIGNORE_AW) -> Self {
        variant as u8 != 0
    }
}
///Field `RXIGNORE` writer - Receive Ignore. This allows data to be transmitted using the SPI without the need to read unneeded data from the receiver. Setting this bit simplifies the transmit process and can be used with the DMA.
pub struct RXIGNORE_W<'a> {
    w: &'a mut W,
}
impl<'a> RXIGNORE_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: RXIGNORE_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Read received data. Received data must be read in order to allow transmission to progress. SPI transmit will halt when the receive data FIFO is full. In slave mode, an overrun error will occur if received data is not read before new data is received.
    #[inline(always)]
    pub fn read(self) -> &'a mut W {
        self.variant(RXIGNORE_AW::READ)
    }
    ///Ignore received data. Received data is ignored, allowing transmission without reading unneeded received data. No receiver flags are generated.
    #[inline(always)]
    pub fn ignore(self) -> &'a mut W {
        self.variant(RXIGNORE_AW::IGNORE)
    }
    ///Sets the field bit
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    ///Clears the field bit
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    ///Writes raw bits to the field
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 22)) | ((value as u32 & 0x01) << 22);
        self.w
    }
}
///Field `LEN` writer - Data Length. Specifies the data length from 4 to 16 bits. Note that transfer lengths greater than 16 bits are supported by implementing multiple sequential transmits. 0x0-2 = Reserved. 0x3 = Data transfer is 4 bits in length. 0x4 = Data transfer is 5 bits in length. 0xF = Data transfer is 16 bits in length.
pub struct LEN_W<'a> {
    w: &'a mut W,
}
impl<'a> LEN_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 24)) | ((value as u32 & 0x0f) << 24);
        self.w
    }
}
impl R {
    ///Bits 0:15 - Transmit data to the FIFO.
    #[inline(always)]
    pub fn txdata(&self) -> TXDATA_R {
        TXDATA_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    ///Bits 0:15 - Transmit data to the FIFO.
    #[inline(always)]
    pub fn txdata(&mut self) -> TXDATA_W {
        TXDATA_W { w: self }
    }
    ///Bit 16 - Transmit slave select. This field asserts SSEL0 in master mode. The output on the pin is active LOW by default.
    #[inline(always)]
    pub fn txssel0_n(&mut self) -> TXSSEL0_N_W {
        TXSSEL0_N_W { w: self }
    }
    ///Bit 17 - Transmit slave select. This field asserts SSEL1 in master mode. The output on the pin is active LOW by default.
    #[inline(always)]
    pub fn txssel1_n(&mut self) -> TXSSEL1_N_W {
        TXSSEL1_N_W { w: self }
    }
    ///Bit 18 - Transmit slave select. This field asserts SSEL2 in master mode. The output on the pin is active LOW by default.
    #[inline(always)]
    pub fn txssel2_n(&mut self) -> TXSSEL2_N_W {
        TXSSEL2_N_W { w: self }
    }
    ///Bit 19 - Transmit slave select. This field asserts SSEL3 in master mode. The output on the pin is active LOW by default.
    #[inline(always)]
    pub fn txssel3_n(&mut self) -> TXSSEL3_N_W {
        TXSSEL3_N_W { w: self }
    }
    ///Bit 20 - End of transfer. The asserted SSEL will be deasserted at the end of a transfer and remain so far at least the time specified by the Transfer_delay value in the DLY register.
    #[inline(always)]
    pub fn eot(&mut self) -> EOT_W {
        EOT_W { w: self }
    }
    ///Bit 21 - End of frame. Between frames, a delay may be inserted, as defined by the Frame_delay value in the DLY register. The end of a frame may not be particularly meaningful if the Frame_delay value = 0. This control can be used as part of the support for frame lengths greater than 16 bits.
    #[inline(always)]
    pub fn eof(&mut self) -> EOF_W {
        EOF_W { w: self }
    }
    ///Bit 22 - Receive Ignore. This allows data to be transmitted using the SPI without the need to read unneeded data from the receiver. Setting this bit simplifies the transmit process and can be used with the DMA.
    #[inline(always)]
    pub fn rxignore(&mut self) -> RXIGNORE_W {
        RXIGNORE_W { w: self }
    }
    ///Bits 24:27 - Data Length. Specifies the data length from 4 to 16 bits. Note that transfer lengths greater than 16 bits are supported by implementing multiple sequential transmits. 0x0-2 = Reserved. 0x3 = Data transfer is 4 bits in length. 0x4 = Data transfer is 5 bits in length. 0xF = Data transfer is 16 bits in length.
    #[inline(always)]
    pub fn len(&mut self) -> LEN_W {
        LEN_W { w: self }
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///FIFO write data.
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [fifowr](index.html) module
pub struct FIFOWR_SPEC;
impl crate::RegisterSpec for FIFOWR_SPEC {
    type Ux = u32;
}
///`read()` method returns [fifowr::R](R) reader structure
impl crate::Readable for FIFOWR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [fifowr::W](W) writer structure
impl crate::Writable for FIFOWR_SPEC {
    type Writer = W;
}
///`reset()` method sets FIFOWR to value 0
impl crate::Resettable for FIFOWR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
