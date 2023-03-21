/// An ImageSpec is an ordered list of image processing operations to be applied to an image.
/// The server will process the image according to the order specified in the spec.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ImageSpec {
    #[prost(message, repeated, tag = "1")]
    pub specs: ::prost::alloc::vec::Vec<Spec>,
}
/// Resize an image to a specific width and height using a specified resize algorithm.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResizeSpec {
    #[prost(uint32, tag = "1")]
    pub width: u32,
    #[prost(uint32, tag = "2")]
    pub height: u32,
    #[prost(enumeration = "resize_spec::ResizeType", tag = "3")]
    pub rtype: i32,
    #[prost(enumeration = "resize_spec::SampleFilter", tag = "4")]
    pub filter: i32,
}
/// Nested message and enum types in `ResizeSpec`.
pub mod resize_spec {
    /// The type of resize algorithm to use.
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum ResizeType {
        Normal = 0,
        SeamCarve = 1,
    }
    impl ResizeType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                ResizeType::Normal => "NORMAL",
                ResizeType::SeamCarve => "SEAM_CARVE",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "NORMAL" => Some(Self::Normal),
                "SEAM_CARVE" => Some(Self::SeamCarve),
                _ => None,
            }
        }
    }
    /// The sampling filter to use for the resize algorithm.
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum SampleFilter {
        Undefined = 0,
        Nearest = 1,
        Triangle = 2,
        CatmullRom = 3,
        Gaussian = 4,
        Lanczos3 = 5,
    }
    impl SampleFilter {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                SampleFilter::Undefined => "UNDEFINED",
                SampleFilter::Nearest => "NEAREST",
                SampleFilter::Triangle => "TRIANGLE",
                SampleFilter::CatmullRom => "CATMULL_ROM",
                SampleFilter::Gaussian => "GAUSSIAN",
                SampleFilter::Lanczos3 => "LANCZOS3",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "UNDEFINED" => Some(Self::Undefined),
                "NEAREST" => Some(Self::Nearest),
                "TRIANGLE" => Some(Self::Triangle),
                "CATMULL_ROM" => Some(Self::CatmullRom),
                "GAUSSIAN" => Some(Self::Gaussian),
                "LANCZOS3" => Some(Self::Lanczos3),
                _ => None,
            }
        }
    }
}
/// Crop an image to a specified rectangle.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CropSpec {
    #[prost(uint32, tag = "1")]
    pub x1: u32,
    #[prost(uint32, tag = "2")]
    pub y1: u32,
    #[prost(uint32, tag = "3")]
    pub x2: u32,
    #[prost(uint32, tag = "4")]
    pub y2: u32,
}
/// Flip an image horizontally.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FlipHSpec {}
/// Flip an image vertically.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FlipVSpec {}
/// Adjust the contrast of an image.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ContrastSpec {
    #[prost(float, tag = "1")]
    pub contrast: f32,
}
/// Apply a filter to an image.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FilterSpec {
    #[prost(enumeration = "filter_spec::FilterType", tag = "1")]
    pub filter: i32,
}
/// Nested message and enum types in `FilterSpec`.
pub mod filter_spec {
    /// The type of filter to apply.
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum FilterType {
        Unspecified = 0,
        Oceanic = 1,
        Islands = 2,
        /// For more filters, see: <https://docs.rs/photon-rs/0.3.1/photon_rs/filters/fn.filter.html>
        Marine = 3,
    }
    impl FilterType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                FilterType::Unspecified => "UNSPECIFIED",
                FilterType::Oceanic => "OCEANIC",
                FilterType::Islands => "ISLANDS",
                FilterType::Marine => "MARINE",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "UNSPECIFIED" => Some(Self::Unspecified),
                "OCEANIC" => Some(Self::Oceanic),
                "ISLANDS" => Some(Self::Islands),
                "MARINE" => Some(Self::Marine),
                _ => None,
            }
        }
    }
}
/// Apply a watermark to an image.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WatermarkSpec {
    #[prost(uint32, tag = "1")]
    pub x: u32,
    #[prost(uint32, tag = "2")]
    pub y: u32,
}
/// An ImageProcessingSpec represents one type of image processing operation that can be applied to an image.
/// Only one of the fields should be populated.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ImageProcessingSpec {
    #[prost(
        oneof = "image_processing_spec::ProcessingSpec",
        tags = "1, 2, 3, 4, 5, 6, 7"
    )]
    pub processing_spec: ::core::option::Option<image_processing_spec::ProcessingSpec>,
}
/// Nested message and enum types in `ImageProcessingSpec`.
pub mod image_processing_spec {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum ProcessingSpec {
        #[prost(message, tag = "1")]
        ResizeSpec(super::ResizeSpec),
        #[prost(message, tag = "2")]
        CropSpec(super::CropSpec),
        #[prost(message, tag = "3")]
        FlipHSpec(super::FlipHSpec),
        #[prost(message, tag = "4")]
        FlipVSpec(super::FlipVSpec),
        #[prost(message, tag = "5")]
        ContrastSpec(super::ContrastSpec),
        #[prost(message, tag = "6")]
        FilterSpec(super::FilterSpec),
        #[prost(message, tag = "7")]
        WatermarkSpec(super::WatermarkSpec),
    }
}
/// A Spec represents one image processing operation to be applied to an image.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Spec {
    #[prost(message, optional, tag = "1")]
    pub spec: ::core::option::Option<ImageProcessingSpec>,
    /// The index of this spec in the overall ImageSpec list. Used for ordering.
    #[prost(int32, tag = "2")]
    pub index: i32,
}
