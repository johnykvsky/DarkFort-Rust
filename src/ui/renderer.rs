use crate::core::character::Character;
use crate::core::monsters::Monster;
use crate::dungeon::map::DungeonGraph;
use crate::ui::ascii_art;
use crate::ui::theme::Theme;
use crate::ui::widgets::combat_widget::EncounterWidget;
use crate::ui::widgets::log_widget::LogWidget;
use crate::ui::widgets::map_widget::MapWidget;
use crate::ui::widgets::stats_widget::StatsWidget;
use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::style::{Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Clear, Paragraph};
use ratatui::Frame;

pub struct UIRenderer;

impl UIRenderer {
    /// Render the main in-game screen with 4 quadrants and header.
    #[allow(clippy::too_many_arguments)]
    pub fn render_game(
        f: &mut Frame,
        dungeon: &DungeonGraph,
        character: &Character,
        active_monster: Option<&Monster>,
        messages: &[String],
        prompt: &str,
        hotkeys: &[(&'static str, &'static str)],
        theme: &Theme,
    ) {
        let size = f.area();

        // Background block
        let background = Block::default().style(Style::default().bg(theme.bg));
        f.render_widget(background, size);

        // Vertical split: Header (3 lines) + Main Body
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Length(3), Constraint::Min(10)])
            .split(size);

        // Render Header
        let room = dungeon.current_room();
        let header_text = format!(
            " DARK FORT  |  Room #{}: {}  |  'The air is thick with dried blood and damp rot'",
            room.id.0,
            room.shape.name()
        );
        let header_p = Paragraph::new(header_text)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(theme.border))
            )
            .style(Style::default().fg(theme.title).add_modifier(Modifier::BOLD));
        f.render_widget(header_p, chunks[0]);

        // Horizontal split for main area: 2 columns
        let main_chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
            .split(chunks[1]);

        let top_row = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
            .split(main_chunks[0]);

        let bottom_row = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
            .split(main_chunks[1]);

        // Top-Left: Dungeon Map
        let map_widget = MapWidget::new(dungeon, theme);
        f.render_widget(map_widget, top_row[0]);

        // Top-Right: Character Stats
        let stats_widget = StatsWidget::new(character, theme);
        f.render_widget(stats_widget, top_row[1]);

        // Bottom-Left: Encounter / ASCII Art
        let encounter_widget = EncounterWidget::new(room, active_monster, theme);
        f.render_widget(encounter_widget, bottom_row[0]);

        // Bottom-Right: Action Log & Prompt
        let log_widget = LogWidget::new(messages, prompt, hotkeys, theme);
        f.render_widget(log_widget, bottom_row[1]);
    }

    /// Render Death Screen.
    pub fn render_death(f: &mut Frame, theme: &Theme, score_info: &str) {
        let size = f.area();
        let block = Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(theme.danger))
            .style(Style::default().bg(theme.bg));

        let inner = block.inner(size);
        f.render_widget(block, size);

        let content = format!("{}\n\n{}\n\nPress [R] to Rise Again  |  [Q] to Succumb & Quit", ascii_art::DEATH_ART, score_info);
        let p = Paragraph::new(content)
            .style(Style::default().fg(theme.danger).add_modifier(Modifier::BOLD))
            .alignment(ratatui::layout::Alignment::Center);

        f.render_widget(p, inner);
    }

    /// Render Victory Screen.
    pub fn render_victory(f: &mut Frame, theme: &Theme, score_info: &str) {
        let size = f.area();
        let block = Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(theme.title))
            .style(Style::default().bg(theme.bg));

        let inner = block.inner(size);
        f.render_widget(block, size);

        let content = format!("{}\n\n{}\n\nPress [R] to Delve Anew  |  [Q] to Retire in Glory", ascii_art::VICTORY_ART, score_info);
        let p = Paragraph::new(content)
            .style(Style::default().fg(theme.title).add_modifier(Modifier::BOLD))
            .alignment(ratatui::layout::Alignment::Center);

        f.render_widget(p, inner);
    }

    /// Render centered Quit Confirmation dialog.
    pub fn render_quit_dialog(f: &mut Frame, theme: &Theme) {
        let size = f.area();
        let dialog_width = 50.min(size.width);
        let dialog_height = 11.min(size.height);

        let area = Rect {
            x: (size.width.saturating_sub(dialog_width)) / 2,
            y: (size.height.saturating_sub(dialog_height)) / 2,
            width: dialog_width,
            height: dialog_height,
        };

        // Clear modal background so underlying screen doesn't show through
        f.render_widget(Clear, area);

        let block = Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(theme.quit_border).add_modifier(Modifier::BOLD))
            .title(" [ ABANDON DELVE ] ")
            .title_alignment(ratatui::layout::Alignment::Center)
            .title_style(Style::default().fg(theme.quit_border).add_modifier(Modifier::BOLD))
            .style(Style::default().bg(theme.bg));

        let inner = block.inner(area);
        f.render_widget(block, area);

        // Build lines dynamically to guarantee all choices are always visible
        let mut lines = Vec::new();
        if inner.height >= 9 {
            lines.push(Line::from(""));
        }
        lines.push(Line::from(Span::styled(
            "ABANDON DELVE?",
            Style::default().fg(theme.quit_fg).add_modifier(Modifier::BOLD),
        )));
        if inner.height >= 8 {
            lines.push(Line::from(""));
        }
        lines.push(Line::from(vec![
            Span::styled("[Y]", Style::default().fg(theme.quit_border).add_modifier(Modifier::BOLD)),
            Span::styled("es   - Quit without saving", Style::default().fg(theme.quit_fg)),
        ]));
        lines.push(Line::from(vec![
            Span::styled("[S]", Style::default().fg(theme.quit_border).add_modifier(Modifier::BOLD)),
            Span::styled("ave  - Save delve & return", Style::default().fg(theme.quit_fg)),
        ]));
        lines.push(Line::from(vec![
            Span::styled("[L]", Style::default().fg(theme.quit_border).add_modifier(Modifier::BOLD)),
            Span::styled("oad  - Load save & return", Style::default().fg(theme.quit_fg)),
        ]));
        lines.push(Line::from(vec![
            Span::styled("[N]", Style::default().fg(theme.quit_border).add_modifier(Modifier::BOLD)),
            Span::styled("o    - Return to delve ([Esc])", Style::default().fg(theme.quit_fg)),
        ]));

        let p = Paragraph::new(lines)
            .alignment(ratatui::layout::Alignment::Center);

        f.render_widget(p, inner);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ui::theme::{Theme, ThemeMode};
    use ratatui::backend::TestBackend;
    use ratatui::Terminal;

    #[test]
    fn test_quit_dialog_renders_all_choices() {
        let backend = TestBackend::new(80, 24);
        let mut terminal = Terminal::new(backend).unwrap();

        let white_theme = Theme::for_mode(ThemeMode::ClassicWhite);
        terminal
            .draw(|f| {
                UIRenderer::render_quit_dialog(f, &white_theme);
            })
            .unwrap();

        let buffer = terminal.backend().buffer();
        let content: String = buffer.content().iter().map(|cell| cell.symbol()).collect();
        assert!(content.contains("[Y]es   - Quit without saving"));
        assert!(content.contains("[S]ave  - Save delve & return"));
        assert!(content.contains("[L]oad  - Load save & return"));
        assert!(content.contains("[N]o    - Return to delve ([Esc])"));
        assert!(content.contains("ABANDON DELVE"));
    }

    #[test]
    fn test_quit_dialog_renders_on_compact_terminal() {
        let backend = TestBackend::new(50, 11);
        let mut terminal = Terminal::new(backend).unwrap();

        let amber_theme = Theme::for_mode(ThemeMode::AmberCrt);
        terminal
            .draw(|f| {
                UIRenderer::render_quit_dialog(f, &amber_theme);
            })
            .unwrap();

        let buffer = terminal.backend().buffer();
        let content: String = buffer.content().iter().map(|cell| cell.symbol()).collect();
        assert!(content.contains("[Y]es   - Quit without saving"));
        assert!(content.contains("[S]ave  - Save delve & return"));
        assert!(content.contains("[L]oad  - Load save & return"));
        assert!(content.contains("[N]o    - Return to delve ([Esc])"));
    }
}
