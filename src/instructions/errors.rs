use core::panic;
use std::fmt::{self};

use wasm_bindgen::{JsError, JsValue};

pub struct AxError {
    detail: Option<String>,
    message: Option<String>,
    js: Option<JsValue>,

    pub(crate) signals_normal_finish: bool,
}

impl AxError {
    pub(crate) fn end_execution(&self) -> Self {
        Self {
            message: self.message.clone(),
            js: self.js.clone(),
            signals_normal_finish: true,
            detail: self.detail.clone(),
        }
    }
}

impl AxError {
    pub(crate) fn add_detail(&self, s: String) -> AxError {
        AxError {
            detail: Some(s),
            message: self.message.clone(),
            js: self.js.clone(),
            signals_normal_finish: self.signals_normal_finish,
        }
    }
}

impl From<&str> for AxError {
    fn from(message: &str) -> Self {
        Self {
            detail: None,
            message: Some(message.to_string()),
            js: None,
            signals_normal_finish: false,
        }
    }
}
impl From<String> for AxError {
    fn from(message: String) -> Self {
        Self {
            detail: None,
            message: Some(message),
            js: None,
            signals_normal_finish: false,
        }
    }
}
impl From<JsError> for AxError {
    fn from(err: JsError) -> Self {
        Self {
            detail: None,
            message: None,
            js: Some(JsValue::from(err)),
            signals_normal_finish: false,
        }
    }
}
impl From<JsValue> for AxError {
    fn from(err: JsValue) -> Self {
        Self {
            detail: None,
            message: None,
            js: Some(err),
            signals_normal_finish: false,
        }
    }
}

impl From<AxError> for JsValue {
    fn from(err: AxError) -> Self {
        if let Some(v) = err.js {
            v
        } else if let Some(m) = err.message {
            JsValue::from(m)
        } else {
            panic!("AxError is empty")
        }
    }
}

impl From<AxError> for JsError {
    fn from(err: AxError) -> Self {
        JsError::new(
            if let Some(v) = err.js {
                format!("{:?}", v)
            } else if let Some(m) = err.message {
                m
            } else {
                panic!("AxError is empty")
            }
            .as_str(),
        )
    }
}

// Implement std::fmt::Display for AxError
impl fmt::Display for AxError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}{}",
            if let Some(m) = &self.detail { m } else { "" },
            if let Some(ref m) = self.message {
                m.to_owned()
            } else if let Some(ref v) = self.js {
                format!("{:?}", v)
            } else {
                panic!("AxError is empty")
            }
        )
    }
}

// Implement std::fmt::Debug for AxError
impl fmt::Debug for AxError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}{}",
            if let Some(m) = &self.detail { m } else { "" },
            if let Some(ref m) = self.message {
                m.to_owned()
            } else if let Some(ref v) = self.js {
                format!("{:?}", v)
            } else {
                panic!("AxError is empty")
            }
        )
    }
}

#[macro_export]
macro_rules! fatal_error {
    ($message:expr, $($arg:tt)*) => {{
        #[cfg(all(target_arch = "wasm32", test))]
        {
            return Err(AxError::from(format!($message, $($arg)*)).into());
        }

        #[cfg(all(target_arch = "wasm32", not(test)))]
        {
            // In WASM we don't panic, as it's not possible to catch panics from JS
            return Err(AxError::from(format!($message, $($arg)*)));
        }

        #[cfg(not(target_arch = "wasm32"))]
        {
            panic!($message, $($arg)*);
        }
    }};
    ($message:expr) => {{
        #[cfg(target_arch = "wasm32")]
        {
            // In WASM we don't panic, as it's not possible to catch panics from JS
            return Err(AxError::from($message));
        }

        #[cfg(not(target_arch = "wasm32"))]
        {
            panic!($message);
        }
    }};
}

#[macro_export]
macro_rules! assert_fatal {
    ($cond:expr, $message:expr, $($arg:tt)*) => {{
        if !($cond) {
            crate::fatal_error!($message, $($arg)*);
        }
    }};
    ($cond:expr, $message:expr) => {{
        if !($cond) {
            crate::fatal_error!($message);
        }
    }};
}

#[macro_export]
macro_rules! opcode_unimplemented {
    ($message:expr) => {{
        #[cfg(target_arch = "wasm32")]
        {
            // In WASM we don't panic, as it's not possible to catch panics from JS
            return Err(AxError::from(format!(
                "Executed unimplemented opcode: {}",
                $message
            )));
        }

        #[cfg(not(target_arch = "wasm32"))]
        {
            panic!("Executed unimplemented opcode: {}", $message);
        }
    }};
}
