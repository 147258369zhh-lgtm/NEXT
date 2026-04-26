use anyhow::{Context, Result};
use nexus_protocol::{RiskLevel, TaskRecord, TaskStatus};
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use std::fs;
use uuid::Uuid;
use chrono::Utc;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkillMetadata {
    pub name: String,
    pub version: String,
    pub author: Option<String>,
    pub description: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Skill {
    pub id: String,
    pub metadata: SkillMetadata,
    pub triggers: Vec<String>,
    pub actions: Vec<String>,
    pub risk_level: RiskLevel,
    pub execution_mode: nexus_protocol::ExecutionMode,
    pub path: String,
}

pub struct SkillManager {
    pub base_dir: PathBuf,
    pub registry: HashMap<String, Skill>,
}

impl SkillManager {
    pub fn new(base_dir: impl AsRef<Path>) -> Self {
        Self {
            base_dir: base_dir.as_ref().to_path_buf(),
            registry: HashMap::new(),
        }
    }

    /// 扫描技能文件夹
    pub fn scan(&mut self) -> Result<Vec<Skill>> {
        let mut skills = Vec::new();
        if !self.base_dir.exists() {
            fs::create_dir_all(&self.base_dir).context("Failed to create skills directory")?;
        }

        self.scan_recursive(&self.base_dir, &mut skills)?;
        
        self.registry.clear();
        for skill in &skills {
            self.registry.insert(skill.id.clone(), skill.clone());
        }

        Ok(skills)
    }

    fn scan_recursive(&self, dir: &Path, acc: &mut Vec<Skill>) -> Result<()> {
        if dir.is_dir() {
            for entry in fs::read_dir(dir)? {
                let entry = entry?;
                let path = entry.path();
                if path.is_dir() {
                    self.scan_recursive(&path, acc)?;
                } else if path.extension().map_or(false, |ext| ext == "skill") {
                    if let Ok(skill) = self.load_skill(&path) {
                        acc.push(skill);
                    }
                }
            }
        }
        Ok(())
    }

    fn load_skill(&self, path: &Path) -> Result<Skill> {
        let content = fs::read_to_string(path).context("Failed to read skill file")?;
        let raw: serde_yaml::Value = serde_yaml::from_str(&content).context("Failed to parse YAML")?;
        
        // 尝试从 YAML 中提取基本信息
        let name = raw["name"].as_str().unwrap_or("Unknown Skill").to_string();
        let version = raw["version"].as_str().unwrap_or("0.1.0").to_string();
        
        Ok(Skill {
            id: Uuid::new_v4().to_string(),
            metadata: SkillMetadata {
                name,
                version,
                author: raw["author"].as_str().map(|s| s.to_string()),
                description: raw["description"].as_str().map(|s| s.to_string()),
            },
            triggers: vec![], 
            actions: vec![],
            risk_level: RiskLevel::L1,
            execution_mode: nexus_protocol::ExecutionMode::Agent,
            path: path.to_string_lossy().to_string(),
        })
    }

    /// 将技能转换为可执行任务（占位逻辑）
    pub fn to_task_record(&self, skill_id: &str, prompt: &str) -> Option<TaskRecord> {
        let skill = self.registry.get(skill_id)?;
        let now = Utc::now();
        
        Some(TaskRecord {
            id: Uuid::new_v4(),
            title: format!("Execute Skill: {}", skill.metadata.name),
            goal: prompt.to_owned(),
            source: "skill_manager".to_string(),
            status: TaskStatus::Executing,
            priority: 2,
            risk_level: skill.risk_level.clone(),
            execution_mode: skill.execution_mode.clone(),
            result_summary: None,
            created_at: now,
            updated_at: now,
        })
    }
}
