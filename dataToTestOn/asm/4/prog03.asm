    LJMP  START
    ORG  100H
START:
    CALL  LCD_CLR
KLAWISZ:
    CALL  WAIT_KEY
    RRC A
    CPL C
    MOV  P1.7, C
    SJMP  KLAWISZ