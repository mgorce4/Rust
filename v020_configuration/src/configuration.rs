use clap::{Parser};

/// Structure de configuration pour la machine de vote
#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
pub struct Configuration {
	/// Liste des candidats (sans Blanc ni Nul)
	#[arg(short = 'c', long = "candidates", required = true, num_args = 1..)]
	pub candidates: Vec<String>,
}

impl Configuration {
	/// Retourne la liste complète des candidats, avec Blanc et Nul ajoutés systématiquement
	pub fn all_candidates(&self) -> Vec<String> {
		let mut all = self.candidates.clone();
		all.push("Blanc".to_string());
		all.push("Nul".to_string());
		all
	}
}
