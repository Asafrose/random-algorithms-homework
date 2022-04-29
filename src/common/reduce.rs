use std::{fmt::Debug};

use anyhow::{Ok, Result};

use super::{algorithm::Algorithm, repeat::Repeat};

pub struct Reduce<TAlgorithm, TReducer> {
    algorithm: TAlgorithm,
    reducer: TReducer,
}

impl<TAlgorithm, TResult : Debug + Sync + Send, TReducer> Algorithm for Reduce<TAlgorithm, TReducer>
where
    TAlgorithm: Algorithm,
    TReducer: Fn(TAlgorithm::Output) -> Result<TResult>,
{
    type Input = TAlgorithm::Input;
    type Output = TResult;

    fn name(&self) -> String {
        format!("{} reduced", { self.algorithm.name() })
    }

    fn input(&self) -> Self::Input {
        self.algorithm.input().clone()
    }

    fn get_repetitions(&self) -> u64 {
        self.algorithm.get_repetitions()
    }

    fn run_internal<F: Fn() + Sync + Send>(&self, update_progress: F) -> Result<Self::Output> {
        Ok((self.reducer)(self.algorithm.run_internal(update_progress)?)?)
    }
}

pub trait IntoReduce<TAlgorithm: Algorithm> {
    fn reduce<TReducer, TResult>(self, reducer: TReducer) -> Reduce<TAlgorithm, TReducer>
    where
        TReducer: Fn(TAlgorithm::Output) -> Result<TResult>;
}

impl<TAlgorithm: Algorithm + Sync + Send> IntoReduce<Repeat<TAlgorithm>> for Repeat<TAlgorithm> {
    fn reduce<TReducer, TResult>(self, reducer: TReducer) -> Reduce<Repeat<TAlgorithm>, TReducer>
    where
        TReducer: Fn(<Repeat<TAlgorithm> as Algorithm>::Output) -> Result<TResult>,
    {
        Reduce {
            algorithm: self,
            reducer,
        }
    }
}
