use super::algorithm::Algorithm;

pub struct WithName<TAlgorithm> {
    algorithm: TAlgorithm,
    name: String,
}

impl<TAlgorithm: Algorithm> Algorithm for WithName<TAlgorithm> {
    type Input = TAlgorithm::Input;
    type Output = TAlgorithm::Output;

    fn name(&self) -> String {
        self.name.clone()
    }

    fn input(&self) -> Self::Input {
        self.algorithm.input()
    }

    fn get_repetitions(&self) -> u64 {
        self.algorithm.get_repetitions()
    }

    fn run_internal<F: Fn() + Sync + Send>(&self, update_progress: F) -> anyhow::Result<Self::Output> {
        self.algorithm.run_internal(update_progress)
    }
}

pub trait IntoWithName<TAlgorithm> {
    fn with_name(self, name: String) -> WithName<TAlgorithm>;
}

impl<TAlgorithm: Algorithm> IntoWithName<TAlgorithm> for TAlgorithm {
    fn with_name(self, name: String) -> WithName<TAlgorithm> {
        WithName {
            algorithm: self,
            name,
        }
    }
}
