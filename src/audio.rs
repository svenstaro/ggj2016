extern crate rodio;

use std::io::BufReader;
use std::fs::File;

pub struct Audio {
    endpoint : rodio::Endpoint,
    sink: rodio::Sink
}

impl Audio {
    pub fn new() -> Audio {
       let endpoint = rodio::get_default_endpoint().unwrap();
       let sink = rodio::Sink::new(&endpoint);
       Audio { 
           endpoint: endpoint,
           sink: sink
       }
    }

    pub fn play(&self, filename: &str) {
        let file = File::open(filename).unwrap();
        let source = rodio::Decoder::new(BufReader::new(file)).unwrap();
        self.sink.append(source);
    }
}
