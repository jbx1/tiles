use std::cmp::Ordering;
use std::cmp::Ordering::Equal;
use std::collections::{HashMap, VecDeque};
use std::fmt::Debug;
use std::hash::Hash;
use std::rc::Rc;
use std::time::{Duration, Instant};

use crate::queue::{Fifo, PriorityCmp, Queue};
use crate::search::Transition::{Action, Initial};

#[derive(Debug)]
pub struct SearchConfig {
    ehc: bool,
    best_first_successors: bool,
}

impl SearchConfig {
    fn default() -> SearchConfig {
        SearchConfig { ehc: false, best_first_successors: false }
    }

    fn ehc() -> SearchConfig {
        SearchConfig { ehc: true, best_first_successors: false }
    }

    fn ehc_steepest_ascent() -> SearchConfig {
        SearchConfig { ehc: true, best_first_successors: true }
    }
}

#[derive(Debug)]
pub struct SearchResult<S: State> {
    pub plan: Option<VecDeque<S>>,
    pub statistics: Statistics,
}

#[derive(Debug)]
pub struct Statistics {
    created: i32,
    queued: i32,
    expanded: i32,
    duration: Duration,
}

pub trait State: PartialEq + Eq + Hash + Sized + Copy + Debug {
    fn successors(&self) -> Vec<Self>;
    fn h(&self) -> i32;
}

#[derive(Debug, Eq)]
enum Transition<S: State> {
    Initial { state: Rc<S>, h: i32 },
    Action { state: Rc<S>, parent: Rc<Transition<S>>, g: u32, index: u32, h: i32 },
}

impl<S: State> Transition<S> {
    fn new(initial: Rc<S>) -> Transition<S> {
        let h = initial.h();
        Initial { state: initial, h }
    }

    fn state(&self) -> &S {
        match self {
            Initial { state, .. } => &state,
            Action { state, .. } => &state
        }
    }

    fn parent(&self) -> Option<&Transition<S>> {
        match self {
            Action { parent, .. } => Some(parent.as_ref()),
            Initial { .. } => None,
        }
    }

    fn h(&self) -> i32 {
        match self {
            Initial { h, ..} => *h,
            Action { h, .. } => *h
        }
    }

    fn g(&self) -> u32 {
        match self {
            Action { g, .. } => *g,
            Initial { .. } => 0,
        }
    }

    fn index(&self) -> u32 {
        match self {
            Action { index, .. } => *index,
            Initial { .. } => 0,
        }
    }

    fn successor(state: Rc<S>, parent: Rc<Transition<S>>, index: u32) -> Transition<S> {
        let h = state.h();
        Action { state, g: parent.g() + 1, parent, index, h }
    }
}

impl<S: State> PartialOrd for Transition<S> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<S: State> Ord for Transition<S> {
    fn cmp(&self, other: &Self) -> Ordering {
        let other_f = other.g() as i32 + other.h();
        let self_f = self.g() as i32 + self.h();

        other_f.partial_cmp(&self_f).unwrap_or_else(|| Equal)
    }
}

impl<S: State> PartialEq for Transition<S> {
    fn eq(&self, other: &Self) -> bool {
        self.state() == other.state()
    }
}

pub fn breadth_first_search<S: State, F: Fn(&S) -> bool>(initial: &S, goal: F) -> SearchResult<S> {
    let mut queue = Fifo::new();
    search(initial, goal, &mut queue, SearchConfig::default())
}

pub fn ehc_search<S: State, F: Fn(&S) -> bool>(initial: &S, goal: F) -> SearchResult<S> {
    let mut queue = Fifo::new();
    search(initial, goal, &mut queue, SearchConfig::ehc())
}

pub fn ehc_steepest_search<S: State, F: Fn(&S) -> bool>(initial: &S, goal: F) -> SearchResult<S> {
    let mut queue = Fifo::new();
    search(initial, goal, &mut queue, SearchConfig::ehc_steepest_ascent())
}

pub fn greedy_best_first_search<S: State, F: Fn(&S) -> bool>(initial: &S, goal: F) -> SearchResult<S> {

    //greedy best first search only considers the heuristic value (h)
    let mut queue = PriorityCmp::new(|s1: &Transition<S>, s2: &Transition<S>| {
        //reverse comparison to get min heap
        s2.h().partial_cmp(&s1.h())
            .unwrap_or_else(|| Equal)
            .then_with(|| s2.index().cmp(&s1.index()))
    });

    search(initial, goal, &mut queue, SearchConfig::default())
}

pub fn a_star_search<S: State, F: Fn(&S) -> bool>(initial: &S, goal: F) -> SearchResult<S> {
    //A* search considers both the distance travelled so far (g) + the heuristic value (h)
    let mut queue = PriorityCmp::new(|s1: &Transition<S>, s2: &Transition<S>| {
        let s1_f = s1.g() as i32 + s1.h();
        let s2_f = s2.g() as i32 + s2.h();
        //reverse comparison to get min heap
        s2_f.partial_cmp(&s1_f)
            .unwrap_or_else(|| Equal)
            .then_with(|| s2.h().partial_cmp(&s1.h()).unwrap_or_else(|| Equal))
            .then_with(|| s2.index().cmp(&s1.index()))
    });

    search(initial, goal, &mut queue, SearchConfig::default())
}

fn search<S, F, Q>(initial: &S, goal: F, queue: &mut Q, config: SearchConfig) -> SearchResult<S>
    where S: State,
          F: Fn(&S) -> bool,
          Q: Queue<Transition<S>>
{
    let mut seen = HashMap::new();

    // the initial state
    let mut statistics = Statistics { created: 1, queued: 1, expanded: 0, duration: Duration::new(0, 0) };
    let start = Instant::now();
    let mut index: u32 = 0;

    let initial_state = Rc::new(*initial);
    let initial_transition = Rc::new(Transition::new(Rc::clone(&initial_state)));
    println!("Starting search with Initial h value {}", initial_transition.h());

    let mut best_h = initial_transition.h();
    print!("Current best H: {:?} ", best_h);

    seen.insert(initial_state, Rc::clone(&initial_transition));
    queue.enqueue(initial_transition);

    while let Some(transition) = queue.dequeue() {
        if (goal)(&transition.state()) {
            let plan = extract_plan(&transition);
            statistics.duration = start.elapsed();
            println!("\nFound plan after seeing {} unique states", seen.len());
            return SearchResult { plan: Some(plan), statistics };
        } else {
            statistics.expanded += 1;
            let mut skip_siblings = false;

            let mut successors: Vec<S> = transition.state().successors()
                .into_iter()
                .filter(|successor| !seen_and_better(&seen, &successor, transition.g() + 1))
                .collect();

            if config.best_first_successors {
                //todo: we are computing this again in the Transition twice, can we avoid it?
                successors.sort_by(|a, b| a.h().partial_cmp(&b.h()).unwrap());
            }

            for successor_state in successors {
                statistics.created += 1;
                index += 1;
                let successor_state_rc = Rc::new(successor_state);
                let succ_transition = Rc::new(Transition::successor(Rc::clone(&successor_state_rc), Rc::clone(&transition), index));
                seen.insert(successor_state_rc, Rc::clone(&succ_transition));

                let current_h = succ_transition.h();
                if current_h < best_h {
                    print!("{:?} ", current_h);
                    best_h = current_h;

                    if config.ehc {
                        queue.clear();
                        skip_siblings = true;
                    }
                }

                queue.enqueue(succ_transition);
                statistics.queued += 1;

                if skip_siblings {
                    break;
                }
            }
        }
    }

    statistics.duration = start.elapsed();
    println!("No plan found. At time {:?} after seeing {} unique states", Instant::now(), seen.len());
    SearchResult { plan: None, statistics }
}


fn seen_and_better<S: State>(seen: &HashMap<Rc<S>, Rc<Transition<S>>>, state: &S, g: u32) -> bool {
    match seen.get(state) {
        Some(seen_transition) if seen_transition.g() <= g => true,
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
            vec![TestState { value: self.value + 1 }, TestState { value: self.value + 2 }, TestState { value: self.value + 3 }]
        }

        fn h(&self) -> i32 {
            if GOAL < self.value {
                i32::MAX
            } else {
                GOAL - self.value
            }
        }
    }


    #[test]
    fn test_breadth_first_search() {
        let initial = TestState { value: 0 };
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
    fn test_ehc_search() {
        let initial = TestState { value: 0 };
        println!("Starting EHC Search");

        let result = ehc_search(&initial, |state| state.value == 5);

        assert!(result.plan.is_some());

        let plan = result.plan.unwrap();
        assert!(plan.len() > 0);

        println!("Plan: {:?}", plan);

        let goal = plan.get(plan.len() - 1).unwrap();
        assert_eq!(goal.value, GOAL);
    }

    #[test]
    fn test_ehc_steepest_search() {
        let initial = TestState { value: 0 };
        println!("Starting EHC Steepest Ascent Search");

        let result = ehc_steepest_search(&initial, |state| state.value == 5);

        assert!(result.plan.is_some());

        let plan = result.plan.unwrap();
        assert!(plan.len() > 0);

        println!("Plan: {:?}", plan);

        let goal = plan.get(plan.len() - 1).unwrap();
        assert_eq!(goal.value, GOAL);
    }

    #[test]
    fn test_greedy_best_first_search() {
        let initial = TestState { value: 0 };
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
        let initial = TestState { value: 0 };
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