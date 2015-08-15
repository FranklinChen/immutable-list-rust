/// Use reference counting instead of GC for sharing.
use std::rc::Rc;

use rc_utils;

/// Simplest possible definition of an immutable list as in FP.
///
/// A `List` is either empty or a shared pointer to a `Cons` cell.
#[derive(PartialEq, Debug)]
pub struct List<T>(Option<Rc<Cons<T>>>);

#[derive(PartialEq, Debug)]
pub struct Cons<T> {
    elem: T,
    next: List<T>
}

impl<T> Cons<T> {
    #[inline(always)]
    pub fn new(elem: T, next: List<T>) -> Self {
        Cons { elem: elem, next: next }
    }

    #[inline(always)]
    pub fn singleton(elem: T) -> Self {
        Cons::new(elem, List::empty())
    }
}

//TODO macro for list syntax.

impl<T> List<T> {
    #[inline(always)]
    pub fn as_ref(&self) -> Option<&Rc<Cons<T>>> {
        self.0.as_ref()
    }

    /// Just bump up the reference count.
    #[inline(always)]
    pub fn clone(&self) -> Self {
        List(self.0.clone())
    }

    #[inline(always)]
    pub fn head(&self) -> Option<&T> {
        self.as_ref().map(|r| &r.elem)
    }

    #[inline(always)]
    pub fn tail(&self) -> Option<&List<T>> {
        self.as_ref().map(|r| &r.next)
    }

    /// Bump up reference count when returning the tail of the list.
    #[inline(always)]
    pub fn into_tail(&self) -> Option<List<T>> {
        self.tail().map(|list| list.clone())
    }

    #[inline(always)]
    pub fn empty() -> Self {
        List(None)
    }

    /// OO API is backwards from FP, but does same thing, prepending.
    ///
    /// This variant does not take ownership of the new tail.
    #[inline(always)]
    pub fn cons(&self, elem: T) -> Self {
        List(Some(Rc::new(Cons::new(elem, self.clone()))))
    }

    /// Special version of cons that takes ownership of the new tail.
    #[inline(always)]
    pub fn into_cons(self, elem: T) -> Self {
        List(Some(Rc::new(Cons::new(elem, self))))
    }

    #[inline(always)]
    pub fn singleton(elem: T) -> Self {
        List(Some(Rc::new(Cons::singleton(elem))))
    }

    /// Danger of stack overflow because of non-tail recursion.
    #[inline(always)]
    pub fn map_recursive<U, F>(&self, f: F) -> List<U>
        where F: Fn(&T) -> U
    {
        self.map_recursive_helper(&f)
    }

    /// Danger of stack overflow because of non-tail recursion.
    ///
    /// Use `into_cons` because nobody else sees the intermediate
    /// list.
    fn map_recursive_helper<U, F>(&self, f: &F) -> List<U>
        where F: Fn(&T) -> U
    {
        match self.as_ref() {
            None => List::empty(),
            Some(r) => r.next.map_recursive_helper(f).into_cons(f(&r.elem))
        }
    }

    /// Iterative rather than recursive.
    pub fn map<U, F>(&self, f: F) -> List<U>
        where F: Fn(&T) -> U
    {
        match self.as_ref() {
            None => List::empty(),
            Some(r) => {
                // New Cons, with an initially empty tail.
                let mut first_rc = Rc::new(Cons::singleton(f(&r.elem)));

                {
                    // Pointer to next in current cell.
                    // We want to write in a new next.
                    let mut current_ptr = unsafe {
                        rc_utils::to_value_ptr(&mut first_rc)
                    };

                    let mut self_remaining = &r.next;

                    while let Some(r) = self_remaining.as_ref() {
                        let mut new_rc =
                            Rc::new(Cons::singleton(f(&r.elem)));

                        let next_ptr = unsafe {
                            rc_utils::to_value_ptr(&mut new_rc)
                        };

                        // Patch in the new tail.
                        unsafe {
                            (*current_ptr).next = List(Some(new_rc));
                        }

                        current_ptr = next_ptr;

                        self_remaining = &r.next;
                    }
                }

                List(Some(first_rc))
            }
        }
    }
}

impl<T: Clone> List<T> {
    /// Append a copy of `xs` to `ys`, preserving `ys` through structural
    /// sharing.
    pub fn append(&self, other: &List<T>) -> List<T> {
        match self.as_ref() {
            None =>
                match other.as_ref() {
                    None =>
                        List::empty(),
                    Some(r) =>
                        // Here's where we increase the reference count.
                        List(Some(r.clone()))
                },
            Some(r) =>
                // Recursive append our tail, then prepend a clone of elem.
                r.next.append(other).into_cons(r.elem.clone())
        }
    }

}

impl<T> List<T> {
    /// List identity rather than structural equality.
    ///
    /// Use unsafe hacking! But it is unlikely `Rc` will be anything
    /// other than a pointer to stuff, so this should be OK.
    pub fn same(&self, other: &List<T>) -> bool {
        match (self.as_ref(), other.as_ref()) {
            (Some(self_rc), Some(other_rc)) =>
                unsafe {
                    rc_utils::as_raw(self_rc) == rc_utils::as_raw(other_rc)
                },
            (None, None) => true,
            (_, _) => false,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    /// [0, 1, 2]
    fn list_012() -> List<usize> {
        List::empty()
            .into_cons(2)
            .into_cons(1)
            .into_cons(0)
    }

    /// [3, 4, 5]
    fn list_345() -> List<usize> {
        List::empty()
            .into_cons(5)
            .into_cons(4)
            .into_cons(3)
    }

    fn list_012345() -> List<usize> {
        List::empty()
            .into_cons(5)
            .into_cons(4)
            .into_cons(3)
            .into_cons(2)
            .into_cons(1)
            .into_cons(0)
    }

    #[test]
    fn sharing_with_immutable_cons_compiles() {
        let list1 = list_012();
        let _x = list1.cons(100);
        let _y = list1.cons(200);
    }

    #[test]
    fn equal_by_structure() {
        let list1 = list_012();
        let list2 = list_012();
        assert_eq!(list1, list2);
    }

    #[test]
    fn equal_but_not_same_list() {
        let list1 = list_012();
        let list2 = list_012();
        assert!(!list1.same(&list2));
    }

    #[test]
    fn same_as_itself() {
        let list1 = list_012();
        assert!(list1.same(&list1));
    }

    impl<T> List<T> {
        fn unsafe_into_tail(&self) -> List<T> {
            self.into_tail().unwrap()
        }
    }

    #[test]
    fn append_copies_first_and_shares_second() {
        let list1 = list_012();
        let list2 = list_345();
        let result = list1.append(&list2);

        let fresh = list_012345();

        // Equal but not the same as a fresh list.
        assert_eq!(result, fresh);
        assert!(!result.same(&fresh));

        // Walk over to the sharing point.
        let sublist = result
            .unsafe_into_tail()
            .unsafe_into_tail()
            .unsafe_into_tail();

        assert_eq!(sublist, list2);

        // Sublist within result is the same as original second list.
        assert!(sublist.same(&list2));
    }

    #[test]
    fn map_recursive_works() {
        let list1 = list_012();
        let list2 = list_345();

        let result = list1.map_recursive(|x| x+3);

        assert_eq!(result, list2);
    }

    #[test]
    fn map_iterative_works() {
        let list1 = list_012();
        let list2 = list_345();

        let result = list1.map(|x| x+3);

        assert_eq!(result, list2);
    }
}
