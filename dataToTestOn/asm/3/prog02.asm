    LJMP  START
    ORG  100H
START:
    CALL  LCD_CLR
KLAWISZ:

    MOV A, #0
    CALL  WAIT_KEY ; 'lewa' cyfra

    ;Przesuń bity w akumulatorze w lewo
    RL A
    RL A
    RL A
    RL A

    ;przenieś zawartość do R3
    MOV  R3, A

    ; pobierz prawą cyfrę
    CALL  WAIT_KEY
    
    ; bitwise or na A i R3 żeby połączyć 2 cyfry w liczbę 2 cyfrową
    ORL A, R3

    MOV  R2, A
    CALL  LCD_CLR
    MOV  A, R2

    CALL HTB 
    CALL  WRITE_HEX

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

