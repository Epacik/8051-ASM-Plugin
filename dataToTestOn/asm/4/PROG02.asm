MRUG      EQU    00100100B
    LJMP  START
    ORG  100H
START:
    MOV  R2, #MRUG
    
DIODA:
    CLR C
    MOV  P1.7, C
    MOV  A, #4
    CALL  DELAY_100MS
    
    SETB C
    MOV  P1.7, C
    MOV  A, #12
    CALL  DELAY_100MS
    
    SJMP  DIODA