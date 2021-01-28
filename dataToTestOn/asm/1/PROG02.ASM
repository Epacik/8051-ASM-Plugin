      LJMP  START
      ORG  100H
START:
      CPL  P1.5
      MOV  A,#5
      CALL  DELAY_100MS
      SJMP  START