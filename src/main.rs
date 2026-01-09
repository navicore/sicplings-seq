//! SICP-lings in Seq - SICP exercises reimagined in a stack-based language
//!
//! A seqlings-inspired tool for working through SICP in Seq.

mod exercise;
mod runner;

use clap::{Parser, Subcommand};
use colored::Colorize;
use exercise::{Exercise, ExerciseStatus, load_exercises};
use std::collections::HashMap;
use std::path::PathBuf;
use std::process;
use std::time::{Duration, SystemTime};

struct StatusCache {
    cache: HashMap<PathBuf, (SystemTime, ExerciseStatus)>,
}

impl StatusCache {
    fn new() -> Self {
        Self {
            cache: HashMap::new(),
        }
    }

    fn get_status(&mut self, exercise: &Exercise) -> ExerciseStatus {
        let current_mtime = match std::fs::metadata(&exercise.path) {
            Ok(meta) => meta.modified().ok(),
            Err(_) => return ExerciseStatus::CompileError,
        };

        if let Ok(content) = std::fs::read_to_string(&exercise.path) {
            if content.contains("# I AM NOT DONE") {
                if let Some(mtime) = current_mtime {
                    self.cache.insert(exercise.path.clone(), (mtime, ExerciseStatus::NotDone));
                }
                return ExerciseStatus::NotDone;
            }
        }

        if let Some(mtime) = current_mtime {
            if let Some((cached_mtime, cached_status)) = self.cache.get(&exercise.path) {
                if *cached_mtime == mtime {
                    return cached_status.clone();
                }
            }
        }

        let status = exercise.status();

        if let Some(mtime) = current_mtime {
            self.cache.insert(exercise.path.clone(), (mtime, status.clone()));
        }

        status
    }
}

#[derive(Parser)]
#[command(name = "sicplings")]
#[command(version, about = "SICP exercises in Seq - a stack-based journey through computation")]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Watch for file changes and auto-verify exercises
    Watch {
        #[arg(short, long)]
        chapter: Option<String>,
    },
    /// List all exercises with their status
    List {
        #[arg(short, long)]
        chapter: Option<String>,
    },
    /// Show hint for the current or specified exercise
    Hint {
        name: Option<String>,
    },
    /// Verify all exercises and show progress
    Verify,
    /// Skip to the next exercise
    Next,
}

fn main() {
    let cli = Cli::parse();

    let exercises = match load_exercises() {
        Ok(ex) => ex,
        Err(e) => {
            eprintln!("{} {}", "Error loading exercises:".red(), e);
            process::exit(1);
        }
    };

    if exercises.is_empty() {
        eprintln!("{}", "No exercises found in exercises/info.toml".red());
        process::exit(1);
    }

    match cli.command {
        Some(Commands::Watch { chapter }) => {
            let filtered = filter_by_chapter(&exercises, chapter.as_deref());
            cmd_watch(&filtered);
        }
        Some(Commands::List { chapter }) => {
            let filtered = filter_by_chapter(&exercises, chapter.as_deref());
            cmd_list(&filtered);
        }
        Some(Commands::Hint { name }) => cmd_hint(&exercises, name),
        Some(Commands::Verify) => cmd_verify(&exercises),
        Some(Commands::Next) => cmd_next(&exercises),
        None => cmd_watch(&exercises),
    }
}

fn filter_by_chapter(exercises: &[Exercise], chapter: Option<&str>) -> Vec<Exercise> {
    match chapter {
        None => exercises.to_vec(),
        Some(prefix) => {
            let filtered: Vec<Exercise> = exercises
                .iter()
                .filter(|e| {
                    let chapter_name = e.path
                        .parent()
                        .and_then(|p| p.file_name())
                        .and_then(|s| s.to_str())
                        .unwrap_or("");
                    chapter_name.starts_with(prefix)
                })
                .cloned()
                .collect();

            if filtered.is_empty() {
                eprintln!(
                    "{} No exercises found for chapter '{}'",
                    "Warning:".yellow(),
                    prefix
                );
                let mut chapters: Vec<&str> = exercises
                    .iter()
                    .filter_map(|e| {
                        e.path
                            .parent()
                            .and_then(|p| p.file_name())
                            .and_then(|s| s.to_str())
                    })
                    .collect();
                chapters.sort();
                chapters.dedup();
                eprintln!("Available chapters:");
                for ch in chapters {
                    eprintln!("  {}", ch);
                }
                process::exit(1);
            }

            println!(
                "{} Filtering to chapter '{}' ({} exercises)\n",
                "Note:".cyan(),
                prefix,
                filtered.len()
            );
            filtered
        }
    }
}

fn cmd_watch(exercises: &[Exercise]) {
    println!(
        "\n{}",
        "Welcome to SICP-lings in Seq!".green().bold()
    );
    println!("{}", "Structure and Interpretation of Computer Programs".dimmed());
    println!("{}", "...but in a stack-based language.".dimmed());
    println!();
    println!("{}", "Edit exercises in your editor. Progress updates automatically.".dimmed());
    println!("{}", "Press Ctrl+C to exit.\n".dimmed());

    let mut cache = StatusCache::new();

    print!("{}", "Checking exercises...".dimmed());
    use std::io::Write;
    std::io::stdout().flush().ok();

    for (i, ex) in exercises.iter().enumerate() {
        cache.get_status(ex);
        if (i + 1) % 5 == 0 {
            print!(".");
            std::io::stdout().flush().ok();
        }
    }
    println!(" {}", "done".green());

    let mut current_exercise_name = String::new();
    display_current_exercise(exercises, &mut current_exercise_name, &mut cache);

    loop {
        std::thread::sleep(Duration::from_millis(250));

        let mut changed = false;
        for ex in exercises {
            if let Ok(meta) = std::fs::metadata(&ex.path) {
                if let Ok(mtime) = meta.modified() {
                    if mtime.elapsed().unwrap_or(Duration::from_secs(1000)) < Duration::from_millis(500) {
                        changed = true;
                        break;
                    }
                }
            }
        }

        if changed {
            clear_screen();
            display_current_exercise(exercises, &mut current_exercise_name, &mut cache);
        }
    }
}

fn clear_screen() {
    print!("\x1B[2J\x1B[1;1H");
    use std::io::Write;
    std::io::stdout().flush().ok();
}

fn display_current_exercise(exercises: &[Exercise], previous_name: &mut String, cache: &mut StatusCache) {
    let current = exercises.iter().find(|e| {
        matches!(
            cache.get_status(e),
            ExerciseStatus::NotDone | ExerciseStatus::CompileError | ExerciseStatus::TestFail
        )
    });

    match current {
        Some(exercise) => {
            let status = cache.get_status(exercise);

            if !previous_name.is_empty() && *previous_name != exercise.name {
                println!(
                    "{} Completed {}!\n",
                    "!!!".green().bold(),
                    previous_name.cyan()
                );
            }
            *previous_name = exercise.name.clone();

            println!(
                "{} {}\n",
                "Current exercise:".green().bold(),
                exercise.name.cyan()
            );

            let abs_path = std::env::current_dir()
                .map(|cwd| cwd.join(&exercise.path))
                .unwrap_or_else(|_| exercise.path.clone());
            println!("  File: {}", abs_path.display().to_string().dimmed());

            match status {
                ExerciseStatus::NotDone => {
                    println!("  Status: {}\n", "Waiting for you to start...".yellow());

                    if let Ok(content) = std::fs::read_to_string(&exercise.path) {
                        let header: Vec<&str> = content
                            .lines()
                            .take_while(|l| l.starts_with('#'))
                            .filter(|l| !l.contains("I AM NOT DONE"))
                            .collect();
                        for line in header {
                            println!("  {}", line.dimmed());
                        }
                    }

                    println!();
                    println!(
                        "  {}",
                        "Delete the '# I AM NOT DONE' line when you've solved it.".yellow()
                    );
                }
                ExerciseStatus::CompileError => {
                    println!("  Status: {}\n", "Compile Error".red().bold());

                    if let Err(e) = runner::compile(&exercise.path) {
                        for line in e.lines().take(15) {
                            println!("  {}", line.red());
                        }
                    }
                }
                ExerciseStatus::TestFail => {
                    println!("  Status: {}\n", "Tests Failed".red().bold());

                    match runner::run_tests(&exercise.path) {
                        Ok(output) | Err(output) => {
                            for line in output.lines().take(20) {
                                if line.contains("FAIL") || line.contains("panicked") {
                                    println!("  {}", line.red());
                                } else if line.contains("ok") {
                                    println!("  {}", line.green());
                                } else {
                                    println!("  {}", line);
                                }
                            }
                        }
                    }
                }
                ExerciseStatus::Done => {
                    println!("  Status: {}", "Done".green());
                }
            }

            println!();
            println!("  {} sicplings hint", "Hint:".cyan());
            show_progress(exercises, cache);
        }
        None => {
            clear_screen();
            println!("\n{}", "=".repeat(60).green());
            println!(
                "{}",
                "  You have completed this chapter of SICP-lings!".green().bold()
            );
            println!("{}\n", "=".repeat(60).green());
            show_progress(exercises, cache);
            println!("\n{}", "The wizard awaits in the next chapter...".cyan().bold());
            process::exit(0);
        }
    }
}

fn cmd_list(exercises: &[Exercise]) {
    let mut cache = StatusCache::new();

    println!("\n{}\n", "SICP-lings Exercises".green().bold());

    let mut current_topic = String::new();
    for exercise in exercises {
        let topic = exercise
            .path
            .parent()
            .and_then(|p| p.file_name())
            .and_then(|s| s.to_str())
            .unwrap_or("unknown");

        if topic != current_topic {
            println!("\n  {}", topic.cyan().bold());
            current_topic = topic.to_string();
        }

        let status = cache.get_status(exercise);
        let status_icon = match status {
            ExerciseStatus::Done => "!!!".green(),
            ExerciseStatus::NotDone => " . ".yellow(),
            ExerciseStatus::CompileError => "err".red(),
            ExerciseStatus::TestFail => "  X".red(),
        };

        println!("    {} {}", status_icon, exercise.name);
    }

    println!();
    show_progress(exercises, &mut cache);
}

fn cmd_hint(exercises: &[Exercise], name: Option<String>) {
    let mut cache = StatusCache::new();
    let exercise = match &name {
        Some(n) => exercises.iter().find(|e| &e.name == n),
        None => exercises.iter().find(|e| {
            matches!(
                cache.get_status(e),
                ExerciseStatus::NotDone | ExerciseStatus::CompileError | ExerciseStatus::TestFail
            )
        }),
    };

    match exercise {
        Some(ex) => {
            let hint_path = ex.hint_path();
            if hint_path.exists() {
                match std::fs::read_to_string(&hint_path) {
                    Ok(content) => {
                        println!("\n{} {}\n", "Hint for".green(), ex.name.cyan());
                        println!("{}", content);
                    }
                    Err(e) => {
                        eprintln!("{} {}", "Error reading hint:".red(), e);
                    }
                }
            } else {
                println!(
                    "\n{} {}",
                    "No hint available for".yellow(),
                    ex.name.cyan()
                );
            }
        }
        None => {
            println!("{}", "All exercises complete!".green());
        }
    }
}

fn cmd_verify(exercises: &[Exercise]) {
    let mut cache = StatusCache::new();

    println!("\n{}\n", "Verifying all exercises...".green().bold());

    for exercise in exercises {
        let status = cache.get_status(exercise);
        let icon = match status {
            ExerciseStatus::Done => "!!!".green(),
            _ => "  X".red(),
        };
        println!("  {} {}", icon, exercise.name);
    }

    println!();
    show_progress(exercises, &mut cache);
}

fn cmd_next(exercises: &[Exercise]) {
    let mut cache = StatusCache::new();

    let current_idx = exercises.iter().position(|e| {
        matches!(
            cache.get_status(e),
            ExerciseStatus::NotDone | ExerciseStatus::CompileError | ExerciseStatus::TestFail
        )
    });

    match current_idx {
        Some(idx) if idx + 1 < exercises.len() => {
            let next = &exercises[idx + 1];
            println!("Skipping to: {}", next.name.cyan());
        }
        _ => {
            println!("{}", "No more exercises to skip to.".yellow());
        }
    }
}

fn show_progress(exercises: &[Exercise], cache: &mut StatusCache) {
    let done = exercises
        .iter()
        .filter(|e| matches!(cache.get_status(e), ExerciseStatus::Done))
        .count();
    let total = exercises.len();
    let pct = if total > 0 { (done as f64 / total as f64 * 100.0) as usize } else { 0 };

    let bar_width = 30;
    let filled = if total > 0 { (done * bar_width) / total } else { 0 };
    let empty = bar_width - filled;

    println!(
        "Progress: [{}{}] {}/{} ({}%)",
        "=".repeat(filled).green(),
        "-".repeat(empty),
        done,
        total,
        pct
    );
}
