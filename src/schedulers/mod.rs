use actix::prelude::*;
use chrono::Local;
use cron_lib::Schedule;
use mongodb::Database;
use std::{fs, str::FromStr, path::Path, time::Duration};

fn duration_timer<T: Into<String>>(duration: T) -> Duration {
    let bindings = duration.into();
    let cron_schedule = Schedule::from_str(&bindings).unwrap();
    let now = Local::now();
    let next = cron_schedule.upcoming(Local).next().unwrap();
    let duration_until = next.signed_duration_since(now);

    duration_until.to_std().unwrap()
}

fn delete_old_logs(logs_folder: &Path, expiry: usize, show_logs: bool) {
    if show_logs {
        let exp = match expiry > 1 {
            true => format!("{} days starting today", expiry.clone()),
            false => format!("{} day starting today", expiry.clone()),
        };

        println!("Deleting logs which is > {exp}...");
    }

    for entry in fs::read_dir(logs_folder).unwrap() {
        let entry = entry.unwrap();
        let filename = entry.file_name().to_str().unwrap_or("").to_string();
        if !filename.starts_with("logs.") {
            continue;
        }

        let date_str = &filename[5..];
        match chrono::NaiveDateTime::parse_from_str(&format!("{date_str} 00:00:00"), "%Y-%m-%d %H:%M:%S") {
            Ok(date) => {
                let now = Local::now().naive_local();
                if now.signed_duration_since(date).num_days() >= expiry as i64 {
                    fs::remove_file(entry.path()).unwrap();
                }
            },
            Err(_) => continue,
        }
    }
}

#[derive(Default, Clone)]
pub struct Scheduler {
    pub database: Option<Database>,
    pub show_logs: bool,
    pub duration: String,
    pub directory: String,
    pub expiry: usize,
    pub func: Option<fn(Database)>
}


impl Actor for Scheduler {
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Context<Self>) {
        if self.show_logs {
            println!("Scheduler for {:?} is now running...", self.duration.clone());
        }

        ctx.run_later(duration_timer(&self.duration), move |this, ctx| {
            this.schedule_task(ctx)
        });
    }

    fn stopped(&mut self, _ctx: &mut Context<Self>) {
        if self.show_logs {
            println!("Scheduler for {} stopped...", self.duration.clone());
        }
    }
}

impl Scheduler {
    pub fn builder() -> Self {
        Self {
            ..Default::default()
        }
    }

    pub fn set_database(&self, database: &Database) -> Self {
        let mut data = self.clone();
        data.database = Some(database.clone());
        data
    }

    pub fn set_show_logs(&self, show_logs: bool) -> Self {
        let mut data = self.clone();
        data.show_logs = show_logs;
        data
    }

    pub fn set_duration<T>(&self, duration: T) -> Self
        where T: ToString
    {
        let mut data = self.clone();
        data.duration = duration.to_string();
        data
    }

    pub fn set_directory<T>(&self, directory: T) -> Self
        where T: ToString
    {
        let mut data = self.clone();
        data.directory = directory.to_string();
        data
    }

    pub fn set_expiry(&self, expiry: usize) -> Self {
        let mut data = self.clone();
        data.expiry = expiry;
        data
    }

    pub fn set_func(&self, func: fn(Database)) -> Self {
        let mut data = self.clone();
        data.func = Some(func);
        data
    }

    fn schedule_task(&self, ctx: &mut Context<Self>) {
        if self.show_logs {
            println!("Scheduled task for {} executed - {}", self.duration.clone(), Local::now());
        }

        let logs_folder = Path::new(&self.directory);
        delete_old_logs(logs_folder, self.expiry, self.show_logs);

        if let Some(database) = &self.database {
            if let Some(func) = self.func {
                func(database.clone());
            }
        }

        ctx.run_later(duration_timer(&self.duration), move |this, ctx| {
            this.schedule_task(ctx)
        });
    }
}