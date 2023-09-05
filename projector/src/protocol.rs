//! There's a company called _Creston_ (probably a trademarked name; we are not affiliated with them whatsoever)
//! that presumably makes/used to make UI solutions for other companies.
//!
//! The projector serves a website using either ActiveX objects, or when JavaScript is disabled, a Flash Player embed.
//!
//! Since those technologies have long been relegated to the dustbin of history, we did researched and used WireShark
//! to find (some of) the binary API used by the projector to communicate with the device by sending the same requests
//! the Creston UI would send.

/// The protocol uses binary values to communicate with the projector.
///
/// This module contains constants representing the individual commands, as well as the prefix and suffix bytes.
pub mod constants {
    /// A string of bytes prefixes every command sent to the projector.
    pub const PREFIX: &[u8] = &[0x05, 0x00, 0x06, 0x00, 0x00, 0x03, 0x00];

    /// A one-byte suffix is used for every power-specific command.
    pub const POWER_SUFFIX: u8 = 0x00;

    /// A one-byte suffix is used for every projector-specific command.
    pub const PROJECTOR_SUFFIX: u8 = 0x13;

    // A one-byte suffix is used for every menu-specific command.
    pub const MENU_SUFFIX: u8 = 0x14;
}

// Values adapted from https://github.com/Grayda/dell-control/blob/master/dellproj.js
pub mod commands {
    pub mod input {
        use super::super::{constants, make_command, Command};

        // FIXME these may not work on all projectors

        pub const VGA_A: Command = make_command(0xcd, constants::PROJECTOR_SUFFIX);
        pub const VGA_B: Command = make_command(0xce, constants::PROJECTOR_SUFFIX);
        pub const COMPOSITE: Command = make_command(0xcf, constants::PROJECTOR_SUFFIX);
        pub const S_VIDEO: Command = make_command(0xd0, constants::PROJECTOR_SUFFIX);
        pub const HDMI: Command = make_command(0xd1, constants::PROJECTOR_SUFFIX);
        pub const WIRELESS: Command = make_command(0xd3, constants::PROJECTOR_SUFFIX);
        pub const USB_DISPLAY: Command = make_command(0xd4, constants::PROJECTOR_SUFFIX);
        pub const USB_VIEWER: Command = make_command(0xd5, constants::PROJECTOR_SUFFIX);
    }

    pub mod volume {
        use super::super::{constants, make_command, Command};

        pub const UP: Command = make_command(0xfa, constants::PROJECTOR_SUFFIX);
        pub const DOWN: Command = make_command(0xfb, constants::PROJECTOR_SUFFIX);
        pub const MUTE: Command = make_command(0xfc, constants::PROJECTOR_SUFFIX);
        pub const UN_MUTE: Command = make_command(0xfd, constants::PROJECTOR_SUFFIX);
    }

    pub mod power {
        use super::super::{constants, make_command, Command};

        pub const ON: Command = make_command(0x00, constants::POWER_SUFFIX);
        pub const OFF: Command = make_command(0x01, constants::POWER_SUFFIX);
    }

    pub mod menu {
        use super::super::{constants, make_command, Command};

        pub const MENU_BUTTON: Command = make_command(0x1d, constants::MENU_SUFFIX);
        pub const UP: Command = make_command(0x1e, constants::MENU_SUFFIX);
        pub const DOWN: Command = make_command(0x1f, constants::MENU_SUFFIX);
        pub const LEFT: Command = make_command(0x20, constants::MENU_SUFFIX);
        pub const RIGHT: Command = make_command(0x21, constants::MENU_SUFFIX);
        pub const OK: Command = make_command(0x23, constants::MENU_SUFFIX);
    }

    pub mod picture {
        use super::super::{constants, make_command, Command};

        pub const BLANK: Command = make_command(0xee, constants::PROJECTOR_SUFFIX);
        pub const UN_BLANK: Command = make_command(0xef, constants::PROJECTOR_SUFFIX);
        pub const FREEZE: Command = make_command(0xf0, constants::PROJECTOR_SUFFIX);
        pub const UN_FREEZE: Command = make_command(0xf1, constants::PROJECTOR_SUFFIX);
        pub const CONTRAST_UP: Command = make_command(0xf6, constants::PROJECTOR_SUFFIX);
        pub const CONTRAST_DOWN: Command = make_command(0xf7, constants::PROJECTOR_SUFFIX);
        pub const BRIGHTNESS_UP: Command = make_command(0xf5, constants::PROJECTOR_SUFFIX);
        pub const BRIGHTNESS_DOWN: Command = make_command(0xf4, constants::PROJECTOR_SUFFIX);
    }
}

pub type Command = [u8; 9];

const fn make_command(command: u8, suffix: u8) -> Command {
    let mut payload = [0; 9];

    // Since this is a const fn, we can't use .copy_from_slice() on the prefix
    // or any other shorthand for that matter.
    // Can't even have for loops.
    // Copy them manually.
    payload[0] = constants::PREFIX[0];
    payload[1] = constants::PREFIX[1];
    payload[2] = constants::PREFIX[2];
    payload[3] = constants::PREFIX[3];
    payload[4] = constants::PREFIX[4];
    payload[5] = constants::PREFIX[5];
    payload[6] = constants::PREFIX[6];

    payload[7] = command;
    payload[8] = suffix;

    payload
}
