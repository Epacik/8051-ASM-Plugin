      LJMP  START
      ORG  100H
START:
      CPL  P1.7
      MOV  A, #2
      CALL  DELAY_100MS
      SJMP  START