use crate::utils::FnBox;

/// The job to run in a `Worker`
pub type Job = Box<dyn FnBox + Send + 'static>;

/// Tasks for `Worker`
pub enum Task {
    Do(Job),
    Terminate,
}
