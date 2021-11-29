use base64::write::EncoderWriter;
use hdrhistogram::serialization::Serializer;
use hdrhistogram::serialization::V2DeflateSerializer;
use hdrhistogram::Histogram;
use std::fs::File;
use std::path::Path;

pub struct DaCapoLatencyDump {
    hist: Histogram<u64>,
}

impl DaCapoLatencyDump {
    pub fn new(path: &Path) -> Self {
        let reader = File::open(path).unwrap();
        Self::from_csv(reader)
    }

    fn from_csv<R: std::io::Read>(reader: R) -> Self {
        let mut rdr = csv::Reader::from_reader(reader);
        let mut hist = Histogram::<u64>::new(5).expect("Failed to create a histogram");
        for result in rdr.records() {
            let row = result.expect("Failed to read records");
            let start: u64 = row[0].trim().parse().unwrap();
            let end: u64 = row[1].trim().parse().unwrap();
            let latency = end - start;
            hist.record(latency).unwrap();
        }
        DaCapoLatencyDump { hist }
    }

    pub fn save_hdrh(&self, path: &Path) {
        V2DeflateSerializer::new()
            .serialize(
                &self.hist,
                // https://github.com/HdrHistogram/HdrHistogram_py/issues/29
                &mut EncoderWriter::new(&mut File::create(path).unwrap(), base64::STANDARD),
            )
            .expect("Failed to save histogram as hdrh");
    }
}
