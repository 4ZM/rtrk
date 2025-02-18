// Copyright (C) 2025 Anders Sundman <anders@4zm.org>
//
// This file is part of RTRK - The Rust Tracker
//
// RTRK is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// RTRK is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with RTRK. If not, see <https://www.gnu.org/licenses/>.

/// Module for focus helpers
use std::cell::RefCell;
use std::rc::Rc;

pub type FocusableRc = Rc<RefCell<dyn super::Focusable>>;

#[macro_export]
macro_rules! impl_focusable_with_focuschain {
    ($outer_type:ident, $inner_field:ident) => {
        impl Focusable for $outer_type {
            fn has_focus(&self) -> bool {
                self.$inner_field.has_focus()
            }
            fn focus(&mut self) {
                self.$inner_field.focus();
            }
            fn defocus(&mut self) {
                self.$inner_field.defocus();
            }
            fn next_focus(&mut self) {
                self.$inner_field.next_focus();
            }
            fn prev_focus(&mut self) {
                self.$inner_field.prev_focus();
            }
        }
    };
}

pub struct FocusChain {
    pub focus_idx: Option<usize>,
    pub focusables: Vec<FocusableRc>,
}

impl super::Focusable for FocusChain {
    fn has_focus(&self) -> bool {
        self.focus_idx.is_some()
    }

    fn defocus(&mut self) {
        for f in self.focusables.iter_mut() {
            f.borrow_mut().defocus();
        }
        self.focus_idx = None;
    }
    fn focus(&mut self) {
        // Reset to get first widget in tree
        self.focus_idx = None;
        self.next_focus();
    }

    fn next_focus(&mut self) {
        self.focus_idx = match self.focus_idx {
            None => {
                // Start a new focus cycle
                self.focusables[0].borrow_mut().next_focus();
                Some(0)
            }
            Some(idx) => {
                // Advance the child tree
                self.focusables[idx].borrow_mut().next_focus();

                // Still same child tree that has focus?
                if self.focusables[idx].borrow().has_focus() {
                    Some(idx)
                } else {
                    // Child tree lost focus
                    if idx == self.focusables.len() - 1 {
                        // Last subtree lost focus, nothing left
                        None
                    } else {
                        // Start traversing next subtree
                        self.focusables[idx + 1].borrow_mut().next_focus();
                        Some(idx + 1)
                    }
                }
            }
        };
    }
    fn prev_focus(&mut self) {
        self.focus_idx = match self.focus_idx {
            None => {
                // Start a new focus cycle
                let last_idx = self.focusables.len() - 1;
                self.focusables[last_idx].borrow_mut().prev_focus();
                Some(last_idx)
            }
            Some(idx) => {
                // Advance the child tree
                self.focusables[idx].borrow_mut().prev_focus();

                // Still same child tree that has focus?
                if self.focusables[idx].borrow().has_focus() {
                    Some(idx)
                } else {
                    // Child tree lost focus
                    if idx == 0 {
                        // Last subtree lost focus, nothing left
                        None
                    } else {
                        // Start traversing next subtree
                        self.focusables[idx - 1].borrow_mut().prev_focus();
                        Some(idx - 1)
                    }
                }
            }
        };
    }
}

impl FocusChain {
    pub fn new() -> Self {
        Self {
            focus_idx: None,
            focusables: vec![],
        }
    }

    pub fn push(&mut self, focusable: Rc<RefCell<dyn super::Focusable>>) {
        self.focusables.push(focusable);
    }

    pub fn clear(&mut self) {
        self.focusables
            .iter()
            .for_each(|f| f.borrow_mut().defocus());
        self.focusables.clear();
        self.focus_idx = None;
    }
}
