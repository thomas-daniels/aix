use aix_chess_compression::{Decoder, EncodedGame};
use criterion::{Criterion, criterion_group, criterion_main};
use std::hint::black_box;

static ENC_HIGH: &[u8] = b"\x7F\x08\x08\x88\xBC\xDDg\x9A\xA1\xAD\x1F\x8C\x11%\x85\x92\xFD\x13G\x195\xA9\x10\x04\xC1\xB3p\x96f\xFAu\xA59\x86\xC0\xB4l\xC8tL\x9B\x931\x95Q\xFD\xC4\xBB\xA7~\x5C\xA6GJ\xAF\xB6\xD2f\xABu\x97\xD1\x90\xE1Z\x19\x0FS_k\xD7z\xDF\x0DI\xC8pLeJ\x82\x90\x9F\xD2wo\xF7zF=%\xB1+W\xE7\xD2ej\xAD\x9Ari\x8D\xF2\x97\xB8";
static ENC_MEDIUM: &[u8] = b"<m\x22\x12\x22\xD7qAGG4\xFCiR\x98]\x96\xD3U\x15\x99\x0B4.MC-\xAF6\xAC\x16l1$@6\x00\xA5\x0AHAd\xEE8(es\xF5\xEEb%\xAA\xC0\x0E\x91H\xE8r\x80d!KF\x83@\x5C\x06.h\x87;\x22\xD1`\xA5\xC7\xDCB?\xAE\xF7dC\xC7\x16I\xE9\x89\x9AZ\x0Fb\x05\xA6X,\xCFq\x0C5\x89<]\x13\xE4\x81LBOfBx\xB0&\xC5\x934\x01<\x8D\xC9\x0B\xBC\xC4]\xC0\x95\xEB\x95^\xE9\x06\x17\xC1i\x87a5\xA5\xBE\xD5\x94z\x06\xDCN\x190\xA7&\xF5\x1C\xBA\xA56Z\xCBWj\x01n\xA9MZdK\xAC\x04[X\x02n";
static ENC_LOW: &[u8] = b"\x0C\x1C4$\x06\x153+\x0A\x122*\x0B\x1B:\x1E\x9B$\x9E\x15\x83\x15\xAB$\x05\x1A>-\x04\x0793\x02&=4\x05\x03<?\xA6-\xB4-\x01\x0B0 \x0B\x111!\x11\x2280\x1A\x0C;4\xA23\xB03\x833\xB43\x00\x0334\x12\x1A!\x19\x15\x14*\x22\x03#=:\x09\x117/\x0F\x17-&\x14\x165-\x17\x1F&\x0B\x0C\x1E:;\xA3;\xB4;\x1E,>?\x16.\x0B\x1D\x0E\x16\x1D\x0B,%;>.\x1E\x0B\x12%,><,#<4\x1F\x27\x12\x0B\x1E\x0C\x0B&\x16\x1E&\x1D\x0D\x154<\x0C\x13<4#*?7\x133\xB43\xAA36.\xA7.\xB7.3%.&\x06\x0E/\x27\x9E\x27\xA6\x27\x0E\x17\x27&%\x1E\x1D\x14\x1E%&\x1D%\x1E\x14\x0D\x17\x0E\x0D\x04\x0E\x05\x04\x1F\x05\x0C\x1D\x16\x0C\x05\x16\x0F\x05\x0C\x0F\x06\x1E%\x1F&%\x1E&\x1D\x1E%\x06\x0E%\x1E\x0E\x16\x1E%\x1D\x02%\x1E\x16\x1D\x1E%\x02\x09%\x1E\x09\x1B\x1E%\x1D&%\x1E\x1B\x09\x1E%\x09\x02%\x1E&/\x1E%/6\x0C\x0D6=\x0D\x16=4\x16\x1F\x02&\x1F\x274+\x27.+*.5*)5, \x18,#\x98\x11\x88\x11&\x14%\x1E)2\x1E,2;,%;4%,4=,%=6#,\x14&,#&\x14#,\x14&%\x1E6.\x1E%.\x27%\x1E\x27\x1F,#&\x14#,\x1F&,#&\x1F#+\x1F\x16+,\x14&,#&\x14#,\x16\x0D\xAC-\x14\x1D-,\x0D\x14,+\x14\x13\xAB\x22\x13\x12\x22!\x00";

static UCI: &str = "e2e4 e7e5 g1f3 d7d6 c2c3 c7c6 d2d4 c8g4 d4e5 g4f3 d1f3 d6e5 f1c4 g8f6 e1g1 b8d7 c1g5 f8e7 f1d1 e8g8 g5f6 e7f6 b1d2 a7a5 d2b3 b7b5 b3c5 a8a7 c4e2 d8e7 c5d7 a7d7 d1d7 e7d7 a1d1 d7e7 c3c4 b5b4 f3e3 c6c5 d1d5 f8c8 b2b3 h7h6 h2h3 f6g5 e3g3 f7f6 h3h4 g5d2 e2g4 c8d8 d5d8 e7d8 g4e6 g8h8 g3g6 d2f4 g2g3 f4d2 e6f5 d8g8 g6g4 d2c3 f5e6 g8e8 e6d5 e8e7 h4h5 c3d2 g4e2 d2g5 g3g4 g5f4 f2f3 e7e8 e2d3 e8e7 d5c6 h8h7 d3d7 e7d7 c6d7 g7g6 h5g6 h7g6 d7f5 g6g5 g1g2 h6h5 g4h5 g5h5 g2h3 h5g5 f5g4 f4e3 g4f5 g5f4 f5g4 e3f2 h3g2 f2e1 g2f1 e1h4 f1e2 f4g3 e2f1 g3h2 f1e2 h2g1 g4f5 h4g5 f5g4 g5f4 g4f5 g1g2 f5g4 g2g3 g4f5 f4c1 f5g4 g3f4 g4f5 c1b2 f5g4 b2d4 g4f5 f4g5 f5g4 d4b2 g4f5 b2c1 f5g4 g5h6 g4f5 h6g7 e2f2 g7f8 f2g3 f8e7 g3h4 c1g5 h4h5 e7d6 h5g6 d6c6 g6f7 c6b6 f7e6 a5a4 e6d5 a4b3 a2b3 g5e3 f5g4 b6c7 g4e6 c7d8 e6f5 d8e7 f5e6 e7f8 e6f5 f8g7 d5e6 e3g5 e6d5 g5e3 d5e6 e3g5 f5g4 g7g6 g4f5 g6h5 f5g4 h5h4 e6d5 g5e3 d5e6 h4g5 e6d5 g5h4 d5d6 h4g3 d6e6 e3g5 e6d5 g5e3 d5e6 g3f2 e6f6 e3f4 f6e6 f2e3 e6d6 e3d3 d6c5 d3c3 c5b5";

fn decode_check_uci(bytes: &[u8], expected_uci: &str) {
    let encoded_game = EncodedGame::from_bytes(bytes).unwrap();
    let decoder = Decoder::new(&encoded_game);
    let uci = decoder.into_uci_string().unwrap();
    assert_eq!(uci, expected_uci);
}

fn bench_decode_low(c: &mut Criterion) {
    let bytes = black_box(ENC_LOW);

    c.bench_function("decode_low", |b| {
        b.iter(|| {
            decode_check_uci(bytes, UCI);
        })
    });
}

fn bench_decode_medium(c: &mut Criterion) {
    let bytes = black_box(ENC_MEDIUM);

    c.bench_function("decode_medium", |b| {
        b.iter(|| {
            decode_check_uci(bytes, UCI);
        })
    });
}

fn bench_decode_high(c: &mut Criterion) {
    let bytes = black_box(ENC_HIGH);

    c.bench_function("decode_high", |b| {
        b.iter(|| {
            decode_check_uci(bytes, UCI);
        })
    });
}

criterion_group!(
    benches,
    bench_decode_low,
    bench_decode_medium,
    bench_decode_high,
);

criterion_main!(benches);
