      LJMP START
      ORG 100H
START:
      CALL LCD_CLR
      
KLAWISZ:
      MOV B, #2
      CALL WAIT_KEY
      MOV R2, A
      CALL LCD_CLR
      MOV A, R2
      NOP
      CALL MRUGAJ
      CALL WRITE_HEX
      SJMP KLAWISZ
      


MRUGAJ:
      ;Mnożenie wartości klawisza przez B, którego wartość to 2, żeby mieć poprawną ilość iteracji
      MUL AB
      MOV R2, A
      
      ;ustaw wartość początkową R3 na 6, żeby mieć po 3 włączeń i wyłączeń diody
      MOV A, R2
      MOV R3, A

MRUGAJ_LOOP:
      CPL P1.7
      MOV A,#5
      CALL DELAY_100MS
      DJNZ R3, MRUGAJ_LOOP ;dekrementuj R3 i skocz do LOOP_A jeśli wartość R3 nie jest równa 0

      RET
