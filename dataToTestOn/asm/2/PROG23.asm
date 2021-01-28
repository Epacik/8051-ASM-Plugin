      LJMP  START
      ORG  100H
START:
      MOV R3, #10

LOOP_A:
      CPL P1.7
      MOV A, #2
      CALL DELAY_100MS
      DEC R3
      CJNE R3, #0, LOOP_A

LOOP_B:
    SJMP LOOP_B

#33
