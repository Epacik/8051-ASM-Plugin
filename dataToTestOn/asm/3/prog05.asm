TAB	EQU	30H
	LJMP  START
	ORG  100H
START:
	MOV  TAB,#2
	MOV  TAB+1,#3
	MOV  TAB+2,#5
    MOV  TAB+3,#7
	MOV  TAB+4,#11h
    MOV  TAB+5,#13h
    MOV  TAB+6,#17h
	MOV  TAB+7,#19h
    MOV  TAB+8,#23h
	MOV  TAB+9,#29h
    MOV  TAB+10,#31h
    MOV  TAB+11,#37h
	MOV  TAB+12,#41h
    MOV  TAB+13,#43h
	MOV  TAB+14,#47h
    MOV  TAB+15,#53h
 	MOV  R0,#TAB
 	CALL  LCD_CLR
ABCD:
    CALL  WAIT_KEY
    MOV R0, A
 	MOV  A,@R0
 	CALL  WRITE_HEX
 	MOV  A,#5
 	CALL  DELAY_100MS
 	CALL  LCD_CLR
 	;INC  R0
 	SJMP ABCD
