use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Clear, Paragraph},
};

use crate::game::{
    TetrominoType,
    board::{BOARD_HEIGHT, BOARD_WIDTH},
    piece::Piece,
    state::GameState,
};

const CELL_RENDER_WIDTH: u16 = 2;
const BOARD_RENDER_WIDTH: u16 = BOARD_WIDTH as u16 * CELL_RENDER_WIDTH;
const BOARD_RENDER_HEIGHT: u16 = BOARD_HEIGHT as u16;
const BOARD_PANEL_WIDTH: u16 = BOARD_RENDER_WIDTH + 2;
const BOARD_PANEL_HEIGHT: u16 = BOARD_RENDER_HEIGHT + 2;

pub fn render(frame: &mut Frame<'_>, state: &GameState, confirm_exit: bool) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Length(BOARD_PANEL_WIDTH), Constraint::Min(20)])
        .split(frame.area());

    let board_area = Rect {
        x: chunks[0].x,
        y: chunks[0].y,
        width: BOARD_PANEL_WIDTH.min(chunks[0].width),
        height: BOARD_PANEL_HEIGHT.min(chunks[0].height),
    };

    draw_board(frame, board_area, state);
    draw_sidebar(frame, chunks[1], state);

    if state.game_over {
        draw_popup(
            frame,
            "Game Over!\nPress r to restart or q to quit the game.",
        );
    } else if confirm_exit {
        draw_popup(frame, "Quit the game?\nPress y to confirm, n to cancel.");
    } else if state.paused {
        draw_popup(frame, "Paused\nPress p to resume.");
    }
}

fn draw_board(frame: &mut Frame<'_>, area: Rect, state: &GameState) {
    let block = Block::default().title("Game Board").borders(Borders::ALL);
    let mut constrained = area;
    constrained.width = constrained.width.min(BOARD_PANEL_WIDTH);
    constrained.height = constrained.height.min(BOARD_PANEL_HEIGHT);

    let inner = block.inner(constrained);
    let board_inner = Rect {
        x: inner.x,
        y: inner.y,
        width: BOARD_RENDER_WIDTH.min(inner.width),
        height: BOARD_RENDER_HEIGHT.min(inner.height),
    };

    frame.render_widget(block, constrained);

    let merged = state.board.merged_with_piece(&state.current);
    let mut rows = Vec::with_capacity(BOARD_HEIGHT);

    for row in merged.iter() {
        let mut spans = Vec::with_capacity(row.len());
        for cell in row.iter() {
            if let Some(kind) = cell {
                spans.push(Span::styled("██", Style::default().fg(color_for(*kind))));
            } else {
                spans.push(Span::raw("  "));
            }
        }
        rows.push(Line::from(spans));
    }

    let paragraph = Paragraph::new(rows).alignment(Alignment::Left);
    frame.render_widget(paragraph, board_inner);
}

fn draw_sidebar(frame: &mut Frame<'_>, area: Rect, state: &GameState) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(10),
            Constraint::Length(7),
            Constraint::Min(6),
        ])
        .split(area);

    draw_next_piece(frame, chunks[0], &state.next);
    draw_stats(frame, chunks[1], state);
    draw_help(frame, chunks[2]);
}

fn draw_next_piece(frame: &mut Frame<'_>, area: Rect, piece: &Piece) {
    let block = Block::default().title("Next Piece").borders(Borders::ALL);
    let inner = block.inner(area);
    frame.render_widget(block, area);

    let mut preview = [[None; 4]; 4];
    for (dx, dy) in piece.offsets() {
        if (0..4).contains(&dx) && (0..4).contains(&dy) {
            preview[dy as usize][dx as usize] = Some(piece.kind);
        }
    }

    let mut rows = Vec::with_capacity(4);
    for row in preview {
        let mut spans = Vec::with_capacity(4);
        for cell in row {
            if let Some(kind) = cell {
                spans.push(Span::styled("██", Style::default().fg(color_for(kind))));
            } else {
                spans.push(Span::raw("  "));
            }
        }
        rows.push(Line::from(spans));
    }

    let paragraph = Paragraph::new(rows).alignment(Alignment::Center);
    frame.render_widget(paragraph, inner);
}

fn draw_stats(frame: &mut Frame<'_>, area: Rect, state: &GameState) {
    let lines = vec![
        Line::from(format!("Score: {}", state.score)),
        Line::from(format!("Lines: {}", state.lines)),
        Line::from(format!(
            "Status: {}",
            if state.paused {
                "Paused"
            } else if state.game_over {
                "Game Over"
            } else {
                "Playing"
            }
        )),
    ];

    let paragraph = Paragraph::new(lines)
        .alignment(Alignment::Left)
        .block(Block::default().title("Stats").borders(Borders::ALL));
    frame.render_widget(paragraph, area);
}

fn draw_help(frame: &mut Frame<'_>, area: Rect) {
    let lines = vec![
        Line::from("←/h →/l : Move"),
        Line::from("↓/j     : Soft drop"),
        Line::from("↑/k     : Rotate"),
        Line::from("Space   : Hard drop"),
        Line::from("p       : Pause"),
        Line::from("q       : Quit (with confirm)"),
    ];

    let paragraph = Paragraph::new(lines)
        .alignment(Alignment::Left)
        .block(Block::default().title("Controls").borders(Borders::ALL));
    frame.render_widget(paragraph, area);
}

fn draw_popup(frame: &mut Frame<'_>, message: &str) {
    let area = centered_rect(60, 30, frame.area());
    frame.render_widget(Clear, area);
    let paragraph = Paragraph::new(message)
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::ALL).title("Notice"));
    frame.render_widget(paragraph, area);
}

fn centered_rect(percent_x: u16, percent_y: u16, area: Rect) -> Rect {
    let horizontal = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(area);

    Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(horizontal[1])[1]
}

fn color_for(kind: TetrominoType) -> Color {
    match kind {
        TetrominoType::I => Color::Cyan,
        TetrominoType::O => Color::Yellow,
        TetrominoType::T => Color::Magenta,
        TetrominoType::S => Color::Green,
        TetrominoType::Z => Color::Red,
        TetrominoType::J => Color::Blue,
        TetrominoType::L => Color::LightYellow,
    }
}
