use crate::{MusicError, Tuning};
use midir::{MidiOutput, MidiOutputConnection, MidiOutputPort};
use std::cell::RefCell;
use std::rc::Rc;

pub struct MidiPlayer {
    name: String,
    midi_out: Option<MidiOutput>,
    port: Option<MidiOutputPort>,
    midi_out_conn: Rc<RefCell<Option<MidiOutputConnection>>>,

    midi_player_channels: Option<[RefCell<MidiPlayerChannel>; 16]>,
}

pub struct MidiPlayerChannel {
    midi_out_conn: Rc<RefCell<Option<MidiOutputConnection>>>,
    channel: u8,
}

impl MidiPlayer {
    pub fn new(name: &str) -> Self {
        let mut midi_player = MidiPlayer {
            name: name.to_owned(),
            midi_out: None,
            port: None,
            midi_out_conn: Rc::new(RefCell::new(None)),
            midi_player_channels: None,
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

    pub fn connect(
        &mut self,
        port_name: &str,
    ) -> Result<&mut [RefCell<MidiPlayerChannel>; 16], String> {
        match &self.port {
            None => Err("No port selected".to_owned()),
            Some(port) => {
                self.midi_out_conn = Rc::new(RefCell::new(
                    self.midi_out
                        .take()
                        .ok_or("Midi output is not initialized")?
                        .connect(port, port_name)
                        .ok(),
                ));

                self.midi_player_channels = Some([
                    RefCell::new(MidiPlayerChannel::new(self.midi_out_conn.clone(), 0)),
                    RefCell::new(MidiPlayerChannel::new(self.midi_out_conn.clone(), 1)),
                    RefCell::new(MidiPlayerChannel::new(self.midi_out_conn.clone(), 2)),
                    RefCell::new(MidiPlayerChannel::new(self.midi_out_conn.clone(), 3)),
                    RefCell::new(MidiPlayerChannel::new(self.midi_out_conn.clone(), 4)),
                    RefCell::new(MidiPlayerChannel::new(self.midi_out_conn.clone(), 5)),
                    RefCell::new(MidiPlayerChannel::new(self.midi_out_conn.clone(), 6)),
                    RefCell::new(MidiPlayerChannel::new(self.midi_out_conn.clone(), 7)),
                    RefCell::new(MidiPlayerChannel::new(self.midi_out_conn.clone(), 8)),
                    RefCell::new(MidiPlayerChannel::new(self.midi_out_conn.clone(), 9)),
                    RefCell::new(MidiPlayerChannel::new(self.midi_out_conn.clone(), 10)),
                    RefCell::new(MidiPlayerChannel::new(self.midi_out_conn.clone(), 11)),
                    RefCell::new(MidiPlayerChannel::new(self.midi_out_conn.clone(), 12)),
                    RefCell::new(MidiPlayerChannel::new(self.midi_out_conn.clone(), 13)),
                    RefCell::new(MidiPlayerChannel::new(self.midi_out_conn.clone(), 14)),
                    RefCell::new(MidiPlayerChannel::new(self.midi_out_conn.clone(), 15)),
                ]);
                Ok(self.midi_player_channels.as_mut().unwrap())
            }
        }
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
        for channel in self.midi_player_channels.iter_mut().flatten() {
            channel.borrow_mut().stop_all();
        }
    }
}

impl Drop for MidiPlayer {
    fn drop(&mut self) {
        self.close();
    }
}

impl<'a> MidiPlayerChannel {
    fn new(midi_out_conn: Rc<RefCell<Option<MidiOutputConnection>>>, channel: u8) -> Self {
        MidiPlayerChannel {
            midi_out_conn,
            channel,
        }
    }

    pub fn play_notes(&mut self, notes: &[u8]) {
        if let Some(conn) = &mut self.midi_out_conn.borrow_mut().as_mut() {
            for (index, note) in notes.iter().enumerate() {
                let _ = conn.send(&[
                    0x90 | (self.channel & 0xF) | (index as u8 & 0x0F),
                    *note,
                    0x64,
                ]);
            }
        }
    }

    pub fn stop_notes(&mut self, notes: &[u8]) {
        if let Some(conn) = &mut self.midi_out_conn.borrow_mut().as_mut() {
            for (index, note) in notes.iter().enumerate() {
                let _ = conn.send(&[
                    0x80 | (self.channel & 0xF) | (index as u8 & 0x0F),
                    *note,
                    0x64,
                ]);
            }
        }
    }

    pub fn stop_all(&mut self) {
        self.stop_notes((0..=127).collect::<Vec<_>>().as_slice());
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
