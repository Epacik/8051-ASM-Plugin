      LJMP  START
      ORG  100H
START:
      CALL  LCD_CLR
KLAWISZ:
      CALL  WAIT_KEY
      MOV  R2, A
      CALL  LCD_CLR
      MOV  A, R2
      CJNE R3, #15, ENTER
      CJNE R3, #14, ESC
      SJMP  KLAWISZ

ESC:


ENTER:
