use std::collections::HashMap;

use ast::ast::*;

/*
  We have a Game-Struct that have FLOWS (= Vec<FlowComponent>).
  Each FlowComponent CAN have FLOWS inside.
  The FLOWS are ordered in what sequence each FlowComponent
  should be applied (First, the first FlowComponent then the second ... ).
  The FlowComponent should be "glued" together when one ends and another one
  begins.

  Sketch:
  We start with an Entry.
  We build the first FlowComponent (recursively).
  We have a terminal: Action.
  And non-terminals: Stage, If-, Choice-, Optional-Rule.

  The non-terminals need special care for building (especially stage).
  A transition from the end of a stage to the beginning should be
  a "stagecounter(StageID) += 1" transition.
  An EndStage should go from this action to the start of the next
  FlowComponent.
*/

type StateID = i32;
type TransitionID = i32;

enum Transition {
  Action(Rule),
  Condition(BoolExpr),
  NotCondition(BoolExpr),
  EndCondition(EndCondition),
  NotEndCondition(EndCondition),
  Empty,
}

struct FSM {
  states: HashMap<StateID, Vec<(TransitionID, StateID)>>,
  transitions: HashMap<TransitionID, Transition>,
  entry: StateID,
  goals: Vec<StateID>,
}

impl Default for FSM {
  fn default() -> Self {
      FSM {
        states: HashMap::new(),
        transitions: HashMap::new(),
        entry: 0,
        goals: Vec::new()
      }
  }
}

impl FSM {
  fn add_transition(
    &mut self,
    from_state: StateID,
    to_state: StateID,
    transition_id: TransitionID,
    transition: Transition
  ) {
    let transitions = self.states.get_mut(&from_state).expect("State could not been found!");
    transitions.push((transition_id, to_state));

    self.transitions.insert(transition_id, transition);
  }

  fn add_state(&mut self, state_id: StateID) {
    self.states.insert(state_id, Vec::new());
  }
}

struct FSMBuilder {
  game: Game,
  fsm: FSM,
  current_stage_id: i32,
  current_transition_id: i32,
  current_fragment: Fragment,
}

struct Fragment {
  entry: StateID,
  // exits to next FlowComponent
  exits: Vec<StateID>, 
}

impl FSMBuilder {
  pub fn build_fsm(&mut self, game: Game) -> FSM {
    self.game = game;
    self.fsm = FSM::default();
    self.current_stage_id = 0;
    self.current_transition_id = -1;
    
    let flows = self.game.flows.clone();

    self.current_fragment = Fragment { entry: 0, exits: vec![self.current_stage_id] };

    for flow in flows.iter() {

      self.current_fragment = self.build_flow(flow);
    }

    todo!()
  }

  fn new_stage(&mut self) -> i32 {
    self.current_stage_id += 1;

    return self.current_stage_id
  }

  fn new_transition(&mut self) -> i32 {
    self.current_transition_id += 1;

    return self.current_transition_id
  }

  fn build_flow(&mut self, flow: &FlowComponent) -> Fragment {
    match flow {
        FlowComponent::ChoiceRule(choice_rule) => {
            self.build_choice_rule(choice_rule)
          }
        FlowComponent::Stage(seq_stage) => {
            self.build_seq_stage(seq_stage)
        },
        FlowComponent::Rule(rule) => {
            self.build_rule(rule)
        },
        FlowComponent::IfRule(if_rule) => {
            self.build_if_rule(if_rule)
        },
        FlowComponent::OptionalRule(optional_rule) => {
            self.build_optional_rule(optional_rule)
        },
    }
  }

  fn build_choice_rule(&mut self, choice_rule: &ChoiceRule) -> Fragment {
    todo!()
  }

  fn build_seq_stage(&mut self, stage: &SeqStage) -> Fragment {
    todo!()
  }
  
  fn build_rule(&mut self, rule: &Rule) -> Fragment {
    todo!()
  }

  fn build_if_rule(&mut self, if_rule: &IfRule) -> Fragment {
    todo!()
  }

  fn build_optional_rule(&mut self, optional_rule: &OptionalRule) -> Fragment {
    todo!()
  }
}



