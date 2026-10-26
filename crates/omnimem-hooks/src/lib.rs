//! Auto-capture hook installers for OmniMem.

use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// Supported agent hosts.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Agent {
    /// Claude Code.
    ClaudeCode,
    /// OpenAI Codex.
    Codex,
    /// Cursor editor.
    Cursor,
    /// Gemini CLI.
    GeminiCli,
    /// OpenCode.
    OpenCode,
}

impl Agent {
    /// Parse from a CLI-friendly name.
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "claude-code" | "claudecode" | "claude" => Some(Self::ClaudeCode),
            "codex" => Some(Self::Codex),
            "cursor" => Some(Self::Cursor),
            "gemini-cli" | "geminicli" | "gemini" => Some(Self::GeminiCli),
            "opencode" | "open-code" => Some(Self::OpenCode),
            _ => None,
        }
    }

    /// All supported agents.
    pub fn all() -> &'static [Self] {
        &[
            Self::ClaudeCode,
            Self::Codex,
            Self::Cursor,
            Self::GeminiCli,
            Self::OpenCode,
        ]
    }
}

/// A generated hook configuration snippet.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HookConfig {
    /// Agent this config targets.
    pub agent: String,
    /// JSON snippet to merge into the agent's settings.
    pub json: serde_json::Value,
}

impl HookConfig {
    /// Generate a Claude Code hooks snippet.
    pub fn claude_code() -> Self {
        let json = serde_json::json!({
            "hooks": {
                "SessionStart": [{"command": "omnimem capture --session-start"}],
                "UserPromptSubmit": [{"command": "omnimem capture --prompt"}],
                "PostToolUse": [{"command": "omnimem capture --tool-use"}],
                "Stop": [{"command": "omnimem capture --stop"}]
            }
        });
        Self {
            agent: "claude-code".to_string(),
            json,
        }
    }

    /// Generate a Codex hooks snippet.
    pub fn codex() -> Self {
        let json = serde_json::json!({
            "hooks": {
                "session_start": [{"command": "omnimem capture --session-start"}],
                "user_prompt": [{"command": "omnimem capture --prompt"}],
                "post_tool_use": [{"command": "omnimem capture --tool-use"}],
                "session_end": [{"command": "omnimem capture --stop"}]
            }
        });
        Self {
            agent: "codex".to_string(),
            json,
        }
    }

    /// Generate a Cursor hooks snippet.
    pub fn cursor() -> Self {
        let json = serde_json::json!({
            "version": 1,
            "hooks": {
                "beforeSubmitPrompt": [{"command": "omnimem capture --prompt"}],
                "afterFileEdit": [{"command": "omnimem capture --tool-use"}],
                "stop": [{"command": "omnimem capture --stop"}]
            }
        });
        Self {
            agent: "cursor".to_string(),
            json,
        }
    }

    /// Generate a Gemini CLI hooks snippet.
    pub fn gemini_cli() -> Self {
        let json = serde_json::json!({
            "hooks": {
                "SessionStart": [{"command": "omnimem capture --session-start"}],
                "BeforeAgent": [{"command": "omnimem capture --prompt"}],
                "AfterTool": [{"command": "omnimem capture --tool-use"}],
                "SessionEnd": [{"command": "omnimem capture --stop"}]
            }
        });
        Self {
            agent: "gemini-cli".to_string(),
            json,
        }
    }

    /// Generate an OpenCode hooks snippet.
    pub fn opencode() -> Self {
        let json = serde_json::json!({
            "hooks": {
                "session.created": [{"command": "omnimem capture --session-start"}],
                "message.updated": [{"command": "omnimem capture --prompt"}],
                "tool.execute.after": [{"command": "omnimem capture --tool-use"}],
                "session.idle": [{"command": "omnimem capture --stop"}]
            }
        });
        Self {
            agent: "opencode".to_string(),
            json,
        }
    }

    /// Generate a generic snippet for any agent.
    pub fn generic(agent: Agent) -> Self {
        let name = match agent {
            Agent::ClaudeCode => "claude-code",
            Agent::Codex => "codex",
            Agent::Cursor => "cursor",
            Agent::GeminiCli => "gemini-cli",
            Agent::OpenCode => "opencode",
        };
        let json = serde_json::json!({
            "agent": name,
            "note": "Merge into your agent's settings; see docs/hooks.md"
        });
        Self {
            agent: name.to_string(),
            json,
        }
    }
}

/// Install hooks for `agent` into `config_dir`. Returns the path written.
pub fn install(agent: Agent, config_dir: &PathBuf) -> std::io::Result<PathBuf> {
    let cfg = match agent {
        Agent::ClaudeCode => HookConfig::claude_code(),
        Agent::Codex => HookConfig::codex(),
        Agent::Cursor => HookConfig::cursor(),
        Agent::GeminiCli => HookConfig::gemini_cli(),
        Agent::OpenCode => HookConfig::opencode(),
    };
    std::fs::create_dir_all(config_dir)?;
    let path = config_dir.join(format!("omnimem-{}.json", cfg.agent));
    let content = serde_json::to_string_pretty(&cfg.json).unwrap_or_default();
    std::fs::write(&path, content)?;
    Ok(path)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_str_roundtrip() {
        assert_eq!(Agent::from_str("claude-code"), Some(Agent::ClaudeCode));
        assert_eq!(Agent::from_str("nope"), None);
    }

    #[test]
    fn claude_config_has_hooks() {
        let c = HookConfig::claude_code();
        assert!(c.json.get("hooks").is_some());
    }

    #[test]
    fn install_writes_file() {
        let dir = std::env::temp_dir().join("omnimem_hooks_test");
        let _ = std::fs::remove_dir_all(&dir);
        let p = install(Agent::ClaudeCode, &dir).unwrap();
        assert!(p.exists());
        let _ = std::fs::remove_dir_all(&dir);
    }
}