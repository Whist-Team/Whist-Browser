use std::fmt::Debug;
use std::future::Future;

use bevy::prelude::*;
use bevy::tasks::TaskPool;
use futures::channel::mpsc::{unbounded, TryRecvError, UnboundedReceiver, UnboundedSender};
use futures::StreamExt;

pub struct Worker<I: Debug, O: Debug> {
    input: UnboundedSender<I>,
    output: UnboundedReceiver<O>,
}

#[cfg(target_family = "wasm")]
impl<I: Debug, O: Debug> Worker<I, O> {
    pub fn spawn<Func, Fut>(thread_pool: &TaskPool, function: Func) -> Self
    where
        Func: FnOnce(Worker<O, I>) -> Fut,
        Fut: Future<Output = ()> + 'static,
    {
        info!("spawning worker");

        let (input_tx, input_rx) = unbounded::<I>();
        let (output_tx, output_rx) = unbounded::<O>();
        thread_pool
            .spawn(function(Worker {
                input: output_tx,
                output: input_rx,
            }))
            .detach();
        Worker {
            input: input_tx,
            output: output_rx,
        }
    }
}

#[cfg(not(target_family = "wasm"))]
impl<I: Debug, O: Debug> Worker<I, O> {
    pub fn spawn<Func, Fut>(thread_pool: &TaskPool, function: Func) -> Self
    where
        Func: FnOnce(Worker<O, I>) -> Fut,
        Fut: Future<Output = ()> + Send + 'static,
    {
        info!("spawning worker");

        let (input_tx, input_rx) = unbounded::<I>();
        let (output_tx, output_rx) = unbounded::<O>();
        thread_pool
            .spawn(async_compat::Compat::new(function(Worker {
                input: output_tx,
                output: input_rx,
            })))
            .detach();
        Worker {
            input: input_tx,
            output: output_rx,
        }
    }
}

impl<I: Debug, O: Debug> Worker<I, O> {
    pub fn send(&self, message: I) {
        self.input.unbounded_send(message).unwrap();
    }

    pub fn try_recv(&mut self) -> Result<Option<O>, TryRecvError> {
        self.output.try_next()
    }

    pub async fn recv(&mut self) -> Option<O> {
        self.output.next().await
    }
}
