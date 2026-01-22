use std::future::Future;

#[cfg(feature = "tokio-rt")]
use tokio::task::JoinHandle;

#[cfg(feature = "smol-rt")]
use smol::Task;

pub fn spawn_worker<Fut>(future: Fut)
where
    Fut: Future + Send + 'static,
    Fut::Output: Send + 'static,
{
    #[cfg(feature = "tokio-rt")]
    {
        tokio::spawn(future);
    }

    #[cfg(feature = "smol-rt")]
    {
        smol::spawn(future).detach();
    }
}

pub fn spawn_server<Fut>(future: Fut) -> GateTask<Fut>
where
    Fut: Future + Send + 'static,
    Fut::Output: Send + 'static,
{
    #[cfg(feature = "tokio-rt")]
    {
        let handle = tokio::spawn(future);
        GateTask::new(Some(handle))
    }

    #[cfg(feature = "smol-rt")]
    {
        let handle = smol::spawn(future);
        GateTask::new(Some(handle))
    }
}

pub struct GateTask<F>
  where F: Future {
    #[cfg(feature = "tokio-rt")]
    inner: Option<JoinHandle<F::Output>>,

    #[cfg(feature = "smol-rt")]
    inner: Option<smol::Task<F::Output>>,
}

impl<F> GateTask<F>
where
    F: Future + Send + 'static,
    F::Output: Send + 'static,
{
    #[cfg(feature = "tokio-rt")]
    pub fn new(inner: Option<JoinHandle<F::Output>>) -> Self {
        Self { inner }
    }

    #[cfg(feature = "smol-rt")]
    pub fn new(inner: Option<smol::Task<F::Output>>) -> Self {
        Self { inner }
    }

    pub async fn cancel(&mut self) {
        #[cfg(feature = "tokio-rt")]
        if let Some(handle) = self.inner.take() {
            handle.abort();
        }

        #[cfg(feature = "smol-rt")]
        if let Some(handle) = self.inner.take() {
            handle.cancel().await;
        }
    }
}
