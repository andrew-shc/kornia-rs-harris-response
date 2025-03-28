use kornia_image::Image;
use kornia_io::jpegturbo::{JpegTurboDecoder, JpegTurboEncoder};
use pyo3::prelude::*;

use crate::image::{FromPyImage, PyImage, PyImageSize, ToPyImage};

#[pyclass(name = "ImageDecoder")]
pub struct PyImageDecoder {
    pub inner: JpegTurboDecoder,
}

#[pymethods]
impl PyImageDecoder {
    #[new]
    pub fn new() -> PyResult<PyImageDecoder> {
        let inner = JpegTurboDecoder::new()
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyException, _>(format!("{}", e)))?;
        Ok(PyImageDecoder { inner })
    }

    pub fn read_header(&mut self, jpeg_data: &[u8]) -> PyResult<PyImageSize> {
        let image_size = self
            .inner
            .read_header(jpeg_data)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyException, _>(format!("{}", e)))?;
        Ok(image_size.into())
    }

    pub fn decode(&mut self, jpeg_data: &[u8]) -> PyResult<PyImage> {
        let image = self
            .inner
            .decode_rgb8(jpeg_data)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyException, _>(format!("{}", e)))?;
        Ok(image.to_pyimage())
    }
}

#[pyclass(name = "ImageEncoder")]
pub struct PyImageEncoder {
    pub inner: JpegTurboEncoder,
}

#[pymethods]
impl PyImageEncoder {
    #[new]
    pub fn new() -> PyResult<PyImageEncoder> {
        let inner = JpegTurboEncoder::new()
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyException, _>(format!("{}", e)))?;
        Ok(PyImageEncoder { inner })
    }

    pub fn encode(&mut self, image: PyImage) -> PyResult<Vec<u8>> {
        let image = Image::from_pyimage(image)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyException, _>(format!("{}", e)))?;
        let jpeg_data = self
            .inner
            .encode_rgb8(&image)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyException, _>(format!("{}", e)))?;
        Ok(jpeg_data)
    }

    pub fn set_quality(&mut self, quality: i32) -> PyResult<()> {
        self.inner
            .set_quality(quality)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyException, _>(format!("{}", e)))?;
        Ok(())
    }
}
