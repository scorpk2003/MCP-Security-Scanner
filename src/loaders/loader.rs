use crate::{Profile, ScanRequest};


impl Profile {
    fn target_detect(request: &ScanRequest) -> Result<Self, String> {
        let mut target_profile = Profile::default();
        if let Some(path) =  request.config_path.clone() {
            let cfg_name = path.file_name().unwrap().to_str().unwrap();
        }

        Ok(target_profile)
    }
    fn cmd_detect(request: &ScanRequest) -> Result<Self, String> {}
    fn config_detect(request: &ScanRequest) -> Result<Self, String> {}
}