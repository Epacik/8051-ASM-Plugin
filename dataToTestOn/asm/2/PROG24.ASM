      LJMP  START
      ORG  100H
START:
      CALL  LCD_CLR
KLAWISZ:
      CALL  WAIT_KEY
      MOV  R2, A
      CALL  LCD_CLR
      MOV  A, R2
      CALL  WRITE_HEX
      SJMP  KLAWISZ