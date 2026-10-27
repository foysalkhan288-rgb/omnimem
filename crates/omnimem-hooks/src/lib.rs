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
    /// Windsurf (Codeium).
    Windsurf,
    /// Kiro (AWS).
    Kiro,
    /// Antigravity.
    Antigravity,
    /// Trae.
    Trae,
    /// OpenClaw.
    OpenClaw,
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
            "windsurf" => Some(Self::Windsurf),
            "kiro" => Some(Self::Kiro),
            "antigravity" => Some(Self::Antigravity),
            "trae" => Some(Self::Trae),
            "openclaw" | "open-claw" => Some(Self::OpenClaw),
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
            Self::Windsurf,
            Self::Kiro,
            Self::Antigravity,
            Self::Trae,
            Self::OpenClaw,
        ]
    }

    /// Agent name string.
    pub fn name(&self) -> &'static str {
        match self {
            Self::ClaudeCode => "claude-code",
            Self::Codex => "codex",
            Self::Cursor => "cursor",
            Self::GeminiCli => "gemini-cli",
            Self::OpenCode => "opencode",
            Self::Windsurf => "windsurf",
            Self::Kiro => "kiro",
            Self::Antigravity => "antigravity",
            Self::Trae => "trae",
            Self::OpenClaw => "openclaw",
        }
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
        Self { agent: "claude-code".to_string(), json }
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
        Self { agent: "codex".to_string(), json }
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
        Self { agent: "cursor".to_string(), json }
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
        Self { agent: "gemini-cli".to_string(), json }
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
        Self { agent: "opencode".to_string(), json }
    }

    /// Generate a Windsurf hooks snippet.
    pub fn windsurf() -> Self {
        let json = serde_json::json!({
            "hooks": {
                "pre_user_prompt": [{"command": "omnimem capture --prompt"}],
                "post_tool_use": [{"command": "omnimem capture --tool-use"}],
                "session_end": [{"command": "omnimem capture --stop"}]
            }
        });
        Self { agent: "windsurf".to_string(), json }
    }

    /// Generate a Kiro hooks snippet.
    pub fn kiro() -> Self {
        let json = serde_json::json!({
            "hooks": {
                "onPrompt": [{"command": "omnimem capture --prompt"}],
                "onToolUse": [{"command": "omnimem capture --tool-use"}],
                "onSessionEnd": [{"command": "omnimem capture --stop"}]
            }
        });
        Self { agent: "kiro".to_string(), json }
    }

    /// Generate an Antigravity hooks snippet.
    pub fn antigravity() -> Self {
        let json = serde_json::json!({
            "hooks": {
                "prompt": [{"command": "omnimem capture --prompt"}],
                "tool_use": [{"command": "omnimem capture --tool-use"}],
                "stop": [{"command": "omnimem capture --stop"}]
            }
        });
        Self { agent: "antigravity".to_string(), json }
    }

    /// Generate a Trae hooks snippet.
    pub fn trae() -> Self {
        let json = serde_json::json!({
            "hooks": {
                "onPromptSubmit": [{"command": "omnimem capture --prompt"}],
                "onToolUse": [{"command": "omnimem capture --tool-use"}],
                "onStop": [{"command": "omnimem capture --stop"}]
            }
        });
        Self { agent: "trae".to_string(), json }
    }

    /// Generate an OpenClaw hooks snippet.
    pub fn openclaw() -> Self {
        let json = serde_json::json!({
            "hooks": {
                "prompt": [{"command": "omnimem capture --prompt"}],
                "tool": [{"command": "omnimem capture --tool-use"}],
                "stop": [{"command": "omnimem capture --stop"}]
            }
        });
        Self { agent: "openclaw".to_string(), json }
    }

    /// Generate a generic snippet for any agent.
    pub fn generic(agent: Agent) -> Self {
        let name = agent.name();
        let json = serde_json::json!({
            "agent": name,
            "note": "Merge into your agent's settings; see docs/hooks.md"
        });
        Self { agent: name.to_string(), json }
    }

    /// Generate config for the given agent.
    pub fn for_agent(agent: Agent) -> Self {
        match agent {
            Agent::ClaudeCode => Self::claude_code(),
            Agent::Codex => Self::codex(),
            Agent::Cursor => Self::cursor(),
            Agent::GeminiCli => Self::gemini_cli(),
            Agent::OpenCode => Self::opencode(),
            Agent::Windsurf => Self::windsurf(),
            Agent::Kiro => Self::kiro(),
            Agent::Antigravity => Self::antigravity(),
            Agent::Trae => Self::trae(),
            Agent::OpenClaw => Self::openclaw(),
        }
    }
}

/// Install hooks for `agent` into `config_dir`. Returns the path written.
pub fn install(agent: Agent, config_dir: &PathBuf) -> std::io::Result<PathBuf> {
    let cfg = HookConfig::for_agent(agent);
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
    fn all_agents_have_config() {
        for agent in Agent::all() {
            let cfg = HookConfig::for_agent(*agent);
            assert!(!cfg.agent.is_empty());
        }
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