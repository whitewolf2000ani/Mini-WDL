mod parser;
use parser::MiniWdlParser;
use tower_lsp::jsonrpc::Result;
use tower_lsp::lsp_types::*;
use tower_lsp::{Client, LanguageServer, LspService, Server};
use std::collections::HashMap;
use std::sync::Mutex;

#[derive(Debug)]
struct MiniWdlLanguageServer {
    client: Client,
    document_symbols: Mutex<HashMap<String, Vec<DocumentSymbol>>>,
    parser:MiniWdlParser,
}

#[tower_lsp::async_trait]
impl LanguageServer for MiniWdlLanguageServer {
    async fn initialize(&self, _: InitializeParams) -> Result<InitializeResult> {
        Ok(InitializeResult {
            capabilities: ServerCapabilities {
                document_symbol_provider: Some(OneOf::Left(true)),
                //text document sync capability
                text_document_sync: Some(TextDocumentSyncCapability::Options(
                    TextDocumentSyncOptions {
                        open_close: Some(true),  // Enable didOpen notifications
                        change: Some(TextDocumentSyncKind::FULL),
                        ..Default::default()
                    }
                )),
                ..ServerCapabilities::default()
            },
            ..InitializeResult::default()
        })
    }
    

    async fn initialized(&self, _: InitializedParams) {
        self.client
            .log_message(MessageType::INFO, "Mini-WDL Language Server initialized!")
            .await;
    }

    async fn shutdown(&self) -> Result<()> {
        Ok(())
    }
    
    async fn document_symbol(
        &self,
        params: DocumentSymbolParams,
    ) -> Result<Option<DocumentSymbolResponse>> {
        let uri = params.text_document.uri.to_string();
        let symbols = self.document_symbols.lock().unwrap();
        
        if let Some(symbols) = symbols.get(&uri) {
            return Ok(Some(DocumentSymbolResponse::Nested(symbols.clone())));
        }
        
        Ok(None)
    }

    async fn did_open(&self, params: DidOpenTextDocumentParams) {
        let uri = params.text_document.uri.to_string();
        let text = params.text_document.text;
        
        // Debug logging
        // self.client.log_message(MessageType::INFO, format!("Parsing document: {}", uri)).await;
        
        // Parse the document using our parser
        let symbols = self.parser.parse(&text);
        
        // Debug logging
        // self.client.log_message(MessageType::INFO, format!("Found {} symbols", symbols.len())).await;
        // if !symbols.is_empty() {
        //     self.client.log_message(MessageType::INFO, format!("First symbol: {:?}", symbols[0])).await;
        // }
        {
        // Store the symbols
        let mut symbol_map = self.document_symbols.lock().unwrap();
        symbol_map.insert(uri, symbols);
        }
        // self.client.log_message(MessageType::INFO, "Document opened and parsed").await;
    }


    async fn did_change(&self, params: DidChangeTextDocumentParams) {
        let uri = params.text_document.uri.to_string();
        
        // Get the latest content after changes
        if let Some(change) = params.content_changes.last() {
            let text = &change.text;
            
            // Re-parse the document and extract symbols
            let symbols = self.parser.parse(text);
            
            // Update the symbols
            let mut symbol_map = self.document_symbols.lock().unwrap();
            symbol_map.insert(uri, symbols);
        }
    }
}


#[tokio::main]
async fn main() {
    let stdin = tokio::io::stdin();
    let stdout = tokio::io::stdout();
    
    let (service, socket) = LspService::new(|client| MiniWdlLanguageServer {
        client,
        document_symbols: Mutex::new(HashMap::new()),
        parser:MiniWdlParser::new(),
    });
    
    Server::new(stdin, stdout, socket).serve(service).await;
}

