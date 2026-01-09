//! Exercise loading and status tracking

use crate::runner;
use serde::Deserialize;
use std::path::PathBuf;

#[derive(Debug, Clone, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum ExerciseMode {
    Compile,
    Test,
}

impl Default for ExerciseMode {
    fn default() -> Self {
        Self::Test
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct ExerciseInfo {
    pub name: String,
    pub path: String,
    #[serde(default)]
    pub mode: ExerciseMode,
}

#[derive(Debug, Deserialize)]
struct ExercisesFile {
    #[serde(rename = "exercises")]
    exercises: Vec<ExerciseInfo>,
}

#[derive(Debug, Clone)]
pub struct Exercise {
    pub name: String,
    pub path: PathBuf,
    pub mode: ExerciseMode,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ExerciseStatus {
    Done,
    NotDone,
    CompileError,
    TestFail,
}

impl Exercise {
    pub fn status(&self) -> ExerciseStatus {
        let content = match std::fs::read_to_string(&self.path) {
            Ok(c) => c,
            Err(_) => return ExerciseStatus::CompileError,
        };

        if content.contains("# I AM NOT DONE") {
            return ExerciseStatus::NotDone;
        }

        if runner::compile(&self.path).is_err() {
            return ExerciseStatus::CompileError;
        }

        if self.mode == ExerciseMode::Test {
            match runner::run_tests(&self.path) {
                Ok(output) => {
                    if output.contains("FAIL") || output.contains("panicked") {
                        return ExerciseStatus::TestFail;
                    }
                }
                Err(_) => return ExerciseStatus::TestFail,
            }
        }

        ExerciseStatus::Done
    }

    pub fn hint_path(&self) -> PathBuf {
        let mut hint_path = PathBuf::from("hints");
        if let Some(parent) = self.path.parent() {
            if let Some(topic) = parent.file_name() {
                hint_path.push(topic);
            }
        }
        if let Some(stem) = self.path.file_stem() {
            hint_path.push(format!("{}.md", stem.to_string_lossy()));
        }
        hint_path
    }

    pub fn solution_path(&self) -> PathBuf {
        let mut solution_path = PathBuf::from("solutions");
        if let Some(parent) = self.path.parent() {
            if let Some(topic) = parent.file_name() {
                solution_path.push(topic);
            }
        }
        if let Some(name) = self.path.file_name() {
            solution_path.push(name);
        }
        solution_path
    }
}

pub fn load_exercises() -> Result<Vec<Exercise>, String> {
    let info_path = PathBuf::from("exercises/info.toml");

    if !info_path.exists() {
        return Err(
            "exercises/info.toml not found. Are you in the sicplings-seq directory?".to_string()
        );
    }

    let content = std::fs::read_to_string(&info_path)
        .map_err(|e| format!("Failed to read info.toml: {}", e))?;

    let exercises_file: ExercisesFile =
        toml::from_str(&content).map_err(|e| format!("Failed to parse info.toml: {}", e))?;

    let exercises = exercises_file
        .exercises
        .into_iter()
        .map(|info| Exercise {
            name: info.name,
            path: PathBuf::from(&info.path),
            mode: info.mode,
        })
        .collect();

    Ok(exercises)
}
