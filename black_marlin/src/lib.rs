//! black_marlin is a simple, small, and extremely fast template engine for Rust.
//! Before reading this reference,
//! I recommend reading [User guide](https://sailfish.netlify.app/en/).
//!
//! This crate contains utilities for rendering black_marlin templates.
//! If you want to use black_marlin templates, import `black_marlin-macros` crate and use
//! derive macro `#[derive(TemplateOnce)]` or `#[derive(Template)]`.
//!
//! In most cases you don't need to care about the `runtime` module in this crate, but
//! if you want to render custom data inside templates, you must implement
//! `runtime::Render` trait for that type.
//!
//! ```ignore
//! use black_marlin::TemplateOnce;
//!
//! #[derive(TemplateOnce)]
//! #[template(path = "hello.stpl")]
//! struct HelloTemplate {
//!     messages: Vec<String>
//! }
//!
//! fn main() {
//!     let ctx = HelloTemplate {
//!         messages: vec!["foo".to_string(), "bar".to_string()]
//!     };
//!
//!     println!("{}", ctx.render_once().unwrap());
//! }
//! ```

#![doc(
    html_logo_url = "https://raw.githubusercontent.com/Kogia-sima/sailfish/master/resources/icon.png"
)]
#![cfg_attr(black_marlin_nightly, feature(core_intrinsics))]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![allow(clippy::redundant_closure)]
#![deny(missing_docs)]

pub mod runtime;

use runtime::Buffer;
pub use runtime::{RenderError, RenderResult};
#[cfg(feature = "derive")]
#[cfg_attr(docsrs, doc(cfg(feature = "derive")))]
pub use black_marlin_macros::TemplateOnce;

/// Template that can be rendered with consuming itself.
pub trait TemplateOnce: Sized + private::Sealed {
    /// Render the template and return the rendering result as `RenderResult`
    ///
    /// This method never returns `Err`, unless you explicitly return RenderError
    /// inside templates
    ///
    /// When you use `render_once` method, total rendered size will be cached, and at
    /// the next time, buffer will be pre-allocated based on the cached length.
    ///
    /// If you don't want this behaviour, you can use `render_once_to` method instead.
    fn render_once(self) -> runtime::RenderResult;

    /// Render the template and append the result to `buf`.
    ///
    /// This method never returns `Err`, unless you explicitly return RenderError
    /// inside templates
    ///
    /// ```
    /// use black_marlin::TemplateOnce;
    /// use black_marlin::runtime::Buffer;
    ///
    /// # pub struct HelloTemplate {
    /// #   messages: Vec<String>,
    /// # }
    /// #
    /// # impl TemplateOnce for HelloTemplate {
    /// #     fn render_once(self) -> Result<String, black_marlin::RenderError> {
    /// #         Ok(String::new())
    /// #     }
    /// #
    /// #     fn render_once_to(self, buf: &mut Buffer)
    /// #             -> Result<(), black_marlin::RenderError> {
    /// #         Ok(())
    /// #     }
    /// # }
    /// #
    /// # impl black_marlin::private::Sealed for HelloTemplate {}
    /// #
    /// let tpl = HelloTemplate {
    ///     messages: vec!["foo".to_string()]
    /// };
    ///
    /// // custom pre-allocation
    /// let mut buffer = Buffer::with_capacity(100);
    /// tpl.render_once_to(&mut buffer).unwrap();
    /// ```
    fn render_once_to(self, buf: &mut Buffer) -> Result<(), RenderError>;
}

/// Work in Progress
pub trait Template: private::Sealed {
    /// Work in progress
    fn render(&self) -> runtime::RenderResult;
}

#[doc(hidden)]
pub mod private {
    pub trait Sealed {}
}
