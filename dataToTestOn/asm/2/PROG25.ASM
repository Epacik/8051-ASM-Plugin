      LJMP  START
      ORG  100H
START:
      CALL  LCD_CLR
KLAWISZ:
      CALL  WAIT_KEY
      CALL  MRUGAJ
      SJMP  KLAWISZ


MRUGAJ:
      MOV R3, #6 ;ustaw wartość początkową R3 na 6, żeby mieć po 3 włączeń i wyłączeń diody
MRUGAJ_LOOP:
      CPL  P1.7
      MOV  A,#5
      CALL  DELAY_100MS
      DJNZ R3, MRUGAJ_LOOP ;dekrementuj R3 i skocz do LOOP_A jeśli wartość R# nie jest równa 0

      RET
