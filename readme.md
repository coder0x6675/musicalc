# Musicalc

> A collection of CLI tools for music production, written in rust.

## Description

This project contains a small collection of command-line tools to aid in music production. It includes utilites which can help with small tasks such as counting BPM, finding scales based on a set of given notes, or convert a note to its fundamental frequency.

## Installation

The project is written in rust and is compatible with all major operating systems including windows, mac and linux. To be able to run the programs you have to have the rust compiler (rustc) installed, and preferably the entire toolchain (mainly for cargo). To compile the programs, simply run:

```
cargo build --release
```

... and the resulting binaries will appear in the `target/release` directory.

You can run the programs from the terminal directly, or by using cargo:

```
cargo run --release --bin *name-of-the-program*
```

## Usage

### bpm-count

Use this tool to find the tempo of a song, measured in *beats per minute* (BPM). If the tempo varies throughout the song, the resulting BPM will vary with it. It's surprisingly accurate compared to many of the BPM counters you will find online.

To "tune" the counter to your specific needs, you can edit the QUEUE\_SIZE variable at the top of the source file. Setting this to a lower value will lower the time it takes for the counter to converge to the latest tempo, but will also be less precise. Increasing it will have the opposite effect.

Usage:

1. Run the program: `bpm-count`
1. To start counting, press **ENTER**.
1. Tap along to the beat of the music.
1. You can continue counting for as long as required.

Initially, the counter will vary wildly and be very unstable. After the twelfth count, the program will print `STABLE!`, which means that the counter has reached its peak stability.

### bpm-ms

Use this tool to convert a tempo/BPM to milliseconds. This is highly useful for setting exact delay parameters, compressor attack/release or other time-based settings to fit the tempo of a song.

Usage: bpm-ms *BPM*

Example:

```
$ bpm-ms 128

 NOTE      WHOLE    TRIPLET     DOTTED
1/  1    1875.00    1250.00    2343.75
1/  2     937.50     625.00    1171.88
1/  4     468.75     312.50     585.94
1/  8     234.38     156.25     292.97
1/ 16     117.19      78.12     146.48
1/ 32      58.59      39.06      73.24
1/ 64      29.30      19.53      36.62
1/128      14.65       9.77      18.31
1/256       7.32       4.88       9.16
```

### fifth

This tool uses the wheel of fifths to find the semi-octave of a note. This can be used to find out which scales are similar to eachother. You can for example temporarily switch to the closest minor scale if your song is in major, in the middle of the song, to immediately shift the mood to a darker, more melancholic feeling while still preserving tonal harmony.

Usage: fifth *note*

Example:

```
$ fifth A#

Root note: A#
- Lower 5th: D#
- Upper 5th: F
- To minor: G
- To major: C#
```

### metronome

This tool simulates a basic metronome. Weirdly enough, it doesn't produce any audio. It is instead meant to pipe it's output into other programs so that they can perform their work in sync with the music.

Usage: metronome *BPM*

Example:

```
$ metronome 128 | nl

1  TICK
2  TOCK
3  TOCK
4  TOCK
5  TICK
6  TOCK
7  TOCK
8  TOCK
```

### note-hz

Use this tool to find the fundamental frequency of a note. This can be used when mixing to figure out the fundamental region of an instrument, or to synchronize LFOs to a specific note. Note that this program requires a number representing the octave after the given note.

Usage: note-hz *note*

Example:

```
$ note-hz A4

A4 = 440.00 hz
```

### scale-finder

The newest addition, and perhaps the most useful tool in the collection. The scale-finder takes in a set of notes and figures out which scale those notes could belong to. This is useful if you have a melody and wonder which scale it is in, or if you are improvising and wondering which scales you can play along with.

The following scales are implemented:

- Major
- Minor (natural)
- Minor (harmonic)
- Minor (melodic)
- Phrygian
- Blues
- Whole tone
- Pentatonic

For anyone wondering, the 'Minor (natural)' is the "normal" minor scale. In addition, the chromatic scale is implemented and can be enabled by uncommenting the line containing 'chromatic' in the source file. Keep in mind that since this scale contains all the notes, it's going to match every set of notes which is why it's disabled by default.

Usage: scale-finder *note1* *note2* *note3* ...

Example:

```
$ scale-finder C C# F G

G# Major
F  Minor (natural)
F  Minor (harmonic)
A# Minor (melodic)
C  Phrygian
G  Blues
```

