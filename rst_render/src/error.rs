use std::fmt::Display;

use wgpu::{CreateSurfaceError, RequestDeviceError};

#[derive(Debug)]
pub enum Error {
    CreateSurface(CreateSurfaceError),
    AdapterNotFound,
    RequestDevice(RequestDeviceError),
}

impl From<CreateSurfaceError> for Error {
    fn from(e: CreateSurfaceError) -> Self {
        Error::CreateSurface(e)
    }
}

impl From<RequestDeviceError> for Error {
    fn from(e: RequestDeviceError) -> Self {
        Error::RequestDevice(e)
    }
}

impl std::error::Error for Error {}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::CreateSurface(e) => write!(f, "failed to create wgpu surface: {e}"),
            Error::AdapterNotFound => write!(f, "no suitable wgpu adapter found"),
            Error::RequestDevice(e) => write!(f, "failed to request wgpu device: {e}"),
        }
    }
}
