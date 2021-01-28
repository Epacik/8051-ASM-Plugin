      LJMP  START
      ORG  100H
START:
      CALL  LCD_CLR
KLAWISZ:
      CALL  WAIT_KEY
      MOV  R2, A
      CALL  LCD_CLR
      MOV  A, R2

      CALL HTB ;wysyłam 2 razy to samo, bo od razu zrobiłem jako podprogram

      CALL  WRITE_HEX

      SJMP  KLAWISZ


HTB:
	
    MOV R0,#00h 	
    MOV R1,#00h 	
    CJNE A,#00h,CALC_HTB 	
    RET 	
	

CALC_HTB:
	
    ;DIV C 	
    MOV B,#100 	; dzielenie przez 100
    DIV AB 	
    MOV R0,A 	; zapisz Akumulator do R0
    ;DIV C 	
    MOV A,B 	
    MOV B,#10 	; podziel przez 10
    DIV AB 	
    SWAP A 	
    MOV R1,A 	; zapisz dziesiątki do R1
    MOV A,B 	
    ORL A,R1 	
    MOV R1,A 	;zapisz jedności do R1
    RET