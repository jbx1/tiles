pub mod queue;

use std::cmp::Ordering;
use std::collections::{VecDeque, HashMap};
use std::fmt::{Debug};
use std::hash::Hash;
use std::rc::Rc;

use crate::search::Transition::{Action, Initial};

use std::cmp::Ordering::Equal;
use crate::search::queue::{Queue, Priority, Fifo};
use std::time::{Duration, Instant};

#[derive(Debug)]
pub struct SearchResult<S: State> {
    pub plan: Option<VecDeque<S>>,
    pub statistics: Statistics
}

#[derive(Debug)]
pub struct Statistics {
    created: i32,
    queued: i32,
    expanded: i32,
    duration: Duration
}

pub trait State: PartialEq + Eq + Hash + Sized + Copy + Debug {
    fn successors(&self) -> Vec<Self>;
    fn h(&self) -> f32;
}

#[derive(Debug, Eq)]
enum Transition<S: State> {
    Initial{state: Rc<S>},
    Action {state: Rc<S>, parent: Rc<Transition<S>>, g: u32},
}

impl<S: State> Transition<S> {
    fn new(initial: Rc<S>) -> Transition<S> {
        Initial{state: initial}
    }

    fn state(&self) -> &S {
        match self {
            Initial { state } => &state,
            Action { state, .. } => &state
        }
    }

    fn parent(&self) -> Option<&Transition<S>> {
       match self {
            Action{  state:_, parent, g:_ } => Some(parent.as_ref()),
            Initial{ state:_ } => None,
        }
    }

    fn h(&self) -> f32 {
        self.state().h()
    }

    fn g(&self) -> u32 {
        match self {
            Action{  state:_, parent:_, g } => *g,
            Initial{ state:_ } => 0,
        }
    }

    fn successor(state: Rc<S>, parent: Rc<Transition<S>>) -> Transition<S> {
        Action{state, g: parent.g() + 1, parent}
    }
}


impl<S : State> PartialOrd for Transition<S> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<S : State> Ord for Transition<S> {
    fn cmp(&self, other: &Self) -> Ordering {
        let other_f = other.g() as f32 + other.h();
        let self_f = self.g() as f32 + self.h();

        other_f.partial_cmp(&self_f).unwrap_or_else(|| Equal)
    }
}

impl<S: State> PartialEq for Transition<S> {
    fn eq(&self, other: &Self) -> bool {
        self.state() == other.state()
    }
}

impl<S: State> Clone for Transition<S> {
    fn clone(&self) -> Transition<S> {
        match self {
            Initial { state } => Initial { state: state.clone() },
            Action {state, parent, g} => Action { state: state.clone(), parent: Rc::clone(parent), g: *g}
        }
    }
}

pub fn breadth_first_search<S: State, F: Fn(&S) -> bool>(initial: &S, goal: F) -> SearchResult<S> {
    let mut queue = Fifo::new();
    search(initial, goal, &mut queue)
}

pub fn greedy_best_first_search<S: State, F: Fn(&S) -> bool>(initial: &S, goal: F) -> SearchResult<S> {
    //todo: currently this behaves like A* because Ord comparison uses g() + h(),
    // change to binary_heap_plus to specify custom comparator

    let mut queue = Priority::new();
    search(initial, goal, &mut queue)
}

pub fn a_star_search<S: State, F: Fn(&S) -> bool>(initial: &S, goal: F) -> SearchResult<S> {
    let mut queue = Priority::new();
    search(initial, goal, &mut queue)
}

fn search<S, F, Q>(initial: &S, goal: F, queue: &mut Q) -> SearchResult<S>
    where S: State,
          F: Fn(&S) -> bool,
          Q: Queue<Transition<S>>
{
    let mut seen = HashMap::new();

    // the initial state
    let mut statistics = Statistics { created: 1, queued: 1, expanded: 0, duration: Duration::new(0,0)};
    let start = Instant::now();

    println!("Initial h value {}", initial.h());

    let initial_state = Rc::new(*initial);
    let initial_transition = Rc::new(Transition::new(Rc::clone(&initial_state)));
    seen.insert(initial_state, Rc::clone(&initial_transition));
    queue.enqueue(initial_transition);

    while let Some(transition) = queue.dequeue() {
        if (goal)(&transition.state()) {
            let plan = extract_plan(&transition);
            statistics.duration = start.elapsed();
            return SearchResult{ plan: Some(plan), statistics };
        }
        else {
            let parent = Rc::new(transition);
            statistics.expanded += 1;
            for successor_state in parent.state().successors() {
                statistics.created += 1;
                if !seen_and_better(&seen, &successor_state, parent.g() + 1) {
                    let successor_state_rc = Rc::new(successor_state);
                    let succ_transition = Rc::new(Transition::successor(Rc::clone(&successor_state_rc),Rc::clone(&parent)));
                    seen.insert(successor_state_rc, Rc::clone(&succ_transition));
                    queue.enqueue(succ_transition);
                    statistics.queued += 1;
                }
            }
        }
    }

    statistics.duration = start.elapsed();
    SearchResult{ plan: None, statistics }
}

fn seen_and_better<S: State>(seen: &HashMap<Rc<S>, Rc<Transition<S>>>, state: &S, g: u32) -> bool {
    match seen.get(state) {
        Some(transition) if transition.g() < g + 1 => true,
        _ => false
    }
}

fn extract_plan<S: State>(goal_transition: &Transition<S>) -> VecDeque<S> {
    let mut plan = VecDeque::new();

    plan.push_front(*goal_transition.state());
    let mut current = goal_transition;

    while let Some(previous) = current.parent() {
        plan.push_front(*previous.state());
        current = previous;
    }

    plan
}

#[cfg(test)]
mod tests {
    use super::*;

    const GOAL: i32 = 5;

    #[derive(Hash, Debug, Copy, Clone, Eq, PartialEq)]
    struct TestState {
        value: i32,
    }

    impl State for TestState {
        fn successors(&self) -> Vec<Self> {
            vec![TestState{value: self.value+1}, TestState{ value: self.value+2}, TestState{value: self.value+3}]
        }

        fn h(&self) -> f32 {
            if GOAL < self.value {
                 f32::INFINITY
            } else {
                (GOAL - self.value) as f32
            }
        }
    }


    #[test]
    fn test_breadth_first_search() {
        let initial = TestState{value: 0};
        println!("Starting Breadth First Search");

        let result = breadth_first_search(&initial, |state| state.value == 5);

        assert!(result.plan.is_some());

        let plan = result.plan.unwrap();
        assert!(plan.len() > 0);

        println!("Plan: {:?}", plan);

        let goal = plan.get(plan.len() - 1).unwrap();
        assert_eq!(goal.value, GOAL);
    }

    #[test]
    fn test_greedy_best_first_search() {
        let initial = TestState{value: 0};
        println!("Starting Greedy Best First Search");
        let result = greedy_best_first_search(&initial, |state| state.value == 5);
        assert!(result.plan.is_some());

        let plan = result.plan.unwrap();
        assert!(plan.len() > 0);

        println!("Plan: {:?}", plan);

        let goal = plan.get(plan.len() - 1).unwrap();
        assert_eq!(goal.value, GOAL);
    }

    #[test]
    fn test_a_star_search() {
        let initial = TestState{value: 0};
        println!("Starting Greedy Best First Search");
        let result = a_star_search(&initial, |state| state.value == 5);
        assert!(result.plan.is_some());

        let plan = result.plan.unwrap();
        assert!(plan.len() > 0);

        println!("Plan: {:?}", plan);

        let goal = plan.get(plan.len() - 1).unwrap();
        assert_eq!(goal.value, GOAL);
    }
}