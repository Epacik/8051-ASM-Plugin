      LJMP  START
      ORG  100H
START:
      CLR  P1.7
; infinite loop that does nothing
STOP:
      SJMP  STOP
