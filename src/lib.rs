/*!
Font introspection, complex text shaping and glyph rendering.

For a comprehensive list of features provided by this crate, please check out
the [readme](https://github.com/dfrg/swash/blob/master/README.md) on GitHub.

# Note

This is a low level library focusing on implementations of OpenType and
various related Unicode specifications for building high quality, high performance
text layout and rendering components with minimal overhead.

If you're looking for something higher level, please stay tuned-- work is in
progress.

# Usage

The primary currency in this crate is the [`FontRef`] struct so you'll want to
start there to learn how to construct and use fonts.

Documentation for [shaping](shape) and [scaling](scale) is provided in
the respective modules.
*/

#![allow(clippy::float_cmp)]
#![allow(clippy::many_single_char_names)]
#![allow(clippy::needless_lifetimes)]
#![allow(clippy::redundant_static_lifetimes)]
#![allow(clippy::too_many_arguments)]

#[cfg(feature = "scale")]
pub use zeno;

#[macro_use]
mod macros;

mod attributes;
mod cache;
mod charmap;
mod feature;
mod font;
mod internal;
mod metrics;
mod palette;
mod setting;
mod strike;
mod string;
mod tag;
mod variation;

#[cfg(feature = "scale")]
pub mod scale;

pub mod shape;
pub mod text;

pub use attributes::*;
pub use cache::CacheKey;
pub use charmap::Charmap;
pub use feature::{Action, Feature, WritingSystem};
pub use font::{FontDataRef, FontRef};
pub use metrics::{GlyphMetrics, Metrics};
pub use palette::{ColorPalette, Usability};
pub use setting::Setting;
pub use strike::BitmapStrike;
pub use string::{LocalizedString, StringId};
pub use tag::{Tag, tag_from_bytes, tag_from_str_lossy};
pub use variation::{Instance, Variation};

/// Collection of various iterators over metadata contained in a font.
pub mod iter {
    pub use super::feature::{Features, WritingSystems};
    pub use super::font::Fonts;
    pub use super::palette::ColorPalettes;
    pub use super::strike::BitmapStrikes;
    pub use super::string::{Chars, LocalizedStrings};
    pub use super::variation::{Instances, Variations};
}

/// Proxies used to efficiently rematerialize metadata.
pub mod proxy {
    pub use super::strike::BitmapStrikesProxy;
    pub use super::charmap::CharmapProxy;
    pub use super::metrics::MetricsProxy;
    pub use super::variation::VariationsProxy;
}

use iter::*;
use proxy::BitmapStrikesProxy;

/// Glyph identifier.
pub type GlyphId = u16;

/// Normalized variation coordinate in 2.14 fixed point format.
pub type NormalizedCoord = i16;

impl<'a> FontRef<'a> {
    /// Returns the primary attributes for the font.
    pub fn attributes(&self) -> Attributes {
        Attributes::from_font(self)
    }

    /// Returns an iterator over the localized strings for the font.
    pub fn localized_strings(&self) -> LocalizedStrings<'a> {
        LocalizedStrings::from_font(self)
    }

    /// Returns an iterator over the variations for the font.
    pub fn variations(&self) -> Variations<'a> {
        Variations::from_font(self)
    }

    /// Returns an iterator over the named instances for the font.
    pub fn instances(&self) -> Instances<'a> {
        Instances::from_font(self)
    }

    /// Returns an iterator over writing systems supported by the font.
    pub fn writing_systems(&self) -> WritingSystems<'a> {
        WritingSystems::from_font(self)
    }

    /// Returns an iterator over the features supported by a font.
    pub fn features(&self) -> Features<'a> {
        Features::from_font(self)
    }

    /// Returns metrics for the font and the specified normalized variation
    /// coordinates.
    pub fn metrics(&self, coords: &'a [NormalizedCoord]) -> Metrics {
        Metrics::from_font(self, coords)
    }

    /// Returns glyph metrics for the font and the specified normalized
    /// variation coordinates.
    pub fn glyph_metrics(&self, coords: &'a [NormalizedCoord]) -> GlyphMetrics<'a> {
        GlyphMetrics::from_font(self, coords)
    }

    /// Returns the character map for the font.
    pub fn charmap(&self) -> Charmap<'a> {
        Charmap::from_font(self)
    }

    /// Returns an iterator over the color palettes for the font.
    pub fn color_palettes(&self) -> ColorPalettes<'a> {
        ColorPalettes::from_font(self)
    }

    /// Returns an iterator over the alpha bitmap strikes for the font.
    pub fn alpha_strikes(&self) -> BitmapStrikes<'a> {
        BitmapStrikesProxy::from_font(self).materialize_alpha(self)
    }

    /// Returns an iterator over the color bitmap strikes for the font.
    pub fn color_strikes(&self) -> BitmapStrikes<'a> {
        BitmapStrikesProxy::from_font(self).materialize_color(self)
    }

    /// Returns the table data for the specified tag.
    pub fn table(&self, tag: Tag) -> Option<&'a [u8]> {
        use internal::RawFont;
        let range = self.table_range(tag)?;
        self.data.get(range.0 as usize..range.1 as usize)
    }
}
