      LJMP  START
      ORG  100H
START:
      CPL P1.5
      MOV A,#5
      MOV B, #10h
      MUL AB
      ADD A, B
      MOV DPTR, #1234h
      MOV R1, #32h
      MOV A, @R1
      CALL DELAY_100MS
      SJMP START
