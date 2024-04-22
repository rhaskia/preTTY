use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::{Arc, Mutex, RwLock, mpsc};

use notify::event::{CreateKind, ModifyKind, RemoveKind};
use notify::{Event, RecommendedWatcher, RecursiveMode, Watcher};

pub struct ConfigWatcher {
    path: PathBuf,
    callback: Box<fn(Event)>,
    watcher: Option<Arc<Mutex<RecommendedWatcher>>>,
    files: RwLock<Vec<(String, String)>>,
}

impl ConfigWatcher {
    pub fn new(path: PathBuf, callback: fn(Event)) -> anyhow::Result<Self> {
        // TODO: pre load all files on startup
        Ok(Self {
            path,
            callback: Box::new(callback),
            watcher: None,
            files: RwLock::new(HashMap::new()),
        })
    }

    pub fn watch_changes(&mut self) {

    }

    pub fn start_watching(&mut self) -> anyhow::Result<()> {
        let listener = self.callback.clone();
        let (tx, rx) = mpsc::channel(); 

        let mut watcher =
            notify::recommended_watcher(move |res: Result<Event, notify::Error>| match res {
                Ok(event) => {
                    let writer = lock.write().unwrap();
                    writer.handle_file_event(event);
                    listener(event.clone());
                }
                Err(e) => log::warn!("watch error: {:?}", e),
            })?;

        watcher.watch(&self.path, RecursiveMode::Recursive)?;
        self.watcher = Some(Arc::new(Mutex::new(watcher)));
        Ok(())
    }

    pub fn watch_config(app: &str, callback: fn(Event)) -> anyhow::Result<Self> {
        let mut path = dirs::config_dir().unwrap();
        path.push(app);
        log::info!("Watching {:?} Directory", path);
        Self::new(path, callback)
    }
}

pub trait FileEvent {
    fn handle_file_event(&mut self, event: Event);
    fn create_file(&mut self, event: Event, info: CreateKind);
    fn modify_file(&mut self, event: Event, info: ModifyKind);
    fn delete_file(&mut self, event: Event, info: RemoveKind);
}

impl FileEvent for Vec<(String, String)> {
    fn handle_file_event(&mut self, event: Event) {
        match event.kind {
            notify::EventKind::Create(info) => self.create_file(event, info),
            notify::EventKind::Modify(info) => self.modify_file(event, info),
            notify::EventKind::Remove(info) => self.delete_file(event, info),
            _ => {}
        }
    }

    fn create_file(&mut self, event: Event, info: CreateKind) {
        let file = std::fs::read_to_string(&event.paths[0]);
        let path = event.paths[0].to_str().unwrap().to_string();
        match file {
            Ok(f) => {
                self.push((path, f));
            },
            Err(_) => {
                //self.remove(&path);
            },
        };
    }

    fn modify_file(&mut self, event: Event, info: ModifyKind) { todo!() }

    fn delete_file(&mut self, event: Event, info: RemoveKind) { todo!() }
}
