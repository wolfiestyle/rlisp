use std::rc::Rc;
use data::{Value, List, Scope, RuntimeError};

impl Value
{
    pub fn type_name(&self) -> &'static str
    {
        match *self {
            Value::Nil => "Nil",
            Value::Number(_) => "Number",
            Value::Symbol(_) => "Symbol",
            Value::String(_) => "String",
            Value::Builtin(_) => "Builtin",
            Value::List(_) => "List",
        }
    }

    pub fn quote(self) -> Value
    {
        Value::List(List::cons(Value::Symbol(Rc::new("quote".to_string())), List::cons(self, List::End)))
    }

    pub fn eval(&self, env: &mut Scope) -> Result<Value, RuntimeError>
    {
        match *self {
            Value::Symbol(ref name) => env.get(name).ok_or(RuntimeError::UnkSymbol(name.clone())),
            Value::List(ref opt) => match *opt {
                List::Node(ref cons) => {
                    match try!(cons.car.eval(env)) {
                        Value::Builtin(func) => {
                            if func.do_eval
                            {
                                (func.call)(&try!(cons.cdr.eval(env)), env)
                            }
                            else
                            {
                                (func.call)(&cons.cdr, env)
                            }
                        },
                        other => Err(RuntimeError::InvalidCall(other.type_name())),
                    }
                },
                List::End => Ok(Value::Nil),
            },
            _ => Ok(self.clone()),
        }
    }
}
