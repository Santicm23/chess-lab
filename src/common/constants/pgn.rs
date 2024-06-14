use super::{Color, Move};

#[derive(Debug, Clone)]
pub struct PgnLine {
    pub lines: Vec<PgnLine>,
    pub move_number: u32,
    pub mov: Move,
}

#[derive(Debug, Clone)]
pub struct PgnTree {
    pub event: Option<String>,
    pub site: Option<String>,
    pub date: Option<String>,
    pub round: Option<String>,
    pub white: Option<String>,
    pub black: Option<String>,
    pub result: Option<String>,
    pub lines: Vec<PgnLine>,
}

impl Default for PgnTree {
    fn default() -> PgnTree {
        PgnTree {
            event: None,
            site: None,
            date: None,
            round: None,
            white: None,
            black: None,
            result: None,
            lines: Vec::new(),
        }
    }
}

impl PgnTree {
    pub fn new(
        event: Option<String>,
        site: Option<String>,
        date: Option<String>,
        round: Option<String>,
        white: Option<String>,
        black: Option<String>,
        result: Option<String>,
    ) -> PgnTree {
        PgnTree {
            event,
            site,
            date,
            round,
            white,
            black,
            result,
            lines: Vec::new(),
        }
    }

    pub fn add_move(&mut self, mov: Move, mov_number: u32) {
        todo!()
    }

    pub fn rm_move(&mut self, mov: Move, mov_number: u32) {
        todo!()
    }

    pub fn get_move(&self, mov_number: u32, color: Color) -> Option<&Move> {
        todo!()
    }

    pub fn pgn_header(&self) -> String {
        todo!()
    }
}
