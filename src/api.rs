//! Public Rust API for librsvg.
//!
//! This gets re-exported from the toplevel `lib.rs`.

#![warn(missing_docs)]

pub use crate::{
    accept_language::{AcceptLanguage, Language, UserLanguage},
    error::{ImplementationLimit, LoadingError, RenderingError},
    length::{LengthUnit, RsvgLength as Length},
};

use url::Url;

use std::path::Path;

use gio::prelude::*; // Re-exposes glib's prelude as well
use gio::Cancellable;

use crate::{
    dpi::Dpi,
    handle::{Handle, LoadOptions},
    url_resolver::UrlResolver,
};

/// Builder for loading an [`SvgHandle`].
///
/// This is the starting point for using librsvg.  This struct
/// implements a builder pattern for configuring an [`SvgHandle`]'s
/// options, and then loading the SVG data.  You can call the methods
/// of `Loader` in sequence to configure how SVG data should be
/// loaded, and finally use one of the loading functions to load an
/// [`SvgHandle`].
#[derive(Default)]
pub struct Loader {
    unlimited_size: bool,
    keep_image_data: bool,
}

impl Loader {
    /// Creates a `Loader` with the default flags.
    ///
    /// * [`unlimited_size`](#method.with_unlimited_size) defaults to `false`, as malicious
    /// SVG documents could cause the XML parser to consume very large amounts of memory.
    ///
    /// * [`keep_image_data`](#method.keep_image_data) defaults to
    /// `false`.  You may only need this if rendering to Cairo
    /// surfaces that support including image data in compressed
    /// formats, like PDF.
    ///
    /// # Example:
    ///
    /// ```
    /// use librsvg;
    ///
    /// let svg_handle = librsvg::Loader::new()
    ///     .read_path("example.svg")
    ///     .unwrap();
    /// ```
    pub fn new() -> Self {
        Self::default()
    }

    /// Controls safety limits used in the XML parser.
    ///
    /// Internally, librsvg uses libxml2, which has set limits for things like the
    /// maximum length of XML element names, the size of accumulated buffers
    /// using during parsing of deeply-nested XML files, and the maximum size
    /// of embedded XML entities.
    ///
    /// Set this to `true` only if loading a trusted SVG fails due to size limits.
    ///
    /// # Example:
    /// ```
    /// use librsvg;
    ///
    /// let svg_handle = librsvg::Loader::new()
    ///     .with_unlimited_size(true)
    ///     .read_path("example.svg")    // presumably a trusted huge file
    ///     .unwrap();
    /// ```
    pub fn with_unlimited_size(mut self, unlimited: bool) -> Self {
        self.unlimited_size = unlimited;
        self
    }

    /// Controls embedding of compressed image data into the renderer.
    ///
    /// Normally, Cairo expects one to pass it uncompressed (decoded)
    /// images as surfaces.  However, when using a PDF rendering
    /// context to render SVG documents that reference raster images
    /// (e.g. those which include a bitmap as part of the SVG image),
    /// it may be more efficient to embed the original, compressed raster
    /// images into the PDF.
    ///
    /// Set this to `true` if you are using a Cairo PDF context, or any other type
    /// of context which allows embedding compressed images.
    ///
    /// # Example:
    ///
    /// ```
    /// # use std::env;
    /// let svg_handle = librsvg::Loader::new()
    ///     .keep_image_data(true)
    ///     .read_path("example.svg")
    ///     .unwrap();
    ///
    /// let mut output = env::temp_dir();
    /// output.push("output.pdf");
    /// let surface = cairo::PdfSurface::new(640.0, 480.0, output)?;
    /// let cr = cairo::Context::new(&surface).expect("Failed to create a cairo context");
    ///
    /// let renderer = librsvg::CairoRenderer::new(&svg_handle);
    /// renderer.render_document(
    ///     &cr,
    ///     &cairo::Rectangle { x: 0.0, y: 0.0, width: 640.0, height: 480.0 },
    /// )?;
    /// # Ok::<(), librsvg::RenderingError>(())
    /// ```
    pub fn keep_image_data(mut self, keep: bool) -> Self {
        self.keep_image_data = keep;
        self
    }

    /// Reads an SVG document from `path`.
    ///
    /// # Example:
    ///
    /// ```
    /// let svg_handle = librsvg::Loader::new()
    ///     .read_path("example.svg")
    ///     .unwrap();
    /// ```
    pub fn read_path<P: AsRef<Path>>(self, path: P) -> Result<SvgHandle, LoadingError> {
        let file = gio::File::for_path(path);
        self.read_file(&file, None::<&Cancellable>)
    }

    /// Reads an SVG document from a `gio::File`.
    ///
    /// The `cancellable` can be used to cancel loading from another thread.
    ///
    /// # Example:
    /// ```
    /// let svg_handle = librsvg::Loader::new()
    ///     .read_file(&gio::File::for_path("example.svg"), None::<&gio::Cancellable>)
    ///     .unwrap();
    /// ```
    pub fn read_file<F: IsA<gio::File>, P: IsA<Cancellable>>(
        self,
        file: &F,
        cancellable: Option<&P>,
    ) -> Result<SvgHandle, LoadingError> {
        let stream = file.read(cancellable)?;
        self.read_stream(&stream, Some(file), cancellable)
    }

    /// Reads an SVG stream from a `gio::InputStream`.
    ///
    /// The `base_file`, if it is not `None`, is used to extract the
    /// [base URL][crate#the-base-file-and-resolving-references-to-external-files] for this stream.
    ///
    /// Reading an SVG document may involve resolving relative URLs if the
    /// SVG references things like raster images, or other SVG files.
    /// In this case, pass the `base_file` that correspondds to the
    /// URL where this SVG got loaded from.
    ///
    /// The `cancellable` can be used to cancel loading from another thread.
    ///
    /// # Example
    ///
    /// ```
    /// use gio::prelude::*;
    ///
    /// let file = gio::File::for_path("example.svg");
    ///
    /// let stream = file.read(None::<&gio::Cancellable>).unwrap();
    ///
    /// let svg_handle = librsvg::Loader::new()
    ///     .read_stream(&stream, Some(&file), None::<&gio::Cancellable>)
    ///     .unwrap();
    /// ```
    pub fn read_stream<S: IsA<gio::InputStream>, F: IsA<gio::File>, P: IsA<Cancellable>>(
        self,
        stream: &S,
        base_file: Option<&F>,
        cancellable: Option<&P>,
    ) -> Result<SvgHandle, LoadingError> {
        let base_file = base_file.map(|f| f.as_ref());

        let base_url = if let Some(base_file) = base_file {
            Some(url_from_file(base_file)?)
        } else {
            None
        };

        let load_options = LoadOptions::new(UrlResolver::new(base_url))
            .with_unlimited_size(self.unlimited_size)
            .keep_image_data(self.keep_image_data);

        Ok(SvgHandle(Handle::from_stream(
            &load_options,
            stream.as_ref(),
            cancellable.map(|c| c.as_ref()),
        )?))
    }
}

fn url_from_file(file: &gio::File) -> Result<Url, LoadingError> {
    Url::parse(&file.uri()).map_err(|_| LoadingError::BadUrl)
}

/// Handle used to hold SVG data in memory.
///
/// You can create this from one of the `read` methods in
/// [`Loader`].
pub struct SvgHandle(pub(crate) Handle);

impl SvgHandle {
    /// Checks if the SVG has an element with the specified `id`.
    ///
    /// Note that the `id` must be a plain fragment identifier like `#foo`, with
    /// a leading `#` character.
    ///
    /// The purpose of the `Err()` case in the return value is to indicate an
    /// incorrectly-formatted `id` argument.
    pub fn has_element_with_id(&self, id: &str) -> Result<bool, RenderingError> {
        self.0.has_sub(id)
    }

    /// Sets a CSS stylesheet to use for an SVG document.
    ///
    /// During the CSS cascade, the specified stylesheet will be used
    /// with a "User" [origin].
    ///
    /// Note that `@import` rules will not be resolved, except for `data:` URLs.
    ///
    /// [origin]: https://drafts.csswg.org/css-cascade-3/#cascading-origins
    pub fn set_stylesheet(&mut self, css: &str) -> Result<(), LoadingError> {
        self.0.set_stylesheet(css)
    }
}

/// Can render an `SvgHandle` to a Cairo context.
pub struct CairoRenderer<'a> {
    pub(crate) handle: &'a SvgHandle,
    pub(crate) dpi: Dpi,
    user_language: UserLanguage,
    is_testing: bool,
}

// Note that these are different than the C API's default, which is 90.
const DEFAULT_DPI_X: f64 = 96.0;
const DEFAULT_DPI_Y: f64 = 96.0;

#[derive(Debug, Copy, Clone, PartialEq)]
/// Contains the computed values of the `<svg>` element's `width`, `height`, and `viewBox`.
///
/// An SVG document has a toplevel `<svg>` element, with optional attributes `width`,
/// `height`, and `viewBox`.  This structure contains the values for those attributes; you
/// can obtain the struct from [`CairoRenderer::intrinsic_dimensions`].
///
/// Since librsvg 2.54.0, there is support for [geometry
/// properties](https://www.w3.org/TR/SVG2/geometry.html) from SVG2.  This means that
/// `width` and `height` are no longer attributes; they are instead CSS properties that
/// default to `auto`.  The computed value for `auto` is `100%`, so for a `<svg>` that
/// does not have these attributes/properties, the `width`/`height` fields will be
/// returned as a [`Length`] of 100%.
///
/// As an example, the following SVG element has a `width` of 100 pixels
/// and a `height` of 400 pixels, but no `viewBox`.
///
/// ```xml
/// <svg xmlns="http://www.w3.org/2000/svg" width="100" height="400">
/// ```
///
/// In this case, the length fields will be set to the corresponding
/// values with [`LengthUnit::Px`] units, and the `vbox` field will be
/// set to to `None`.
pub struct IntrinsicDimensions {
    /// Computed value of the `width` property of the `<svg>`.
    pub width: Length,

    /// Computed value of the `height` property of the `<svg>`.
    pub height: Length,

    /// `viewBox` attribute of the `<svg>`, if present.
    pub vbox: Option<cairo::Rectangle>,
}

impl<'a> CairoRenderer<'a> {
    /// Creates a `CairoRenderer` for the specified `SvgHandle`.
    ///
    /// The default dots-per-inch (DPI) value is set to 96; you can change it
    /// with the [`with_dpi`] method.
    ///
    /// [`with_dpi`]: #method.with_dpi
    pub fn new(handle: &'a SvgHandle) -> Self {
        CairoRenderer {
            handle,
            dpi: Dpi::new(DEFAULT_DPI_X, DEFAULT_DPI_Y),
            user_language: UserLanguage::new(&Language::FromEnvironment),
            is_testing: false,
        }
    }

    /// Configures the dots-per-inch for resolving physical lengths.
    ///
    /// If an SVG document has physical units like `5cm`, they must be resolved
    /// to pixel-based values.  The default pixel density is 96 DPI in
    /// both dimensions.
    pub fn with_dpi(self, dpi_x: f64, dpi_y: f64) -> Self {
        assert!(dpi_x > 0.0);
        assert!(dpi_y > 0.0);

        CairoRenderer {
            dpi: Dpi::new(dpi_x, dpi_y),
            ..self
        }
    }

    /// Configures the set of languages used for rendering.
    ///
    /// SVG documents can use the `<switch>` element, whose children have a
    /// `systemLanguage` attribute; only the first child which has a `systemLanguage` that
    /// matches the preferred languages will be rendered.
    ///
    /// This function sets the preferred languages.  The default is
    /// `Language::FromEnvironment`, which means that the set of preferred languages will
    /// be obtained from the program's environment.  To set an explicit list of languages,
    /// you can use `Language::AcceptLanguage` instead.
    pub fn with_language(self, language: &Language) -> Self {
        let user_language = UserLanguage::new(language);

        CairoRenderer {
            user_language,
            ..self
        }
    }

    /// Queries the `width`, `height`, and `viewBox` attributes in an SVG document.
    ///
    /// If you are calling this function to compute a scaling factor to render the SVG,
    /// consider simply using [`render_document`] instead; it will do the scaling
    /// computations automatically.
    ///
    /// See also [`intrinsic_size_in_pixels`], which does the conversion to pixels if
    /// possible.
    ///
    /// [`render_document`]: #method.render_document
    /// [`intrinsic_size_in_pixels`]: #method.intrinsic_size_in_pixels
    pub fn intrinsic_dimensions(&self) -> IntrinsicDimensions {
        let d = self.handle.0.get_intrinsic_dimensions();

        IntrinsicDimensions {
            width: Into::into(d.width),
            height: Into::into(d.height),
            vbox: d.vbox.map(|v| cairo::Rectangle::from(*v)),
        }
    }

    /// Converts the SVG document's intrinsic dimensions to pixels, if possible.
    ///
    /// Returns `Some(width, height)` in pixel units if the SVG document has `width` and
    /// `height` attributes with physical dimensions (CSS pixels, cm, in, etc.) or
    /// font-based dimensions (em, ex).
    ///
    /// Note that the dimensions are floating-point numbers, so your application can know
    /// the exact size of an SVG document.  To get integer dimensions, you should use
    /// [`f64::ceil()`] to round up to the nearest integer (just using [`f64::round()`],
    /// may may chop off pixels with fractional coverage).
    ///
    /// If the SVG document has percentage-based `width` and `height` attributes, or if
    /// either of those attributes are not present, returns `None`.  Dimensions of that
    /// kind require more information to be resolved to pixels; for example, the calling
    /// application can use a viewport size to scale percentage-based dimensions.
    pub fn intrinsic_size_in_pixels(&self) -> Option<(f64, f64)> {
        let dim = self.intrinsic_dimensions();
        let width = dim.width;
        let height = dim.height;

        if width.unit == LengthUnit::Percent || height.unit == LengthUnit::Percent {
            return None;
        }

        Some(self.handle.0.width_height_to_user(self.dpi))
    }

    /// Renders the whole SVG document fitted to a viewport
    ///
    /// The `viewport` gives the position and size at which the whole SVG
    /// document will be rendered.
    ///
    /// The `cr` must be in a `cairo::Status::Success` state, or this function
    /// will not render anything, and instead will return
    /// `RenderingError::Cairo` with the `cr`'s current error state.
    pub fn render_document(
        &self,
        cr: &cairo::Context,
        viewport: &cairo::Rectangle,
    ) -> Result<(), RenderingError> {
        self.handle
            .0
            .render_document(cr, viewport, &self.user_language, self.dpi, self.is_testing)
    }

    /// Computes the (ink_rect, logical_rect) of an SVG element, as if
    /// the SVG were rendered to a specific viewport.
    ///
    /// Element IDs should look like an URL fragment identifier; for
    /// example, pass `Some("#foo")` to get the geometry of the
    /// element that has an `id="foo"` attribute.
    ///
    /// The "ink rectangle" is the bounding box that would be painted
    /// for fully- stroked and filled elements.
    ///
    /// The "logical rectangle" just takes into account the unstroked
    /// paths and text outlines.
    ///
    /// Note that these bounds are not minimum bounds; for example,
    /// clipping paths are not taken into account.
    ///
    /// You can pass `None` for the `id` if you want to measure all
    /// the elements in the SVG, i.e. to measure everything from the
    /// root element.
    ///
    /// This operation is not constant-time, as it involves going through all
    /// the child elements.
    ///
    /// FIXME: example
    pub fn geometry_for_layer(
        &self,
        id: Option<&str>,
        viewport: &cairo::Rectangle,
    ) -> Result<(cairo::Rectangle, cairo::Rectangle), RenderingError> {
        self.handle
            .0
            .get_geometry_for_layer(id, viewport, &self.user_language, self.dpi, self.is_testing)
            .map(|(i, l)| (i, l))
    }

    /// Renders a single SVG element in the same place as for a whole SVG document
    ///
    /// This is equivalent to `render_document`, but renders only a single element and its
    /// children, as if they composed an individual layer in the SVG.  The element is
    /// rendered with the same transformation matrix as it has within the whole SVG
    /// document.  Applications can use this to re-render a single element and repaint it
    /// on top of a previously-rendered document, for example.
    ///
    /// Note that the `id` must be a plain fragment identifier like `#foo`, with
    /// a leading `#` character.
    ///
    /// The `viewport` gives the position and size at which the whole SVG
    /// document would be rendered.  This function will effectively place the
    /// whole SVG within that viewport, but only render the element given by
    /// `id`.
    ///
    /// The `cr` must be in a `cairo::Status::Success` state, or this function
    /// will not render anything, and instead will return
    /// `RenderingError::Cairo` with the `cr`'s current error state.
    pub fn render_layer(
        &self,
        cr: &cairo::Context,
        id: Option<&str>,
        viewport: &cairo::Rectangle,
    ) -> Result<(), RenderingError> {
        self.handle.0.render_layer(
            cr,
            id,
            viewport,
            &self.user_language,
            self.dpi,
            self.is_testing,
        )
    }

    /// Computes the (ink_rect, logical_rect) of a single SVG element
    ///
    /// While `geometry_for_layer` computes the geometry of an SVG element subtree with
    /// its transformation matrix, this other function will compute the element's geometry
    /// as if it were being rendered under an identity transformation by itself.  That is,
    /// the resulting geometry is as if the element got extracted by itself from the SVG.
    ///
    /// This function is the counterpart to `render_element`.
    ///
    /// Element IDs should look like an URL fragment identifier; for
    /// example, pass `Some("#foo")` to get the geometry of the
    /// element that has an `id="foo"` attribute.
    ///
    /// The "ink rectangle" is the bounding box that would be painted
    /// for fully- stroked and filled elements.
    ///
    /// The "logical rectangle" just takes into account the unstroked
    /// paths and text outlines.
    ///
    /// Note that these bounds are not minimum bounds; for example,
    /// clipping paths are not taken into account.
    ///
    /// You can pass `None` for the `id` if you want to measure all
    /// the elements in the SVG, i.e. to measure everything from the
    /// root element.
    ///
    /// This operation is not constant-time, as it involves going through all
    /// the child elements.
    ///
    /// FIXME: example
    pub fn geometry_for_element(
        &self,
        id: Option<&str>,
    ) -> Result<(cairo::Rectangle, cairo::Rectangle), RenderingError> {
        self.handle
            .0
            .get_geometry_for_element(id, &self.user_language, self.dpi, self.is_testing)
            .map(|(i, l)| (i, l))
    }

    /// Renders a single SVG element to a given viewport
    ///
    /// This function can be used to extract individual element subtrees and render them,
    /// scaled to a given `element_viewport`.  This is useful for applications which have
    /// reusable objects in an SVG and want to render them individually; for example, an
    /// SVG full of icons that are meant to be be rendered independently of each other.
    ///
    /// Note that the `id` must be a plain fragment identifier like `#foo`, with
    /// a leading `#` character.
    ///
    /// The `element_viewport` gives the position and size at which the named element will
    /// be rendered.  FIXME: mention proportional scaling.
    ///
    /// The `cr` must be in a `cairo::Status::Success` state, or this function
    /// will not render anything, and instead will return
    /// `RenderingError::Cairo` with the `cr`'s current error state.
    pub fn render_element(
        &self,
        cr: &cairo::Context,
        id: Option<&str>,
        element_viewport: &cairo::Rectangle,
    ) -> Result<(), RenderingError> {
        self.handle.0.render_element(
            cr,
            id,
            element_viewport,
            &self.user_language,
            self.dpi,
            self.is_testing,
        )
    }

    /// Turns on test mode.  Do not use this function; it is for librsvg's test suite only.
    pub fn test_mode(self, is_testing: bool) -> Self {
        CairoRenderer { is_testing, ..self }
    }
}
