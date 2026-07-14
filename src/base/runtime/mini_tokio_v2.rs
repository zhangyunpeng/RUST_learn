use futures_util::task;
use futures_util::task::ArcWake;
use std::pin::Pin;
use std::sync::{Arc, Mutex, mpsc};
use std::task::{Context, Poll};

pub struct MiniTokio {
    schedule: mpsc::Receiver<Arc<Task>>,
    sender: mpsc::Sender<Arc<Task>>,
}

struct TaskFuture {
    future: Pin<Box<dyn Future<Output = ()> + Send + 'static>>,
    poll: Poll<()>,
}

struct Task {
    task_future: Mutex<TaskFuture>,
    sender: mpsc::Sender<Arc<Task>>,
}

impl TaskFuture {
    fn new<F>(f: F) -> Self
    where
        F: Future<Output = ()> + Send + 'static,
    {
        TaskFuture {
            future: Box::pin(f),
            poll: Poll::Pending,
        }
    }

    fn poll(&mut self, cx: &mut Context<'_>) {
        if self.poll.is_pending() {
            self.poll = self.future.as_mut().poll(cx);
        }
    }
}

impl Task {
    fn spawn<F>(future: F, sender: &mpsc::Sender<Arc<Task>>)
    where
        F: Future<Output = ()> + Send + 'static,
    {
        let task_future = TaskFuture::new(future);
        let task = Arc::new(Task {
            task_future: Mutex::new(task_future),
            sender: sender.clone(),
        });
        let _ = sender.send(task);
    }

    fn poll(self: &Arc<Task>) {
        let waker = task::waker(self.clone());
        let mut cx = Context::from_waker(&waker);
        let mut future = self.task_future.try_lock().unwrap();
        future.poll(&mut cx);
    }

    fn schedule(self: &Arc<Self>) {
        self.sender.send(self.clone()).unwrap();
    }
}

impl ArcWake for Task {
    fn wake_by_ref(arc_self: &Arc<Self>) {
        arc_self.schedule();
    }
}

impl MiniTokio {
    fn new() -> Self {
        let (sender, schedule) = mpsc::channel();
        MiniTokio { schedule, sender }
    }

    pub fn spawn<F>(&self, f: F)
    where
        F: Future<Output = ()> + Send + 'static,
    {
        Task::spawn(f, &self.sender);
    }

    pub fn run(&self) {
        while let Ok(task) = self.schedule.recv() {
            task.poll();
        }
    }
}

impl Default for MiniTokio {
    fn default() -> Self {
        MiniTokio::new()
    }
}
