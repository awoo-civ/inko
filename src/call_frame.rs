use register::Register;
use variable_scope::VariableScope;

pub struct CallFrame<'l> {
    pub name: String,
    pub file: String,
    pub line: usize,
    pub parent: Option<Box<CallFrame<'l>>>,
    pub register: Register<'l>,
    pub variables: VariableScope<'l>
}

impl<'l> CallFrame<'l> {
    pub fn new(name: String, file: String, line: usize) -> CallFrame<'l> {
        let frame = CallFrame {
            name: name,
            file: file,
            line: line,
            parent: Option::None,
            register: Register::new(),
            variables: VariableScope::new()
        };

        frame
    }

    pub fn set_parent(&mut self, parent: CallFrame<'l>) {
        self.parent = Option::Some(Box::new(parent));
    }

    pub fn each_frame<F>(&self, mut closure: F) where F : FnMut(&CallFrame<'l>) {
        let mut frame = self;

        closure(frame);

        while frame.parent.is_some() {
            frame = frame.parent.as_ref().unwrap();

            closure(frame);
        }
    }
}
