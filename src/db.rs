use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CommandModel {
    pub title: String,
    pub description: String,
    pub script: String,
    #[serde(default)]
    pub group: Option<String>,
    pub use_count: u32,
    #[serde(default)]
    pub favorite: bool,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GroupModel {
    pub name: String,
    pub description: String,
    #[serde(default)]
    pub commands: Vec<String>,
    #[serde(default)]
    pub use_count: u32,
    #[serde(default)]
    pub favorite: bool,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct HistoryModel {
    pub command_title: String,
    pub script: String,
    pub timestamp: String,
    pub duration_ms: u64,
    pub status: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Database {
    pub commands: Vec<CommandModel>,
    pub groups: Vec<GroupModel>,
    pub history: Vec<HistoryModel>,
}

pub fn db_path() -> std::path::PathBuf {
    let mut path = if let Ok(profile) = std::env::var("USERPROFILE") {
        std::path::PathBuf::from(profile)
    } else if let Ok(home) = std::env::var("HOME") {
        std::path::PathBuf::from(home)
    } else {
        std::env::current_dir().unwrap_or_default()
    };
    path.push(".aliace.json");
    path
}

impl Database {
    pub fn load() -> Self {
        let path = db_path();
        if path.exists() {
            if let Ok(content) = std::fs::read_to_string(&path) {
                if let Ok(db) = serde_json::from_str::<Database>(&content) {
                    return db;
                }
            }
        }
        Database {
            commands: vec![],
            groups: vec![],
            history: vec![],
        }
    }

    pub fn save(&self) -> Result<(), std::io::Error> {
        let path = db_path();
        let content = serde_json::to_string_pretty(self)?;
        std::fs::write(path, content)?;
        Ok(())
    }
}
