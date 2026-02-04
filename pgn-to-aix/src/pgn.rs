use aix_chess_compression::{CompressionLevel, Encode, Encoder};
use duckdb::{Appender, params};
use lazy_regex::regex_captures;
use pgn_reader::{SanPlus, Skip, Visitor};
use shakmaty::san::San;
use shakmaty::{Chess, Position};
use std::collections::HashMap;
use std::ops::ControlFlow;

pub struct PgnProcessor<'a> {
    appender: Appender<'a>,
    count: u32,
    level: CompressionLevel,
    header_list: Option<Vec<String>>,
    continue_on_invalid_move: bool,
}

#[derive(Debug)]
pub enum Headers {
    Lichess(LichessHeaders),
    Custom(CustomHeaders),
}

#[derive(Default, Debug)]
pub struct LichessHeaders {
    white: Option<String>,
    black: Option<String>,
    white_rating: Option<i16>,
    black_rating: Option<i16>,
    result: Option<String>,
    termination: Option<String>,
    time_control: Option<(u16, u8)>,
    white_rating_diff: Option<i16>,
    black_rating_diff: Option<i16>,
    eco: Option<String>,
    opening: Option<String>,
    white_title: Option<String>,
    black_title: Option<String>,
    lichess_id: Option<String>,
    tournament: Option<String>,
    utc_date: Option<String>,
    utc_time: Option<String>,
}

pub type CustomHeaders = HashMap<String, String>;

impl<'a> PgnProcessor<'a> {
    pub fn new(
        appender: Appender<'a>,
        level: CompressionLevel,
        header_list: Option<Vec<String>>,
        continue_on_invalid_move: bool,
    ) -> PgnProcessor<'a> {
        PgnProcessor {
            appender,
            count: 0,
            level,
            header_list,
            continue_on_invalid_move,
        }
    }

    pub fn flush(&mut self) {
        self.appender.flush().unwrap();
    }
}

pub struct GameInProcessing<'a> {
    headers: Headers,
    encoder: Encoder<'a>,
    evals: Vec<(u16, i16)>,
    clocks_white: Vec<(u16, u16)>,
    clocks_black: Vec<(u16, u16)>,
    pos: Chess,
    ply: u16,
}

impl GameInProcessing<'_> {
    fn new(headers: Headers, level: CompressionLevel) -> Self {
        GameInProcessing {
            headers,
            encoder: Encoder::new(level),
            evals: vec![],
            clocks_white: vec![],
            clocks_black: vec![],
            pos: Chess::new(),
            ply: 0,
        }
    }

    fn finalize_evals(&self) -> Option<String> {
        if self.evals.is_empty() {
            return None;
        }

        let mut index = 1;
        let mut evals = vec![];
        for &(ply, eval) in &self.evals {
            if index != ply {
                let n = ply - index;
                let last = evals.last().cloned().unwrap_or(0);
                evals.append(&mut vec![last; n.into()]);
            }

            evals.push(eval);
            index = ply + 1;
        }

        Some(format!("{:?}", evals))
    }

    fn finalize_clocks(&self, white: bool) -> Option<String> {
        let clocks = if white {
            &self.clocks_white
        } else {
            &self.clocks_black
        };

        if clocks.is_empty() {
            return None;
        }

        let mut index = 1;
        let mut clocks_out = vec![];
        for &(ply, clock) in clocks {
            if index != ply {
                let n = (ply - index) / 2;
                let last = clocks_out.last().cloned().unwrap_or(0);
                clocks_out.append(&mut vec![last; n.into()]);
            }

            clocks_out.push(clock);
            index = ply + 2;
        }

        Some(format!("{:?}", clocks_out))
    }
}

impl<'a> Visitor for PgnProcessor<'a> {
    type Tags = Headers;
    type Movetext = GameInProcessing<'a>;
    type Output = ();

    fn begin_tags(&mut self) -> ControlFlow<Self::Output, Self::Tags> {
        ControlFlow::Continue(match &self.header_list {
            Some(_) => Headers::Custom(CustomHeaders::new()),
            None => Headers::Lichess(LichessHeaders::default()),
        })
    }

    fn begin_movetext(&mut self, tags: Self::Tags) -> ControlFlow<Self::Output, Self::Movetext> {
        ControlFlow::Continue(GameInProcessing::new(tags, self.level))
    }

    fn san(
        &mut self,
        movetext: &mut Self::Movetext,
        san_plus: SanPlus,
    ) -> ControlFlow<Self::Output> {
        match san_plus
            .san
            .to_string()
            .parse::<San>()
            .unwrap()
            .to_move(&movetext.pos)
        {
            Ok(m) => {
                movetext.pos.play_unchecked(m);
                movetext.encoder.encode_move(m).unwrap();
                movetext.ply += 1;
            }
            Err(_) => {
                let error_msg = format!(
                    "Invalid move '{}' at ply {} in game with headers {:?}",
                    san_plus.san.to_string(),
                    movetext.ply + 1,
                    movetext.headers
                );
                if self.continue_on_invalid_move {
                    eprintln!("{error_msg}");

                    let mut swap = GameInProcessing {
                        headers: Headers::Custom(CustomHeaders::new()),
                        encoder: Encoder::new(CompressionLevel::Low),
                        evals: vec![],
                        clocks_white: vec![],
                        clocks_black: vec![],
                        pos: Chess::new(),
                        ply: 0,
                    };

                    std::mem::swap(&mut swap, movetext);
                    self.end_game_inner(swap);

                    return ControlFlow::Break(());
                } else {
                    panic!(
                        "{error_msg}\nCheck the PGN file or if you want to use --continue-on-invalid-move."
                    );
                }
            }
        }

        ControlFlow::Continue(())
    }

    fn begin_variation(
        &mut self,
        _movetext: &mut Self::Movetext,
    ) -> ControlFlow<Self::Output, Skip> {
        ControlFlow::Continue(Skip(true)) // stay in the mainline
    }

    fn end_game(&mut self, movetext: Self::Movetext) -> Self::Output {
        self.end_game_inner(movetext)
    }

    fn tag(
        &mut self,
        tags: &mut Self::Tags,
        key: &[u8],
        value: pgn_reader::RawTag<'_>,
    ) -> ControlFlow<Self::Output> {
        match tags {
            Headers::Custom(custom_headers) => {
                let header_list = self.header_list.as_ref().unwrap();
                let key_str = String::from_utf8_lossy(key).into_owned().to_lowercase();
                if header_list.contains(&key_str) {
                    custom_headers.insert(key_str, value.decode_utf8_lossy().into_owned());
                }
            }
            Headers::Lichess(lichess_tags) => match key {
                b"White" => lichess_tags.white = Some(value.decode_utf8_lossy().into_owned()),
                b"Black" => lichess_tags.black = Some(value.decode_utf8_lossy().into_owned()),
                b"WhiteElo" => lichess_tags.white_rating = value.decode_utf8_lossy().parse().ok(),
                b"BlackElo" => lichess_tags.black_rating = value.decode_utf8_lossy().parse().ok(),
                b"Result" => lichess_tags.result = Some(value.decode_utf8_lossy().into_owned()),
                b"Termination" => {
                    lichess_tags.termination = Some(value.decode_utf8_lossy().into_owned())
                }
                b"TimeControl" => {
                    lichess_tags.time_control = parse_time_control(&value.decode_utf8_lossy())
                }
                b"Site" => {
                    lichess_tags.lichess_id = value
                        .decode_utf8_lossy()
                        .split("/")
                        .last()
                        .map(|s| s.to_owned())
                }
                b"ECO" => lichess_tags.eco = Some(value.decode_utf8_lossy().into_owned()),
                b"Opening" => lichess_tags.opening = Some(value.decode_utf8_lossy().into_owned()),
                b"WhiteTitle" => {
                    lichess_tags.white_title = Some(value.decode_utf8_lossy().into_owned())
                }
                b"BlackTitle" => {
                    lichess_tags.black_title = Some(value.decode_utf8_lossy().into_owned())
                }
                b"WhiteRatingDiff" => {
                    lichess_tags.white_rating_diff = value.decode_utf8_lossy().parse().ok()
                }
                b"BlackRatingDiff" => {
                    lichess_tags.black_rating_diff = value.decode_utf8_lossy().parse().ok()
                }
                b"Event" => {
                    lichess_tags.tournament =
                        extract_tournament_from_event(&value.decode_utf8_lossy())
                }
                b"UTCDate" => lichess_tags.utc_date = Some(value.decode_utf8_lossy().into_owned()),
                b"UTCTime" => lichess_tags.utc_time = Some(value.decode_utf8_lossy().into_owned()),
                _ => {}
            },
        }

        ControlFlow::Continue(())
    }

    fn comment(
        &mut self,
        movetext: &mut Self::Movetext,
        comment: pgn_reader::RawComment<'_>,
    ) -> ControlFlow<Self::Output> {
        let cmt = std::str::from_utf8(comment.as_bytes()).unwrap_or("");

        if let Some(eval_cp) = extract_eval_cp_from_comment(cmt) {
            movetext.evals.push((movetext.ply, eval_cp));
        }

        if let Some(clock_seconds) = extract_clock_seconds_from_comment(cmt) {
            if movetext.ply % 2 == 1 {
                movetext.clocks_white.push((movetext.ply, clock_seconds));
            } else {
                movetext.clocks_black.push((movetext.ply, clock_seconds));
            }
        }

        ControlFlow::Continue(())
    }
}

impl<'a> PgnProcessor<'a> {
    fn end_game_inner(
        &mut self,
        movetext: <Self as Visitor>::Movetext,
    ) -> <Self as Visitor>::Output {
        let clocks_w = movetext.finalize_clocks(true);
        let clocks_b = movetext.finalize_clocks(false);
        let evals = movetext.finalize_evals();
        let moves = movetext.encoder.finish();
        let bytes = moves.into_bytes();

        match movetext.headers {
            Headers::Lichess(headers) => self
                .appender
                .append_row(params![
                    headers.lichess_id,
                    headers.tournament,
                    bytes,
                    clocks_w,
                    clocks_b,
                    evals,
                    movetext.ply,
                    headers.white,
                    headers.black,
                    headers.white_rating,
                    headers.black_rating,
                    headers.time_control.map(|c| c.0),
                    headers.time_control.map(|c| c.1),
                    headers.result,
                    headers.termination,
                    headers.white_rating_diff,
                    headers.black_rating_diff,
                    headers.eco,
                    headers.opening,
                    headers.white_title,
                    headers.black_title,
                    headers.utc_date.and_then(|date| {
                        headers
                            .utc_time
                            .map(|time| format!("{} {}", date.replace(".", "-"), time))
                    }),
                ])
                .unwrap(),
            Headers::Custom(headers) => {
                let mut params_vec: Vec<Box<dyn duckdb::ToSql>> = vec![];
                for header in self
                    .header_list
                    .as_ref()
                    .expect("header_list cannot be None for Custom headers")
                {
                    params_vec.push(Box::new(headers.get(header)));
                }

                params_vec.push(Box::new(bytes));
                params_vec.push(Box::new(clocks_w));
                params_vec.push(Box::new(clocks_b));
                params_vec.push(Box::new(evals));
                params_vec.push(Box::new(movetext.ply));

                self.appender
                    .append_row(duckdb::appender_params_from_iter(params_vec))
                    .unwrap();
            }
        }

        self.count += 1;
        if self.count % 10000 == 0 {
            println!("{} done", self.count);
        }
    }
}

fn extract_eval_cp_from_comment(comment: &str) -> Option<i16> {
    regex_captures!(r"\[%eval (-?\d+\.\d+|#-?\d+)\]", comment)
        .and_then(|(_whole, eval)| eval_capture_to_cp(eval))
}

fn eval_capture_to_cp(eval: &str) -> Option<i16> {
    if &eval[0..1] == "#" {
        eval[1..].parse().ok().map(|mate: i16| {
            if mate > 0 {
                i16::MAX - mate + 1
            } else {
                i16::MIN - mate - 1
            }
        })
    } else {
        eval.parse().ok().map(|p: f64| (p * 100.0).round() as i16)
    }
}

fn extract_clock_seconds_from_comment(comment: &str) -> Option<u16> {
    regex_captures!(r"\[%clk ([0-9]+):([0-9]{2}):([0-9]{2})\]", comment).map(|(_whole, h, m, s)| {
        h.parse::<u16>().unwrap() * 3600
            + m.parse::<u16>().unwrap() * 60
            + s.parse::<u16>().unwrap()
    })
}

fn parse_time_control(s: &str) -> Option<(u16, u8)> {
    if s == "-" {
        return None;
    }

    let parts: Vec<&str> = s.split("+").collect();
    Some((parts[0].parse().unwrap(), parts[1].parse().unwrap()))
}

fn extract_tournament_from_event(s: &str) -> Option<String> {
    regex_captures!(r"lichess\.org/tournament/(\w+)", s)
        .map(|(_whole, tournament)| tournament.to_owned())
}
