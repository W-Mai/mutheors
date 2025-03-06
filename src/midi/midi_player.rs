use crate::{MusicError, Tuning};
use midir::{MidiOutput, MidiOutputConnection, MidiOutputPort};

pub struct MidiPlayer {
    name: String,
    midi_out: Option<MidiOutput>,
    port: Option<MidiOutputPort>,
    midi_out_conn: Option<MidiOutputConnection>,
}

impl MidiPlayer {
    pub fn new(name: &str) -> Self {
        let mut midi_player = MidiPlayer {
            name: name.to_owned(),
            midi_out: None,
            port: None,
            midi_out_conn: None,
        };

        let midi_out = MidiOutput::new(&midi_player.name).ok();
        midi_player.midi_out = midi_out;
        midi_player
    }

    pub fn list_ports(&self) -> Vec<String> {
        if let Some(midi_out) = &self.midi_out {
            let midi_out_ports = midi_out.ports();
            midi_out_ports
                .iter()
                .filter_map(|port| midi_out.port_name(port).ok())
                .collect()
        } else {
            vec![]
        }
    }

    pub fn select_port(&mut self, port_index: usize) -> Result<(), String> {
        if let Some(midi_out) = &self.midi_out {
            let midi_out_ports = midi_out.ports();
            if port_index >= midi_out_ports.len() {
                return Err("Port index out of range".to_owned());
            }
            self.port = Some(midi_out_ports[port_index].to_owned());
        } else {
            return Err("No MIDI output".to_owned());
        }
        Ok(())
    }

    pub fn connect(&mut self, port_name: &str) -> Result<(), String> {
        match &self.port {
            None => Err("No port selected".to_owned()),
            Some(port) => {
                self.midi_out_conn = self
                    .midi_out
                    .take()
                    .ok_or("Midi output is not initialized")?
                    .connect(port, port_name)
                    .ok();
                Ok(())
            }
        }
    }

    pub fn play_notes(&mut self, ch: u8, notes: &[u8]) {
        if let Some(conn) = &mut self.midi_out_conn {
            for (index, note) in notes.iter().enumerate() {
                let _ = conn.send(&[0x90 | (ch & 0xF), *note, 0x64]);
            }
        }
    }

    pub fn stop_notes(&mut self, ch: u8, notes: &[u8]) {
        if let Some(conn) = &mut self.midi_out_conn {
            for (index, note) in notes.iter().enumerate() {
                let _ = conn.send(&[0x80 | (ch & 0xF), *note, 0x64]);
            }
        }
    }

    pub fn play_note(&mut self, ch: u8, note: u8) {
        self.play_notes(ch, &[note]);
    }

    pub fn stop_note(&mut self, ch: u8, note: u8) {
        self.stop_notes(ch, &[note]);
    }

    pub fn close(&mut self) {
        self.reset_notes();
        match self.midi_out_conn.take() {
            None => {}
            Some(conn) => {
                self.midi_out = Some(conn.close());
            }
        }
    }

    fn reset_notes(&mut self) {
        for i in 0..15 {
            self.stop_notes(i, (0..=127).collect::<Vec<_>>().as_slice());
        }
    }
}

impl Drop for MidiPlayer {
    fn drop(&mut self) {
        self.close();
    }
}

impl Tuning {
    /// Get MIDI pitch number (A4=69)
    pub fn midi_number(&self) -> Result<u8, MusicError> {
        let base = self.class as u8;
        if base == 0 {
            return Ok(0);
        }
        let base = base - 1;
        let num = (self.octave + 1) * 12 + base as i8;
        num.try_into().map_err(|_| MusicError::InvalidPitch)
    }
}
