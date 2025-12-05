// use ast::ast::*;

// pub type StateID = usize;
// pub type TransitionID = usize;


// // #[derive()]
// #[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
// pub struct FSM {
//     pub states: Vec<(StateID, Vec<(TransitionID, StateID)>)>,
//     pub transitions: Vec<(TransitionID, Transition)>,
//     pub start: StateID,
//     pub end: StateID,
// }

// #[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
// pub struct Transition {
//     pub event: TransitionEvent,
// }

// #[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
// pub enum TransitionEvent {
//     Empty,                 // epsilon transition
//     Condition(Condition),  // for If
//     ConditionNegated(Condition),  // for Else  
//     Action(Action),        // for RuleBlock or atomic operations
//     ChooseInput(ChooseInput),
//     StageEnd,              // produced by EndStage
//     TurnEnd,               // produced by EndTurn
//     CreateTypes(Types),
//     UserChoice,
// }

// #[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
// pub struct StageContext {
//     pub entry: StateID,
//     pub exit: StateID,
// }

// pub struct FSMBuilder {
//     next_state: usize,
//     next_trans: usize,
//     pub fsm: FSM,

//     stage_stack: Vec<StageContext>,
// }

// pub struct BuiltFSM {
//     pub start: StateID,
//     pub end: StateID,
// }

// impl FSMBuilder {
//     pub fn new() -> Self {
//         FSMBuilder {
//             next_state: 0,
//             next_trans: 0,
//             fsm: FSM {
//                 states: vec![],
//                 transitions: vec![],
//                 start: 0,
//                 end: 0,
//             },
//             stage_stack: Vec::new(),
//         }
//     }

//     fn new_state(&mut self) -> StateID {
//         let id = self.next_state;
//         self.next_state += 1;
//         self.fsm.states.push((id, vec![]));
//         id
//     }

//     fn add_transition(
//         &mut self,
//         from: StateID,
//         to: StateID,
//         event: TransitionEvent,
//     ) -> TransitionID {
//         let tid = self.next_trans;
//         self.next_trans += 1;
//         self.fsm.transitions.push((tid, Transition { event }));
//         self.fsm.states[from].1.push((tid, to));
//         tid
//     }

//     // ---- Top level ----

//     pub fn build_game(&mut self, game: &Game) {
//         let start = self.new_state();
//         let mut curr = start;

//         for flow in &game.flows {
//             let built = self.build_flow(flow);
//             self.add_transition(curr, built.start, TransitionEvent::Empty);
//             curr = built.end;
//         }

//         let end = self.new_state();
//         self.add_transition(curr, end, TransitionEvent::Empty);

//         self.fsm.start = start;
//         self.fsm.end = end;

//         self.remove_empty_transitions();
//     }

//     // ---- Generic flow ----

//     fn build_flow(&mut self, flow: &FlowComponent) -> BuiltFSM {
//         match flow {
//             FlowComponent::Stage(s) => self.build_stage(s),
//             FlowComponent::Rule(r) => self.build_rule(r),
//         }
//     }

//     // ---- Stage ----

//     fn build_stage(&mut self, st: &Stage) -> BuiltFSM {
//         let entry = self.new_state();
//         let exit  = self.new_state();

//         let stage_first_sid = entry;

//         // Push the active stage context
//         self.stage_stack.push(StageContext { entry, exit});

//         // Build body
//         let mut curr = entry;
//         for f in &st.body {
//             let built = self.build_flow(f);
//             self.add_transition(curr, built.start, TransitionEvent::Empty);
//             curr = built.end;
//         }

//         // -------------------------------
//         // First pass: collect dangling states
//         // -------------------------------
//         let dangling_states: Vec<StateID> = self.fsm
//             .states
//             .iter()
//             .map(|(sid, edges)| (*sid, edges.is_empty()))
//             .filter(|(sid, is_empty)| {
//                 *is_empty &&
//                 *sid >= stage_first_sid &&
//                 *sid != exit
//             })
//             .map(|(sid, _)| sid)
//             .collect();

//         // -------------------------------
//         // Second pass: add implicit loops
//         // -------------------------------
//         for sid in dangling_states {
//             self.add_transition(sid, entry, TransitionEvent::Empty);
//         }

//         // Pop stage after fully building it
//         self.stage_stack.pop();

//         BuiltFSM { start: entry, end: exit }
//     }


//     // ---- Rules ----

//     fn build_rule(&mut self, rule: &RuleComponent) -> BuiltFSM {
//         match rule {
//             RuleComponent::If(ie) => self.build_if(ie),
//             RuleComponent::Choose(c) => self.build_choose(c),
//             RuleComponent::RuleBlock(rb) => self.build_ruleblock(rb),
//             RuleComponent::Action(action) => self.build_action(action),
//         }
//     }


//     // ---- Action ----
    
//     fn build_action(&mut self, action: &Action) -> BuiltFSM {
//         let start = self.new_state();

//         match action {
//             Action::EndStage => {
//                 let stage = self.stage_stack.last()
//                     .expect("EndStage used outside of any stage");

//                 let end = stage.exit;

//                 self.add_transition(start, end, TransitionEvent::StageEnd);
//                 BuiltFSM { start, end }
//             }

//             Action::EndTurn => {
//                 let stage = self.stage_stack.last()
//                     .expect("EndTurn used outside of any stage");

//                 // loop back to entry
//                 let end = stage.entry;
//                 self.add_transition(start, end, TransitionEvent::TurnEnd);
//                 return BuiltFSM { start, end };
//             }

//             _ => {
//                 let end = self.new_state();
//                 self.add_transition(start, end, TransitionEvent::Action(action.clone()));
//                 return BuiltFSM { start, end };
//             }
//         }
//     }

//     // ---- If / Else ----

//     fn build_if(&mut self, ie: &If) -> BuiltFSM {
//         let entry = self.new_state();
//         let merge = self.new_state();

//         // Build the IF branch and connect guarded transition
//         let if_built = self.build_sequence(&ie.if_body);
//         self.add_transition(
//             entry,
//             if_built.start,
//             TransitionEvent::Condition(ie.cond.clone()),
//         );

//         self.add_transition(entry, merge, TransitionEvent::ConditionNegated(ie.cond.clone()));

//         BuiltFSM { start: entry, end: merge }
//     }

//     // ---- Choose ----

//     fn build_choose(&mut self, ch: &Choose) -> BuiltFSM {
//         let entry = self.new_state();
//         let merge = self.new_state();

//         for opt in &ch.options {
//             let opt_built = self.build_flow(opt);
//             // empty transition to each option
//             self.add_transition(entry, opt_built.start, TransitionEvent::UserChoice);
//             // return to merge
//             // self.add_transition(opt_built.end, merge, TransitionEvent::UserChoice);
//         }

//         BuiltFSM { start: entry, end: merge }
//     }

//     // ---- RuleBlock ----

//     fn build_ruleblock(&mut self, rb: &RuleBlock) -> BuiltFSM {
//         let start = self.new_state();
//         let mut curr = start;

//         // Choose inputs
//         for ci in &rb.choose_inputs {
//             let next = self.new_state();
//             self.add_transition(curr, next, TransitionEvent::ChooseInput(ci.clone()));
//             curr = next;
//         }

//         // Actions
//         for act in &rb.actions {
//             let next = self.new_state();
//             self.add_transition(curr, next, TransitionEvent::Action(act.clone()));
//             curr = next;
//         }

//         BuiltFSM { start, end: curr }
//     }

//     // ---- Flatten list of flows ----

//     fn build_sequence(&mut self, flows: &[FlowComponent]) -> BuiltFSM {
//         if flows.is_empty() {
//             let s = self.new_state();
//             return BuiltFSM { start: s, end: s };
//         }

//         let mut iter = flows.iter();
//         let first = iter.next().unwrap();
//         let mut built = self.build_flow(first);
//         let start = built.start;
//         let mut curr_end = built.end;

//         for f in iter {
//             let next_built = self.build_flow(f);
//             // Only add transition if the next component is not directly connected
//             if curr_end != next_built.start {
//                 self.add_transition(curr_end, next_built.start, TransitionEvent::Empty);
//             }
//             curr_end = next_built.end;
//         }

//         BuiltFSM { start, end: curr_end }
//     }

//     fn remove_empty_transitions(&mut self) {
//         // Find all connected states which are connected with Empty Transitions
//         let mut pairs = Vec::new();
//         for (s_from, ts) in self.fsm.states.clone() {
//             for (t, s_to) in ts {
//                 let (_, tr) = self.fsm.transitions
//                     .iter()
//                     .find(|(tid, _)| *tid == t)
//                     .expect("Transition ID not found");
//                 if tr.event == TransitionEvent::Empty {
//                     pairs.push((s_from, s_to));
//                 }
//             }
//         }
        
//         // Replace every occurrence of second
//         while !pairs.is_empty() {
//             let (first, second) = pairs[0];
//             if first == second {
//                 pairs.remove(0);
//                 continue;
//             }
//             let mut new_second;
//             let mut new_first;
//             for i in 0..pairs.len() {
//                 new_first = pairs[i].0;
//                 new_second = pairs[i].1;

//                 if pairs[i].0 == second {
//                     new_first = first
//                 }
//                 if pairs[i].1 == second {
//                     new_second = first
//                 }
//                 pairs[i] = (new_first, new_second);

//                 // replace all occurrences of second in the fsm
//                 self.replace_state_id(second, first);
//             } 
            
//             // remove the pair
//             pairs.remove(0);
//         }

//         self.clean_fsm();
//     }

//      /// Replace every occurrence of `old_id` with `new_id` in the FSM.
//     pub fn replace_state_id(&mut self, old_id: StateID, new_id: StateID) {

//         // Replace state IDs in the list of states
//         for (state_id, transitions) in &mut self.fsm.states {
//             if *state_id == old_id {
//                 *state_id = new_id;
//             }

//             // Replace target state IDs inside transitions
//             for (_, target) in transitions {
//                 if *target == old_id {
//                     *target = new_id;
//                 }
//             }
//         }

//         // Replace start and end state if needed
//         if self.fsm.start == old_id {
//             self.fsm.start = new_id;
//         }
//         if self.fsm.end == old_id {
//             self.fsm.end = new_id;
//         }
//     }

//     fn clean_fsm(&mut self) {
//         // Build a lookup table for transitions by ID
//         let transition_map: std::collections::HashMap<TransitionID, &Transition> =
//             self.fsm.transitions.iter().map(|(id, t)| (*id, t)).collect();

//         for (_state_id, edges) in &mut self.fsm.states {
//             edges.retain(|(tid, _)| {
//                 !matches!(transition_map[tid].event, TransitionEvent::Empty)
//             });
//         }

//         // clean empty states with no end
//         self.fsm.states = self.fsm.states.clone().into_iter().filter(|(_, v)| !v.is_empty()).collect(); 

//         // clean all states that do not have an entry and are not the starting node
//         self.fsm.transitions = self.fsm.transitions.clone().into_iter().filter(|(_, t)| t.event != TransitionEvent::Empty).collect::<Vec<(usize, Transition)>>();
//     }
// }



// pub fn build_fsm(game: &Game) -> FSM {
//     let mut builder = FSMBuilder::new();
//     builder.build_game(game);
//     builder.fsm
// }

// use std::fs::File;
// use std::io::Write;

// pub fn fsm_to_dot(fsm: &FSM) -> Result<(), Box<dyn std::error::Error>> {
//     let mut file = File::create("./fsm.dot").unwrap();
//     writeln!(file, "digraph FSM {{").unwrap();
//     writeln!(file, "  rankdir=LR;").unwrap();
//     writeln!(file, "  node [shape = circle];").unwrap();

//     for (tid, trans) in &fsm.transitions {
//         for (state_id, edges) in &fsm.states {
//             for (edge_tid, to) in edges {
//                 if *edge_tid == *tid {
//                     let label = format!("{:?}", trans.event);
//                     writeln!(file, "  {} -> {} [label=\"{}\"];", state_id, to, label).unwrap();
//                 }
//             }
//         }
//     }

//     // mark start and end
//     writeln!(file, "  start [shape=point];").unwrap();
//     writeln!(file, "  start -> {};", fsm.start).unwrap();
//     writeln!(file, "  {} [shape=doublecircle];", fsm.end).unwrap();

//     writeln!(file, "}}").unwrap();

//     clean_dot_file("./fsm.dot", "./fsm.dot")
// }

// use regex::Regex;
// use std::fs;

// /// Reads a DOT file, removes internal quotes inside label="...",
// /// and writes the cleaned result to another file.
// pub fn clean_dot_file(
//     input_path: &str,
//     output_path: &str,
// ) -> Result<(), Box<dyn std::error::Error>> {
//     // Read file
//     let data = fs::read_to_string(input_path)?;

//     // Regex to capture label=" ... "
//     let re = Regex::new(r#"label="(.*?)"\];"#)?;

//     // Clean all labels
//     let cleaned = re.replace_all(&data, |caps: &regex::Captures| {
//         let inner = &caps[1];
//         let inner_clean = inner.replace('"', "");
//         format!("label=\"{}\"];", inner_clean)
//     });

//     // Write output
//     let mut out = fs::File::create(output_path)?;
//     out.write_all(cleaned.as_bytes())?;

//     Ok(())
// }
