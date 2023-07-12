# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/)

## [[0.2.1] - 2923-07-12]
- Change: `TasdFile::new()` now includes the DUMP_CREATED packet in the returned `TasdFile` instance
- Change: PacketKind's Display implementation now returns the name in SNAKE_CASE
- Added: `From<T> for Packet` implementations for all packet structs

## [0.2.0] - 2023-07-06
- Changed: Rewrote packet structures
- Added: SNES_LATCH_TRAIN packet
