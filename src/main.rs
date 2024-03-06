/// todo! Document [`crate::atom`].
pub mod atom;

/// Main entrypoint into the program.
///
/// This function simply calls [`atom::valence::main`] and forwards the result.
fn main() -> color_eyre::Result<()> {
    atom::valence::main()
}
