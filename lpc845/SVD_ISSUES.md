# SVD issues
Found by diffing with the patch lpc82x.svd & comparing differences with the manual
## SWM
- The PINASSIGN registers are defined two times, the second time with the
  name PINASSIGN_DATA and different field names. Not sure what to do about this.
  (They have an alternateGroup tag)
## CMP
- The reset value of CTRL in the datasheet has two different definitons.
  In the resetMask this field is ignored
## IOCON
- PIO0_17/PIO0_13/PIO0_12/PIO0_5/PIO0_4/PIO0_3/PIO0_2/PIO0_16/PIO0_15/PIO0_1/PIO0_9/0_8/0_7(and
  probably all other faim bits for the PIO registers) resetMask (FAIM value
  dependent bit)
## SYSCON
- WDTOSCCTRL FREQSEL is just a 4 bit wide field, maybe implement enumeratedValues
## CAPT
- FDIV has enumerated Fields for a nibble from FDIV_0 to FDIV_15
## DMA
- ENABLESET/ACTIVE0/BUSY0/ERRINT0/INTSET0/INTA0/INTB0 resetMask is wrong
## GPIO
- B_/W_/PIN/MPIN/ resetMask is wrong


