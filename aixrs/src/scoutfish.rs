use std::{io::Write, str::FromStr};

use crate::ffi::{ScoutfishQueryParseError, Subfen};
use aix_chess_compression::{Decode, Decoder, EncodedGame};
use serde::Deserialize;
use shakmaty::{san::San, Chess, Color, Move, Position};

#[derive(Deserialize)]
#[serde(untagged, deny_unknown_fields)]
#[cfg_attr(test, derive(Debug, PartialEq))]
enum Strings {
    Single(String),
    Multiple(Vec<String>),
}

impl Strings {
    fn process<TFunc, TOut>(&self, func: TFunc) -> Result<Vec<TOut>, ScoutfishQueryParseError>
    where
        TFunc: Fn(&str) -> Result<TOut, ScoutfishQueryParseError>,
    {
        match self {
            Strings::Single(s) => Ok(vec![func(s)?]),
            Strings::Multiple(v) => {
                let mut results = Vec::with_capacity(v.len());
                for s in v {
                    results.push(func(s)?);
                }
                Ok(results)
            }
        }
    }
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
#[cfg_attr(test, derive(Debug, PartialEq))]
struct RawRule {
    #[serde(rename = "sub-fen")]
    sub_fen: Option<Strings>,
    material: Option<Strings>,
    imbalance: Option<Strings>,
    #[serde(rename = "white-move")]
    white_move: Option<Strings>,
    #[serde(rename = "black-move")]
    black_move: Option<Strings>,
    moved: Option<String>,
    captured: Option<String>,
    stm: Option<String>,
    pass: Option<String>,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
#[cfg_attr(test, derive(Debug, PartialEq))]
struct RawStreak {
    streak: Vec<RawRule>,
}

#[derive(Deserialize)]
#[serde(untagged, deny_unknown_fields)]
#[cfg_attr(test, derive(Debug, PartialEq))]
enum RawSequenceElement {
    Rule(RawRule),
    Streak(RawStreak),
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
#[cfg_attr(test, derive(Debug, PartialEq))]
struct RawSequence {
    sequence: Vec<RawSequenceElement>,
}

#[derive(Deserialize)]
#[serde(untagged, deny_unknown_fields)]
#[cfg_attr(test, derive(Debug, PartialEq))]
enum RawQuery {
    Rule(RawRule),
    Sequence(RawSequence),
    Streak(RawStreak),
}

impl RawQuery {
    fn parse(s: &[u8]) -> Result<RawQuery, serde_json::Error> {
        serde_json::from_slice(s)
    }
}

#[cfg_attr(test, derive(Debug, PartialEq))]
#[derive(bincode::Encode, bincode::Decode)]
pub struct ColoredPieceCounts {
    w_k: u8,
    w_p: u8,
    w_n: u8,
    w_b: u8,
    w_r: u8,
    w_q: u8,
    b_k: u8,
    b_p: u8,
    b_n: u8,
    b_b: u8,
    b_r: u8,
    b_q: u8,
}

impl ColoredPieceCounts {
    fn imbalance(s: &str) -> Result<ColoredPieceCounts, ScoutfishQueryParseError> {
        let mut counts = ColoredPieceCounts {
            w_k: 0,
            w_p: 0,
            w_n: 0,
            w_b: 0,
            w_r: 0,
            w_q: 0,
            b_k: 0,
            b_p: 0,
            b_n: 0,
            b_b: 0,
            b_r: 0,
            b_q: 0,
        };

        let parts = s.split('v').collect::<Vec<_>>();
        if parts.len() == 2 {
            for c in parts[0].chars() {
                match c {
                    'P' => counts.w_p += 1,
                    'N' => counts.w_n += 1,
                    'B' => counts.w_b += 1,
                    'R' => counts.w_r += 1,
                    'Q' => counts.w_q += 1,
                    _ => return Err(ScoutfishQueryParseError::InvalidPiece),
                }
            }
            for c in parts[1].chars() {
                match c {
                    'P' => counts.b_p += 1,
                    'N' => counts.b_n += 1,
                    'B' => counts.b_b += 1,
                    'R' => counts.b_r += 1,
                    'Q' => counts.b_q += 1,
                    _ => return Err(ScoutfishQueryParseError::InvalidPiece),
                }
            }
        } else {
            return Err(ScoutfishQueryParseError::InvalidImbalanceFormat);
        }

        Ok(counts)
    }

    fn material(s: &str) -> Result<ColoredPieceCounts, ScoutfishQueryParseError> {
        let mut counts = ColoredPieceCounts {
            w_k: 1,
            w_p: 0,
            w_n: 0,
            w_b: 0,
            w_r: 0,
            w_q: 0,
            b_k: 0,
            b_p: 0,
            b_n: 0,
            b_b: 0,
            b_r: 0,
            b_q: 0,
        };

        if &s[0..1] != "K" {
            return Err(ScoutfishQueryParseError::InvalidMaterialFormat);
        }
        let mut white = true;

        for c in s[1..].chars() {
            if white {
                match c {
                    'P' => counts.w_p += 1,
                    'N' => counts.w_n += 1,
                    'B' => counts.w_b += 1,
                    'R' => counts.w_r += 1,
                    'Q' => counts.w_q += 1,
                    'K' => {
                        white = false;
                        counts.b_k = 1;
                    }
                    _ => {
                        return Err(ScoutfishQueryParseError::InvalidPiece);
                    }
                }
            } else {
                match c {
                    'P' => counts.b_p += 1,
                    'N' => counts.b_n += 1,
                    'B' => counts.b_b += 1,
                    'R' => counts.b_r += 1,
                    'Q' => counts.b_q += 1,
                    'K' => {
                        return Err(ScoutfishQueryParseError::InvalidMaterialFormat);
                    }
                    _ => {
                        return Err(ScoutfishQueryParseError::InvalidPiece);
                    }
                }
            }
        }

        if counts.b_k == 0 {
            return Err(ScoutfishQueryParseError::InvalidMaterialFormat);
        }

        Ok(counts)
    }
}

#[cfg_attr(test, derive(Debug, PartialEq))]
#[derive(bincode::Encode, bincode::Decode)]
pub struct PieceFlags {
    k: bool,
    p: bool,
    n: bool,
    b: bool,
    r: bool,
    q: bool,
}

impl PieceFlags {
    fn moved_captured(s: &str) -> Result<PieceFlags, ScoutfishQueryParseError> {
        let mut flags = PieceFlags {
            k: false,
            p: false,
            n: false,
            b: false,
            r: false,
            q: false,
        };

        for c in s.chars() {
            match c {
                'K' => flags.k = true,
                'P' => flags.p = true,
                'N' => flags.n = true,
                'B' => flags.b = true,
                'R' => flags.r = true,
                'Q' => flags.q = true,
                _ => return Err(ScoutfishQueryParseError::InvalidPiece),
            }
        }

        Ok(flags)
    }
}

#[cfg_attr(test, derive(Debug, PartialEq))]
#[derive(Copy, Clone, bincode::Encode, bincode::Decode)]
enum SanSubset {
    Normal {
        role: shakmaty::Role,
        file: Option<shakmaty::File>,
        rank: Option<shakmaty::Rank>,
        capture: bool,
        to: shakmaty::Square,
        promotion: Option<shakmaty::Role>,
    },
    Castle(shakmaty::CastlingSide),
}

impl SanSubset {
    fn from_san(san: San) -> Result<SanSubset, ScoutfishQueryParseError> {
        match san {
            San::Normal {
                role,
                file,
                rank,
                capture,
                to,
                promotion,
            } => Ok(SanSubset::Normal {
                role,
                file,
                rank,
                capture,
                to,
                promotion,
            }),
            San::Castle(side) => Ok(SanSubset::Castle(side)),
            _ => Err(ScoutfishQueryParseError::InvalidSan),
        }
    }

    pub fn to_san(self) -> San {
        match self {
            SanSubset::Normal {
                role,
                file,
                rank,
                capture,
                to,
                promotion,
            } => San::Normal {
                role,
                file,
                rank,
                capture,
                to,
                promotion,
            },
            SanSubset::Castle(side) => San::Castle(side),
        }
    }

    fn from_str(s: &str) -> Result<SanSubset, ScoutfishQueryParseError> {
        let san = San::from_str(s).map_err(|_| ScoutfishQueryParseError::InvalidSan)?;
        SanSubset::from_san(san)
    }
}

#[cfg_attr(test, derive(Debug, PartialEq))]
#[derive(bincode::Encode, bincode::Decode)]
pub struct Rule {
    sub_fen: Option<Vec<Subfen>>,
    material: Option<Vec<ColoredPieceCounts>>,
    imbalance: Option<Vec<ColoredPieceCounts>>,
    white_move: Option<Vec<SanSubset>>,
    black_move: Option<Vec<SanSubset>>,
    moved: Option<PieceFlags>,
    captured: Option<PieceFlags>,
    stm: Option<bool>,
    pass: bool,
}

#[cfg_attr(test, derive(Debug, PartialEq))]
#[derive(bincode::Encode, bincode::Decode)]
pub struct Streak(Vec<Rule>);

#[cfg_attr(test, derive(Debug, PartialEq))]
#[derive(bincode::Encode, bincode::Decode)]
pub enum SequenceElement {
    Rule(Rule),
    Streak(Streak),
}

#[cfg_attr(test, derive(Debug, PartialEq))]
#[derive(bincode::Encode, bincode::Decode)]
pub struct Sequence(Vec<SequenceElement>);

struct SequenceState {
    index: usize,
    inner_streak_state: Option<StreakState>,
    plies: Option<Vec<u16>>,
}

enum SequenceFlow {
    FullMatch,
    Continue,
    NeverMatch,
}

enum StreakFlow {
    FullMatch,
    Continue,
    #[allow(dead_code)]
    NeverMatch,
}

#[cfg_attr(test, derive(Debug, PartialEq))]
#[derive(bincode::Encode, bincode::Decode)]
pub enum Query {
    Rule(Rule),
    Sequence(Sequence),
    Streak(Streak),
}

fn parse_subfen(s: &str) -> Result<Subfen, ScoutfishQueryParseError> {
    crate::subfen::try_parse(s.as_bytes()).map_err(|_| ScoutfishQueryParseError::InvalidPiece)
}

impl Rule {
    fn frow_raw(raw: &RawRule) -> Result<Rule, ScoutfishQueryParseError> {
        Ok(Rule {
            sub_fen: raw
                .sub_fen
                .as_ref()
                .map(|s| s.process(parse_subfen))
                .transpose()?,
            material: raw
                .material
                .as_ref()
                .map(|m| m.process(ColoredPieceCounts::material))
                .transpose()?,
            imbalance: raw
                .imbalance
                .as_ref()
                .map(|i| i.process(ColoredPieceCounts::imbalance))
                .transpose()?,
            white_move: raw
                .white_move
                .as_ref()
                .map(|m| m.process(SanSubset::from_str))
                .transpose()?,
            black_move: raw
                .black_move
                .as_ref()
                .map(|m| m.process(SanSubset::from_str))
                .transpose()?,
            moved: raw
                .moved
                .as_deref()
                .map(PieceFlags::moved_captured)
                .transpose()?,
            captured: raw
                .captured
                .as_deref()
                .map(PieceFlags::moved_captured)
                .transpose()?,
            stm: match raw.stm.as_deref() {
                Some("white") => Some(true),
                Some("black") => Some(false),
                Some(_) => return Err(ScoutfishQueryParseError::InvalidSideToMove),
                None => None,
            },
            pass: raw.pass.is_some(),
        })
    }

    pub fn apply(&self, mv_opt: Option<Move>, pos: &Chess) -> bool {
        if let Some(stm) = self.stm {
            if stm != (pos.turn() == Color::White) {
                return false;
            }
        }

        if pos.turn() == Color::White && self.black_move.is_some() {
            return false;
        }

        if pos.turn() == Color::Black && self.white_move.is_some() {
            return false;
        }

        let board = pos.board();

        if let Some(moved) = &self.moved {
            if let Some(mv) = mv_opt {
                let in_moved = match board.role_at(
                    mv.from()
                        .expect(".from() cannot be None because move is not a Crazyhouse drop"),
                ) {
                    Some(role) => match role {
                        shakmaty::Role::King => moved.k,
                        shakmaty::Role::Pawn => moved.p,
                        shakmaty::Role::Knight => moved.n,
                        shakmaty::Role::Bishop => moved.b,
                        shakmaty::Role::Rook => moved.r,
                        shakmaty::Role::Queen => moved.q,
                    },
                    None => false,
                };

                if !in_moved {
                    return false;
                }
            } else {
                return false;
            }
        }

        if let Some(captured) = &self.captured {
            if let Some(mv) = mv_opt {
                let in_captured = match mv.capture() {
                    Some(role) => match role {
                        shakmaty::Role::King => captured.k,
                        shakmaty::Role::Pawn => captured.p,
                        shakmaty::Role::Knight => captured.n,
                        shakmaty::Role::Bishop => captured.b,
                        shakmaty::Role::Rook => captured.r,
                        shakmaty::Role::Queen => captured.q,
                    },
                    None => false,
                };

                if !in_captured {
                    return false;
                }
            } else {
                return false;
            }
        }

        if let Some(material) = &self.material {
            let mut matched = false;

            let white = board.white();
            let black = board.black();

            for mat in material {
                if usize::from(mat.w_q) == (board.queens() & white).count()
                    && usize::from(mat.w_r) == (board.rooks() & white).count()
                    && usize::from(mat.w_b) == (board.bishops() & white).count()
                    && usize::from(mat.w_n) == (board.knights() & white).count()
                    && usize::from(mat.w_p) == (board.pawns() & white).count()
                    && usize::from(mat.b_q) == (board.queens() & black).count()
                    && usize::from(mat.b_r) == (board.rooks() & black).count()
                    && usize::from(mat.b_b) == (board.bishops() & black).count()
                    && usize::from(mat.b_n) == (board.knights() & black).count()
                    && usize::from(mat.b_p) == (board.pawns() & black).count()
                {
                    matched = true;
                    break;
                }
            }

            if !matched {
                return false;
            }
        }

        if let Some(imbalance) = &self.imbalance {
            let mut matched = false;

            let white = board.white();
            let black = board.black();

            for imb in imbalance {
                let gq = imb.w_q as isize - imb.b_q as isize;
                let gr = imb.w_r as isize - imb.b_r as isize;
                let gb = imb.w_b as isize - imb.b_b as isize;
                let gn = imb.w_n as isize - imb.b_n as isize;
                let gp = imb.w_p as isize - imb.b_p as isize;

                let aq = (board.queens() & white).count() as isize
                    - (board.queens() & black).count() as isize;
                let ar = (board.rooks() & white).count() as isize
                    - (board.rooks() & black).count() as isize;
                let ab = (board.bishops() & white).count() as isize
                    - (board.bishops() & black).count() as isize;
                let an = (board.knights() & white).count() as isize
                    - (board.knights() & black).count() as isize;
                let ap = (board.pawns() & white).count() as isize
                    - (board.pawns() & black).count() as isize;

                if gq == aq && gr == ar && gb == ab && gn == an && gp == ap {
                    matched = true;
                    break;
                }
            }

            if !matched {
                return false;
            }
        }

        let moves = if self.white_move.is_some() {
            self.white_move.as_ref()
        } else if self.black_move.is_some() {
            self.black_move.as_ref()
        } else {
            None
        };

        if let Some(moves) = moves {
            if let Some(mv) = mv_opt {
                let mut matched = false;

                for candidate_move in moves {
                    let san = candidate_move.to_san();
                    if san.matches(mv) {
                        matched = true;
                        break;
                    }
                }

                if !matched {
                    return false;
                }
            } else {
                return false;
            }
        }

        if let Some(sub_fen) = &self.sub_fen {
            let mut matched = false;

            for sf in sub_fen {
                if crate::subfen::matches_board(sf, board) {
                    matched = true;
                    break;
                }
            }

            if !matched {
                return false;
            }
        }

        true
    }
}

impl Sequence {
    fn from_raw(raw: &RawSequence) -> Result<Sequence, ScoutfishQueryParseError> {
        let mut elements = Vec::with_capacity(raw.sequence.len());
        for e in &raw.sequence {
            match e {
                RawSequenceElement::Rule(r) => {
                    elements.push(SequenceElement::Rule(Rule::frow_raw(r)?));
                }
                RawSequenceElement::Streak(s) => {
                    elements.push(SequenceElement::Streak(Streak::from_raw(s)?));
                }
            }
        }
        Ok(Sequence(elements))
    }

    fn apply(
        &self,
        mv_opt: Option<Move>,
        pos: &Chess,
        state: &mut SequenceState,
        ply: u16,
    ) -> SequenceFlow {
        match &self.0[state.index] {
            SequenceElement::Rule(r) => {
                if r.apply(mv_opt, pos) {
                    if let Some(plies) = state.plies.as_mut() {
                        plies.push(ply);
                    }
                    state.index += 1;
                    if state.index == self.0.len() {
                        return SequenceFlow::FullMatch;
                    }
                }

                SequenceFlow::Continue
            }
            SequenceElement::Streak(s) => {
                if state.inner_streak_state.is_none() {
                    state.inner_streak_state = Some(s.new_empty_state());
                }

                match s.apply(
                    mv_opt,
                    pos,
                    state
                        .inner_streak_state
                        .as_mut()
                        .expect("inner_streak_state cannot be None because the value was just set"),
                ) {
                    StreakFlow::FullMatch => {
                        state.index += 1;
                        state.inner_streak_state = None;

                        if let Some(plies) = state.plies.as_mut() {
                            plies.extend(ply + 1 - (s.0.len() as u16)..=ply);
                        }

                        if state.index == self.0.len() {
                            return SequenceFlow::FullMatch;
                        }
                        SequenceFlow::Continue
                    }
                    StreakFlow::Continue => SequenceFlow::Continue,
                    StreakFlow::NeverMatch => SequenceFlow::NeverMatch,
                }
            }
        }
    }
}

struct StreakState {
    check_index: Vec<bool>,
}

impl Streak {
    fn from_raw(raw: &RawStreak) -> Result<Streak, ScoutfishQueryParseError> {
        let mut rules = Vec::with_capacity(raw.streak.len());
        for r in &raw.streak {
            rules.push(Rule::frow_raw(r)?);
        }
        Ok(Streak(rules))
    }

    fn apply(&self, mv_opt: Option<Move>, pos: &Chess, state: &mut StreakState) -> StreakFlow {
        let len = self.0.len();
        for i in (0..len).rev() {
            if state.check_index[i] {
                if self.0[i].apply(mv_opt, pos) {
                    if i == len - 1 {
                        return StreakFlow::FullMatch;
                    } else {
                        state.check_index[i + 1] = true;
                    }
                } else if i != 0 {
                    state.check_index[i] = false;
                }
            }
        }

        StreakFlow::Continue
    }

    fn new_empty_state(&self) -> StreakState {
        let mut check_index = vec![false; self.0.len()];
        check_index[0] = true;
        StreakState { check_index }
    }
}

impl Query {
    fn from_raw(raw: &RawQuery) -> Result<Query, ScoutfishQueryParseError> {
        match raw {
            RawQuery::Rule(r) => Ok(Query::Rule(Rule::frow_raw(r)?)),
            RawQuery::Sequence(s) => Ok(Query::Sequence(Sequence::from_raw(s)?)),
            RawQuery::Streak(s) => Ok(Query::Streak(Streak::from_raw(s)?)),
        }
    }

    fn parse(s: &[u8]) -> Result<Query, ScoutfishQueryParseError> {
        let raw =
            RawQuery::parse(s).map_err(|_| ScoutfishQueryParseError::InvalidSyntaxOrStructure)?;
        Query::from_raw(&raw)
    }

    pub fn parse_into_bytes(s: &[u8], out: &mut [u8]) -> Result<usize, ScoutfishQueryParseError> {
        Query::parse(s).and_then(|query| {
            bincode::encode_to_vec(&query, bincode::config::standard())
                .map_err(|_| ScoutfishQueryParseError::BincodeError)
                .and_then(|bytes| {
                    if bytes.len() > out.len() {
                        return Err(ScoutfishQueryParseError::BufferTooSmall);
                    }

                    let mut cursor = std::io::Cursor::new(out);
                    cursor
                        .write(&bytes)
                        .map_err(|_| ScoutfishQueryParseError::CursorWriteError)
                        .map(|w| {
                            assert_eq!(w, bytes.len());
                            w
                        })
                })
        })
    }

    pub fn decode_bytes(data: &[u8]) -> Result<Query, ()> {
        bincode::decode_from_slice::<Query, _>(data, bincode::config::standard())
            .map_err(|_| ())
            .map(|(query, _)| query)
    }

    pub fn apply(
        &self,
        game: &EncodedGame,
        return_plies: bool,
    ) -> Result<(bool, Option<Vec<u16>>), crate::ffi::DecodeError> {
        let decoder = Decoder::new(game);
        let mut pos_opt = Some(Chess::new());

        let mut sequence_state = if let Query::Sequence(_) = self {
            Some(SequenceState {
                index: 0,
                inner_streak_state: None,
                plies: if return_plies { Some(Vec::new()) } else { None },
            })
        } else {
            None
        };

        let mut streak_state = if let Query::Streak(streak) = self {
            Some(streak.new_empty_state())
        } else {
            None
        };

        for (ply, res) in decoder
            .into_iter_moves_and_positions()
            .map(|r| r.map(|(m, p)| (Some(m), Some(p))))
            .chain(vec![Ok((None, None))]) // need an extra iteration to process the last position
            .enumerate()
        {
            let (mv, next_pos) = res?;

            let pos = pos_opt
                .as_ref()
                .expect("Internal error (Query.apply): pos_opt cannot be None");
            let ply = ply as u16;

            match self {
                Query::Rule(r) => {
                    if r.apply(mv, &pos) {
                        return Ok((true, if return_plies { Some(vec![ply]) } else { None }));
                    }
                }
                Query::Sequence(s) => {
                    match s.apply(
                        mv,
                        pos,
                        sequence_state
                            .as_mut()
                            .expect("sequence_state cannot be None if Query::Sequence matches"),
                        ply,
                    ) {
                        SequenceFlow::FullMatch => {
                            return Ok((
                                true,
                                sequence_state
                                    .expect(
                                        "sequence_state cannot be None if Query::Sequence matches",
                                    )
                                    .plies,
                            ));
                        }
                        SequenceFlow::Continue => {}
                        SequenceFlow::NeverMatch => {
                            return Ok((false, None));
                        }
                    }
                }
                Query::Streak(s) => match s.apply(
                    mv,
                    &pos,
                    streak_state
                        .as_mut()
                        .expect("streak_state cannot be None if Query::Streak matches"),
                ) {
                    StreakFlow::FullMatch => {
                        return Ok((
                            true,
                            if return_plies {
                                Some((ply + 1 - (s.0.len() as u16)..=ply).collect())
                            } else {
                                None
                            },
                        ));
                    }
                    StreakFlow::Continue => {}
                    StreakFlow::NeverMatch => {
                        return Ok((false, None));
                    }
                },
            }

            pos_opt = next_pos;
        }

        Ok((false, None))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_rule1() {
        let query = RawQuery::parse(br#"{ "sub-fen": "8/8/p7/8/8/1B3N2/8/8" }"#).unwrap();
        assert_eq!(
            query,
            RawQuery::Rule(RawRule {
                sub_fen: Some(Strings::Single("8/8/p7/8/8/1B3N2/8/8".to_string())),
                material: None,
                imbalance: None,
                white_move: None,
                black_move: None,
                moved: None,
                captured: None,
                stm: None,
                pass: None,
            })
        );
    }

    #[test]
    fn test_parse_rule2() {
        let query =
            RawQuery::parse(br#"{ "sub-fen": "8/8/8/8/1k6/8/8/8", "material": "KBNKP" }"#).unwrap();
        assert_eq!(
            query,
            RawQuery::Rule(RawRule {
                sub_fen: Some(Strings::Single("8/8/8/8/1k6/8/8/8".to_string())),
                material: Some(Strings::Single("KBNKP".to_string())),
                imbalance: None,
                white_move: None,
                black_move: None,
                moved: None,
                captured: None,
                stm: None,
                pass: None,
            })
        );
    }

    #[test]
    fn test_parse_rule3() {
        let query = RawQuery::parse(br#"{ "white-move": "O-O-O" }"#).unwrap();
        assert_eq!(
            query,
            RawQuery::Rule(RawRule {
                sub_fen: None,
                material: None,
                imbalance: None,
                white_move: Some(Strings::Single("O-O-O".to_string())),
                black_move: None,
                moved: None,
                captured: None,
                stm: None,
                pass: None,
            })
        );

        let processed = Query::from_raw(&query).unwrap();
        assert_eq!(
            processed,
            Query::Rule(Rule {
                sub_fen: None,
                material: None,
                imbalance: None,
                white_move: Some(vec![SanSubset::Castle(shakmaty::CastlingSide::QueenSide)]),
                black_move: None,
                moved: None,
                captured: None,
                stm: None,
                pass: false,
            })
        );
    }

    #[test]
    fn test_parse_rule4() {
        let query =
            RawQuery::parse(br#"{ "sub-fen": ["8/8/8/q7/8/8/8/8", "8/8/8/r7/8/8/8/8"] }"#).unwrap();
        assert_eq!(
            query,
            RawQuery::Rule(RawRule {
                sub_fen: Some(Strings::Multiple(vec![
                    "8/8/8/q7/8/8/8/8".to_string(),
                    "8/8/8/r7/8/8/8/8".to_string()
                ])),
                material: None,
                imbalance: None,
                white_move: None,
                black_move: None,
                moved: None,
                captured: None,
                stm: None,
                pass: None,
            })
        );
    }

    #[test]
    fn test_parse_rule5() {
        let query = RawQuery::parse(br#"{ "material": ["KBNKNN", "KBNPKNN"] }"#).unwrap();
        assert_eq!(
            query,
            RawQuery::Rule(RawRule {
                sub_fen: None,
                material: Some(Strings::Multiple(vec![
                    "KBNKNN".to_string(),
                    "KBNPKNN".to_string()
                ])),
                imbalance: None,
                white_move: None,
                black_move: None,
                moved: None,
                captured: None,
                stm: None,
                pass: None,
            })
        );

        let processed = Query::from_raw(&query).unwrap();
        assert_eq!(
            processed,
            Query::Rule(Rule {
                sub_fen: None,
                material: Some(vec![
                    ColoredPieceCounts {
                        w_k: 1,
                        w_p: 0,
                        w_n: 1,
                        w_b: 1,
                        w_r: 0,
                        w_q: 0,
                        b_k: 1,
                        b_p: 0,
                        b_n: 2,
                        b_b: 0,
                        b_r: 0,
                        b_q: 0,
                    },
                    ColoredPieceCounts {
                        w_k: 1,
                        w_p: 1,
                        w_n: 1,
                        w_b: 1,
                        w_r: 0,
                        w_q: 0,
                        b_k: 1,
                        b_p: 0,
                        b_n: 2,
                        b_b: 0,
                        b_r: 0,
                        b_q: 0,
                    },
                ]),
                imbalance: None,
                white_move: None,
                black_move: None,
                moved: None,
                captured: None,
                stm: None,
                pass: false,
            })
        );
    }

    #[test]
    fn test_parse_rule6() {
        let query = RawQuery::parse(br#"{ "imbalance": ["PPPv", "PPv"] }"#).unwrap();
        assert_eq!(
            query,
            RawQuery::Rule(RawRule {
                sub_fen: None,
                material: None,
                imbalance: Some(Strings::Multiple(vec![
                    "PPPv".to_string(),
                    "PPv".to_string()
                ])),
                white_move: None,
                black_move: None,
                moved: None,
                captured: None,
                stm: None,
                pass: None,
            })
        );

        let processed = Query::from_raw(&query).unwrap();
        assert_eq!(
            processed,
            Query::Rule(Rule {
                sub_fen: None,
                material: None,
                imbalance: Some(vec![
                    ColoredPieceCounts {
                        w_k: 0,
                        w_p: 3,
                        w_n: 0,
                        w_b: 0,
                        w_r: 0,
                        w_q: 0,
                        b_k: 0,
                        b_p: 0,
                        b_n: 0,
                        b_b: 0,
                        b_r: 0,
                        b_q: 0,
                    },
                    ColoredPieceCounts {
                        w_k: 0,
                        w_p: 2,
                        w_n: 0,
                        w_b: 0,
                        w_r: 0,
                        w_q: 0,
                        b_k: 0,
                        b_p: 0,
                        b_n: 0,
                        b_b: 0,
                        b_r: 0,
                        b_q: 0,
                    },
                ]),
                white_move: None,
                black_move: None,
                moved: None,
                captured: None,
                stm: None,
                pass: false,
            })
        );
    }

    #[test]
    fn test_parse_rule7() {
        let query = RawQuery::parse(br#"{ "imbalance": "PPvN" }"#).unwrap();
        assert_eq!(
            query,
            RawQuery::Rule(RawRule {
                sub_fen: None,
                material: None,
                imbalance: Some(Strings::Single("PPvN".to_string())),
                white_move: None,
                black_move: None,
                moved: None,
                captured: None,
                stm: None,
                pass: None,
            })
        );

        let processed = Query::from_raw(&query).unwrap();
        assert_eq!(
            processed,
            Query::Rule(Rule {
                sub_fen: None,
                material: None,
                imbalance: Some(vec![ColoredPieceCounts {
                    w_k: 0,
                    w_p: 2,
                    w_n: 0,
                    w_b: 0,
                    w_r: 0,
                    w_q: 0,
                    b_k: 0,
                    b_p: 0,
                    b_n: 1,
                    b_b: 0,
                    b_r: 0,
                    b_q: 0,
                }]),
                white_move: None,
                black_move: None,
                moved: None,
                captured: None,
                stm: None,
                pass: false,
            })
        );
    }

    #[test]
    fn test_parse_rule8() {
        let query = RawQuery::parse(br#"{"black-move": ["O-O-O", "O-O"]}"#).unwrap();
        assert_eq!(
            query,
            RawQuery::Rule(RawRule {
                sub_fen: None,
                material: None,
                imbalance: None,
                white_move: None,
                black_move: Some(Strings::Multiple(vec![
                    "O-O-O".to_string(),
                    "O-O".to_string()
                ])),
                moved: None,
                captured: None,
                stm: None,
                pass: None,
            })
        );

        let processed = Query::from_raw(&query).unwrap();
        assert_eq!(
            processed,
            Query::Rule(Rule {
                sub_fen: None,
                material: None,
                imbalance: None,
                white_move: None,
                black_move: Some(vec![
                    SanSubset::Castle(shakmaty::CastlingSide::QueenSide),
                    SanSubset::Castle(shakmaty::CastlingSide::KingSide)
                ]),
                moved: None,
                captured: None,
                stm: None,
                pass: false,
            })
        );
    }

    #[test]
    fn test_parse_rule9() {
        let query = RawQuery::parse(br#"{"stm": "black", "captured": "QR" }"#).unwrap();
        assert_eq!(
            query,
            RawQuery::Rule(RawRule {
                sub_fen: None,
                material: None,
                imbalance: None,
                white_move: None,
                black_move: None,
                moved: None,
                captured: Some("QR".to_string()),
                stm: Some("black".to_string()),
                pass: None,
            })
        );

        let processed = Query::from_raw(&query).unwrap();
        assert_eq!(
            processed,
            Query::Rule(Rule {
                sub_fen: None,
                material: None,
                imbalance: None,
                white_move: None,
                black_move: None,
                moved: None,
                captured: Some(PieceFlags {
                    k: false,
                    p: false,
                    n: false,
                    b: false,
                    r: true,
                    q: true,
                }),
                stm: Some(false),
                pass: false,
            })
        );
    }

    #[test]
    fn test_parse_rule10() {
        let query = RawQuery::parse(br#"{"pass": "" }"#).unwrap();
        assert_eq!(
            query,
            RawQuery::Rule(RawRule {
                sub_fen: None,
                material: None,
                imbalance: None,
                white_move: None,
                black_move: None,
                moved: None,
                captured: None,
                stm: None,
                pass: Some("".to_string()),
            })
        );

        let processed = Query::from_raw(&query).unwrap();
        assert_eq!(
            processed,
            Query::Rule(Rule {
                sub_fen: None,
                material: None,
                imbalance: None,
                white_move: None,
                black_move: None,
                moved: None,
                captured: None,
                stm: None,
                pass: true,
            })
        );
    }

    #[test]
    fn test_parse_sequence1() {
        let query = RawQuery::parse(
            br#"{ "sequence": [ { "sub-fen": "r1bqkb1r/pppp1ppp/2n2n2/1B2p3/4P3/2N2N2/PPPP1PPP/R1BQK2R" },
                { "sub-fen": "8/8/8/8/2B5/8/8/8" },
                { "sub-fen": "8/8/8/8/8/5B2/8/8" } ] }"#).unwrap();
        assert_eq!(
            query,
            RawQuery::Sequence(RawSequence {
                sequence: vec![
                    RawSequenceElement::Rule(RawRule {
                        sub_fen: Some(Strings::Single(
                            "r1bqkb1r/pppp1ppp/2n2n2/1B2p3/4P3/2N2N2/PPPP1PPP/R1BQK2R".to_string()
                        )),
                        material: None,
                        imbalance: None,
                        white_move: None,
                        black_move: None,
                        moved: None,
                        captured: None,
                        stm: None,
                        pass: None,
                    }),
                    RawSequenceElement::Rule(RawRule {
                        sub_fen: Some(Strings::Single("8/8/8/8/2B5/8/8/8".to_string())),
                        material: None,
                        imbalance: None,
                        white_move: None,
                        black_move: None,
                        moved: None,
                        captured: None,
                        stm: None,
                        pass: None,
                    }),
                    RawSequenceElement::Rule(RawRule {
                        sub_fen: Some(Strings::Single("8/8/8/8/8/5B2/8/8".to_string())),
                        material: None,
                        imbalance: None,
                        white_move: None,
                        black_move: None,
                        moved: None,
                        captured: None,
                        stm: None,
                        pass: None,
                    }),
                ]
            })
        );
    }

    #[test]
    fn test_parse_streak1() {
        let query = RawQuery::parse(
            br#"{ "streak": [ { "imbalance": "vP" }, { "imbalance": "vP" }, { "imbalance": "vP" } ] }"#
        ).unwrap();
        assert_eq!(
            query,
            RawQuery::Streak(RawStreak {
                streak: vec![
                    RawRule {
                        sub_fen: None,
                        material: None,
                        imbalance: Some(Strings::Single("vP".to_string())),
                        white_move: None,
                        black_move: None,
                        moved: None,
                        captured: None,
                        stm: None,
                        pass: None,
                    },
                    RawRule {
                        sub_fen: None,
                        material: None,
                        imbalance: Some(Strings::Single("vP".to_string())),
                        white_move: None,
                        black_move: None,
                        moved: None,
                        captured: None,
                        stm: None,
                        pass: None,
                    },
                    RawRule {
                        sub_fen: None,
                        material: None,
                        imbalance: Some(Strings::Single("vP".to_string())),
                        white_move: None,
                        black_move: None,
                        moved: None,
                        captured: None,
                        stm: None,
                        pass: None,
                    },
                ]
            })
        );
    }

    #[test]
    fn test_parse_streak2() {
        let query = RawQuery::parse(
            br#"{ "streak": [ { "captured": "" }, { "stm": "white", "captured": "Q" }, { "captured": "" } ] }"#
        ).unwrap();
        assert_eq!(
            query,
            RawQuery::Streak(RawStreak {
                streak: vec![
                    RawRule {
                        sub_fen: None,
                        material: None,
                        imbalance: None,
                        white_move: None,
                        black_move: None,
                        moved: None,
                        captured: Some("".to_string()),
                        stm: None,
                        pass: None,
                    },
                    RawRule {
                        sub_fen: None,
                        material: None,
                        imbalance: None,
                        white_move: None,
                        black_move: None,
                        moved: None,
                        captured: Some("Q".to_string()),
                        stm: Some("white".to_string()),
                        pass: None,
                    },
                    RawRule {
                        sub_fen: None,
                        material: None,
                        imbalance: None,
                        white_move: None,
                        black_move: None,
                        moved: None,
                        captured: Some("".to_string()),
                        stm: None,
                        pass: None,
                    },
                ]
            })
        );
    }

    #[test]
    fn test_parse_streak3() {
        let query = RawQuery::parse(
            br#"{ "streak": [ { "white-move": "e5"}, { "pass": "" }, { "white-move": "f5" } ] }"#,
        )
        .unwrap();
        assert_eq!(
            query,
            RawQuery::Streak(RawStreak {
                streak: vec![
                    RawRule {
                        sub_fen: None,
                        material: None,
                        imbalance: None,
                        white_move: Some(Strings::Single("e5".to_string())),
                        black_move: None,
                        moved: None,
                        captured: None,
                        stm: None,
                        pass: None,
                    },
                    RawRule {
                        sub_fen: None,
                        material: None,
                        imbalance: None,
                        white_move: None,
                        black_move: None,
                        moved: None,
                        captured: None,
                        stm: None,
                        pass: Some("".to_string()),
                    },
                    RawRule {
                        sub_fen: None,
                        material: None,
                        imbalance: None,
                        white_move: Some(Strings::Single("f5".to_string())),
                        black_move: None,
                        moved: None,
                        captured: None,
                        stm: None,
                        pass: None,
                    },
                ]
            })
        );
    }

    #[test]
    fn test_parse_streak_in_sequence1() {
        let query = RawQuery::parse(
            br#"{ "sequence": [ { "sub-fen": "rnbqkb1r/pp1p1ppp/4pn2/2pP4/2P5/2N5/PP2PPPP/R1BQKBNR"},
                { "streak": [ { "white-move": "e5"}, { "black-move": "dxe5"}, { "white-move": "f5"} ] },
                { "white-move": "Ne4"} ] }"#,
        ).unwrap();
        assert_eq!(
            query,
            RawQuery::Sequence(RawSequence {
                sequence: vec![
                    RawSequenceElement::Rule(RawRule {
                        sub_fen: Some(Strings::Single(
                            "rnbqkb1r/pp1p1ppp/4pn2/2pP4/2P5/2N5/PP2PPPP/R1BQKBNR".to_string()
                        )),
                        material: None,
                        imbalance: None,
                        white_move: None,
                        black_move: None,
                        moved: None,
                        captured: None,
                        stm: None,
                        pass: None,
                    }),
                    RawSequenceElement::Streak(RawStreak {
                        streak: vec![
                            RawRule {
                                sub_fen: None,
                                material: None,
                                imbalance: None,
                                white_move: Some(Strings::Single("e5".to_string())),
                                black_move: None,
                                moved: None,
                                captured: None,
                                stm: None,
                                pass: None,
                            },
                            RawRule {
                                sub_fen: None,
                                material: None,
                                imbalance: None,
                                white_move: None,
                                black_move: Some(Strings::Single("dxe5".to_string())),
                                moved: None,
                                captured: None,
                                stm: None,
                                pass: None,
                            },
                            RawRule {
                                sub_fen: None,
                                material: None,
                                imbalance: None,
                                white_move: Some(Strings::Single("f5".to_string())),
                                black_move: None,
                                moved: None,
                                captured: None,
                                stm: None,
                                pass: None,
                            },
                        ]
                    }),
                    RawSequenceElement::Rule(RawRule {
                        sub_fen: None,
                        material: None,
                        imbalance: None,
                        white_move: Some(Strings::Single("Ne4".to_string())),
                        black_move: None,
                        moved: None,
                        captured: None,
                        stm: None,
                        pass: None,
                    }),
                ]
            })
        );
    }
}
