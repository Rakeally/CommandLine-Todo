use crate::task::{read_tasks, write_tasks, Status, Task};
use clap::{Arg, Command};

pub fn build_cli() -> Command {
    Command::new("todo")
        .version("1.0")
        .author("Jane K.")
        .about("Command line todo")
        .subcommand(Command::new("add").about("Add a new task").args([
            {
                Arg::new("title")
                    .help("The title of the task")
                    .required(true)
                    .index(1)
            },
            {
                Arg::new("description")
                    .help("The description of the task")
                    .required(true)
                    .index(2)
            },
        ]))
        .subcommand(Command::new("list").about("Display list of task"))
        .subcommand(
            Command::new("remove").about("Remove a task from list").arg(
                Arg::new("id")
                    .help("Id of the task to remove")
                    .required(true)
                    .index(1),
            ),
        )
        .subcommand(
            Command::new("complete")
                .about("Mark a task as completed")
                .arg(
                    Arg::new("id")
                        .help("Id of task to mark complete")
                        .required(true)
                        .index(1),
                ),
        )
}

pub fn handle_matches(matches: clap::ArgMatches) {
    match matches.subcommand() {
        Some(("add", sub_m)) => {
            let title = sub_m.get_one::<String>("title").unwrap().to_string();
            let description = sub_m.get_one::<String>("description").unwrap().to_string();
            let mut tasks = read_tasks().unwrap();
            let id = tasks.len() + 1;
            tasks.push(Task::new(id, &title, &description));
            write_tasks(&tasks).unwrap();
            println!("Task added to task list");
        }
        Some(("list", _)) => {
            let tasks = read_tasks().unwrap();
            for task in tasks {
                println!(
                    "{}: {} {} {:?}",
                    task.id, task.title, task.description, task.status
                );
            }
        }
        Some(("remove", sub_m)) => {
            let id: usize = sub_m.get_one::<String>("id").unwrap().parse().unwrap();
            let mut tasks = read_tasks().unwrap();
            tasks.retain(|task| task.id != id);
            write_tasks(&tasks).unwrap();
            println!("Task removed from task list");
        }
        Some(("complete", sub_m)) => {
            let id: usize = sub_m.get_one::<String>("id").unwrap().parse().unwrap();
            let mut tasks = read_tasks().unwrap();
            if let Some(task) = tasks.iter_mut().find(|task| task.id == id) {
                task.status = Status::Completed;
                write_tasks(&tasks).unwrap();
                println!("Task marked completed");
            } else {
                println!("Task not found");
            }
        }
        _ => {
            let mut cmd = build_cli();
            cmd.print_help().unwrap();
            println!();
        }
    }
}
