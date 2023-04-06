    LJMP       START             
    ORG       100H

; lol it works!
text EQU "123\x0D\x0A?565752"

; let's test if that works
START:
    MOV    R0,#31H             ;adres bufora wyboru wskaźnika        
    MOV    R1,#38h             ;adres bufora danych wskaźnika        
    MOV   R3, #23
    MOV       A,#01111110b             
    MOVX       @R0,A              ;wpisz wybrane wskaźniki        
    MOV       A,#00000110B              
    MOVX       @R1,A              ;wpisz wybrane segmenty              
    CLR       P1.6              ;włącz wyświetlacz 7-segm        
    
    ADD A, @R0
    CALL TEST
    JMP START

; ### is that working?
;please tell me it is
; ---
; ```asm8051
; MOV A, 32
; ```
TEST:
    MOV @A+DPTR, #43H
    RET
    