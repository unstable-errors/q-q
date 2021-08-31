use dialoguer::{theme::ColorfulTheme, Input};
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

        println!(
            "Hello {}, I'm Python {}! Please enter some python code below.",
            user, version
        );
        let code: String = Input::with_theme(&ColorfulTheme::default())
            .with_prompt("Code:")
            .default("print('Hello World!')".to_string())
            .interact_text()
            .unwrap();

        let code_slice: &str = &code[..];

        let _result = py.eval(code_slice, None, None).map_err(|e| {
            e.print_and_set_sys_last_vars(py);
        });

        Ok(())
    })
}
