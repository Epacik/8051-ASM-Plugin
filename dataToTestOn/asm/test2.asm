    LJMP  START
    ORG  100H
START:
    MOV A, #0
    MOV R1, #1
LOOP:
    ADD A, R1
    CALL WRITE_HEX
    SJMP LOOP
    DEC R7
    RET 

