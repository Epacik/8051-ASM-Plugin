SEG_ON EQU P1.6

ZERO EQU 03FH	;0
ONE EQU 006H	;1
TWO EQU 05BH	;2
THREE EQU 04FH	;3
FOUR EQU 066H	;4
FIVE EQU 01101101B	;5

DISPLAY EQU 30H

      LJMP  START
      ORG  100H


START:

      MOV  DISPLAY, #ZERO
      MOV  DISPLAY+1, #ONE
      MOV  DISPLAY+2, #TWO
      MOV  DISPLAY+3, #THREE
      MOV  DISPLAY+4, #FOUR
      MOV  DISPLAY+5, #FIVE

      MOV  R3, #00111111B

      MOV  R1, #DISPLAY
START2:
      MOV  R2, #0
LOOP:
      SETB  P1.6
      MOV  R0, #CSDS
      MOV  A, R3
      MOVX  @R0, A
      MOV  R0, #CSDB
    
      MOV  A, @R1
      MOVX  @R0, A
      CLR  P1.6
      MOV  A, #1
      CALL  DELAY_MS
      INC  R2
      CJNE  R2, #6, LOOP

      MOV A, #5
      CALL DELAY_100MS

      INC R1
      CJNE R1, #DISPLAY+5, START2
      DEC R1
      DEC R1
      DEC R1
      DEC R1
      DEC R1
      DEC R1

      SJMP  START2