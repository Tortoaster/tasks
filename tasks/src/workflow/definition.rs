use std::collections::{HashMap, HashSet};
use std::fmt::{Display, Formatter};
use std::str::FromStr;

use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::interface::definition::UIChange;
use crate::internal::task_eval::{TaskEvalInfo, TaskEvalOpts};
use crate::internal::world::World;

/// Definition of a task.
#[derive(Debug)]
pub struct Task<T> {
    value: T,
}

impl<T> Task<T> {
    /// Create a new task.
    pub fn new(value: T) -> Self {
        Task { value }
    }

    /// Apply a task to an event.
    pub fn apply(self, _: Event, _: TaskEvalOpts, _: &mut World) {
        todo!()
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Event {
    /// Update something in an interaction: Task id, edit name, value.
    Edit(TaskId, String, serde_json::Value),
    /// Progress in a step combinator: Task id, action id.
    Action(TaskId, String),
    /// Recalcalutate the tasks with given IDs, using the current SDS values.
    Refresh(HashSet<TaskId>),
    /// Nop event, recalculate the entire task and reset output stream.
    Reset,
    /// Server is interrupted but the task should prepare to be restarted.
    ServerInterrupted,
    /// Cleanup and remove a task.
    Destroy,
}

#[derive(Debug)]
pub enum TaskResult<T> {
    /// If all goes well, a task computes its current value, a ui effect and a new task state.
    ValueResult(TaskValue<T>, TaskEvalInfo, UIChange, Task<T>),
    /// If something went wrong, a task produces an exception value.
    ExceptionResult(TaskException),
    /// If a task finalizes and cleaned up it gives this result.
    DestroyedResult,
}

/// Task results.
#[derive(Debug)]
pub enum TaskValue<T> {
    NoValue,
    Value(T, Stability),
}

pub type Stability = bool;

impl<T> TaskValue<T> {
    pub fn stable(t: T) -> Self {
        TaskValue::Value(t, true)
    }

    pub fn unstable(t: T) -> Self {
        TaskValue::Value(t, false)
    }

    pub fn is_stable(&self) -> bool {
        match self {
            TaskValue::Value(_, s) => *s,
            TaskValue::NoValue => false,
        }
    }

    pub fn map<U, F: FnOnce(T) -> U>(self, f: F) -> TaskValue<U> {
        match self {
            TaskValue::NoValue => TaskValue::NoValue,
            TaskValue::Value(value, stability) => TaskValue::Value(f(value), stability),
        }
    }
}

/// The dynamic contains the actual exception which can be matched, the string is an error message.
#[derive(Debug, Error)]
pub enum TaskException {}

/// Each task instance can be identified by two numbers:
/// - A unique number identifying the top-level state
/// - A unique number identifying the task within the the state
#[derive(Debug, Default, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub struct TaskId(pub InstanceNo, pub TaskNo);

pub type InstanceNo = u32;
pub type TaskNo = u32;

#[derive(Debug)]
pub struct TaskAttributes(pub HashMap<String, serde_json::Value>);

pub type InstanceKey = String;

impl Display for TaskId {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}-{}", self.0, self.1)
    }
}

impl FromStr for TaskId {
    type Err = <u32 as FromStr>::Err;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<_> = s.splitn(2, '-').collect();
        Ok(TaskId(parts[0].parse()?, parts[1].parse()?))
    }
}

pub type TaskList<T> = (TaskId, Vec<TaskListItem<T>>);

#[derive(Debug)]
pub struct TaskListItem<T> {
    task_id: TaskId,
    list_id: TaskId,
    detached: bool,
    self_: bool,
    value: TaskValue<T>,
    task_attributes: TaskAttributes,
    management_attributes: TaskAttributes,
}

#[derive(Debug, Default)]
pub struct TaskListFilter {
    only_index: Option<Vec<usize>>,
    only_task_id: Option<Vec<TaskId>>,
    not_task_id: Option<Vec<TaskId>>,
    only_attribute: Option<(String, serde_json::Value)>,
    only_self: bool,
    include_value: bool,
    include_task_attributes: bool,
    include_management_attributes: bool,
    include_progress: bool,
}

impl TaskListFilter {
    pub fn full() -> Self {
        Self::default()
    }
}

pub type Cookies = HashMap<String, String>;

pub trait ITask<'de, T>: Serialize + Deserialize<'de> + Eq {}
