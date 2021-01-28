      LJMP  START
      ORG  100H
START:
      MOV R3, #10 ;ustaw wartość początkową R3 na 10, żeby mieć po 5 włączeń i wyłączeń diody

LOOP_A:
      CPL  P1.7
      MOV  A,#5
      CALL  DELAY_100MS
      DJNZ R3, LOOP_A ;dekrementuj R3 i skocz do LOOP_A jeśli wartość R# nie jest równa 0

LOOP_B:
    SJMP LOOP_B