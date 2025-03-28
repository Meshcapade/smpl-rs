use super::pose_hands::PyHandType;
use gloss_hecs::Entity;
use gloss_py_macros::PyComponent;
use gloss_renderer::scene::Scene;
use pyo3::prelude::*;
use smpl_core::common::{pose_hands::HandType, pose_override::PoseOverride};
#[pyclass(name = "PoseOverride", module = "smpl_rs.components", unsendable)]
#[derive(Clone, PyComponent)]
pub struct PyPoseOverride {
    pub inner: PoseOverride,
}
#[pymethods]
impl PyPoseOverride {
    #[staticmethod]
    #[pyo3(text_signature = "() -> PoseOverride")]
    pub fn allow_all() -> Self {
        Self {
            inner: PoseOverride::allow_all(),
        }
    }
    #[staticmethod]
    #[pyo3(text_signature = "() -> PoseOverride")]
    pub fn deny_all() -> Self {
        Self {
            inner: PoseOverride::deny_all(),
        }
    }
    #[pyo3(text_signature = "($self, hand_type: HandType) -> PoseOverride")]
    pub fn overwrite_hands(mut slf: PyRefMut<'_, Self>, pyhandtype: PyHandType) -> PyRefMut<'_, Self> {
        let hand_type = HandType::from(pyhandtype);
        Python::with_gil(|_py| {
            let new_pose_override = slf.inner.clone().overwrite_hands(hand_type);
            slf.inner = new_pose_override;
        });
        slf
    }
}
