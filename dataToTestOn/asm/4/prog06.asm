ARR	EQU	30H
    LJMP  START
    ORG  100H
START:
    MOV  ARR,#1
	MOV  ARR+1,#2
	MOV  ARR+2,#3
    MOV  ARR+3,#4
	MOV  ARR+4,#5
    MOV  ARR+5,#6
    MOV  ARR+6,#7
	MOV  ARR+7,#8
    MOV  ARR+8,#9
	MOV  ARR+9,#10

    MOV R0, #ARR
    CALL LCD_CLR
    MOV A, #0

LOOP:
    CJNE R0, #ARR+10, CALC
    SJMP END_LOOP
CALC:
    ADD A, @R0
    INC R0
    SJMP LOOP
END_LOOP:

    CALL HTB 
    CALL WRITE_HEX

KLAWISZ:
    SJMP  KLAWISZ


HTB:
    MOV R0,#00h 	
    MOV R1,#00h 	
    CJNE A,#00h,CALC_HTB 	
    RET
	

CALC_HTB:
    MOV B,#100 	; dzielenie przez 100
    DIV AB 	
    MOV R0,A 	; zapisz Akumulator do R0
    MOV A,B 	
    MOV B,#10 	; podziel przez 10
    DIV AB 	
    SWAP A 	
    MOV R1,A 	; zapisz dziesiątki do R1
    MOV A,B 	
    ORL A,R1 	
    MOV R1,A 	;zapisz jedności do R1
    RET