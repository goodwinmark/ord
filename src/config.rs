use super::*;

#[derive(Deserialize, Default)]
pub(crate) struct Config {
  hidden: HashSet<InscriptionId>,
}

impl Config {
  pub(crate) fn hidden(&self, inscription_id: InscriptionId) -> bool {
    self.hidden.contains(&inscription_id)
  }
}
