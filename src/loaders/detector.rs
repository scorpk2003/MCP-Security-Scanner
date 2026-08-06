use crate::ScanRequest;

pub trait Detector {
    type Output;
    fn detect(request: &ScanRequest) -> Result<Self::Output, String>;
}