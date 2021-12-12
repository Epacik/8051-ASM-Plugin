use crate::types::Documentation;

#[allow(dead_code)]
pub struct Eng {

}

impl Eng {

    pub const ADD: Documentation = Documentation {
        title: "ADD",
        detail: "Add to accumulator",
        description: "Adds a byte value to a value stored in the accumulator, and stores the results back in the accumulator.",
        syntax: r#" ADD A,  [operand]
 ADD A,  #41H
 ADD A,  05H
 ADD A,  @R0
 ADD A,  R2"#,
        affected_flags: r#"- **Carry** set if result exceedes 255, cleared if it does not
- **Auxillary Carry** set if result exceedes 15, cleared if it does not
- **Overflow** set if result is out of signed byte value (-128 trough 127), cleared if it does not"#,
        valid_operands: r#"- #number (e.g. #41H, #100101B)
- Internal ram address (e.g. 05H),
- Address stored in register @R0 or @R1
- Register R0 trough R7"#
    };

    pub const ADDC: Documentation = Documentation {
        title: "ADDC",
        detail: "Add to Accumulator with Carry flag",
        description: "Adds a byte value and a carry flag to the accumulator, and stores the results in the accumulator.",
        syntax: r#" ADDC A,  [operand]
 ADDC A,  #41H
 ADDC A,  05H
 ADDC A,  @R0
 ADDC A,  R2"#,
        valid_operands: r#"- #number (e.g. #41H)
- Internal ram address (e.g. 05H),
- Register with a data address @R0, @R1
- Register R0 trough R7"#,
        affected_flags: r#"- **Carry** set if result exceedes 255, cleared if it does not
- **Auxillary Carry** set if result exceedes 15, cleared if it does not
- **Overflow** set if result is out of signed byte value (-128 trough 127), cleared if it does not"#,
    };
    
    pub const SUBB: Documentation = Documentation {
        title: "SUBB",
        detail: "Subtract from Accumulator With Borrow",
        description: "Subtract the value of operand and the Carry Flag from the value of the Accumulator, and stores the results in the accumulator.",
        syntax: r#" SUBB A, [operand]
 SUBB A, #41H
 SUBB A, 05H
 SUBB A, @R0
 SUBB A, R2"#,
        affected_flags: "- **Carry** set if operand value was greater than Accumulator value, cleared if it was not
- **Auxillary Carry** set if lower nibble of operand (bits 0 trough 3) was greater than value of lower nibble of an Accumulator, cleared if it was not
- **Overflow** set if result is out of signed byte value (-128 trough 127), cleared if it does not",
        valid_operands: "- #number (e.g. #41H)
- Internal ram address (e.g. 05H),
- Register with a data address @R0, @R1
- Register R0 trough R7"
    };
}

#[allow(dead_code)]
pub struct Pol {

}

impl Pol {

    pub const ADD: Documentation = Documentation {
        title: "ADD",
        detail: "Dodaj do Akumulatora",
        description: "Dodaje bajt do wartości przechowywanej w Akumulatorze po czym zapisuje wynik tego działania w Akumulatorze",
        syntax: r#" ADD A,  [operand]
 ADD A,  #41H
 ADD A,  05H
 ADD A,  @R0
 ADD A,  R2"#,
        affected_flags: r#"- **Przeniesienie** ustawiona jeśli wynik przekroczył 255, w przeciwnym wypadku wyczyszczona
- **Przeniesienie Pomocnicze** ustawiona jeśli wynik przekroczył 15, w przeciwnym wypadku wyczyszczona
- **Przepełnienie** ustawiona jeśli wynik interpretowany jako bajt ze znakiem przekroczy jego zakres (od -128 do 127), w przeciwnym wypadku wyczyszczona"#,
        valid_operands: r#"- #liczba (e.g. #41H, #100101B)
- Wewnętrzny adres RAM (e.g. 05H),
- Adres przechowywany w rejestrze @R0 lub @R1
- Rejestry od R0 do R7"#
    };

    pub const ADDC: Documentation = Documentation {
        title: "ADDC",
        detail: "Dodaj do Akumulatora używając flagi przeniesienia",
        description: "Dodaje bajt oraz wartość flagi Przeniesienia do wartości przechowywanej w Akumulatorze po czym zapisuje wynik tego działania w Akumulatorze",
        syntax: r#" ADDC A,  [operand]
 ADDC A,  #41H
 ADDC A,  05H
 ADDC A,  @R0
 ADDC A,  R2"#,
        affected_flags: r#"- **Przeniesienie** ustawiona jeśli wynik przekroczył 255, w przeciwnym wypadku wyczyszczona
- **Przeniesienie Pomocnicze** ustawiona jeśli wynik przekroczył 15, w przeciwnym wypadku wyczyszczona
- **Przepełnienie** ustawiona jeśli wynik interpretowany jako bajt ze znakiem przekroczy jego zakres (od -128 do 127), w przeciwnym wypadku wyczyszczona"#,
        valid_operands: r#"- #liczba (e.g. #41H, #100101B)
- Wewnętrzny adres RAM (e.g. 05H),
- Adres przechowywany w rejestrze @R0 lub @R1
- Rejestry od R0 do R7"#
    };

    pub const SUBB: Documentation = Documentation {
        title: "SUBB",
        detail: "Odejmij od Akumulatora używając flagi pożyczenia",
        description: "Odejmij bajt oraz flagę pożyczenia (przeniesienia) od wartości Akumulatora po czym zapisz wynik w Akumulatorze",
        syntax: r#" SUBB A, [operand]
 SUBB A, #41H
 SUBB A, 05H
 SUBB A, @R0
 SUBB A, R2"#,
        affected_flags: "- **Przeniesienia** ustawiona jeśli odejmowana wartość była wyższa niż wartość Akumulatora, w przeciwnym wypadku wyczyszczona
- **Przeniesienie Pomocnicze** ustawiona jeśli wartość dolnego półbajtu odejmowanej wartości (bity od 0 do 3) była większa niż wartość dolnego półbajtu Akumulatora, w przeciwnym wypadku wyczyszczona
- **Przepełnienie** ustawiona jeśli wynik interpretowany jako bajt ze znakiem przekroczy jego zakres (od -128 do 127), w przeciwnym wypadku wyczyszczona",
        valid_operands: r#"- #liczba (e.g. #41H, #100101B)
- Wewnętrzny adres RAM (e.g. 05H),
- Adres przechowywany w rejestrze @R0 lub @R1
- Rejestry od R0 do R7"#
    };
}