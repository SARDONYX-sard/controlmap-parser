//! Input scan codes with unknown details defined at the following sites.
//! Direct input enumerator for Skyrim
//!
//! Wouldn't windows-sys or other crates be sufficient?
//! - I wrote a new one because it is not easy to use in windows-sys.
//!
//! # References
//! - [DirectInput keyboard scan codes](https://gist.github.com/tracend/912308)
//! - [Microsoft Deevice Constants](https://learn.microsoft.com/en-us/previous-versions/windows/desktop/ee416859(v=vs.85))
//! - [Github/Hawkbat/SkyrimControlMapper](https://github.com/Hawkbat/SkyrimControlMapper/blob/12c28d6bae3d5c6c898027fd714cb6ac29b752e7/index.tsx#L198)
//! - [DirectInput Key Code Table](http://www.flint.jp/misc/?q=dik&lang=en)
//! - [HumanInterface](https://docs.rs/windows-sys/latest/windows_sys/Win32/Devices/HumanInterfaceDevice/index.html)
//!   - search DIK(Direst input key)
use core::{fmt, str::FromStr};
use num_derive::{FromPrimitive, ToPrimitive};
use num_traits::FromPrimitive;

#[derive(
    Debug, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash, FromPrimitive, ToPrimitive,
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum MouseCode {
    #[default]
    None = 0xff,
    Mouse1 = 0x0,
    /// Maybe DIDEVTYPEMOUSE_UNKNOWN
    Mouse2 = 0x1,
    /// Maybe DIDEVTYPEMOUSE_TRADITIONAL
    Mouse3 = 0x2,
    /// Maybe DIDEVTYPEMOUSE_FINGERSTICK
    Mouse4 = 0x3,
    /// Maybe DIDEVTYPEMOUSE_TOUCHPAD
    Mouse5 = 0x4,
    ///Maybe DIDEVTYPEMOUSE_TRACKBALL
    Mouse6 = 0x5,
    Mouse7 = 0x6,
    Mouse8 = 0x7,
    MouseWheelUp = 0x8,
    MouseWheelDown = 0x9,
    MouseMove = 0xa,

    // This is the definition from the following website
    // - See: [Input Script](https://www.creationkit.com/index.php?title=Input_Script)
    LeftMouseButton = 256,
    RightMouseButton = 257,
    MiddleWheelMouseButton = 258,
    MouseButton3 = 259,
    MouseButton4 = 260,
    MouseButton5 = 261,
    MouseButton6 = 262,
    MouseButton7 = 263,
    MouseWheelUp_ = 264,
    MouseWheelDown_ = 265,
}

#[allow(non_camel_case_types)]
#[derive(
    Debug, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash, FromPrimitive, ToPrimitive,
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum GamepadCode {
    #[default]
    None = 0xff,
    Up = 0x0001,
    Down = 0x0002,
    Left = 0x0004,
    Right = 0x008,
    _360_Start = 0x0010,
    _360_Back = 0x0020,
    _360_L3 = 0x0040,
    _360_R3 = 0x0080,
    _360_LB = 0x0100,
    _360_RB = 0x0200,
    _360_A = 0x1000,
    _360_B = 0x2000,
    _360_X = 0x4000,
    _360_Y = 0x8000,
    _360_LT = 0x0009,
    _360_RT = 0x000a,
    _360_LS = 0x000b,
    _360_RS = 0x000c,

    // This is the definition from the following website
    // - See: [Input Script](https://www.creationkit.com/index.php?title=Input_Script)
    DpadUp = 266,
    DpadDown = 267,
    DpadLeft = 268,
    DpadRight = 269,
    Start = 270,
    Back = 271,
    LeftThumb = 272,
    RightThumb = 273,
    LeftShoulder = 274,
    RightShoulder = 275,
    AButton = 276,
    BButton = 277,
    XButton = 278,
    YButton = 279,
    LeftTrigger = 280,
    RightTrigger = 281,
}

/// - [HumanInterface](https://docs.rs/windows-sys/latest/windows_sys/Win32/Devices/HumanInterfaceDevice/index.html)
///   - search "DIK_"(Direst input key)
#[allow(non_camel_case_types)]
#[derive(
    Debug, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash, FromPrimitive, ToPrimitive,
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum KeyboardCode {
    #[default]
    None = 0xff,
    Esc = 0x01,
    _1 = 0x02,
    _2 = 0x03,
    _3 = 0x04,
    _4 = 0x05,
    _5 = 0x06,
    _6 = 0x07,
    _7 = 0x08,
    _8 = 0x09,
    _9 = 0x0a,
    _0 = 0x0b,
    Hyphen = 0x0c,
    Equal = 0x0d,
    Backspace = 0x0e,
    Tab = 0x0f,
    Q = 0x10,
    W = 0x11,
    E = 0x12,
    R = 0x13,
    T = 0x14,
    Y = 0x15,
    U = 0x16,
    I = 0x17,
    O = 0x18,
    P = 0x19,
    Bracketleft = 0x1a,
    Bracketright = 0x1b,
    Enter = 0x1c,
    LCtrl = 0x1d,
    A = 0x1e,
    S = 0x1f,
    D = 0x20,
    F = 0x21,
    G = 0x22,
    H = 0x23,
    J = 0x24,
    K = 0x25,
    L = 0x26,
    Semicolon = 0x27,
    Quotesingle = 0x28,
    Tilde = 0x29,
    LShift = 0x2a,
    Backslash = 0x2b,
    Z = 0x2c,
    X = 0x2d,
    C = 0x2e,
    V = 0x2f,
    B = 0x30,
    N = 0x31,
    M = 0x32,
    Comma = 0x33,
    Period = 0x34,
    Slash = 0x35,
    RShift = 0x36,
    NumpadMult = 0x37,
    LAlt = 0x38,
    Space = 0x39,
    CapsLock = 0x3a,
    F1 = 0x3b,
    F2 = 0x3c,
    F3 = 0x3d,
    F4 = 0x3e,
    F5 = 0x3f,
    F6 = 0x40,
    F7 = 0x41,
    F8 = 0x42,
    F9 = 0x43,
    F10 = 0x44,
    NumLock = 0x45,
    ScrollLock = 0x46,
    Numpad7 = 0x47,
    Numpad8 = 0x48,
    Numpad9 = 0x49,
    NumpadMinus = 0x4a,
    Numpad4 = 0x4b,
    Numpad5 = 0x4c,
    Numpad6 = 0x4d,
    NumpadPlus = 0x4e,
    Numpad1 = 0x4f,
    Numpad2 = 0x50,
    Numpad3 = 0x51,
    Numpad0 = 0x52,
    NumpadDec = 0x53,
    DIK_OEM_102 = 0x56,
    F11 = 0x57,
    F12 = 0x58,
    F13 = 0x64,
    F14 = 0x65,
    F15 = 0x66,
    DIK_KANA = 0x70,
    DIK_ABNT_C1 = 0x73,
    DIK_CONVERT = 0x79,
    DIK_NOCONVERT = 0x7b,
    Unknown = 0x7d,
    DIK_ABNT_C2 = 0x7e,
    NumPadEqual = 0x8d,
    PrevTrack = 0x90,
    DIK_AT = 0x91,
    Colon = 0x92,
    DIK_UNDERLINE = 0x93,
    DIK_KANJI = 0x94,
    DIK_STOP = 0x95,
    DIK_AX = 0x96,
    DIK_UNLABELED = 0x97,
    NextTrack = 0x99,
    NumPadEnter = 0x9c,
    RCtrl = 0x9d,
    Mute = 0xa0,
    Calc = 0xa1,
    PlayPause = 0xa2,
    MediaStop = 0xa4,
    VolMinus = 0xae,
    VolPlus = 0xb0,
    WebHome = 0xb2,
    NumpadComma = 0xb3,
    NumpadDivide = 0xb5,
    PrintSrc = 0xb7,
    RAlt = 0xb8,
    Pause = 0xc5,
    Home = 0xc7,
    UpArrow = 0xc8,
    PgUp = 0xc9,
    Left = 0xcb,
    Right = 0xcd,
    End = 0xcf,
    DownArrow = 0xd0,
    PgDn = 0xd1,
    Insert = 0xd2,
    Delete = 0xd3,
    LWindows = 0xdb,
    RWindows = 0xdc,
    Apps = 0xdd,
    Power = 0xde,
    Sleep = 0xdf,
    Wake = 0xe3,
    WebSearch = 0xe5,
    WebFavorites = 0xe6,
    WebRefresh = 0xe7,
    WebStop = 0xe8,
    WebForward = 0xe9,
    WebBack = 0xea,
    MyComputer = 0xeb,
    Mail = 0xec,
    MediaSelect = 0xed,
}

/// # IntoRaw
/// cast to usize
trait ToRaw {
    /// Get usize by clone.
    fn to_raw(&self) -> usize;
}

macro_rules! impl_cast {
    ($($self:ident),+ $(,)?) => {
        $(
            impl ToRaw for $self {
                fn to_raw(&self) -> usize {
                    self.clone() as usize
                }
            }
            impl TryFrom<usize> for $self {
                type Error = ScanCodeError;

                fn try_from(value: usize) -> Result<Self, Self::Error> {
                    FromPrimitive::from_usize(value)
                        .ok_or_else(|| ScanCodeError::InvalidDigit(format!("{}", value)))
                }
            }
            impl FromStr for $self {
                type Err = ScanCodeError;

                fn from_str(s: &str) -> Result<Self, Self::Err> {
                    let radix = match s.chars().next() {
                        Some('0') => {
                            match s.chars().nth(1) {
                                Some('x') | Some('X') => 16, // Hexadecimal
                                Some('o') | Some('O') => 8,  // Octal
                                Some('b') | Some('B') => 2,  // Binary
                                _ => 10,                     // Decimal
                            }
                        }
                        _ => 10, // Decimal
                    };

                    let s = if radix != 10 { &s[2..] } else { s };
                    Self::try_from(
                        usize::from_str_radix(s, radix)
                            .map_err(|_| ScanCodeError::InvalidDigit(s.into()))?,
                    )
                }
            }
            impl fmt::Display for $self {
                fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                    write!(f, "0x{:x}", self.to_raw())
                }
            }
        )+
    };
}

impl_cast!(GamepadCode, KeyboardCode, MouseCode);

#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum ScanCodeError {
    #[error("invalid digit found in string. got {0}")]
    InvalidDigit(String),
}
