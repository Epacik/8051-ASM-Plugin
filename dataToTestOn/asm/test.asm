    LJMP       START             
    ORG       100H
START:
    MOV    R0,#30H             ;adres bufora wyboru wskaźnika        
    MOV    R1,#38H             ;adres bufora danych wskaźnika        
    MOV   R3, #23
    MOV       A,#01111110B             
    MOVX       @R0,A              ;wpisz wybrane wskaźniki        
    MOV       A,#00000110B              
    MOVX       @R1,A              ;wpisz wybrane segmenty              
    CLR       P1.6              ;włącz wyświetlacz 7-segm        
    SJMP       $   