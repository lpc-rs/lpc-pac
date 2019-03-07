# SVD issues
Found by diffing with the patch lpc82x.svd & comparing differences with the manual
## SWM
- TODO The PINASSIGN registers are defined two times, the second time with the
  name PINASSIGN_DATA and different field names. Not sure what to do about this.
  (They have an alternateGroup tag)
## DAC
- TODO check
## ADC
- TODO Check Until SEQ_CTRL field
## CMP
- The reset value of CTRL in the datasheet has two different definitons.
  In the resetMask this field is ignored
## INPUTMUX
- TODO
## CTIMER0
- TODO
## IOCON
- TODO
  PIO0_17/PIO0_13/PIO0_12/PIO0_5/PIO0_4/PIO0_3/PIO0_2/PIO0_16/PIO0_15/PIO0_1/PIO0_9/0_8/0_7(and
  probably all other faim bits for the PIO registers) resetMask (FAIM value
  dependent bit)
## SYSCON
- TODO until EXTTRACECMD
## CAPT
- TODO
## DMA
- TODO ENABLESET/ACTIVE0/BUSY0/ERRINT0/INTSET0/INTA0/INTB0 resetMask is wrong
## MTB
- TODO
## GPIO
- TODO B_/W_/PIN/MPIN/ resetMask is wrong


