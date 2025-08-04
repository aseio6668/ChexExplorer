use std::path::{Path, PathBuf};
use tokio::sync::RwLock;
use anyhow::Result;
use notify::{Event, RecursiveMode, Watcher, RecommendedWatcher};
use std::sync::Arc;
use tokio::sync::mpsc;

use crate::core::file_item::{FileItem, SortBy, SortOrder};

pub struct FileManager {
    current_path: Arc<RwLock<PathBuf>>,
    items: Arc<RwLock<Vec<FileItem>>>,
    selected_items: Arc<RwLock<Vec<usize>>>,
    clipboard: Arc<RwLock<Vec<PathBuf>>>,
    history: Arc<RwLock<Vec<PathBuf>>>,
    history_index: Arc<RwLock<usize>>,
    sort_by: Arc<RwLock<SortBy>>,
    sort_order: Arc<RwLock<SortOrder>>,
    show_hidden: Arc<RwLock<bool>>,
    watcher: Option<RecommendedWatcher>,
    watcher_rx: Option<mpsc::UnboundedReceiver<notify::Result<Event>>>,
}

impl FileManager {
    pub fn new() -> Self {
        let home_dir = dirs::home_dir().unwrap_or_else(|| PathBuf::from("/"));
        
        Self {
            current_path: Arc::new(RwLock::new(home_dir.clone())),
            items: Arc::new(RwLock::new(Vec::new())),
            selected_items: Arc::new(RwLock::new(Vec::new())),
            clipboard: Arc::new(RwLock::new(Vec::new())),
            history: Arc::new(RwLock::new(vec![home_dir])),
            history_index: Arc::new(RwLock::new(0)),
            sort_by: Arc::new(RwLock::new(SortBy::Name)),
            sort_order: Arc::new(RwLock::new(SortOrder::Ascending)),
            show_hidden: Arc::new(RwLock::new(false)),
            watcher: None,
            watcher_rx: None,
        }
    }

    pub async fn navigate_to(&mut self, path: &Path) -> Result<()> {
        if !path.exists() {
            return Err(anyhow::anyhow!("Path does not exist: {}", path.display()));
        }

        if !path.is_dir() {
            return Err(anyhow::anyhow!("Path is not a directory: {}", path.display()));
        }

        // Update current path
        {
            let mut current_path = self.current_path.write().await;
            *current_path = path.to_path_buf();
        }

        // Update history
        {
            let mut history = self.history.write().await;
            let mut history_index = self.history_index.write().await;
            
            // Remove any history after current index
            history.truncate(*history_index + 1);
            
            // Add new path if it's different from current
            if history.last() != Some(&path.to_path_buf()) {
                history.push(path.to_path_buf());
                *history_index = history.len() - 1;
            }
        }

        // Refresh items
        self.refresh_items().await?;

        // Setup file watcher
        self.setup_watcher(path)?;

        Ok(())
    }

    pub async fn refresh_items(&mut self) -> Result<()> {
        let current_path = {
            let path = self.current_path.read().await;
            path.clone()
        };

        let show_hidden = *self.show_hidden.read().await;
        let sort_by = *self.sort_by.read().await;
        let sort_order = *self.sort_order.read().await;

        let mut items = Vec::new();

        // Read directory entries
        let entries = std::fs::read_dir(&current_path)?;
        
        for entry in entries {
            let entry = entry?;
            let path = entry.path();
            
            match FileItem::from_path(&path) {
                Ok(item) => {
                    if show_hidden || !item.is_hidden {
                        items.push(item);
                    }
                }
                Err(e) => {
                    log::warn!("Failed to read file item {}: {}", path.display(), e);
                }
            }
        }

        // Sort items
        FileItem::sort_items(&mut items, sort_by, sort_order);

        // Update items
        {
            let mut items_lock = self.items.write().await;
            *items_lock = items;
        }

        // Clear selection
        {
            let mut selected = self.selected_items.write().await;
            selected.clear();
        }

        Ok(())
    }

    pub async fn get_current_path(&self) -> PathBuf {
        self.current_path.read().await.clone()
    }

    pub async fn get_items(&self) -> Vec<FileItem> {
        self.items.read().await.clone()
    }

    pub async fn get_selected_items(&self) -> Vec<usize> {
        self.selected_items.read().await.clone()
    }

    pub async fn select_item(&self, index: usize, multiple: bool) {
        let mut selected = self.selected_items.write().await;
        
        if multiple {
            if selected.contains(&index) {
                selected.retain(|&x| x != index);
            } else {
                selected.push(index);
            }
        } else {
            selected.clear();
            selected.push(index);
        }
    }

    pub async fn select_all(&self) {
        let items_count = self.items.read().await.len();
        let mut selected = self.selected_items.write().await;
        selected.clear();
        selected.extend(0..items_count);
    }

    pub async fn clear_selection(&self) {
        let mut selected = self.selected_items.write().await;
        selected.clear();
    }

    pub async fn can_go_back(&self) -> bool {
        let history_index = self.history_index.read().await;
        *history_index > 0
    }

    pub async fn can_go_forward(&self) -> bool {
        let history = self.history.read().await;
        let history_index = self.history_index.read().await;
        *history_index < history.len() - 1
    }

    pub async fn go_back(&mut self) -> Result<()> {
        let (can_go_back, new_path) = {
            let mut history_index = self.history_index.write().await;
            if *history_index > 0 {
                *history_index -= 1;
                let history = self.history.read().await;
                (true, history[*history_index].clone())
            } else {
                (false, PathBuf::new())
            }
        };

        if can_go_back {
            {
                let mut current_path = self.current_path.write().await;
                *current_path = new_path;
            }
            self.refresh_items().await?;
        }

        Ok(())
    }

    pub async fn go_forward(&mut self) -> Result<()> {
        let (can_go_forward, new_path) = {
            let mut history_index = self.history_index.write().await;
            let history = self.history.read().await;
            if *history_index < history.len() - 1 {
                *history_index += 1;
                (true, history[*history_index].clone())
            } else {
                (false, PathBuf::new())
            }
        };

        if can_go_forward {
            {
                let mut current_path = self.current_path.write().await;
                *current_path = new_path;
            }
            self.refresh_items().await?;
        }

        Ok(())
    }

    pub async fn go_up(&mut self) -> Result<()> {
        let parent = {
            let current_path = self.current_path.read().await;
            current_path.parent().map(|p| p.to_path_buf())
        };

        if let Some(parent_path) = parent {
            self.navigate_to(&parent_path).await?;
        }

        Ok(())
    }

    pub async fn set_sort(&mut self, sort_by: SortBy, sort_order: SortOrder) -> Result<()> {
        {
            let mut sort_by_lock = self.sort_by.write().await;
            let mut sort_order_lock = self.sort_order.write().await;
            *sort_by_lock = sort_by;
            *sort_order_lock = sort_order;
        }
        
        self.refresh_items().await
    }

    pub async fn toggle_show_hidden(&mut self) -> Result<()> {
        {
            let mut show_hidden = self.show_hidden.write().await;
            *show_hidden = !*show_hidden;
        }
        
        self.refresh_items().await
    }

    pub async fn get_show_hidden(&self) -> bool {
        *self.show_hidden.read().await
    }

    fn setup_watcher(&mut self, path: &Path) -> Result<()> {
        let (tx, rx) = mpsc::unbounded_channel();
        let mut watcher = notify::recommended_watcher(move |res| {
            if let Err(_) = tx.send(res) {
                // Channel closed, ignore
            }
        })?;
        watcher.watch(path, RecursiveMode::NonRecursive)?;
        
        self.watcher = Some(watcher);
        self.watcher_rx = Some(rx);
        
        Ok(())
    }

    pub fn check_file_changes(&mut self) -> Vec<Event> {
        let mut events = Vec::new();
        
        if let Some(ref mut rx) = self.watcher_rx {
            while let Ok(result) = rx.try_recv() {
                if let Ok(event) = result {
                    events.push(event);
                }
            }
        }
        
        events
    }
}

impl Default for FileManager {
    fn default() -> Self {
        Self::new()
    }
}
