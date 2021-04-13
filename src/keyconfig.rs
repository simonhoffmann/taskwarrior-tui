#![allow(clippy::eval_order_dependence)]
use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::error::Error;
use std::hash::Hash;
use crossterm::event::{KeyEvent, KeyCode};

#[derive(Debug)]
pub struct KeyConfig {
    pub quit: KeyEvent,
    pub refresh: KeyEvent,
    pub go_to_bottom: KeyEvent,
    pub go_to_top: KeyEvent,
    pub down: KeyEvent,
    pub up: KeyEvent,
    pub page_down: KeyEvent,
    pub page_up: KeyEvent,
    pub delete: KeyEvent,
    pub done: KeyEvent,
    pub start_stop: KeyEvent,
    pub select: KeyEvent,
    pub select_all: KeyEvent,
    pub undo: KeyEvent,
    pub edit: KeyEvent,
    pub modify: KeyEvent,
    pub shell: KeyEvent,
    pub log: KeyEvent,
    pub add: KeyEvent,
    pub annotate: KeyEvent,
    pub help: KeyEvent,
    pub filter: KeyEvent,
    pub zoom: KeyEvent,
    pub context_menu: KeyEvent,
    pub next_tab: KeyEvent,
    pub previous_tab: KeyEvent,
    pub shortcut0: KeyEvent,
    pub shortcut1: KeyEvent,
    pub shortcut2: KeyEvent,
    pub shortcut3: KeyEvent,
    pub shortcut4: KeyEvent,
    pub shortcut5: KeyEvent,
    pub shortcut6: KeyEvent,
    pub shortcut7: KeyEvent,
    pub shortcut8: KeyEvent,
    pub shortcut9: KeyEvent,
}

impl Default for KeyConfig {
    fn default() -> Self {
        Self {
            quit: KeyCode::Char('q').into(),
            refresh: KeyCode::Char('r').into(),
            go_to_bottom: KeyCode::Char('G').into(),
            go_to_top: KeyCode::Char('g').into(),
            down: KeyCode::Char('j').into(),
            up: KeyCode::Char('k').into(),
            page_down: KeyCode::Char('J').into(),
            page_up: KeyCode::Char('K').into(),
            delete: KeyCode::Char('x').into(),
            done: KeyCode::Char('d').into(),
            start_stop: KeyCode::Char('s').into(),
            select: KeyCode::Char('v').into(),
            select_all: KeyCode::Char('V').into(),
            undo: KeyCode::Char('u').into(),
            edit: KeyCode::Char('e').into(),
            modify: KeyCode::Char('m').into(),
            shell: KeyCode::Char('!').into(),
            log: KeyCode::Char('l').into(),
            add: KeyCode::Char('a').into(),
            annotate: KeyCode::Char('A').into(),
            help: KeyCode::Char('?').into(),
            filter: KeyCode::Char('/').into(),
            zoom: KeyCode::Char('z').into(),
            context_menu: (KeyCode::Char('c')).into(),
            next_tab: KeyCode::Char(']').into(),
            previous_tab: KeyCode::Char('[').into(),
            shortcut0: KeyCode::Char('0').into(),
            shortcut1: KeyCode::Char('1').into(),
            shortcut2: KeyCode::Char('2').into(),
            shortcut3: KeyCode::Char('3').into(),
            shortcut4: KeyCode::Char('4').into(),
            shortcut5: KeyCode::Char('5').into(),
            shortcut6: KeyCode::Char('6').into(),
            shortcut7: KeyCode::Char('7').into(),
            shortcut8: KeyCode::Char('8').into(),
            shortcut9: KeyCode::Char('9').into(),
        }
    }
}

impl KeyConfig {
    pub fn new(data: &str) -> Result<Self> {
        let mut kc = Self::default();
        kc.update(data)?;
        Ok(kc)
    }

    pub fn update(&mut self, data: &str) -> Result<()> {
        let quit = self.get_config("uda.taskwarrior-tui.keyconfig.quit", data);
        let refresh = self.get_config("uda.taskwarrior-tui.keyconfig.refresh", data);
        let go_to_bottom = self.get_config("uda.taskwarrior-tui.keyconfig.go-to-bottom", data);
        let go_to_top = self.get_config("uda.taskwarrior-tui.keyconfig.go-to-top", data);
        let down = self.get_config("uda.taskwarrior-tui.keyconfig.down", data);
        let up = self.get_config("uda.taskwarrior-tui.keyconfig.up", data);
        let page_down = self.get_config("uda.taskwarrior-tui.keyconfig.page-down", data);
        let page_up = self.get_config("uda.taskwarrior-tui.keyconfig.page-up", data);
        let delete = self.get_config("uda.taskwarrior-tui.keyconfig.delete", data);
        let done = self.get_config("uda.taskwarrior-tui.keyconfig.done", data);
        let start_stop = self.get_config("uda.taskwarrior-tui.keyconfig.start-stop", data);
        let select = self.get_config("uda.taskwarrior-tui.keyconfig.select", data);
        let select_all = self.get_config("uda.taskwarrior-tui.keyconfig.select-all", data);
        let undo = self.get_config("uda.taskwarrior-tui.keyconfig.undo", data);
        let edit = self.get_config("uda.taskwarrior-tui.keyconfig.edit", data);
        let modify = self.get_config("uda.taskwarrior-tui.keyconfig.modify", data);
        let shell = self.get_config("uda.taskwarrior-tui.keyconfig.shell", data);
        let log = self.get_config("uda.taskwarrior-tui.keyconfig.log", data);
        let add = self.get_config("uda.taskwarrior-tui.keyconfig.add", data);
        let annotate = self.get_config("uda.taskwarrior-tui.keyconfig.annotate", data);
        let filter = self.get_config("uda.taskwarrior-tui.keyconfig.filter", data);
        let zoom = self.get_config("uda.taskwarrior-tui.keyconfig.zoom", data);
        let context_menu = self.get_config("uda.taskwarrior-tui.keyconfig.context-menu", data);
        let next_tab = self.get_config("uda.taskwarrior-tui.keyconfig.next-tab", data);
        let previous_tab = self.get_config("uda.taskwarrior-tui.keyconfig.previous-tab", data);

        self.quit = quit.unwrap_or(self.quit);
        self.refresh = refresh.unwrap_or(self.refresh);
        self.go_to_bottom = go_to_bottom.unwrap_or(self.go_to_bottom);
        self.go_to_top = go_to_top.unwrap_or(self.go_to_top);
        self.down = down.unwrap_or(self.down);
        self.up = up.unwrap_or(self.up);
        self.page_down = page_down.unwrap_or(self.page_down);
        self.page_up = page_up.unwrap_or(self.page_up);
        self.delete = delete.unwrap_or(self.delete);
        self.done = done.unwrap_or(self.done);
        self.start_stop = start_stop.unwrap_or(self.start_stop);
        self.select = select.unwrap_or(self.select);
        self.select_all = select_all.unwrap_or(self.select_all);
        self.undo = undo.unwrap_or(self.undo);
        self.edit = edit.unwrap_or(self.edit);
        self.modify = modify.unwrap_or(self.modify);
        self.shell = shell.unwrap_or(self.shell);
        self.log = log.unwrap_or(self.log);
        self.add = add.unwrap_or(self.add);
        self.annotate = annotate.unwrap_or(self.annotate);
        self.filter = filter.unwrap_or(self.filter);
        self.zoom = zoom.unwrap_or(self.zoom);
        self.context_menu = context_menu.unwrap_or(self.context_menu);
        self.next_tab = next_tab.unwrap_or(self.next_tab);
        self.previous_tab = previous_tab.unwrap_or(self.previous_tab);

        self.check()
    }

    pub fn check(&self) -> Result<()> {
        let mut elements = vec![
            &self.quit,
            &self.refresh,
            &self.go_to_bottom,
            &self.go_to_top,
            &self.down,
            &self.up,
            &self.page_down,
            &self.page_up,
            &self.delete,
            &self.done,
            &self.select,
            &self.select_all,
            &self.start_stop,
            &self.undo,
            &self.edit,
            &self.modify,
            &self.shell,
            &self.log,
            &self.add,
            &self.annotate,
            &self.help,
            &self.filter,
            &self.zoom,
            &self.context_menu,
            &self.next_tab,
            &self.previous_tab,
        ];
        let l = elements.len();
        elements.dedup();
        if l == elements.len() {
            Ok(())
        } else {
            Err(anyhow!("Duplicate keys found in key config"))
        }
    }

    fn get_config(&self, config: &str, data: &str) -> Option<KeyEvent> {
        for line in data.split('\n') {
            if line.starts_with(config) {
                let line = line.trim_start_matches(config).trim_start().trim_end().to_string();
                if line.len() == 1 {
                    return Some(KeyCode::Char(line.chars().next().unwrap()).into());
                } else {
                    return None;
                }
            } else if line.starts_with(&config.replace('-', "_")) {
                let line = line
                    .trim_start_matches(&config.replace('-', "_"))
                    .trim_start()
                    .trim_end()
                    .to_string();
                if line.len() == 1 {
                    return Some(KeyCode::Char(line.chars().next().unwrap()).into());
                } else {
                    return None;
                }
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}
