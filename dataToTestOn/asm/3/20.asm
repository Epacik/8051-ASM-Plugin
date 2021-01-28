TAB    EQU    30H
    LJMP  START
    ORG  100H
START:
    MOV TAB,#1
    MOV TAB+1,#2
    MOV TAB+2,#3
    MOV TAB+3,#5
    MOV TAB+4,#7
    MOV TAB+5,#11
    MOV TAB+6,#13
    MOV TAB+7,#17
    MOV TAB+8,#19
    MOV TAB+9,#23
    MOV TAB+10,#29
    MOV TAB+11,#31
    MOV TAB+12,#37
    MOV TAB+13,#41
    MOV TAB+14,#43
    MOV TAB+15,#47
    MOV TAB+16,#53
    MOV R0,#TAB
    CALL LCD_CLR
ABCD:
    MOV  A,@R0
    CALL HTB
    CALL WRITE_HEX
    MOV A,#5
    CALL DELAY_100MS
    CALL LCD_CLR
    INC  R0
    CJNE  R0,#TAB+17,ABCD
    MOV  R0,#TAB
    SJMP  ABCD
    XRL A, @R1
    CLR C
    CPL C
    SETB address
    ANL A, @R0
    ORL address, address
    
HTB:
	
    MOV R1,#00h 	
    MOV R2,#00h 	
    CJNE A,#00h,CALC_HTB 	
    RET 

CALC_HTB:
    MOV B,#100     ; dzielenie przez 100
    DIV AB     
    MOV R1,A     ; zapisz Akumulator do R0
    MOV A,B     
    MOV B,#10     ; podziel przez 10
    DIV AB     
    SWAP A     
    MOV R2,A     ; zapisz dziesiątki do R1
    MOV A,B     
    ORL A,R2     
    MOV R2,A     ;zapisz jedności do R1
    RET