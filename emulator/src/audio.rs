use rodio::{source::SineWave, OutputStream, Sink, Source};

pub struct Audio {
    _stream: OutputStream,
    sink: Sink,
    beep: bool,
}

impl Audio {
    pub fn new() -> Self {
        let (stream, stream_handle) = OutputStream::try_default().unwrap();

        Self {
            _stream: stream,
            sink: Sink::try_new(&stream_handle).unwrap(),
            beep: false,
        }
    }

    pub fn beep_start(&mut self) {
        if !self.beep {
            let source = SineWave::new(660.0).amplify(0.20);
            self.sink.append(source);
            self.beep = true;
        }
    }

    pub fn beep_stop(&mut self) {
        if self.beep {
            self.sink.stop();
            self.beep = false;
        }
    }
}
