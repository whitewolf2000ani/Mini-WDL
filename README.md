## Mini-WDL Language Server Extension

Welcome to the Mini-WDL Language Server Extension! This project is a Visual Studio Code extension designed to enhance developer productivity for Workflow Description Language (WDL) authors. It integrates advanced language features like syntax highlighting, document symbol detection, and more using a Rust-based Language Server Protocol (LSP) implementation.

## Features
### Implemented
- Syntax Highlighting: Provides basic syntax highlighting for .wdl files using TextMate grammars.
- Document Symbol Detection: Detects tasks and workflows in .wdl files and displays them in the Outline view.

### Planned Features
- Go-to Definition: Navigate to the definition of tasks and workflows.
- Find References: Locate all references to tasks and workflows across documents.
- Semantic Code Highlighting: Highlight WDL constructs semantically.
- Diagnostics: Provide error checking and diagnostics for WDL files.

### Motivation
The Mini-WDL extension is to improve developer tooling for Workflow Description Language (WDL). It aims to provide a seamless development experience for workflow authors by integrating essential language features into VS Code.

### Installation
To install the Mini-WDL extension:

Clone the repository:

```bash
git clone https://github.com/<your-username>/mini-wdl-extension.git
```
Navigate to the project directory:
```bash
cd mini-wdl-extension
```
Install dependencies:
```bash
npm install
```

Build the extension:
```bash
npm run compile
```
Launch the Extension Development Host:
```
Open the project in VS Code.
Press F5 to start debugging.
```

### Usage
- Open a .wdl file in VS Code.
- The extension will automatically activate and provide syntax highlighting.
- Use the Outline view (Ctrl+Shift+O) to navigate tasks and workflows.


### How to Contribute
To bootsrap a developement enviroment, please use the following commands:

```
Clone the repository
git clone git@github.com:whitewolf2000ani/Mini-WDL.git

Create a new branch:
```bash
git checkout -b feature-name
```
Make your changes and commit them:
```bash
git commit -m "Add feature-name"
```
Push your changes and create a pull request.
```
License
This project is licensed under the MIT License.

Acknowledgments
This project is to improve WDL tooling through Sprocket's LSP integration within VSCode.

