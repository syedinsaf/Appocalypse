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
                    background: color!(0x0d1117),    
                    foreground: color!(0x161b22),    
                },
                normal: NormalColors {
                    primary: color!(0x6e40c9),      
                    surface: color!(0x8b949e),     
                    error: color!(0xda3633),        
                },
                bright: BrightColors {
                    primary: color!(0xa371f7),      
                    secondary: color!(0x3fb950),   
                    surface: color!(0xf0f6fc),       
                    error: color!(0xff7b72),         
                },
            },
            Self::Light => ColorPalette {
                base: BaseColors {
                    background: color!(0xffffff),    
                    foreground: color!(0xf6f8fa),    
                },
                normal: NormalColors {
                    primary: color!(0x0969da),       
                    surface: color!(0x57606a),       
                    error: color!(0xcf222e),        
                },
                bright: BrightColors {
                    primary: color!(0x0550ae),     
                    secondary: color!(0x1a7f37),     
                    surface: color!(0x24292f),      
                    error: color!(0xa40e26),         
                },
            },
            Self::Lupin => ColorPalette {
                base: BaseColors {
                    background: color!(0x282a36),    
                    foreground: color!(0x383a59),    
                },
                normal: NormalColors {
                    primary: color!(0x6272a4),       
                    surface: color!(0xa2a4a3),       
                    error: color!(0xff5555),     
                },
                bright: BrightColors {
                    primary: color!(0xbd93f9),       
                    secondary: color!(0x50fa7b),     
                    surface: color!(0xf8f8f2),       
                    error: color!(0xff6e6e),         
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