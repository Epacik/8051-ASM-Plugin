ZNAKI     EQU    30H
    LJMP  START
    ORG  100H
START:
    MOV  ZNAKI, #01011011B
    MOV  R3, #111111B
START2:
    MOV  R2, #0
WYSW:
    SETB  P1.6
    MOV  R0, #CSDS
    MOV  A, R3
    MOVX  @R0, A
    MOV  R0, #CSDB
    ;MOV  R1, #ZNAKI
    ;MOV  A, @R1
    CALL WAIT_KEY

    MOVX  @R0, A
    CALL HEX_TO_7S
    CLR  P1.6
    MOV  A, #100
    CALL  DELAY_MS
    INC  R2
    CJNE  R2, #6, WYSW
    SJMP  START2

HEX_TO_7S:
    ; BRUTE FORCE TIME!
    ;czy to 0
    CJNE A, #0H, CHECK1
    MOV A, #00111111B
    RET
    ;czy to 1?
CHECK1:
    CJNE A, #1, CHECK2
    MOV A, #00000110B
    RET
CHECK2:
    CJNE A, #2, CHECK3
    MOV A, #01011011B
    RET
CHECK3:
    CJNE A, #3, CHECK4
    MOV A, #01001111B
    RET
CHECK4:
    CJNE A, #4, CHECK5
    MOV A, #01100110B
    RET
CHECK5:
    CJNE A, #5, CHECK6
    MOV A, #01101101B
    RET
CHECK6:
    CJNE A, #6, CHECK7
    MOV A, #01111101B
    RET
CHECK7:
    CJNE A, #7, CHECK8
    MOV A, #00000111B
    RET
CHECK8:
    CJNE A, #8, CHECK9
    MOV A, #01111111B
    RET
CHECK9:
    CJNE A, #9, CHECK10
    MOV A, #01101111B
    RET
CHECK10:
    CJNE A, #10, CHECK11
    MOV A, #01110111B
    RET
CHECK11:
    CJNE A, #11, CHECK12
    MOV A, #01111100B
    RET
CHECK12:
    CJNE A, #12, CHECK13
    MOV A, #00111001B
    RET
CHECK13:
    CJNE A, #13, CHECK14
    MOV A, #01011110B
    RET
CHECK14:
    CJNE A, #14, CHECK15
    MOV A, #01111001B
    RET
CHECK15:
    MOV A, #01110001B
    RET