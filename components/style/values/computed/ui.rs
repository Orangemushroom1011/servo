/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

//! Computed values for UI properties

use values::{Auto, Either};
use values::computed::Number;
use values::computed::color::Color;
use values::computed::url::ComputedImageUrl;
use values::generics::ui as generics;

pub use values::specified::ui::MozForceBrokenImageIcon;

/// auto | <color>
pub type ColorOrAuto = Either<Color, Auto>;

/// A computed value for the `cursor` property.
pub type Cursor = generics::Cursor<CursorImage>;

/// A computed value for item of `image cursors`.
pub type CursorImage = generics::CursorImage<ComputedImageUrl, Number>;

/// A computed value for `scrollbar-color` property.
pub type ScrollbarColor = generics::ScrollbarColor<Color>;
