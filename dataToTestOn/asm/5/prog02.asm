ZNAKI     EQU    30H
      LJMP  START
      ORG  100H
START:
      MOV  ZNAKI, #01011011B
      MOV  R3, #100011B
START2:
      MOV  R2, #0
WYSW:
      SETB  P1.6
      MOV  R0, #CSDS
      MOV  A, R3
      MOVX  @R0, A
      MOV  R0, #CSDB
      ;MOV  R1, #ZNAKI
      MOV  A, ZNAKI
      MOVX  @R0, A
      CLR  P1.6
      MOV  A, #1
      CALL  DELAY_MS
      INC  R2
      CJNE  R2, #6, WYSW
      SJMP  START2