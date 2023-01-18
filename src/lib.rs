mod opcode;
mod scripting;
mod uid;

pub use rs_sb3;

macro_rules! derive_everything {
    ($($item:item)*) => {
        $(
            #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)] $item
        )*
    };
}

pub(crate) use derive_everything;
