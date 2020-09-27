use crate::environment::Environment;
use crate::ifn::IFn;
use crate::value::{Evaluable, ToValue, Value};
use std::rc::Rc;

use crate::error_message;
use std::ops::Deref;
use std::sync::Arc;

/// (eval form)
///
#[derive(Debug, Clone)]
pub struct EvalFn {
    enclosing_environment: Arc<Environment>,
}
impl EvalFn {
    pub fn new(enclosing_environment: Arc<Environment>) -> EvalFn {
        EvalFn {
            enclosing_environment,
        }
    }
}

impl ToValue for EvalFn {
    fn to_value(&self) -> Value {
        Value::IFn(Rc::new(self.clone()))
    }
}
impl IFn for EvalFn {
    fn invoke(&self, args: Vec<Rc<Value>>) -> Value {
        // @TODO generalize arity exceptions, and other exceptions
        if args.len() != 1 {
            return error_message::wrong_arg_count(1, args.len());
        }
        let arg = args.get(0).unwrap();
        let env = Rc::new(self.enclosing_environment.deref().to_owned());
        arg.eval(Arc::from(env.deref().to_owned()))
    }
}
