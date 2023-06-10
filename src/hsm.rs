use kaori_hsm::*;

// Evt definition
#[derive(Debug)]
pub enum BasicEvt {
    A,
    B,
    C,
    D,
    E,
    F
}

pub struct BasicStateMachine {
}

impl BasicStateMachine {
    pub fn new() -> BasicStateMachine {
        BasicStateMachine {}
    }
}
//type BasicStateMachine = StateMachine<BasicData, BasicEvt>;

impl ProtoStateMachine for BasicStateMachine {
    type Evt = BasicEvt;

    fn init(&mut self) -> InitResult<Self> {
        init_transition!(S11)
    }
}

#[state(super_state= Top)]
impl State<S1> for BasicStateMachine {
    fn init(&mut self) -> InitResult<Self> {
        init_transition!(S11)
    }

    fn exit(&mut self) {
    }

    fn entry(&mut self) {
    }

    fn handle(&mut self, evt: &BasicEvt) -> HandleResult<Self> {
        match evt {
            BasicEvt::A => {
                handled!()
            }
            BasicEvt::C => {
                transition!(S122)
            }
            BasicEvt::F => {
                transition!(S1)
            }
            _ => ignored!(),
        }
    }
}

#[state(super_state= S1)]
impl State<S11> for BasicStateMachine {
    fn exit(&mut self) {
    }

    fn entry(&mut self) {
    }

    fn handle(&mut self, evt: &BasicEvt) -> HandleResult<Self> {
        match evt {
            BasicEvt::A => {
                transition!(S121)
            }
            BasicEvt::B => {
                transition!(S12)
            }
            BasicEvt::E => {
                handled!()
            }
            _ => ignored!(),
        }
    }
}

#[state(super_state= S1)]
impl State<S12> for BasicStateMachine {
    fn init(&mut self) -> InitResult<Self> {
        init_transition!(S121)
    }

    fn exit(&mut self) {
    }

    fn entry(&mut self) {
    }

    fn handle(&mut self, evt: &BasicEvt) -> HandleResult<Self> {
        match evt {
            BasicEvt::D => {
                transition!(S121)
            }
            BasicEvt::E => {
                transition!(S11)
            }
            _ => ignored!(),
        }
    }
}

#[state(super_state= S12)]
impl State<S121> for BasicStateMachine {
    fn exit(&mut self) {
    }

    fn entry(&mut self) {
    }

    fn handle(&mut self, evt: &BasicEvt) -> HandleResult<Self> {
        match evt {
            BasicEvt::A => {
                transition!(S122)
            }
            BasicEvt::B => {
                transition!(S12)
            }
            BasicEvt::C => {
                transition!(S11)
            }
            _ => ignored!(),
        }
    }
}

#[state(super_state= S12)]
impl State<S122> for BasicStateMachine {
    fn exit(&mut self) {
    }

    fn entry(&mut self) {
    }

    fn handle(&mut self, evt: &BasicEvt) -> HandleResult<Self> {
        match evt {
            BasicEvt::B => {
                handled!()
            }
            BasicEvt::C => {
                transition!(S122)
            }
            BasicEvt::D => {
                transition!(S1)
            }
            _ => ignored!(),
        }
    }
}
