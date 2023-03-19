pub mod image_processing;

#[cfg(test)]
mod tests {

    use crate::protos::image_processing::*;
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
        assert_eq!(resize_spec.filter, resize_spec::SampleFilter::Gaussian as i32);
    }
}