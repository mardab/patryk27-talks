use crate::*;

const BPM: u16 = 120;
const Q: u16 = bpm_to_quarter_ms(BPM);

track!([
    note("D4", Q, 100),
    note("E4", Q, 100),
    note("F#4", Q / 2, 10),
    note("G4", Q, 150),
    note("A4", 2 * Q, 100),
    note("C5", Q, 5),
    note("C5", Q, 5),
    note("B4", 2 * Q, 150),
    note("G4", Q, 1),
    note("A4", Q, 75),
    note("G4", Q, 1),
    note("A4", Q, 75),
    note("A#4", 2 * Q, 100),
    note("A4", Q, 1),
    note("B4", Q, 40),
    note("C#5", Q, 1),
    note("D5", Q, 1),
    note("D#5", 2 * Q, 80),
    note("B4", 2 * Q, 40),
    note("E5", Q, 40),
    note("E5", Q, 40),
    note("D5", Q / 2, 40),
    note("D5", Q, 80),
    note("C#5", 2 * Q, 40),
    note("A4", 2 * Q / 3, 40),
    note("B4", Q / 3, 1),
    note("A4", Q / 3, 40),
    note("A4", 2 * Q, 150),
    note("G4", 2 * Q / 3, 40),
    note("A4", Q / 3, 1),
    note("G4", Q / 3, 40),
    note("F#4", 2 * Q, 200),
    note("F#5", Q, 1),
    note("F#5", Q, 40),
    note("D5", 2 * Q, 200),
    note("C#5", Q, 40),
    note("D5", Q, 40),
    note("E5", Q),
    pause(40),
    note("B4", Q),
    pause(80),
    note("B4", Q / 2),
    pause(120),
    note("B4", Q),
    pause(160),
    note("A4", 3 * Q),
]);
