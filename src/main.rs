mod ticket;
use std::sync::mpsc::{Sender, Receiver, channel};
use std::collections::HashMap;
use ticket::Ticket;
use std::fs::{File, read_dir, DirEntry};
use notify::{Config, Watcher, RecursiveMode, RecommendedWatcher};
use std::thread;

// the maximum depth of recursion that find_tickets will run for
const MAX_DEPTH: usize = 64;

fn find_tickets(root: DirEntry, depth: usize) -> Result<Vec<Ticket>, ()> {
    let mut result: Vec<Ticket> = Vec::new();
    // prevent infinite recursion
    if depth == MAX_DEPTH {
        return Ok(result);
    }
    let entries = read_dir(root.path());
    if let Ok(entries) = entries {
        for entry in entries {
            if let Ok(entry) = entry {
                let filetype = entry.file_type().map_err(|err| -> () {
                    println!("Could not determine filetype for file {:?}: {err}", entry);
                })?;
                if filetype.is_dir() {
                    result.append(&mut find_tickets(entry, depth+1)?);
                }
                else {
                    let file = File::open(entry.path()).map_err(|err| -> () {
                        println!("Could not open ticket file: {err}");
                    })?;
                    let ticket = Ticket::from_file(file)?;
                    result.push(ticket);
                }
            }
        }
    }
    return Ok(result);
}

fn load_tickets(ticket_root: &str,tickets_by_status: &mut HashMap<String, Vec<Ticket>>) -> Result<(), ()> {
    if let Ok(entries) = read_dir(ticket_root) {
        for entry in entries {
            if let Ok(entry) = entry {
                let filetype = entry.file_type().map_err(|err| -> () {
                    println!("Could not determine filetype for file {:?}: {err}", entry);
                })?;
                if filetype.is_dir() {
                    match entry.file_name().into_string() {
                        Ok(filename) => {
                            tickets_by_status.insert(filename, find_tickets(entry, 0)?);
                        }
                        Err(filename) => {
                            println!("Invalid string in filename: {:?}", filename);
                        }
                    }
                }
            } else {
                println!("Error: {:?}", entry);
            }
        }
    }
    Ok(())
}

fn watch_tickets(path: &str, fileupdate_sender: Sender<notify::Result<notify::Event>>) -> notify::Result<()> {
    let (tx, rx) = channel();
    let mut watcher = RecommendedWatcher::new(tx, Config::default())?;
    
    watcher.watch(path.as_ref(), RecursiveMode::Recursive)?;

    println!("Watching for changes in {path}");

    for res in rx {
        fileupdate_sender.send(res).unwrap();
    }

    Ok(())
}

enum FileUpdate {
    FullRescan,
    Paths(Vec<String>)
}

fn main() -> Result<(), ()> {
    let ticket_root = "tickets";
    let mut tickets_by_status: HashMap<String, Vec<Ticket>> = HashMap::new();
    load_tickets(ticket_root, &mut tickets_by_status)?;
    let (event_tx, event_rx) = channel();
    let (file_updates_tx, file_updates_rx): (Sender<FileUpdate>, Receiver<FileUpdate>) = channel();
    let watch_thread = thread::spawn(move || {
        watch_tickets(ticket_root, event_tx).unwrap();
        for event in event_rx {
            match event {
                Ok(event) => {
                        file_updates_tx.send(if event.need_rescan() {
                            FileUpdate::FullRescan
                        } else {
                            FileUpdate::Paths(event.paths.iter().map(|path| -> String {
                                path.to_string_lossy().to_string()
                            }).collect())
                        }).expect("Could not send file update");
                },
                Err(err) => {
                    println!("Notify event error: {}", err);
                }
            }
        }
    });

    let file_updater_thread = thread::spawn(move || {
        for file_update in file_updates_rx {
            match file_update {
                FileUpdate::FullRescan => {
                    // reload all tickets
                    load_tickets(ticket_root, &mut tickets_by_status).expect("Could not load tickets");
                },
                FileUpdate::Paths(paths) => {
                    todo!("do a partial rescan of path {:?}", paths);
                }
            }
        }
    });

    // join all threads.  I'm not sure if this is the most idiomatic/sane way to do it,
    // but it's somewhat concise and clear to me at least.
    vec![file_updater_thread, watch_thread].into_iter().map(|h| h.join().unwrap()).for_each(drop);

    Ok(())
}
