    LJMP       START             
    ORG       100H

; Jeśli linijkę przed etykietą umieścimy komentarz, to będzie on wyświetlony po najechaniu na etykietę
text EQU "123\x0D\x0A?565752"

; Taki komentarz
; może się rozciągać
; pomiędzy wieloma linijkami
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

; ## A gdyby tego było mało:
; ---
; ***Takie komentarze mają wsparcie dla języka Markdown***
; a co za tym idzie: *można dodać takiemu komentarzowi nieco stylu* 😎
; ---
; Można tutaj też umieszczać przykłady
; ```asm8051
; CALL TEST
; ```
TEST:
    MOV @A+DPTR, #43H
    RET
    