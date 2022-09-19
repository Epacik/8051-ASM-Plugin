    LJMP  START
    ORG  100H
START:
    MOV A, #0
    MOV R1, #1
LOOP:
    ADD A, R1
    ADDC A, R2
    SUBB A, R3
    CALL WRITE_HEX
    SJMP LOOP
    DEC R7
    RET ;🤨🤨🤨a̐éö̲\r\n

