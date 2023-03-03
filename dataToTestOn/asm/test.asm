    LJMP       START             
    ORG       100H
text EQU "123\x4?565752"
START:
    MOV    R0,#31H             ;adres bufora wyboru wskaźnika        
    MOV    R1,#38H             ;adres bufora danych wskaźnika        
    MOV   R3, #23
    MOV       A,#01111110B             
    MOVX       @R0,A              ;wpisz wybrane wskaźniki        
    MOV       A,#00000110B              
    MOVX       @R1,A              ;wpisz wybrane segmenty              
    CLR       P1.6              ;włącz wyświetlacz 7-segm        
    SJMP       $   
    
    ADD A, @R0
    CALL TEST

TEST:
    MOV @A+DPTR, #43H
    RET
    