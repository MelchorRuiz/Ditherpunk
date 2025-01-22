use argh::FromArgs;

#[derive(Debug, Clone, PartialEq, FromArgs)]
/// Convertit une image en monochrome ou vers une palette réduite de couleurs.
pub struct DitherArgs {
    /// le fichier d’entrée
    #[argh(positional)]
    pub input: String,

    /// le fichier de sortie (optionnel)
    #[argh(positional)]
    pub output: Option<String>,

    /// le mode d’opération
    #[argh(subcommand)]
    pub mode: Mode,
}

#[derive(Debug, Clone, PartialEq, FromArgs)]
#[argh(subcommand)]
pub enum Mode {
    WhitePixelAlternation(OptsWhitePixelAlternation),
    Umbralization(OptsUmbralization),
}

#[derive(Debug, Clone, PartialEq, FromArgs)]
#[argh(subcommand, name = "alternance-pixels-blancs")]
/// Rendu de l’image avec une alternance de pixels blancs.
pub struct OptsWhitePixelAlternation {}

#[derive(Debug, Clone, PartialEq, FromArgs)]
#[argh(subcommand, name = "seuillage")]
/// Rendu de l’image en noir et blanc.
pub struct OptsUmbralization {
    /// couleur claire
    #[argh(option, long = "couleur-claire")]
    pub light: Option<String>,
    /// couleur foncee
    #[argh(option, long = "couleur-foncee")]
    pub dark: Option<String>,
}