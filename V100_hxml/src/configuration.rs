use clap::{Parser, ValueEnum};

/// Structure de configuration pour la machine de vote
#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
pub struct Configuration {
	/// Liste des candidats (sans Blanc ni Nul)
	#[arg(short = 'c', long = "candidates", required = true, num_args = 1..)]
	pub candidates: Vec<String>,
	/// Type de stockage
	#[arg(short = 's', long = "storage", default_value = "file")]
	pub storage: StorageType,
	/// Langue de l'interface (en, fr)
	#[arg(short = 'l', long = "language", default_value = "fr")]
	pub language: String,
	/// Type de service (stdio, udp, web)
	#[arg(long = "service", default_value = "stdio")]
	pub service: String,
	/// Port d'écoute (pour UDP)
	#[arg(long = "port")]
	pub port: Option<u16>,
}

#[derive(Clone, Copy, ValueEnum, Debug)]
pub enum StorageType {
    File,
    Memory,
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
