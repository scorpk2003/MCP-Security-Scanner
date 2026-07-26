use std::{env, fs, path::PathBuf};

use crate::{Args, TargetType};

#[derive(Debug, Clone)]
pub struct ScanRequest {
    pub input_mode: Vec<InputType>,
    pub target_path: Option<PathBuf>, // One target
    pub cmd: Option<String>,
    pub config_path: Option<PathBuf>, // One config
    pub format: TargetType,
    pub output_path: Option<PathBuf>,
    pub timeout_ms: u64,
    pub verbose: bool,
    pub no_introspection: bool,
}

impl Default for ScanRequest {
    fn default() -> Self {
        let output = env::current_dir().expect("Get current directory failed!!!").join("docs/");
        fs::create_dir_all(output.clone()).expect("Create directory for output failed!!!");
        let output_file = output.join("test_output.txt");

        let input_mode = Vec::new();
        let target_path = None;
        let cmd = None;
        let config_path = None;
        let format = TargetType::CONSOLE;
        let output_path = Some(output_file);
        let timeout_ms = 1000 as u64;
        let verbose = false;
        let no_introspection = false;
        Self { input_mode, target_path, cmd, config_path, format, output_path, timeout_ms, verbose, no_introspection }
    }
}

impl ScanRequest {
    pub fn mapping_args(args: &Args) -> Self {
        let input_scan = args.get_input();

        let mut scan_request = ScanRequest::default();
        input_scan.iter().for_each(|(t, s)| {
            scan_request.input_mode.push(t.clone());
            match t {
                InputType::COMMAND => {
                    scan_request.cmd = Some(s.clone());
                },
                InputType::CONFIG => {
                    scan_request.config_path = Some(PathBuf::from(s.clone()));
                },
                InputType::TARGET => {
                    scan_request.target_path = Some(PathBuf::from(s.clone()));
                }
            }
        });

        if args.format.is_some() {scan_request.format = args.format.clone().unwrap()}
        if args.output.is_some() {scan_request.output_path = args.output.clone()}
        if args.timeout.is_some() {scan_request.timeout_ms = args.timeout.unwrap()}
        scan_request.verbose = args.verbose;
        scan_request.no_introspection = args.no_instrospection;

        scan_request
    }

    // fn binding_input(&mut self, input_scan: &Vec<(InputType, String)>)
    // {
    //     let _ = 
    // }
}

#[derive(Debug, Clone)]
pub enum InputType {
    TARGET,
    COMMAND,
    CONFIG,
}