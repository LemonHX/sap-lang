pub mod value;

use alloc::{collections::BTreeMap, rc::Rc};
use core::{
    borrow::BorrowMut,
    cell::UnsafeCell,
    fmt::Debug,
    ops::{Deref, DerefMut},
};

pub mod type_checker;
use crate::parser::{
    expr::{CommentedExpr, Expr},
    ty::Type,
};

use self::{
    interpreter::{eval_expr, EvalContext, Value},
    std::add_std,
    type_checker::{type_check_expr, TypeCheckContext},
};

pub mod interpreter;
pub mod std;

pub struct Runner {
    type_check_context: Rc<UnsafeCell<TypeCheckContext>>,
    eval_context: Rc<UnsafeCell<EvalContext>>,
}
impl Debug for Runner {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("Runner")
            .field("type_check_context", unsafe {
                &*self.type_check_context.as_ref().get()
            })
            .field("eval_context", unsafe {
                &*self.eval_context.as_ref().get()
            })
            .finish()
    }
}

impl Runner {
    pub fn new() -> Self {
        let type_check_context = Rc::new(UnsafeCell::new(TypeCheckContext {
            parent: None,
            free_var: BTreeMap::new(),
            alias: BTreeMap::new(),
        }));
        let eval_context = Rc::new(UnsafeCell::new(EvalContext {
            parent: None,
            free_var: BTreeMap::new(),
        }));
        Self {
            type_check_context,
            eval_context,
        }
    }
    pub fn new_with_std() -> Self {
        let mut s = Self::new();
        add_std(
            unsafe { &mut *s.type_check_context.as_ref().get() },
            unsafe { &mut *s.eval_context.as_ref().get() },
        );
        s
    }
    pub fn run(
        &mut self,
        mut e: CommentedExpr,
    ) -> (
        Result<Type, &'static str>,
        Result<Rc<UnsafeCell<Value>>, &'static str>,
    ) {
        (
            type_check_expr(&mut e, self.type_check_context.clone()),
            eval_expr(&mut e, self.eval_context.clone()),
        )
    }
}
