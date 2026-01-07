#![allow(clippy::new_without_default)]
#![allow(dead_code)]

use serde_json::json;
use wit_bindgen::generate;

generate!({ path: "wit", world: "mcp-router" });

use exports::wasix::mcp::router::{
    ContentBlock, Guest, MetaEntry, ProgressNotification, Response, TextContent, Tool,
    ToolAnnotations, ToolError, ToolResult,
};

pub struct Router;

fn echo_tool(arguments: &str) -> Result<Response, ToolError> {
    let meta = Some(vec![MetaEntry {
        key: "echoed".to_string(),
        value: arguments.to_string(),
    }]);
    let content = vec![ContentBlock::Text(TextContent {
        text: arguments.to_string(),
        annotations: None,
    })];
    let result = ToolResult {
        content,
        structured_content: Some(arguments.to_string()),
        progress: Some(vec![ProgressNotification {
            progress: Some(1.0),
            message: Some("done".to_string()),
            annotations: None,
        }]),
        meta,
        is_error: Some(false),
    };
    Ok(Response::Completed(result))
}

impl Guest for Router {
    fn list_tools() -> Vec<Tool> {
        vec![Tool {
            name: "echo".to_string(),
            title: Some("Echo".to_string()),
            description: "Echo back the provided JSON".to_string(),
            input_schema: json!({}).to_string(),
            output_schema: Some(json!({}).to_string()),
            annotations: Some(ToolAnnotations {
                read_only: Some(true),
                destructive: None,
                streaming: None,
                experimental: None,
            }),
            meta: None,
        }]
    }

    fn call_tool(tool_name: String, arguments: String) -> Result<Response, ToolError> {
        match tool_name.as_str() {
            "echo" => echo_tool(&arguments),
            other => Err(ToolError::NotFound(format!("unknown tool {other}"))),
        }
    }
}

export!(Router);
