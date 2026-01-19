use std::future::Future;

#[cfg(feature = "tokio-rt")]
use tokio::task::JoinHandle;

#[cfg(feature = "smol-rt")]
use smol::Task;

pub fn spawn_worker<Fut>(future: Fut)
where
    Fut: Future<Output = ()> + Send + 'static,
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

pub fn spawn_server<Fut>(future: Fut) -> GateTask
where
    Fut: Future<Output = ()> + Send + 'static,
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

pub struct GateTask {
    #[cfg(feature = "tokio-rt")]
    inner: Option<JoinHandle<()>>,

    #[cfg(feature = "smol-rt")]
    inner: Option<smol::Task<()>>,
}

impl GateTask {
    #[cfg(feature = "tokio-rt")]
    pub fn new(inner: Option<JoinHandle<()>>) -> Self {
        Self { inner }
    }

    #[cfg(feature = "smol-rt")]
    pub fn new(inner: Option<smol::Task<()>>) -> Self {
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
