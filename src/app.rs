use std::io;

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Layout, Rect},
    prelude::Backend,
    style::{palette::tailwind::SLATE, Modifier, Style, Stylize},
    symbols,
    text::Line,
    widgets::{
        Block, Borders, Clear, HighlightSpacing, List, ListItem, ListState, Padding, Paragraph,
        StatefulWidget, Widget,
    },
    Terminal,
};
use tui_textarea::{CursorMove, TextArea};

use crate::sshconfig::{self, Config, ALL_KEYS};

const SELECTED_STYLE: Style = Style::new().bg(SLATE.c800).add_modifier(Modifier::BOLD);

/// This struct holds the current state of the app. In particular, it has the `todo_list` field
/// which is a wrapper around `ListState`. Keeping track of the state lets us render the
/// associated widget with its state and have access to features such as natural scrolling.
///
/// Check the event handling at the bottom to see how to change the state on incoming events. Check
/// the drawing logic for items on how to specify the highlighting style for selected items.
pub struct App {
    should_exit: bool,
    config_list: ConfigList,
    config_content_list: ConfigContentList,
    current_state: Option<AppState>,
}
enum AppState {
    Main(Main),
    Select(Select),
    Edit(Edit),
}

pub struct Main;

pub struct Select {
    idx: usize,
}

pub struct Edit {
    config_idx: usize,
    key: sshconfig::Key,
    textarea: TextArea<'static>,
}

impl Main {
    fn handle_key_main(self, app: &mut App, key: KeyEvent) {
        let new_state = match key.code {
            KeyCode::Char('q') | KeyCode::Esc => self.quit(app),
            KeyCode::Char('j') | KeyCode::Down => self.select_next(app),
            KeyCode::Char('k') | KeyCode::Up => self.select_previous(app),
            KeyCode::Char('l') | KeyCode::Right => self.state_next(app),
            KeyCode::Char('g') | KeyCode::Home => self.select_first(app),
            KeyCode::Char('G') | KeyCode::End => self.select_last(app),
            KeyCode::Char('d') | KeyCode::Delete => self.delete(app),
            _ => AppState::Main(self),
        };

        app.current_state = Some(new_state);
    }

    fn quit(self, app: &mut App) -> AppState {
        app.should_exit = true;
        AppState::Main(self)
    }

    fn select_next(self, app: &mut App) -> AppState {
        app.config_content_list.state = ListState::default();
        app.config_list.state.select_next();
        AppState::Main(self)
    }

    fn select_previous(self, app: &mut App) -> AppState {
        app.config_content_list.state = ListState::default();
        app.config_list.state.select_previous();
        AppState::Main(self)
    }

    fn state_next(self, app: &mut App) -> AppState {
        if let Some(idx) = app.config_list.state.selected() {
            app.config_content_list.state = ListState::default().with_selected(Some(0));
            AppState::Select(Select { idx })
        } else {
            AppState::Main(self)
        }
    }

    fn select_first(self, app: &mut App) -> AppState {
        app.config_list.state.select_first();
        AppState::Main(self)
    }

    fn select_last(self, app: &mut App) -> AppState {
        app.config_list.state.select_last();
        AppState::Main(self)
    }

    fn delete(self, app: &mut App) -> AppState {
        if let Some(i) = app.config_list.state.selected() {
            app.config_list.items.remove(i);
        }

        AppState::Main(self)
    }
}

impl Select {
    fn handle_key_select(self, app: &mut App, key: KeyEvent) {
        let new_state = match key.code {
            KeyCode::Char('q') | KeyCode::Esc => self.quit(app),
            KeyCode::Char('j') | KeyCode::Down => self.select_next(app),
            KeyCode::Char('k') | KeyCode::Up => self.select_previous(app),
            KeyCode::Char('h') | KeyCode::Left => self.state_back(),
            KeyCode::Char('l') | KeyCode::Right => self.state_next(app),
            KeyCode::Char('d') | KeyCode::Delete => self.delete(app),
            _ => AppState::Select(self),
        };

        app.current_state = Some(new_state);
    }

    fn quit(self, app: &mut App) -> AppState {
        app.should_exit = true;
        AppState::Select(self)
    }

    fn select_next(self, app: &mut App) -> AppState {
        app.config_content_list.state.select_next();
        AppState::Select(self)
    }

    fn select_previous(self, app: &mut App) -> AppState {
        app.config_content_list.state.select_previous();
        AppState::Select(self)
    }

    fn state_back(self) -> AppState {
        AppState::Main(Main)
    }

    fn state_next(self, app: &mut App) -> AppState {
        let cfg_idx = self.idx;

        if let Some(i) = app.config_content_list.state.selected() {
            let value = app.config_list.items[cfg_idx]
                .columns
                .get(&ALL_KEYS[i])
                .cloned();
            let mut v = vec![];
            if let Some(x) = value {
                v.push(x);
            }
            let mut textarea = TextArea::new(v);
            textarea.set_cursor_line_style(Style::default());
            textarea.move_cursor(CursorMove::End);
            AppState::Edit(Edit {
                key: ALL_KEYS[i],
                config_idx: cfg_idx,
                textarea,
            })
        } else {
            AppState::Select(self)
        }
    }

    fn delete(self, app: &mut App) -> AppState {
        if let Some(i) = app.config_content_list.state.selected() {
            let cfg_idx = self.idx;
            let key = ALL_KEYS[i];

            app.config_list.items[cfg_idx].columns.remove(&key);
        }

        AppState::Select(self)
    }
}

impl Edit {
    fn handle_key_edit(self, app: &mut App, key: KeyEvent) {
        let new_state = match key.code {
            KeyCode::Esc => self.state_back(),
            KeyCode::Enter => self.state_save(app),
            _ => self.other_input(key),
        };

        app.current_state = Some(new_state);
    }

    fn state_back(self) -> AppState {
        let idx = self.config_idx;
        AppState::Select(Select { idx })
    }

    fn state_save(self, app: &mut App) -> AppState {
        let textarea = &self.textarea;
        let idx = self.config_idx;

        let content = textarea.lines()[0].to_owned();
        let cfg = &mut app.config_list.items[self.config_idx];

        if let Some(v) = cfg.columns.get_mut(&self.key) {
            *v = content;
        } else {
            cfg.columns.insert(self.key, content);
        }

        AppState::Select(Select { idx })
    }

    fn other_input(mut self, key: KeyEvent) -> AppState {
        _ = self.textarea.input(key);
        AppState::Edit(self)
    }
}

struct ConfigList {
    items: Vec<Config>,
    state: ListState,
}

#[derive(Default)]
struct ConfigContentList {
    state: ListState,
}

impl Default for App {
    fn default() -> Self {
        Self {
            should_exit: false,
            config_list: ConfigList::new(),
            config_content_list: ConfigContentList::default(),
            current_state: Some(AppState::Main(Main)),
        }
    }
}

impl ConfigList {
    fn new() -> Self {
        ConfigList {
            items: Config::mock(),
            state: ListState::default(),
        }
    }
}

impl App {
    pub fn run(&mut self, mut terminal: Terminal<impl Backend>) -> io::Result<()> {
        while !self.should_exit {
            terminal.draw(|f| f.render_widget(&mut *self, f.size()))?;
            if let Event::Key(key) = event::read()? {
                self.handle_key(key);
            };
        }
        Ok(())
    }

    fn handle_key(&mut self, key: KeyEvent) {
        if key.kind != KeyEventKind::Press {
            return;
        }

        match self.current_state.take() {
            Some(AppState::Main(main)) => main.handle_key_main(self, key),
            Some(AppState::Select(select)) => select.handle_key_select(self, key),
            Some(AppState::Edit(edit)) => edit.handle_key_edit(self, key),
            None => unreachable!(),
        }
    }
}

impl Widget for &mut App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let [header_area, main_area, footer_area] = Layout::vertical([
            Constraint::Length(2),
            Constraint::Fill(1),
            Constraint::Length(1),
        ])
        .areas(area);

        let [index_area, selected_area] =
            Layout::horizontal([Constraint::Fill(1), Constraint::Fill(1)]).areas(main_area);

        App::render_header(header_area, buf);
        App::render_footer(footer_area, buf);
        self.render_list(index_area, buf);
        self.render_selected(selected_area, buf);
        self.render_textarea(buf);
    }
}

/// Rendering logic for the app
impl App {
    fn render_textarea(&mut self, buf: &mut Buffer) {
        let border_style = match self.current_state {
            Some(AppState::Edit(..)) => symbols::border::ROUNDED,
            _ => symbols::border::PLAIN,
        };

        let (key, textarea) = match &mut self.current_state {
            Some(AppState::Edit(edit)) => (edit.key, &mut edit.textarea),
            _ => return,
        };

        let pref_width = 30;
        let pref_height = 3;
        let width = std::cmp::min(buf.area.width, pref_width);
        let height = std::cmp::min(buf.area.height, pref_height);

        let s_x = (buf.area.width - width) / 2;
        let s_y = (buf.area.height - height) / 2;

        let area = Rect::new(s_x, s_y, width, height);
        let block = Block::bordered()
            .title(key.to_str())
            .border_set(border_style)
            .padding(Padding::horizontal(1));
        Clear.render(area, buf);
        textarea.set_block(block);
        textarea.widget().render(area, buf);
    }

    fn render_header(area: Rect, buf: &mut Buffer) {
        Paragraph::new("Ratatui List Example")
            .bold()
            .centered()
            .render(area, buf);
    }

    fn render_footer(area: Rect, buf: &mut Buffer) {
        Paragraph::new("Use ↓↑ to move, → to select, g/G to go top/bottom.")
            .centered()
            .render(area, buf);
    }

    fn render_selected(&mut self, area: Rect, buf: &mut Buffer) {
        let border_style = match self.current_state {
            Some(AppState::Select(..)) => symbols::border::THICK,
            _ => symbols::border::PLAIN,
        };

        let block = Block::bordered()
            .title(Line::raw(" Config ").centered())
            .borders(Borders::ALL)
            .border_set(border_style)
            .padding(Padding::uniform(1));

        if let Some(i) = self.config_list.state.selected() {
            let config = &self.config_list.items[i];
            let mut items = vec![];

            ALL_KEYS.iter().for_each(|k| {
                let key = k.to_str();
                let value = config
                    .columns
                    .get(k)
                    .map(|x| x.to_string())
                    .unwrap_or("<None>".to_owned());

                items.push(ListItem::new(format!("{}: {}", key, value)));
            });

            let list = List::new(items)
                .block(block)
                .highlight_style(SELECTED_STYLE)
                .highlight_symbol("> ")
                .highlight_spacing(HighlightSpacing::Always);

            StatefulWidget::render(list, area, buf, &mut self.config_content_list.state);
        } else {
            Paragraph::new("Nothing selected")
                .centered()
                .block(block)
                .render(area, buf);
        };
    }

    fn render_list(&mut self, area: Rect, buf: &mut Buffer) {
        let border_style = match self.current_state {
            Some(AppState::Main(..)) => symbols::border::THICK,
            _ => symbols::border::PLAIN,
        };

        let block = Block::bordered()
            .title(Line::raw(" Config list ").centered())
            .borders(Borders::ALL)
            .border_set(border_style)
            .padding(Padding::uniform(1));

        // Iterate through all elements in the `items` and stylize them.
        let items: Vec<ListItem> = self
            .config_list
            .items
            .iter()
            .map(|config| ListItem::from(config))
            .collect();

        // Create a List from all list items and highlight the currently selected one
        let list = List::new(items)
            .block(block)
            .highlight_style(SELECTED_STYLE)
            .highlight_symbol("> ")
            .highlight_spacing(HighlightSpacing::Always);

        // We need to disambiguate this trait method as both `Widget` and `StatefulWidget` share the
        // same method name `render`.
        StatefulWidget::render(list, area, buf, &mut self.config_list.state);
    }
}

impl From<&Config> for ListItem<'_> {
    fn from(value: &Config) -> Self {
        let line = Line::raw(format!("{}", value.host));
        ListItem::new(line)
    }
}
