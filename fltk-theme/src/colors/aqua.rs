#![allow(unused_imports)]
#![allow(non_upper_case_globals)]

#[cfg(target_os = "macos")]
use cocoa_colors::*;

#[cfg(target_os = "macos")]
fn convert_colors(colors: (f64, f64, f64, f64)) -> (u8, u8, u8, u8) {
    let r = (colors.0 * 255.0) as u8;
    let g = (colors.1 * 255.0) as u8;
    let b = (colors.2 * 255.0) as u8;
    let a = (colors.3 * 255.0) as u8;
    (r, g, b, a)
}

#[cfg(target_os = "macos")]
macro_rules! get_colors {
    ($s:ident) => {{
        let mut r = 1.0;
        let mut g = 1.0;
        let mut b = 1.0;
        let mut a = 1.0;
        unsafe {
            $s(&mut r, &mut g, &mut b, &mut a);
        }
        convert_colors((r, g, b, a))
    }};
}

#[cfg(target_os = "macos")]
pub mod sys {
    use super::*;
    lazy_static::lazy_static! {
        pub static ref windowBackgroundColor: (u8, u8, u8, u8) = get_colors!(get_windowBackgroundColor);
        pub static ref labelColor: (u8, u8, u8, u8) = get_colors!(get_labelColor);
        pub static ref controlBackgroundColor: (u8, u8, u8, u8) = get_colors!(get_controlBackgroundColor);
        pub static ref secondaryLabelColor: (u8, u8, u8, u8) = get_colors!(get_secondaryLabelColor);
        pub static ref tertiaryLabelColor: (u8, u8, u8, u8) = get_colors!(get_tertiaryLabelColor);
        pub static ref quaternaryLabelColor: (u8, u8, u8, u8) = get_colors!(get_quaternaryLabelColor);
        pub static ref textColor: (u8, u8, u8, u8) = get_colors!(get_textColor);
        pub static ref placeholderTextColor: (u8, u8, u8, u8) = get_colors!(get_placeholderTextColor);
        pub static ref selectedTextColor: (u8, u8, u8, u8) = get_colors!(get_selectedTextColor);
        pub static ref textBackgroundColor: (u8, u8, u8, u8) = get_colors!(get_textBackgroundColor);
        pub static ref selectedTextBackgroundColor: (u8, u8, u8, u8) = get_colors!(get_selectedTextBackgroundColor);
        pub static ref keyboardFocusIndicatorColor: (u8, u8, u8, u8) = get_colors!(get_keyboardFocusIndicatorColor);
        pub static ref unemphasizedSelectedTextColor: (u8, u8, u8, u8) = get_colors!(get_unemphasizedSelectedTextColor);
        pub static ref unemphasizedSelectedTextBackgroundColor: (u8, u8, u8, u8) = get_colors!(get_unemphasizedSelectedTextBackgroundColor);
        pub static ref linkColor: (u8, u8, u8, u8) = get_colors!(get_linkColor);
        pub static ref separatorColor: (u8, u8, u8, u8) = get_colors!(get_separatorColor);
        pub static ref selectedContentBackgroundColor: (u8, u8, u8, u8) = get_colors!(get_selectedContentBackgroundColor);
        pub static ref unemphasizedSelectedContentBackgroundColor: (u8, u8, u8, u8) = get_colors!(get_unemphasizedSelectedContentBackgroundColor);
        pub static ref selectedMenuItemTextColor: (u8, u8, u8, u8) = get_colors!(get_selectedMenuItemTextColor);
        pub static ref gridColor: (u8, u8, u8, u8) = get_colors!(get_gridColor);
        pub static ref headerTextColor: (u8, u8, u8, u8) = get_colors!(get_headerTextColor);
        pub static ref controlAccentColor: (u8, u8, u8, u8) = get_colors!(get_controlAccentColor);
        pub static ref controlColor: (u8, u8, u8, u8) = get_colors!(get_controlColor);
        pub static ref controlTextColor: (u8, u8, u8, u8) = get_colors!(get_controlTextColor);
        pub static ref disabledControlTextColor: (u8, u8, u8, u8) = get_colors!(get_disabledControlTextColor);
        pub static ref selectedControlColor: (u8, u8, u8, u8) = get_colors!(get_selectedControlColor);
        pub static ref selectedControlTextColor: (u8, u8, u8, u8) = get_colors!(get_selectedControlTextColor);
        pub static ref alternateSelectedControlTextColor: (u8, u8, u8, u8) = get_colors!(get_alternateSelectedControlTextColor);
        pub static ref scrubberTexturedBackgroundColor: (u8, u8, u8, u8) = get_colors!(get_scrubberTexturedBackgroundColor);
        pub static ref windowFrameTextColor: (u8, u8, u8, u8) = get_colors!(get_windowFrameTextColor);
        pub static ref underPageBackgroundColor: (u8, u8, u8, u8) = get_colors!(get_underPageBackgroundColor);
        pub static ref findHighlightColor: (u8, u8, u8, u8) = get_colors!(get_findHighlightColor);
        pub static ref highlightColor: (u8, u8, u8, u8) = get_colors!(get_highlightColor);
        pub static ref shadowColor: (u8, u8, u8, u8) = get_colors!(get_shadowColor);
        pub static ref systemBrownColor: (u8, u8, u8, u8) = get_colors!(get_systemBrownColor);
        pub static ref systemGrayColor: (u8, u8, u8, u8) = get_colors!(get_systemGrayColor);
        pub static ref systemGreenColor: (u8, u8, u8, u8) = get_colors!(get_systemGreenColor);
        pub static ref systemIndigoColor: (u8, u8, u8, u8) = get_colors!(get_systemIndigoColor);
        pub static ref systemOrangeColor: (u8, u8, u8, u8) = get_colors!(get_systemOrangeColor);
        pub static ref systemPinkColor: (u8, u8, u8, u8) = get_colors!(get_systemPinkColor);
        pub static ref systemPurpleColor: (u8, u8, u8, u8) = get_colors!(get_systemPurpleColor);
        pub static ref systemRedColor: (u8, u8, u8, u8) = get_colors!(get_systemRedColor);
        pub static ref systemTealColor: (u8, u8, u8, u8) = get_colors!(get_systemTealColor);
        pub static ref systemYellowColor: (u8, u8, u8, u8) = get_colors!(get_systemYellowColor);
        pub static ref systemBlueColor: (u8, u8, u8, u8) = get_colors!(get_systemBlueColor);
        // pub static ref systemCyanColor: (u8, u8, u8, u8) = get_colors!(get_systemCyanColor); // beta
    }
}

pub mod dark {
    lazy_static::lazy_static! {
        pub static ref backgroundColor2: (u8, u8, u8, u8) = (0, 0, 0, 255);
        pub static ref windowBackgroundColor: (u8, u8, u8, u8) = (37, 37, 37, 255);
        pub static ref labelColor: (u8, u8, u8, u8) = (255, 254, 254, 216);
        pub static ref controlBackgroundColor: (u8, u8, u8, u8) = (22, 22, 22, 255);
        pub static ref secondaryLabelColor: (u8, u8, u8, u8) = (255, 254, 254, 140);
        pub static ref tertiaryLabelColor: (u8, u8, u8, u8) = (255, 254, 254, 63);
        pub static ref quaternaryLabelColor: (u8, u8, u8, u8) = (255, 254, 254, 25);
        pub static ref textColor: (u8, u8, u8, u8) = (255, 254, 254, 255);
        pub static ref placeholderTextColor: (u8, u8, u8, u8) = (255, 254, 254, 63);
        pub static ref selectedTextColor: (u8, u8, u8, u8) = (255, 255, 255, 255);
        pub static ref textBackgroundColor: (u8, u8, u8, u8) = (22, 22, 22, 255);
        pub static ref selectedTextBackgroundColor: (u8, u8, u8, u8) = (48, 79, 120, 255);
        pub static ref keyboardFocusIndicatorColor: (u8, u8, u8, u8) = (27, 149, 254, 76);
        pub static ref unemphasizedSelectedTextColor: (u8, u8, u8, u8) = (255, 255, 255, 255);
        pub static ref unemphasizedSelectedTextBackgroundColor: (u8, u8, u8, u8) = (54, 54, 54, 255);
        pub static ref linkColor: (u8, u8, u8, u8) = (52, 134, 254, 255);
        pub static ref separatorColor: (u8, u8, u8, u8) = (255, 254, 254, 25);
        pub static ref selectedContentBackgroundColor: (u8, u8, u8, u8) = (5, 63, 197, 255);
        pub static ref unemphasizedSelectedContentBackgroundColor: (u8, u8, u8, u8) = (54, 54, 54, 255);
        pub static ref selectedMenuItemTextColor: (u8, u8, u8, u8) = (255, 254, 254, 255);
        pub static ref gridColor: (u8, u8, u8, u8) = (20, 20, 20, 255);
        pub static ref headerTextColor: (u8, u8, u8, u8) = (255, 254, 254, 255);
        pub static ref controlAccentColor: (u8, u8, u8, u8) = (10, 95, 254, 255);
        pub static ref controlColor: (u8, u8, u8, u8) = (255, 254, 254, 63);
        pub static ref controlTextColor: (u8, u8, u8, u8) = (255, 254, 254, 216);
        pub static ref disabledControlTextColor: (u8, u8, u8, u8) = (255, 254, 254, 63);
        pub static ref selectedControlColor: (u8, u8, u8, u8) = (48, 79, 120, 255);
        pub static ref selectedControlTextColor: (u8, u8, u8, u8) = (255, 254, 254, 216);
        pub static ref alternateSelectedControlTextColor: (u8, u8, u8, u8) = (255, 254, 254, 255);
        pub static ref scrubberTexturedBackgroundColor: (u8, u8, u8, u8) = (255, 254, 254, 255);
        pub static ref windowFrameTextColor: (u8, u8, u8, u8) = (255, 254, 254, 216);
        pub static ref underPageBackgroundColor: (u8, u8, u8, u8) = (29, 29, 29, 255);
        pub static ref findHighlightColor: (u8, u8, u8, u8) = (255, 255, 10, 255);
        pub static ref highlightColor: (u8, u8, u8, u8) = (164, 164, 164, 255);
        pub static ref shadowColor: (u8, u8, u8, u8) = (0, 0, 0, 255);
        pub static ref systemBrownColor  : (u8, u8, u8, u8) = (155, 123, 85, 255);
        pub static ref systemGrayColor : (u8, u8, u8, u8) = (133, 133, 139, 255);
        pub static ref systemGreenColor  : (u8, u8, u8, u8) = (48, 211, 58, 255);
        pub static ref systemIndigoColor : (u8, u8, u8, u8) = (74, 64, 223, 255);
        pub static ref systemOrangeColor : (u8, u8, u8, u8) = (252, 141, 13, 255);
        pub static ref systemPinkColor  : (u8, u8, u8, u8) = (251, 25, 76, 255);
        pub static ref systemPurpleColor : (u8, u8, u8, u8) = (175, 56, 238, 255);
        pub static ref systemRedColor : (u8, u8, u8, u8) = (251, 43, 44, 255);
        pub static ref systemTealColor : (u8, u8, u8, u8) = (76, 187, 242, 255);
        pub static ref systemYellowColor : (u8, u8, u8, u8) = (254, 207, 14, 255);
        pub static ref systemBlueColor : (u8, u8, u8, u8) = (16, 106, 254, 255);
        pub static ref systemCyanColor: (u8, u8, u8, u8) = (90, 200 , 245, 255);
    }
}

pub mod light {
    lazy_static::lazy_static! {
       pub static ref backgroundColor2: (u8, u8, u8, u8) = (255, 255, 255, 255);
       pub static ref windowBackgroundColor: (u8, u8, u8, u8) = (231, 231, 231, 255);
       pub static ref labelColor: (u8, u8, u8, u8) = (0, 0, 0, 216);
       pub static ref controlBackgroundColor: (u8, u8, u8, u8) = (255, 254, 254, 255);
       pub static ref secondaryLabelColor: (u8, u8, u8, u8) = (0, 0, 0, 127);
       pub static ref tertiaryLabelColor: (u8, u8, u8, u8) = (0, 0, 0, 66);
       pub static ref quaternaryLabelColor: (u8, u8, u8, u8) = (0, 0, 0, 25);
       pub static ref textColor: (u8, u8, u8, u8) = (0, 0, 0, 255);
       pub static ref placeholderTextColor: (u8, u8, u8, u8) = (0, 0, 0, 63);
       pub static ref selectedTextColor: (u8, u8, u8, u8) = (0, 0, 0, 255);
       pub static ref textBackgroundColor: (u8, u8, u8, u8) = (255, 254, 254, 255);
       pub static ref selectedTextBackgroundColor: (u8, u8, u8, u8) = (164, 204, 254, 255);
       pub static ref keyboardFocusIndicatorColor: (u8, u8, u8, u8) = (7, 75, 240, 63);
       pub static ref unemphasizedSelectedTextColor: (u8, u8, u8, u8) = (0, 0, 0, 255);
       pub static ref unemphasizedSelectedTextBackgroundColor: (u8, u8, u8, u8) = (211, 211, 211, 255);
       pub static ref linkColor: (u8, u8, u8, u8) = (8, 79, 209, 255);
       pub static ref separatorColor: (u8, u8, u8, u8) = (0, 0, 0, 25);
       pub static ref selectedContentBackgroundColor: (u8, u8, u8, u8) = (7, 73, 217, 255);
       pub static ref unemphasizedSelectedContentBackgroundColor: (u8, u8, u8, u8) = (211, 211, 211, 255);
       pub static ref selectedMenuItemTextColor: (u8, u8, u8, u8) = (255, 254, 254, 255);
       pub static ref gridColor: (u8, u8, u8, u8) = (223, 223, 223, 255);
       pub static ref headerTextColor: (u8, u8, u8, u8) = (0, 0, 0, 216);
       pub static ref controlAccentColor: (u8, u8, u8, u8) = (10, 95, 254, 255);
       pub static ref controlColor: (u8, u8, u8, u8) = (255, 254, 254, 255);
       pub static ref controlTextColor: (u8, u8, u8, u8) = (0, 0, 0, 216);
       pub static ref disabledControlTextColor: (u8, u8, u8, u8) = (0, 0, 0, 63);
       pub static ref selectedControlColor: (u8, u8, u8, u8) = (164, 204, 254, 255);
       pub static ref selectedControlTextColor: (u8, u8, u8, u8) = (0, 0, 0, 216);
       pub static ref alternateSelectedControlTextColor: (u8, u8, u8, u8) = (255, 254, 254, 255);
       pub static ref scrubberTexturedBackgroundColor: (u8, u8, u8, u8) = (255, 254, 254, 255);
       pub static ref windowFrameTextColor: (u8, u8, u8, u8) = (0, 0, 0, 216);
       pub static ref underPageBackgroundColor: (u8, u8, u8, u8) = (131, 131, 131, 229);
       pub static ref findHighlightColor: (u8, u8, u8, u8) = (255, 255, 10, 255);
       pub static ref highlightColor: (u8, u8, u8, u8) = (255, 254, 254, 255);
       pub static ref shadowColor: (u8, u8, u8, u8) = (0, 0, 0, 255);
       pub static ref systemBrownColor : (u8, u8, u8, u8)  = (144, 113, 75, 255);
       pub static ref systemGrayColor: (u8, u8, u8, u8)  = (123, 123, 128, 255);
       pub static ref systemGreenColor : (u8, u8, u8, u8)  = (40, 199, 50, 255);
       pub static ref systemIndigoColor: (u8, u8, u8, u8)  = (69, 59, 204, 255);
       pub static ref systemOrangeColor: (u8, u8, u8, u8)  = (252, 129, 8, 255);
       pub static ref systemPinkColor : (u8, u8, u8, u8)  = (251, 12, 67, 255);
       pub static ref systemPurpleColor: (u8, u8, u8, u8)  = (157, 51, 213, 255);
       pub static ref systemRedColor: (u8, u8, u8, u8)  = (251, 32, 37, 255);
       pub static ref systemTealColor: (u8, u8, u8, u8)  = (71, 175, 235, 255);
       pub static ref systemYellowColor: (u8, u8, u8, u8)  = (253, 194, 9, 255);
       pub static ref systemBlueColor : (u8, u8, u8, u8) = (10, 95, 254, 255);
       pub static ref systemCyanColor: (u8, u8, u8, u8) = (85, 190 , 240, 255);
    }
}
