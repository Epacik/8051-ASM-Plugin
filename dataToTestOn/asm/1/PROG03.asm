      LJMP  START
      ORG  100H

START:
      CPL  P1.5
      MOV  A,#5
      CALL  DELAY_100MS
LOOP:
      CPL  P1.5
      MOV  A,#5
      CPL  P1.7
      MOV  B,#7
      CALL  DELAY_100MS
      
      SJMP  LOOP