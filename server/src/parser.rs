use tower_lsp::lsp_types::*;

#[derive(Debug)]
pub struct MiniWdlParser {}

impl MiniWdlParser {
    pub fn new() -> Self {
        MiniWdlParser {}
    }
    
    pub fn parse(&self, text: &str) -> Vec<DocumentSymbol> {
        let mut symbols = Vec::new();
        
        // Simple line-by-line parsing
        for (line_idx, line) in text.lines().enumerate() {
            let trimmed = line.trim();
            
            // Skip empty lines and comments
            if trimmed.is_empty() || trimmed.starts_with('#') {
                continue;
            }
            
            // Find task definitions
            if trimmed.starts_with("task ") {
                let parts: Vec<&str> = trimmed.split_whitespace().collect();
                if parts.len() >= 2 {
                    let name = parts[1].trim_end_matches('{').trim();
                    
                    // Create a symbol for the task
                    symbols.push(DocumentSymbol {
                        name: name.to_string(),
                        detail: Some("task".to_string()),
                        kind: SymbolKind::FUNCTION,
                        range: Range {
                            start: Position { line: line_idx as u32, character: 0 },
                            end: Position { line: line_idx as u32, character: line.len() as u32 },
                        },
                        selection_range: Range {
                            start: Position { line: line_idx as u32, character: line.find("task ").unwrap_or(0) as u32 + 5 },
                            end: Position { line: line_idx as u32, character: (line.find("task ").unwrap_or(0) as u32 + 5 + name.len() as u32) },
                        },
                        children: None,
                        tags: None,
                        deprecated: None,
                    });
                }
            }
            
            // Find workflow definitions
            else if trimmed.starts_with("workflow ") {
                let parts: Vec<&str> = trimmed.split_whitespace().collect();
                if parts.len() >= 2 {
                    let name = parts[1].trim_end_matches('{').trim();
                    
                    // Create a symbol for the workflow
                    symbols.push(DocumentSymbol {
                        name: name.to_string(),
                        detail: Some("workflow".to_string()),
                        kind: SymbolKind::NAMESPACE,
                        range: Range {
                            start: Position { line: line_idx as u32, character: 0 },
                            end: Position { line: line_idx as u32, character: line.len() as u32 },
                        },
                        selection_range: Range {
                            start: Position { line: line_idx as u32, character: line.find("workflow ").unwrap_or(0) as u32 + 9 },
                            end: Position { line: line_idx as u32, character: (line.find("workflow ").unwrap_or(0) as u32 + 9 + name.len() as u32) },
                        },
                        children: None,
                        tags: None,
                        deprecated: None,
                    });
                }
            }
        }
        
        symbols
    }
}
