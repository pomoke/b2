use crate::io::console::{AcceleratorKey, Key};
use uefi::proto::console::text::{Key as EFIKey, ScanCode};

impl From<EFIKey> for Key {
    fn from(value: EFIKey) -> Self {
        match value {
            EFIKey::Printable(u) => {
                let u: char = u.into();
                match u {
                    '\x08' => Self::Accelerator(AcceleratorKey::Backspace),
                    '\t' => Self::Accelerator(AcceleratorKey::Tab),
                    '\r' => Self::Accelerator(AcceleratorKey::Enter),
                    k => Self::Printable(k),
                }
            }
            EFIKey::Special(ScanCode::UP) => Self::Accelerator(AcceleratorKey::Up),
            EFIKey::Special(ScanCode::DOWN) => Self::Accelerator(AcceleratorKey::Down),
            EFIKey::Special(ScanCode::LEFT) => Self::Accelerator(AcceleratorKey::Left),
            EFIKey::Special(ScanCode::RIGHT) => Self::Accelerator(AcceleratorKey::Right),
            EFIKey::Special(ScanCode::PAGE_UP) => Self::Accelerator(AcceleratorKey::PgUp),
            EFIKey::Special(ScanCode::PAGE_DOWN) => Self::Accelerator(AcceleratorKey::PgDn),
            EFIKey::Special(ScanCode::ESCAPE) => Self::Accelerator(AcceleratorKey::Esc),
            EFIKey::Special(ScanCode::HOME) => Self::Accelerator(AcceleratorKey::Home),
            EFIKey::Special(ScanCode::END) => Self::Accelerator(AcceleratorKey::End),
            EFIKey::Special(ScanCode::INSERT) => Self::Accelerator(AcceleratorKey::Insert),
            EFIKey::Special(ScanCode::DELETE) => Self::Accelerator(AcceleratorKey::Delete),
            EFIKey::Special(ScanCode::FUNCTION_1) => Self::Accelerator(AcceleratorKey::F(1)),
            EFIKey::Special(ScanCode::FUNCTION_2) => Self::Accelerator(AcceleratorKey::F(2)),
            EFIKey::Special(ScanCode::FUNCTION_3) => Self::Accelerator(AcceleratorKey::F(3)),
            EFIKey::Special(ScanCode::FUNCTION_4) => Self::Accelerator(AcceleratorKey::F(4)),
            EFIKey::Special(ScanCode::FUNCTION_5) => Self::Accelerator(AcceleratorKey::F(5)),
            EFIKey::Special(ScanCode::FUNCTION_6) => Self::Accelerator(AcceleratorKey::F(6)),
            EFIKey::Special(ScanCode::FUNCTION_7) => Self::Accelerator(AcceleratorKey::F(7)),
            EFIKey::Special(ScanCode::FUNCTION_8) => Self::Accelerator(AcceleratorKey::F(8)),
            EFIKey::Special(ScanCode::FUNCTION_9) => Self::Accelerator(AcceleratorKey::F(9)),
            EFIKey::Special(ScanCode::FUNCTION_10) => Self::Accelerator(AcceleratorKey::F(10)),
            EFIKey::Special(ScanCode::FUNCTION_11) => Self::Accelerator(AcceleratorKey::F(11)),
            EFIKey::Special(ScanCode::FUNCTION_12) => Self::Accelerator(AcceleratorKey::F(12)),
            EFIKey::Special(k) => Self::Unknown(k.0 as u8),
        }
    }
}
