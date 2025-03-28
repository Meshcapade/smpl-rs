use burn::backend::{candle::CandleDevice, Candle};
use gloss_hecs::Entity;
use gloss_py_macros::PyComponent;
use gloss_renderer::scene::Scene;
use gloss_utils::bshare::{ToBurn, ToNdArray};
use numpy::{PyArray2, PyArrayMethods, PyReadonlyArray2, ToPyArray};
use pyo3::prelude::*;
use smpl_core::common::outputs::{SmplOutputDynamic, SmplOutputPoseTDynamic, SmplOutputPosedDynamic};
#[pyclass(name = "SmplOutputPoseT", module = "smpl_rs.models", unsendable)]
#[derive(Clone, PyComponent)]
pub struct PySmplOutputPoseT {
    pub inner: SmplOutputPoseTDynamic<Candle>,
}
#[pymethods]
impl PySmplOutputPoseT {
    #[getter]
    pub fn verts(&mut self, py: Python<'_>) -> Py<PyArray2<f32>> {
        self.inner.verts.to_ndarray().to_pyarray_bound(py).into()
    }
    #[pyo3(text_signature = "($self) -> NDArray[np.float32]")]
    pub fn verts_without_expression(&mut self, py: Python<'_>) -> Py<PyArray2<f32>> {
        self.inner.verts_without_expression.to_ndarray().to_pyarray_bound(py).into()
    }
}
#[pyclass(name = "SmplOutputPosed", module = "smpl_rs.models", unsendable)]
#[derive(Clone, PyComponent)]
pub struct PySmplOutputPosed {
    pub inner: SmplOutputPosedDynamic<Candle>,
}
#[pymethods]
impl PySmplOutputPosed {
    #[getter]
    pub fn joints(&mut self, py: Python<'_>) -> Py<PyArray2<f32>> {
        self.inner.joints.to_ndarray().to_pyarray_bound(py).into()
    }
    #[getter]
    pub fn verts(&mut self, py: Python<'_>) -> Py<PyArray2<f32>> {
        self.inner.verts.to_ndarray().to_pyarray_bound(py).into()
    }
}
#[pyclass(name = "SmplOutput", module = "smpl_rs.models", unsendable)]
#[derive(Clone, PyComponent)]
pub struct PySmplOutput {
    pub inner: SmplOutputDynamic<Candle>,
}
#[pymethods]
impl PySmplOutput {
    #[getter]
    pub fn verts(&mut self, py: Python<'_>) -> Py<PyArray2<f32>> {
        self.inner.verts.to_ndarray().to_pyarray_bound(py).into()
    }
    #[getter]
    pub fn faces(&mut self, py: Python<'_>) -> Py<PyArray2<u32>> {
        self.inner.faces.to_ndarray().to_pyarray_bound(py).into()
    }
    #[getter]
    pub fn uvs(&mut self, py: Python<'_>) -> Option<Py<PyArray2<f32>>> {
        self.inner.uvs.as_ref().map(|x| x.to_ndarray().to_pyarray_bound(py).into())
    }
    #[getter]
    pub fn normals(&mut self, py: Python<'_>) -> Option<Py<PyArray2<f32>>> {
        self.inner.normals.as_ref().map(|x| x.to_ndarray().to_pyarray_bound(py).into())
    }
    #[getter]
    pub fn joints(&mut self, py: Python<'_>) -> Py<PyArray2<f32>> {
        self.inner.joints.to_ndarray().to_pyarray_bound(py).into()
    }
    #[pyo3(text_signature = "($self) -> None")]
    pub fn compute_normals(&mut self) {
        self.inner.compute_normals();
    }
    #[setter]
    fn set_verts(&mut self, v: PyReadonlyArray2<f32>) {
        self.inner.verts = v.to_owned_array().to_burn(&CandleDevice::Cpu);
    }
}
