use pyo3::prelude::*;
use pyo3::types::IntoPyDict;

pub fn python() -> PyResult<()> {
    Python::with_gil(|py| {
        let sys = py.import("sys")?;
        #[allow(deprecated)]
        let version: String = sys.get("version")?.extract()?;

        let locals = [("os", py.import("os")?)].into_py_dict(py);
        let code = "os.getenv('USER') or os.getenv('USERNAME') or 'Unknown'";
        let user: String = py.eval(code, None, Some(&locals))?.extract()?;

        println!("Hello {}, I'm Python {}", user, version);
        Ok(())
    })
}