// Copyright (C) 2021 electron271
//
// This file is part of q>q.
//
// q>q is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// q>q is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with q>q.  If not, see <http://www.gnu.org/licenses/>.

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
