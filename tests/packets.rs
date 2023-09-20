use tasd::spec::packets::{Attribution, Category, ConsoleRegion, ConsoleType, Encode, GameTitle, Packet, RomName};
use tasd::spec::writer::Writer;

/// Small wrapper around [`Writer`] for creating a packet using a key and some data.
fn packet<D: AsRef<[u8]>>(key: &[u8], data: D) -> Vec<u8> {
    let mut w = Writer::new();
    w.write_slice(data.as_ref());
    
    w.into_packet(key, key.len() as u8)
}

macro_rules! assert_packet {
    ($packet:expr, $key:expr, $data:expr) => {
        assert_eq!(
            Packet::from($packet).encode($key.len() as u8),
            packet(&$key, $data),
        );
    };
}

#[test]
fn console_type() {
    assert_packet!(ConsoleType { kind: 0x01, custom: None }, [0x00, 0x01], [0x01]);
    assert_packet!(ConsoleType { kind: 0x05, custom: None }, [0x00, 0x01], [0x05]);
    assert_packet!(ConsoleType { kind: 0xFF, custom: None }, [0x00, 0x01], [0xFF]);
    assert_packet!(ConsoleType { kind: 0xFF, custom: Some("something".into()) }, [0x00, 0x01], [&[0xFF], "something".as_bytes()].concat());
}

#[test]
fn console_region() {
    assert_packet!(ConsoleRegion { region: 0x01 }, [0x00, 0x02], [0x01]);
    assert_packet!(ConsoleRegion { region: 0x02 }, [0x00, 0x02], [0x02]);
    assert_packet!(ConsoleRegion { region: 0xFF }, [0x00, 0x02], [0xFF]);
}

#[test]
fn game_title() {
    assert_packet!(GameTitle { title: "This is the title!".into() }, [0x00, 0x03], "This is the title!".as_bytes());
    assert_packet!(GameTitle { title: "12345? That's amazing, I've got the same password on my luggage!".into() }, [0x00, 0x03], "12345? That's amazing, I've got the same password on my luggage!".as_bytes());
}

#[test]
fn rom_name() {
    assert_packet!(RomName { name: "Super Mario Bros. Super Hack World With Ultra Mods".into() }, [0x00, 0x04], "Super Mario Bros. Super Hack World With Ultra Mods".as_bytes());
}

#[test]
fn attribution() {
    assert_packet!(Attribution { kind: 0x01, name: "Arthur".into() }, [0x00, 0x05], [&[0x01], "Arthur".as_bytes()].concat());
    assert_packet!(Attribution { kind: 0x02, name: "a replay device".into() }, [0x00, 0x05], [&[0x02], "a replay device".as_bytes()].concat());
    assert_packet!(Attribution { kind: 0x03, name: "Arthur".into() }, [0x00, 0x05], [&[0x03], "Arthur".as_bytes()].concat());
    assert_packet!(Attribution { kind: 0x04, name: "Arthur".into() }, [0x00, 0x05], [&[0x04], "Arthur".as_bytes()].concat());
    assert_packet!(Attribution { kind: 0xFF, name: "Arthur".into() }, [0x00, 0x05], [&[0xFF], "Arthur".as_bytes()].concat());
}

#[test]
fn category() {
    assert_packet!(Category { category: "any%, no jumps, invisible hud, 2 players".into() }, [0x00, 0x06], "any%, no jumps, invisible hud, 2 players".as_bytes());
}

#[test]
fn emulator_name() {
    
}

#[test]
fn emulator_version() {
    
}

#[test]
fn emulator_core() {
    
}

#[test]
fn tas_last_modified() {
    
}

#[test]
fn dump_created() {
    
}

#[test]
fn dump_last_modified() {
    
}

#[test]
fn total_frames() {
    
}

#[test]
fn rerecords() {
    
}

#[test]
fn source_link() {
    
}

#[test]
fn blank_frames() {
    
}

#[test]
fn verified() {
    
}

#[test]
fn memory_init() {
    
}

#[test]
fn game_identifier() {
    
}

#[test]
fn movie_license() {
    
}

#[test]
fn movie_file() {
    
}

#[test]
fn port_controller() {
    
}

#[test]
fn nes_latch_filter() {
    
}

#[test]
fn nes_clock_filter() {
    
}

#[test]
fn nes_overread() {
    
}

#[test]
fn nes_game_genie_code() {
    
}

#[test]
fn snes_clock_filter() {
    
}

#[test]
fn snes_overread() {
    
}

#[test]
fn snes_game_genie_code() {
    
}

#[test]
fn snes_latch_train() {
    
}

#[test]
fn genesis_game_genie_code() {
    
}

#[test]
fn input_chunk() {
    
}

#[test]
fn input_moment() {
    
}

#[test]
fn transition() {
    
}

#[test]
fn lag_frame_chunk() {
    
}

#[test]
fn movie_transition() {
    
}

#[test]
fn comment() {
    
}

#[test]
fn experimental() {
    
}

#[test]
fn unspecified() {
    
}

#[test]
fn unsupported() {
    
}
