#[macro_use] extern crate vst2;

use vst2::buffer::AudioBuffer;
use vst2::plugin::{Plugin, Info};

struct Sodoi {
    threshold: f32,
}

impl Plugin for Sodoi {
    fn get_parameter(&self, index: i32) -> f32 {
        match index {
            0 => self.threshold,
            _ => 0.0,
        }
    }

    fn set_parameter(&mut self, index: i32, value: f32) {
        match index {
            0 => self.threshold = value.max(0.01),
            _ => (),
        }
    }

    fn get_parameter_name(&self, index: i32) -> String {
        match index {
            0 => "Threshold".to_string(),
            _ => "".to_string(),
        }
    }

    fn get_parameter_text(&self, index: i32) -> String {
        match index {
            0 => format!("{}", self.threshold * 100.0),
            _ => "".to_string(),
        }
    }

    fn get_parameter_label(&self, index: i32) -> String {
        match index {
            0 => "%".to_string(),
            _ => "".to_string(),
        }
    }

    fn get_info(&self) -> Info {
        Info {
            name: "Sodoi".to_string(),
            vendor: "Elliot".to_string(),
            unique_id: 285710847,
            inputs: 2,
            outputs: 2,
            parameters: 1,
            ..Info::default()
        }
    }

    fn process(&mut self, buffer: AudioBuffer<f32>) {
        // Split the input and output buffers into two vectors
        let (inputs, outputs) = buffer.split();
        // For each buffer, transform the sample
        for (input_buffer, output_buffer) in inputs.into_iter().zip(outputs) {
            for (input_sample, output_sample) in input_buffer.into_iter().zip(output_buffer) {
                if *input_sample >= 0.0 {
                    *output_sample = input_sample.min(self.threshold) / self.threshold;
                } else {
                    *output_sample = input_sample.max(-self.threshold) / self.threshold;
                }
            }
        }
    }
}

impl Default for Sodoi {
    fn default() -> Sodoi {
        Sodoi {
            threshold: 1.0,
        }
    }
}

plugin_main!(Sodoi);
