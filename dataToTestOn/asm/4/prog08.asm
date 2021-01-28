
    LJMP START
    ORG 100H
START:

    MOV A, #0
    CALL WAIT_KEY ; 'lewa' cyfra

    MOV B, #10
    MUL AB
    

    ;przenieś zawartość do R3
    MOV  R3, A

    ; pobierz prawą cyfrę
    CALL  WAIT_KEY
    
    ADD A, R3

    MOV  R2, A
    CALL  LCD_CLR
    MOV  A, R2

    MOV R5, A
    MOV A, #0

    CALL  WAIT_KEY ; 'lewa' cyfra

    MOV B, #10
    MUL AB
    

    ;przenieś zawartość do R3
    MOV  R3, A

    ; pobierz prawą cyfrę
    CALL WAIT_KEY
    
    ADD A, R3

    MOV  R2, A
    CALL  LCD_CLR
    MOV  A, R2
    MOV  R6, A
    MOV A, R5
    SUBB A, R6
    CALL HTB 
    CALL WRITE_HEX

KLAWISZ:
    SJMP KLAWISZ


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


NUM_1:
    
    RET