use wasmer_runtime_core::{
    codegen::{Event, EventSink, FunctionMiddleware, InternalEvent},
    module::ModuleInfo,
    vm::{InternalField},
    wasmparser::{Operator, Type as WpType, TypeOrFuncType as WpTypeOrFuncType},
    error::{RuntimeError},
    Instance,
};

static RUNTIME_BREAKPOINT_VALUE: InternalField = InternalField::allocate();
pub const BREAKPOINT_VALUE__NO_BREAKPOINT: u64 = 0;

#[derive(Copy, Clone, Debug)]
pub struct RuntimeBreakpointReachedError;


pub struct RuntimeBreakpointHandler {}

impl RuntimeBreakpointHandler {
    pub fn new() -> RuntimeBreakpointHandler {
        RuntimeBreakpointHandler {}
    }
}

impl FunctionMiddleware for RuntimeBreakpointHandler {
    type Error = String;
    fn feed_event<'a, 'b: 'a>(
        &mut self,
        op: Event<'a, 'b>,
        _module_info: &ModuleInfo,
        sink: &mut EventSink<'a, 'b>,
        _source_loc: u32,
    ) -> Result<(), Self::Error> {

        let must_add_breakpoint = match op {
            Event::Wasm(&ref op) | Event::WasmOwned(ref op) => {
                match *op {
                    Operator::Call { .. }
                    | Operator::CallIndirect { .. } => {
                        true
                    }
                    _ => false
                }
            }
            _ => false
        };

        sink.push(op);

        if must_add_breakpoint {
            sink.push(Event::Internal(InternalEvent::GetInternal(
                RUNTIME_BREAKPOINT_VALUE.index() as _,
            )));
            sink.push(Event::WasmOwned(Operator::I64Const {
                value: BREAKPOINT_VALUE__NO_BREAKPOINT as i64,
            }));
            sink.push(Event::WasmOwned(Operator::I64Ne));
            sink.push(Event::WasmOwned(Operator::If {
                ty: WpTypeOrFuncType::Type(WpType::EmptyBlockType),
            }));
            sink.push(Event::Internal(InternalEvent::Breakpoint(Box::new(|_| {
                Err(Box::new(RuntimeError(Box::new("breakpoint reached".to_string()))))
            }))));
            sink.push(Event::WasmOwned(Operator::End));
        }

        Ok(())
    }
}

pub fn set_runtime_breakpoint_value(instance: &mut Instance, value: u64) {
    instance.set_internal(&RUNTIME_BREAKPOINT_VALUE, value);
}

pub fn get_runtime_breakpoint_value(instance: &mut Instance) -> u64 {
    instance.get_internal(&RUNTIME_BREAKPOINT_VALUE)
}