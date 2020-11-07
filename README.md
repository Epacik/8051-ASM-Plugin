# 8051-ASM-Plugin

Syntax highlighting, snippets and documentation for 8051 Assembly language

## Structure

```
.
├── client // Language Client
│   ├── src
│   │   ├── test // End to End tests for Language Client / Server
│   │   └── extension.ts // Language Client entry point
├── package.json // The extension manifest.
└── server // Language Server
    └── src
        └── server.ts // Language Server entry point
```

## Running

- Run `npm install` in this folder. This installs all necessary npm modules in both the client and server folder
- Open VS Code on this folder.
- Press Ctrl+Shift+B to compile the client and server.
- Switch to the Debug viewlet.
- Select `Launch Client` from the drop down.
- Run the launch config.
- If you want to debug the server as well use the launch configuration `Attach to Server`
- In the [Extension Development Host] instance of VSCode, open a document in '8051 Assembly' language mode.

#### This extention is based on LSP Extention example by Microsoft (https://github.com/microsoft/vscode-extension-samples/tree/master/lsp-sample)[https://github.com/microsoft/vscode-extension-samples/tree/master/lsp-sample]
