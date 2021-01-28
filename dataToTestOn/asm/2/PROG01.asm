      LJMP  START
      ORG  100H
START:
      SETB P1.7
      ACALL ABCD
      CLR  P1.5
STOP:

      SJMP  STOP
ABCD:
      CLR  P1.7
      RET