      LJMP  START
      ORG  100H
START:
      CALL  LCD_CLR
      MOV A, #0
LOOP:
    MOV R2, A ; zapisanie heksadecymalnej wartości w R2, żeby przeliczenie na BCD nie zniszczyło jej
    CALL HTB 
    CALL  WRITE_HEX

    MOV A, R2 ; przywróć wartość do A i inkrementuj ją
    INC A 

    CJNE A, #64H, AFTER_RESET ; Jeśli dotrzemy do 99/64H to zresetuj A, jeśli nie skocz do AFTER_RESET
    MOV A, #0

AFTER_RESET:

    MOV R2, A
    MOV  A, #5 ;poczekaj 500ms
    CALL DELAY_100MS

    CALL  LCD_CLR ;wyczyść ekran
    MOV  A, R2

    SJMP  LOOP


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