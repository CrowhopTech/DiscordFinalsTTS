pub enum MediaFormat {
    MP3,
}

impl MediaFormat {
    pub fn to_str(&self) -> &str {
        match self {
            MediaFormat::MP3 => "mp3",
        }
    }
}

pub struct OutputFormat(MediaFormat, i32, i32); // Use &str as it's a constant string

pub static DEFAULT_OUTPUT_FORMAT: &OutputFormat = MP3_44100HZ_128KBPS;
pub static MP3_44100HZ_128KBPS: &OutputFormat = &OutputFormat(MediaFormat::MP3, 44100, 128000);

impl OutputFormat {
    #[allow(dead_code)]
    pub fn get_format(&self) -> &MediaFormat {
        &self.0
    }

    #[allow(dead_code)]
    pub fn get_sample_rate(&self) -> i32 {
        self.1
    }

    #[allow(dead_code)]
    pub fn get_bitrate(&self) -> i32 {
        self.2
    }

    pub fn to_string(&self) -> String {
        format!("{}_{}_{}", self.0.to_str(), self.1, self.2)
    }
}
