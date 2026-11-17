use crate::ui::theme::{Theme, ThemeMode};
use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::style::{Modifier, Style};
use ratatui::widgets::{Block, Borders, Widget};

pub struct LogWidget<'a> {
    messages: &'a [String],
    prompt: &'a str,
    hotkeys: &'a [(&'static str, &'static str)],
    theme: &'a Theme,
}

impl<'a> LogWidget<'a> {
    pub fn new(
        messages: &'a [String],
        prompt: &'a str,
        hotkeys: &'a [(&'static str, &'static str)],
        theme: &'a Theme,
    ) -> Self {
        Self {
            messages,
            prompt,
            hotkeys,
            theme,
        }
    }
}

impl<'a> Widget for LogWidget<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let title = if self.theme.mode == ThemeMode::SciFiVoid {
            " [ MISSION LOG & TELEMETRY ] "
        } else {
            " [ CHRONICLE & ACTIONS ] "
        };

        let block = Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(self.theme.border))
            .title(title)
            .title_style(Style::default().fg(self.theme.title).add_modifier(Modifier::BOLD));

        let inner = block.inner(area);
        block.render(area, buf);

        if inner.width < 10 || inner.height < 3 {
            return;
        }

        // Reserve space for hotkeys bar and prompt lines
        let max_width = inner.width as usize;
        let has_hotkeys = !self.hotkeys.is_empty();
        let hotkey_height = if has_hotkeys { 1 } else { 0 };

        // Wrap prompt into lines so long shop menus and instructions never truncate
        let prompt_lines = if self.prompt.is_empty() {
            Vec::new()
        } else {
            wrap_text(self.prompt, max_width)
        };

        // Allow prompt to take needed lines, reserving at least 1 line for messages if possible
        let max_prompt_lines = (inner.height as usize).saturating_sub(hotkey_height + 1).max(1);
        let prompt_height = prompt_lines.len().min(max_prompt_lines);
        let log_height = (inner.height as usize).saturating_sub(hotkey_height + prompt_height);

        // Wrap messages into display lines while preserving per-message styling
        let mut wrapped_lines: Vec<(String, Style)> = Vec::new();

        for msg in self.messages {
            let style = if msg.contains("HIT") || msg.contains("slain") || msg.contains("deals") || msg.contains("Slew") {
                Style::default().fg(self.theme.success)
            } else if msg.contains("MISS") || msg.contains("Damage") || msg.contains("damage") || msg.contains("wound") || msg.contains("WOUND") {
                Style::default().fg(self.theme.danger)
            } else if msg.contains("LEVEL UP") || msg.contains("reward") || msg.contains("CLEARANCE") || msg.contains("CRITICAL") || msg.contains("WARNING") || msg.contains("ALERT") {
                Style::default().fg(self.theme.title).add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(self.theme.fg)
            };

            for line in wrap_text(msg, max_width) {
                wrapped_lines.push((line, style));
            }
        }

        let start_line_idx = wrapped_lines.len().saturating_sub(log_height);
        let visible_lines = &wrapped_lines[start_line_idx..];

        for (i, (line, style)) in visible_lines.iter().enumerate() {
            let y = inner.y + i as u16;
            buf.set_string(inner.x, y, line, *style);
        }

        // Draw Wrapped Prompt lines
        let prompt_start_y = inner.y + log_height as u16;
        for (i, p_line) in prompt_lines.iter().take(prompt_height).enumerate() {
            buf.set_string(
                inner.x,
                prompt_start_y + i as u16,
                p_line,
                Style::default().fg(self.theme.title).add_modifier(Modifier::BOLD),
            );
        }

        // Draw Hotkey bar
        if has_hotkeys {
            let hotkey_y = inner.y + inner.height - 1;
            let mut curr_x = inner.x;
            for (key, desc) in self.hotkeys {
                let key_str = format!("[{}]", key);
                let desc_str = format!("{} ", desc);

                buf.set_string(
                    curr_x,
                    hotkey_y,
                    &key_str,
                    Style::default().fg(self.theme.accent).add_modifier(Modifier::BOLD),
                );
                curr_x += key_str.len() as u16;

                buf.set_string(curr_x, hotkey_y, &desc_str, Style::default().fg(self.theme.fg));
                curr_x += desc_str.len() as u16;

                if curr_x >= inner.x + inner.width {
                    break;
                }
            }
        }
    }
}

/// Wrap text into lines of at most `max_width` characters, breaking on whitespace where possible.
pub fn wrap_text(text: &str, max_width: usize) -> Vec<String> {
    if max_width == 0 {
        return Vec::new();
    }
    let mut lines = Vec::new();
    let mut current_line = String::new();
    let mut current_len = 0;

    for word in text.split_whitespace() {
        let word_len = word.chars().count();
        if current_line.is_empty() {
            if word_len > max_width {
                // Word is longer than line width: chunk by chars
                let chars: Vec<char> = word.chars().collect();
                let mut start = 0;
                while start < chars.len() {
                    let end = (start + max_width).min(chars.len());
                    let chunk: String = chars[start..end].iter().collect();
                    if end == chars.len() {
                        current_line = chunk;
                        current_len = chars.len() - start;
                    } else {
                        lines.push(chunk);
                    }
                    start = end;
                }
            } else {
                current_line.push_str(word);
                current_len = word_len;
            }
        } else if current_len + 1 + word_len <= max_width {
            current_line.push(' ');
            current_line.push_str(word);
            current_len += 1 + word_len;
        } else {
            lines.push(current_line);
            current_line = String::new();
            if word_len > max_width {
                let chars: Vec<char> = word.chars().collect();
                let mut start = 0;
                while start < chars.len() {
                    let end = (start + max_width).min(chars.len());
                    let chunk: String = chars[start..end].iter().collect();
                    if end == chars.len() {
                        current_line = chunk;
                        current_len = chars.len() - start;
                    } else {
                        lines.push(chunk);
                    }
                    start = end;
                }
            } else {
                current_line.push_str(word);
                current_len = word_len;
            }
        }
    }

    if !current_line.is_empty() {
        lines.push(current_line);
    }

    if lines.is_empty() && !text.is_empty() {
        lines.push(text.to_string());
    }

    lines
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_wrap_text_splits_on_word_boundaries() {
        let text = "WARNING: USCSS ATROPOS DOCKING COMPLETE. LIFE SUPPORT FAILING.";
        let wrapped = wrap_text(text, 25);
        assert!(wrapped.len() >= 3);
        for line in &wrapped {
            assert!(line.chars().count() <= 25);
        }
        assert_eq!(wrapped[0], "WARNING: USCSS ATROPOS");
    }

    #[test]
    fn test_wrap_text_handles_utf8() {
        let text = "Found a Mighty Zweihänder (1d6+2 damage) — MÖRK BORG doom!";
        let wrapped = wrap_text(text, 30);
        for line in &wrapped {
            assert!(line.chars().count() <= 30);
        }
    }

    #[test]
    fn test_wrap_text_single_long_word() {
        let text = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
        let wrapped = wrap_text(text, 10);
        assert_eq!(wrapped.len(), 3);
        assert_eq!(wrapped[0], "ABCDEFGHIJ");
        assert_eq!(wrapped[1], "KLMNOPQRST");
        assert_eq!(wrapped[2], "UVWXYZ");
    }

    #[test]
    fn test_log_widget_renders_multiline_prompt_without_truncation() {
        let theme = Theme::for_mode(ThemeMode::SciFiVoid);
        let messages = vec!["Encountered requisition kiosk.".to_string()];
        let prompt = "Requisition Kiosk: [1] Medkit (15cr) | [2] Stimpack (20cr) | [3] Heavy Boarding Armor (35cr) | [4] M41 Pulse Rifle (40cr) | [X] Exit";
        let hotkeys = [("1", "Buy 1"), ("X", "Leave")];
        let widget = LogWidget::new(&messages, prompt, &hotkeys, &theme);

        let area = Rect::new(0, 0, 40, 10);
        let mut buf = Buffer::empty(area);
        widget.render(area, &mut buf);

        // Verify that tokens from later parts of prompt were written and not truncated
        let mut full_rendered = String::new();
        for y in 0..area.height {
            for x in 0..area.width {
                full_rendered.push_str(buf[(x, y)].symbol());
            }
            full_rendered.push('\n');
        }

        assert!(full_rendered.contains("Requisition Kiosk:"));
        assert!(full_rendered.contains("Pulse"));
        assert!(full_rendered.contains("Rifle"));
        assert!(full_rendered.contains("[X] Exit"));
    }
}
