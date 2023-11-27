mod args;
mod settings;
mod run_tasks;
mod ui;
mod etcher;
mod timer;
mod embedsource;
use crate::args::Arguments;
use pyo3::prelude::*;
use dict_derive::FromPyObject;

#[derive(FromPyObject)]
pub struct EncodeParams {
    in_path: String,
    out_path: String,
    block_size: Option<i32>,
    threads: Option<usize>,
    fps: Option<i32>,
    resolution: Option<String>
}

#[pyfunction]
pub fn encode(params: EncodeParams) -> PyResult<()> {
    let args = Arguments {
        command: Some(args::Commands::Embed(args::EmbedParams { 
            in_path: Some(params.in_path), 
            out_path: Some(params.out_path), 
            preset: Some(args::EmbedPreset::Paranoid), 
            mode: Some(args::EmbedOutputMode::Binary),
            block_size: params.block_size,
            threads: params.threads,
            fps: params.fps,
            resolution: params.resolution 
        }))
    };
    let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap();
    let result = runtime.block_on(run_tasks::run_by_arguments(args));
    Ok(())
}

#[derive(FromPyObject)]
pub struct DecodeParams {
    in_path: String,
    out_path: String
}

#[pyfunction]
pub fn decode(params: DecodeParams) -> PyResult<()> {
    let args = Arguments {
        command: Some(args::Commands::Dislodge(args::DislodgeParams { 
            in_path: Some(params.in_path), 
            out_path: Some(params.out_path) 
        }))
    };
    let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap();
    let result = runtime.block_on(run_tasks::run_by_arguments(args));
    Ok(())
}
