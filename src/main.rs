/// TODO: Document [`crate::core`].
pub mod core;

/// TODO: Document [`crate::nucleus`].
pub mod nucleus;

/// TODO: Document [`crate::valence`].
pub mod valence;

/// Main entrypoint into the program.
///
/// This function simply calls [`valence::main`] and forwards the result.
fn main() -> anyhow::Result<()> {
    valence::main()
}
