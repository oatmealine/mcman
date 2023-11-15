use anyhow::Result;
use console::style;
use dialoguer::{theme::ColorfulTheme, Input, Confirm, Select};

use crate::util::SelectItem;

use super::App;

impl App {
    pub fn warn<S: std::fmt::Display>(&self, message: S) -> Result<()> {
        Ok(self.multi_progress.println(format!(
            "  {} {message}",
            style("⚠ Warn").yellow().bold()
        ))?)
    }

    pub fn error<S: std::fmt::Display>(&self, message: S) -> Result<()> {
        Ok(self.multi_progress.println(format!(
            "  {} {message}",
            style("⚠ Error").red().bold()
        ))?)
    }

    pub fn success<S: std::fmt::Display>(&self, message: S) -> Result<()> {
        Ok(self.multi_progress.suspend(|| println!(
            "  {} {message}",
            ColorfulTheme::default().success_prefix
        )))
    }

    pub fn info<S: std::fmt::Display>(&self, message: S) -> Result<()> {
        Ok(self.multi_progress.suspend(||
            println!(
                "  {} {message}",
                style("🛈 Info").bold()
            )
        ))
    }

    pub fn log<S: std::fmt::Display>(&self, message: S) -> Result<()> {
        Ok(self.multi_progress.suspend(|| println!(
            "  {}",
           style(message).dim()
        )))
    }

    pub fn println<S: std::fmt::Display>(&self, message: S) -> Result<()> {
        Ok(self.multi_progress.suspend(|| println!("{message}")))
    }

    pub fn dbg<S: std::fmt::Display>(&self, message: S) -> Result<()> {
        Ok(self.multi_progress.suspend(|| println!(
            "  {} {}",
           style("[dbg]").dim(),
           style(message).dim()
        )))
    }

    pub fn print_job(&self, job: &str) -> Result<()> {
        Ok(self.multi_progress.suspend(|| println!(
            "{} {}",
            ColorfulTheme::default().active_item_prefix,
           style(job).cyan().bold()
        )))
    }

    pub fn is_ci(&self) -> bool {
        std::env::var("CI").ok() == Some("true".to_owned())
    }

    pub fn ci(&self, cmd: &str) {
        if self.is_ci() {
            self.multi_progress.suspend(|| println!("{cmd}"))
        }
    }

    pub fn prompt_string(&self, prompt: &str) -> Result<String> {
        Ok(self.multi_progress.suspend(|| {
            Input::with_theme(&ColorfulTheme::default())
                .with_prompt(prompt)
                .interact_text()
        })?)
    }

    pub fn prompt_string_default(&self, prompt: &str, default: &str) -> Result<String> {
        Ok(self.multi_progress.suspend(|| {
            Input::with_theme(&ColorfulTheme::default())
                .with_prompt(prompt)
                .default(default.to_owned())
                .interact_text()
        })?)
    }

    pub fn prompt_string_filled(&self, prompt: &str, default: &str) -> Result<String> {
        Ok(self.multi_progress.suspend(|| {
            Input::with_theme(&ColorfulTheme::default())
                .with_prompt(prompt)
                .default(default.to_owned())
                .with_initial_text(default.to_owned())
                .interact_text()
        })?)
    }

    pub fn confirm(&self, prompt: &str) -> Result<bool> {
        Ok(self.multi_progress.suspend(|| {
            Confirm::with_theme(&ColorfulTheme::default())
                .with_prompt(prompt)
                .wait_for_newline(true)
                .interact()
        })?)
    }

    pub fn select<T: Clone>(&self, prompt: &str, items: &[SelectItem<T>]) -> Result<T> {
        let item = &items[self.multi_progress.suspend(|| {
            Select::with_theme(&ColorfulTheme::default())
                .items(items)
                .with_prompt(prompt)
                .default(0)
                .interact()
        })?];

        Ok(item.0.clone())
    }

    pub fn select_with_default<T: Clone>(&self, prompt: &str, items: &[SelectItem<T>], def: usize) -> Result<T> {
        let item = &items[self.multi_progress.suspend(|| {
            Select::with_theme(&ColorfulTheme::default())
                .items(items)
                .with_prompt(prompt)
                .default(def)
                .interact()
        })?];

        Ok(item.0.clone())
    }
}
