use aix_chess_compression::{CompressionLevel, Encode, Encoder};
use duckdb::{Appender, params};
use lazy_regex::regex_captures;
use pgn_reader::{SanPlus, Skip, Visitor};
use shakmaty::san::San;
use shakmaty::{Chess, Position};
use std::ops::ControlFlow;

pub struct PgnProcessor<'a> {
    appender: Appender<'a>,
    count: u32,
    level: CompressionLevel,
}

#[derive(Default)]
pub struct Headers {
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

impl<'a> PgnProcessor<'a> {
    pub fn new(appender: Appender<'a>, level: CompressionLevel) -> PgnProcessor<'a> {
        PgnProcessor {
            appender,
            count: 0,
            level,
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
        ControlFlow::Continue(Headers::default())
    }

    fn begin_movetext(&mut self, tags: Self::Tags) -> ControlFlow<Self::Output, Self::Movetext> {
        ControlFlow::Continue(GameInProcessing::new(tags, self.level))
    }

    fn san(
        &mut self,
        movetext: &mut Self::Movetext,
        san_plus: SanPlus,
    ) -> ControlFlow<Self::Output> {
        let m = san_plus
            .san
            .to_string()
            .parse::<San>()
            .unwrap()
            .to_move(&movetext.pos)
            .unwrap();
        movetext.pos.play_unchecked(m);
        movetext.encoder.encode_move(m).unwrap();
        movetext.ply += 1;

        ControlFlow::Continue(())
    }

    fn begin_variation(
        &mut self,
        _movetext: &mut Self::Movetext,
    ) -> ControlFlow<Self::Output, Skip> {
        ControlFlow::Continue(Skip(true)) // stay in the mainline
    }

    fn end_game(&mut self, movetext: Self::Movetext) -> Self::Output {
        let clocks_w = movetext.finalize_clocks(true);
        let clocks_b = movetext.finalize_clocks(false);
        let evals = movetext.finalize_evals();
        let moves = movetext.encoder.finish();
        let bytes = moves.into_bytes();

        self.appender
            .append_row(params![
                movetext.headers.lichess_id,
                movetext.headers.tournament,
                bytes,
                clocks_w,
                clocks_b,
                evals,
                movetext.ply,
                movetext.headers.white,
                movetext.headers.black,
                movetext.headers.white_rating,
                movetext.headers.black_rating,
                movetext.headers.time_control.map(|c| c.0),
                movetext.headers.time_control.map(|c| c.1),
                movetext.headers.result,
                movetext.headers.termination,
                movetext.headers.white_rating_diff,
                movetext.headers.black_rating_diff,
                movetext.headers.eco,
                movetext.headers.opening,
                movetext.headers.white_title,
                movetext.headers.black_title,
                movetext.headers.utc_date.and_then(|date| {
                    movetext
                        .headers
                        .utc_time
                        .map(|time| format!("{} {}", date.replace(".", "-"), time))
                }),
            ])
            .unwrap();

        self.count += 1;
        if self.count % 10000 == 0 {
            println!("{} done", self.count);
        }
    }

    fn tag(
        &mut self,
        tags: &mut Self::Tags,
        key: &[u8],
        value: pgn_reader::RawTag<'_>,
    ) -> ControlFlow<Self::Output> {
        match key {
            b"White" => tags.white = Some(value.decode_utf8_lossy().into_owned()),
            b"Black" => tags.black = Some(value.decode_utf8_lossy().into_owned()),
            b"WhiteElo" => tags.white_rating = value.decode_utf8_lossy().parse().ok(),
            b"BlackElo" => tags.black_rating = value.decode_utf8_lossy().parse().ok(),
            b"Result" => tags.result = Some(value.decode_utf8_lossy().into_owned()),
            b"Termination" => tags.termination = Some(value.decode_utf8_lossy().into_owned()),
            b"TimeControl" => tags.time_control = parse_time_control(&value.decode_utf8_lossy()),
            b"Site" => {
                tags.lichess_id = value
                    .decode_utf8_lossy()
                    .split("/")
                    .last()
                    .map(|s| s.to_owned())
            }
            b"ECO" => tags.eco = Some(value.decode_utf8_lossy().into_owned()),
            b"Opening" => tags.opening = Some(value.decode_utf8_lossy().into_owned()),
            b"WhiteTitle" => tags.white_title = Some(value.decode_utf8_lossy().into_owned()),
            b"BlackTitle" => tags.black_title = Some(value.decode_utf8_lossy().into_owned()),
            b"WhiteRatingDiff" => tags.white_rating_diff = value.decode_utf8_lossy().parse().ok(),
            b"BlackRatingDiff" => tags.black_rating_diff = value.decode_utf8_lossy().parse().ok(),
            b"Event" => tags.tournament = extract_tournament_from_event(&value.decode_utf8_lossy()),
            b"UTCDate" => tags.utc_date = Some(value.decode_utf8_lossy().into_owned()),
            b"UTCTime" => tags.utc_time = Some(value.decode_utf8_lossy().into_owned()),
            _ => {}
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
