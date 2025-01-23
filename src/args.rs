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
    Thresholding(OptsThresholding),
    Pallet(OptsPallet),
    Dithering(OptsDithering),
    OrderedDithering(OptsOrderedDithering),
    ErrorDiffusion(OptsErrorDiffusion),
}

#[derive(Debug, Clone, PartialEq, FromArgs)]
#[argh(subcommand, name = "alternance-pixels-blancs")]
/// Rendu de l’image avec une alternance de pixels blancs.
pub struct OptsWhitePixelAlternation {}

#[derive(Debug, Clone, PartialEq, FromArgs)]
#[argh(subcommand, name = "seuillage")]
/// Rendu de l’image en noir et blanc.
pub struct OptsThresholding {
    /// couleur claire
    #[argh(option, long = "couleur-claire")]
    pub light: Option<String>,
    /// couleur foncee
    #[argh(option, long = "couleur-foncee")]
    pub dark: Option<String>,
}

#[derive(Debug, Clone, PartialEq, FromArgs)]
#[argh(subcommand, name = "palette")]
/// Rendu de l’image avec une palette contenant un nombre limité de couleurs
pub struct OptsPallet {
    /// le nombre de couleurs à utiliser, dans la liste [NOIR, BLANC, ROUGE, VERT, BLEU, JAUNE, CYAN, MAGENTA]
    #[argh(option, long = "nombre-de-couleurs", short = 'n')]
    pub n: usize,
}

#[derive(Debug, Clone, PartialEq, FromArgs)]
#[argh(subcommand, name = "tramage")]
/// Rendu de l’image avec un tre tramage aléatoire
pub struct OptsDithering {}

#[derive(Debug, Clone, PartialEq, FromArgs)]
#[argh(subcommand, name = "tramage-ordonne")]
/// Rendu de l’image avec un tramage ordonné en utilisant la matrice de Bayer
pub struct OptsOrderedDithering {
    /// l’ordre de la matrice de Bayer
    #[argh(option, long = "ordre", short = 'n')]
    pub n: u32,
}

#[derive(Debug, Clone, PartialEq, FromArgs)]
#[argh(subcommand, name = "diffusion-erreur")]
/// Rendu de l’image avec une diffusion d’erreur
pub struct OptsErrorDiffusion {
    /// l’algorithme de diffusion d’erreur à utiliser, dans la liste [FLOYD_STEINBERG, JARVIS_JUDICE_NINKE, ATKINSON]
    #[argh(option, long = "algorithme", short = 'a')]
    pub algorithm: String,
}