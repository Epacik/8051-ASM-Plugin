      LJMP  START
      ORG  100H
START:
      CPL  P1.7
      MOV  A,#250
      CALL  DELAY_MS
      SJMP  START