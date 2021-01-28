MRUG      EQU    00100100B
      LJMP  START
      ORG  100H
START:
      MOV  R2, #MRUG
      SETB C
DIODA:
      MOV  P1.7, C
      MOV  A, #5
      CALL  DELAY_100MS
      MOV  A, R2
      RLC  A
      MOV  R2, A
      SJMP  DIODA