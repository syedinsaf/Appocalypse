use iced::{color, Color};

#[derive(Default, Debug, PartialEq, Eq, Copy, Clone)]
pub enum Theme {
    #[default]
    Lupin,
    Dark,
    Light,
}

#[derive(Debug, Clone, Copy)]
pub struct BaseColors {
    pub background: Color,
    pub foreground: Color,
}

#[derive(Debug, Clone, Copy)]
pub struct NormalColors {
    pub primary: Color,
    pub surface: Color,
    pub error: Color,
}

#[derive(Debug, Clone, Copy)]
pub struct BrightColors {
    pub primary: Color,
    pub secondary: Color,
    pub surface: Color,
    pub error: Color,
}

#[derive(Debug, Clone, Copy)]
pub struct ColorPalette {
    pub base: BaseColors,
    pub normal: NormalColors,
    pub bright: BrightColors,
}

impl Theme {
    pub const ALL: [Self; 3] = [Self::Lupin, Self::Dark, Self::Light];
    
    pub fn palette(self) -> ColorPalette {
        match self {
            Self::Dark => ColorPalette {
                base: BaseColors {
                    background: color!(0x0d1117),    // Deeper, richer black
                    foreground: color!(0x161b22),    // Slightly lighter surface
                },
                normal: NormalColors {
                    primary: color!(0x6e40c9),       // Vibrant purple
                    surface: color!(0x8b949e),       // Better mid-tone gray
                    error: color!(0xda3633),         // Brighter red
                },
                bright: BrightColors {
                    primary: color!(0xa371f7),       // Lighter purple for contrast
                    secondary: color!(0x3fb950),     // Brighter green
                    surface: color!(0xf0f6fc),       // Clean white
                    error: color!(0xff7b72),         // Coral red
                },
            },
            Self::Light => ColorPalette {
                base: BaseColors {
                    background: color!(0xffffff),    // Pure white
                    foreground: color!(0xf6f8fa),    // Subtle gray
                },
                normal: NormalColors {
                    primary: color!(0x0969da),       // Professional blue
                    surface: color!(0x57606a),       // Good contrast gray
                    error: color!(0xcf222e),         // Strong red
                },
                bright: BrightColors {
                    primary: color!(0x0550ae),       // Deeper blue
                    secondary: color!(0x1a7f37),     // Forest green
                    surface: color!(0x24292f),       // Dark text
                    error: color!(0xa40e26),         // Deep red
                },
            },
            Self::Lupin => ColorPalette {
                base: BaseColors {
                    background: color!(0x282a36),    // Classic Dracula background
                    foreground: color!(0x383a59),    // Subtle elevation
                },
                normal: NormalColors {
                    primary: color!(0x6272a4),       // Muted purple-blue
                    surface: color!(0xa2a4a3),       // Neutral gray
                    error: color!(0xff5555),         // Dracula red
                },
                bright: BrightColors {
                    primary: color!(0xbd93f9),       // Iconic Dracula purple
                    secondary: color!(0x50fa7b),     // Dracula green
                    surface: color!(0xf8f8f2),       // Dracula foreground
                    error: color!(0xff6e6e),         // Lighter red accent
                },
            },
        }
    }
}

impl std::fmt::Display for Theme {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Dark => "Dark",
                Self::Light => "Light",
                Self::Lupin => "Lupin",
            }
        )
    }
}