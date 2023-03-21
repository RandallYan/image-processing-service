use crate::protos::image_processing::*;
use base64::{
    alphabet,
    engine::{self, general_purpose},
    Engine as _,
};
use photon_rs::transform::SamplingFilter;
use prost::Message;
pub mod image_processing;

const CUSTOM_ENGINE: engine::GeneralPurpose =
    engine::GeneralPurpose::new(&alphabet::URL_SAFE, general_purpose::NO_PAD);

impl ImageSpec {
    pub fn new(specs: Vec<Spec>) -> Self {
        ImageSpec { specs }
    }
}

impl TryFrom<&str> for ImageSpec {
    type Error = prost::DecodeError;

    fn try_from(spec: &str) -> Result<Self, Self::Error> {
        let data = CUSTOM_ENGINE.decode(spec.as_bytes()).unwrap();
        ImageSpec::decode_length_delimited(&data[..])
    }
}

impl From<&ImageSpec> for String {
    fn from(spec: &ImageSpec) -> Self {
        let data = spec.encode_length_delimited_to_vec();
        CUSTOM_ENGINE.encode(&data)
    }
}

impl From<resize_spec::SampleFilter> for SamplingFilter {
    fn from(filter: resize_spec::SampleFilter) -> Self {
        match filter {
            resize_spec::SampleFilter::Nearest => SamplingFilter::Nearest,
            resize_spec::SampleFilter::Triangle => SamplingFilter::Triangle,
            resize_spec::SampleFilter::CatmullRom => SamplingFilter::CatmullRom,
            resize_spec::SampleFilter::Gaussian => SamplingFilter::Gaussian,
            resize_spec::SampleFilter::Lanczos3 => SamplingFilter::Lanczos3,
            _ => SamplingFilter::Nearest,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_image_spec_encoding_decoding() {
        // Create a new ImageSpec with some dummy Spec values
        let specs = vec![
            Spec {
                index: 0,
                spec: Some(ImageProcessingSpec {
                    processing_spec: Some(image_processing_spec::ProcessingSpec::ResizeSpec(
                        ResizeSpec {
                            width: 100,
                            height: 100,
                            rtype: resize_spec::ResizeType::Normal as i32,
                            filter: resize_spec::SampleFilter::Gaussian as i32,
                        },
                    )),
                }),
            },
            Spec {
                index: 1,
                spec: Some(ImageProcessingSpec {
                    processing_spec: Some(image_processing_spec::ProcessingSpec::CropSpec(
                        CropSpec {
                            x1: 100,
                            y1: 100,
                            x2: 50,
                            y2: 50,
                        },
                    )),
                }),
            },
            Spec {
                index: 2,
                spec: Some(ImageProcessingSpec {
                    processing_spec: Some(image_processing_spec::ProcessingSpec::FlipHSpec(
                        FlipHSpec {},
                    )),
                }),
            },
            // Spec { spec: Some(RotateSpec { degrees: 90 }), ..Default::default() },
        ];
        let image_spec = ImageSpec::new(specs);

        // Encode the ImageSpec to a base64 string
        let encoded_spec = String::from(&image_spec);
        println!("Encoded ImageSpec: {}", encoded_spec);

        // Decode the base64 string back to an ImageSpec
        let decoded_spec = ImageSpec::try_from(encoded_spec.as_str()).unwrap();

        // Assert that the original and decoded ImageSpecs are the same
        assert_eq!(image_spec, decoded_spec);
    }

    #[test]
    fn test_resize_spec() {
        let resize_type = resize_spec::ResizeType::Normal;
        let resize_spec = ResizeSpec {
            width: 100,
            height: 100,
            rtype: resize_type as i32,
            filter: resize_spec::SampleFilter::Gaussian as i32,
        };
        assert_eq!(resize_spec.width, 100);
        assert_eq!(resize_spec.height, 100);
        assert_eq!(resize_spec.rtype, resize_spec::ResizeType::Normal as i32);
        assert_eq!(
            resize_spec.filter,
            resize_spec::SampleFilter::Gaussian as i32
        );
    }
}
