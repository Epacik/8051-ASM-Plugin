ZNAKI     EQU    30H
      LJMP  START
      ORG  100H
START:
    MOV ZNAKI, #01100110B
	MOV ZNAKI + 1, #00000110B
	MOV ZNAKI + 2, #01111101B
    MOV ZNAKI + 3, #01000000B
	MOV ZNAKI + 4, #00000110B
    MOV ZNAKI + 5, #01111101B
    
    MOV  R3, #00000001B

    MOV  R1, #ZNAKI
START2:
    MOV  R2, #0
WYSW:
    SETB  P1.6
    CALL CLEAR

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
    CJNE  R2, #6, WYSW

    MOV A, #1
    CALL DELAY_MS

    INC R1
    
    MOV A, R3
    RL A
    MOV R3, A
    CJNE R1, #ZNAKI+6, START2
    ;Cofnij do 0
    DEC R1
    DEC R1
    DEC R1
    DEC R1
    DEC R1
    DEC R1

    MOV A, R3
    RR A
    RR A
    RR A
    RR A
    RR A
    RR A
    MOV R3, A

    SJMP  START2

CLEAR:
    MOV  R0, #CSDS
    MOV  A, R3
    MOVX  @R0, A
    MOV  R0, #CSDB
    MOV A,  #00000000B
    MOVX  @R0, A
    RET