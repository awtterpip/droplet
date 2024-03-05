/// TODO: Document [`crate::atom`].
pub mod atom;

/// Main entrypoint into the program.
///
/// This function simply calls [`atom::valence::main`] and forwards the result.
fn main() -> anyhow::Result<()> {
    atom::valence::main()
}
