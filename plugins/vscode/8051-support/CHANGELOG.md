## Release Notes

### 0.4.2

- Disabled mnemonic validity checks, because of too many false positives

### 0.4.1

- fix translations

### 0.4.0

- Respecting "kit" setting
  - Disabling documentation for DSM-51 if it's not a selected kit
  - Disabling hovers for DSM-51 subprocedures if DSM-51 is not a selected kit
- Checking validity of mnemonics
- Translation updates

### 0.3.0

- Amount of bytes on stack are now showed for DSM-51 subroutines

- Used and changed registers are now showed for DSM-51 subroutines

- fixed hover for DSM-51 subroutines

### 0.2.6

- Fixed script a documentation page script

### 0.2.3

- fixed parsing non-decimal numbers

### 0.2.2

- Optimizing size of plugin

- fixed failure to load documentation list in a sidebar

- packaging language server with webpack

### 0.2.0

- A lexer was added to improve parsing edited documents

- Symbol user is hovering over is now resolved by a lexer

- Documentation comments for labels are now being resolved by a lexer

### 0.1.0

Initial release:

- Built in documentation of 8051 microcontroller and DSM-51 learning kit

- Comments above labels are treated as documentation of those labels

- Assembly syntax