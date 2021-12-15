use bio::stats::probs::LogProb;

use crate::variants::evidence::observation::ProcessedObservation;
use crate::variants::model::bias::Bias;

#[derive(Copy, Clone, PartialOrd, PartialEq, Eq, Debug, Ord, EnumIter, Hash)]
pub(crate) enum SoftclipBias {
    None,
    Some,
}

impl Default for SoftclipBias {
    fn default() -> Self {
        SoftclipBias::None
    }
}

impl Bias for SoftclipBias {
    fn prob_alt(&self, observation: &ProcessedObservation) -> LogProb {
        match (self, observation.softclipped) {
            (SoftclipBias::Some, true) => LogProb::ln_one(),
            (SoftclipBias::Some, false) => LogProb::ln_zero(),
            (SoftclipBias::None, _) => LogProb::ln_one(),
        }
    }

    fn prob_any(&self, _observation: &ProcessedObservation) -> LogProb {
        LogProb::ln_one()
    }

    fn is_artifact(&self) -> bool {
        *self != SoftclipBias::None
    }

    fn is_informative(&self, pileups: &[Vec<ProcessedObservation>]) -> bool {
        if !self.is_artifact() {
            return true;
        }
        // METHOD: this bias is only relevant if there is at least one softclip.
        pileups
            .iter()
            .any(|pileup| pileup.iter().any(|obs| obs.softclipped))
    }
}
