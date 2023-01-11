
use {
    crate::{
        cx::Cx,
        audio::*,
        midi::*,
        media_api::CxMediaApi,
        os::apple::core_midi::*
    }
};


impl CxMediaApi for Cx {
    fn midi_input(&mut self) -> MidiInput {
        self.os.core_midi().lock().unwrap().create_midi_input()
    }
    
    fn midi_output(&mut self) -> MidiOutput {
        MidiOutput(Some(OsMidiOutput(self.os.core_midi())))
    }
    
    fn use_midi_inputs(&mut self, ports: &[MidiPortId]) {
        self.os.core_midi().lock().unwrap().use_midi_inputs(ports);
    }
    
    fn use_midi_outputs(&mut self, ports: &[MidiPortId]) {
        self.os.core_midi().lock().unwrap().use_midi_outputs(ports);
    }
    
    fn use_audio_inputs(&mut self, devices: &[AudioDeviceId]) {
        self.os.audio_unit().lock().unwrap().use_audio_inputs(devices);
    }
    
    fn use_audio_outputs(&mut self, devices: &[AudioDeviceId]) {
        self.os.audio_unit().lock().unwrap().use_audio_outputs(devices);
    }
    
    fn audio_output<F>(&mut self, index:usize, f: F) where F: FnMut(AudioDeviceId, AudioTime, &mut AudioBuffer) + Send + 'static {
        *self.os.audio_unit().lock().unwrap().audio_output_cb[index].lock().unwrap() = Some(Box::new(f));
    }
    
    fn audio_input<F>(&mut self, index:usize, f: F)
    where F: FnMut(AudioDeviceId, AudioTime, AudioBuffer) -> AudioBuffer + Send + 'static {
        *self.os.audio_unit().lock().unwrap().audio_input_cb[index].lock().unwrap() = Some(Box::new(f));
    }
    
}

