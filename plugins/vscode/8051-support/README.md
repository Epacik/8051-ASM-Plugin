# 8051 Microcontroller Assembly Tools

Adds language support for 8051 microcontroller assembly, including documentation on hover, syntax highlighting and handy built in documentation.

Made as a part of my Engineering Thesis.

## Features

##### Built-in documentation

This extension includes a handy built in documentation to help you with writing programs. This also includes an documentation on hover.

![Built-in documentation](https://raw.githubusercontent.com/Epacik/8051-ASM-Plugin/main/plugins/vscode/images/docs.png)

##### Documenting your code!

Comments right above labels are turned into documentation that can be shown on hover.

## Extension Settings

This extension contributes the following settings:

* `asm8051.maxNumberOfProblems`: will control how many errors/warnings are shown (currently does nothing)
* `asm8051.trace.server`: controls verbosity of server trace, 
  values:
  * off
  * messages
  * verbose 
* `asm8051.kit`: changes things that are included in microcontroller/development board (currently does nothing)
  values:
  * 8051
  * DSM-51

# 

## Known Issues

Opening "8051 Documentation" side panel too soon after launch may result in empty list.

## Release Notes

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

-----------------------------------------------------------------------------------------------------------
