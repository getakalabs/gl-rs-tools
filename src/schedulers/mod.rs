use actix::prelude::*;
use chrono::Local;
use cron_lib::Schedule;
use std::{fs, str::FromStr, path::Path, time::Duration};

use crate::DBClient;

fn duration_timer<T: Into<String>>(duration: T) -> Duration {
    let bindings = duration.into();
    let cron_schedule = Schedule::from_str(&bindings).unwrap();
    let now = Local::now();
    let next = cron_schedule.upcoming(Local).next().unwrap();
    let duration_until = next.signed_duration_since(now);

    duration_until.to_std().unwrap()
}


fn delete_old_logs(logs_folder: &Path, expiry: i32, show_logs: bool) {
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

pub struct Scheduler {
    pub db: DBClient,
    pub show_logs: bool,
    pub duration: String,
    pub directory: String,
    pub expiry: i32,
    pub func: fn(DBClient)
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
    pub fn new<D1, D2>(db: &DBClient, func: fn(DBClient), show_logs:bool, duration: D1, directory: D2, expiry: i32) -> Self
        where D1: Into<String>,
              D2: Into<String>
    {
        Scheduler{
            db: db.clone(),
            show_logs,
            duration: duration.into(),
            directory: directory.into(),
            expiry,
            func
        }
    }

    fn schedule_task(&self, ctx: &mut Context<Self>) {
        if self.show_logs {
            println!("Scheduled task for {} executed - {}", self.duration.clone(), Local::now());
        }

        let logs_folder = Path::new(&self.directory);
        delete_old_logs(logs_folder, self.expiry, self.show_logs);

        (self.func)(self.db.clone());

        ctx.run_later(duration_timer(&self.duration), move |this, ctx| {
            this.schedule_task(ctx)
        });
    }
}