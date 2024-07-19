use nih_plug::prelude::*;
use std::sync::Arc;

struct DigiDist2 {
    params: Arc<DigiDist2Params>,
}

#[derive(Params)]
struct DigiDist2Params {
    #[id = "threshold"]
    pub threshold: FloatParam,
}



impl Default for DigiDist2 {
    fn default() -> Self {
        Self {
            params: Arc::new(DigiDist2Params::default()),
        }
    }
}

impl Default for DigiDist2Params {
    fn default() -> Self {
        Self {
            threshold: FloatParam::new(
                "Threshold",
                1.0,
                FloatRange::Linear {
                    min: 0.0,
                    max: 1.0
                }
            )
            .with_unit(" %")
            .with_value_to_string(formatters::v2s_f32_percentage(3))
            .with_string_to_value(formatters::s2v_f32_percentage())
        }
    }
}

impl Plugin for DigiDist2 {
    const NAME: &'static str = "DigiDist2";
    const VENDOR: &'static str = "MalekDeKalemDSP";
    const URL: &'static str = "https://github.com/MalekDeKalem/digidist2";
    const EMAIL: &'static str = "dimitri200169@gmail.com";
    const VERSION: &'static str = env!("CARGO_PKG_VERSION");

    const AUDIO_IO_LAYOUTS: &'static [AudioIOLayout] = &[
        AudioIOLayout {
            main_input_channels: NonZeroU32::new(2),
            main_output_channels: NonZeroU32::new(2),
            aux_input_ports: &[],
            aux_output_ports: &[],
            names: PortNames::const_default(),
        },
        AudioIOLayout {
            main_input_channels: NonZeroU32::new(1),
            main_output_channels: NonZeroU32::new(1),
            ..AudioIOLayout::const_default()
        },
    ];

    const MIDI_INPUT: MidiConfig = MidiConfig::None;

    const SAMPLE_ACCURATE_AUTOMATION: bool = true;

    type SysExMessage = ();
    type BackgroundTask = ();


    fn params(&self) -> Arc<dyn Params> {
        self.params.clone()
    }

    fn process(&mut self, buffer: &mut Buffer, _aux: &mut AuxiliaryBuffers, _context: &mut impl ProcessContext<Self>) -> ProcessStatus {
        
        for channel_samples in buffer.iter_samples() {
            let threshold = self.params.threshold.smoothed.next();
            for sample in channel_samples {
                if *sample >= 0.0 {
                    *sample = sample.min(threshold)  / threshold;
                } else {
                    *sample = sample.max(-threshold) / threshold;
                }
            }
        }

        ProcessStatus::Normal
    }
    fn deactivate(&mut self) {}

}

impl ClapPlugin for DigiDist2 {
    const CLAP_ID: &'static str = "com.digidist2.distortion";
    const CLAP_DESCRIPTION: Option<&'static str> = Some("A Digital Distortion Plugin with threshold parameter");
    const CLAP_MANUAL_URL: Option<&'static str> = None;
    const CLAP_SUPPORT_URL: Option<&'static str> = None;
    const CLAP_FEATURES: &'static [ClapFeature] = &[
        ClapFeature::AudioEffect,
        ClapFeature::Stereo,
        ClapFeature::Mono,
        ClapFeature::Utility,
    ];
}

impl Vst3Plugin for DigiDist2 {
    const VST3_CLASS_ID: [u8; 16] = *b"MalekDeKalemPlug";
    const VST3_SUBCATEGORIES: &'static [Vst3SubCategory] =
        &[Vst3SubCategory::Fx, Vst3SubCategory::Tools];
}


nih_export_clap!(DigiDist2);
nih_export_vst3!(DigiDist2);
