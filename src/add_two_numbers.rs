use std::sync::Arc;

type Link = Option<Box<ListNode>>;

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

// Implementing a Linked List based on book
pub struct List {
    head: Link,
}

impl List {
    /// Create a new empty linked list
    pub fn new() -> Self {
        List { head: None }
    }

    /// Push an element on to the head list.
    pub fn push_head(&mut self, val: i32) {
        let new_node = Box::new(ListNode {
            val,
            next: self.head.take(),
        });

        self.head = Some(new_node);
    }

    /// Peek at the current value
    pub fn peek_head(&self) -> Option<&i32> {
        self.head.as_ref().map(|node| &node.val)
    }

    /// Reverse the list
    pub fn reverse(&mut self) {
        // We initialize the prev_node to self basically
        let mut left_unreversed = List {
            head: self.head.take(),
        };

        // let mut current_node: Option<Box<ListNode>>;
        // let mut prev_node = None;

        // Start a loop
        loop {
            match left_unreversed.pop_head() {
                Some(val) => {
                    self.push_head(val);
                }
                None => break,
            }
        }
    }

    /// Pop an element off the list
    pub fn pop_head(&mut self) -> Option<i32> {
        match self.head.take() {
            Some(node) => {
                self.head = node.next;
                Some(node.val)
            }
            None => None,
        }
    }
}

pub struct Solution {}

impl Solution {
    pub fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        // Two linked lists with digits in reverse order:
        // l1: DN -> DN-1 -> ... -> D0
        // l2: EN -> EN-1 -> ... -> E0
        // We need to perform addition
        add_linked_list_digits(l1, l2)
    }
}

fn add_linked_list_digits(
    l1: Option<Box<ListNode>>,
    l2: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
    // Get the first digits to add
    let mut l1 = List { head: l1 };

    let mut l2 = List { head: l2 };

    // Initialize the answer
    let mut answer = List::new();
    // Initialize the carry
    let mut carry: i32 = 0;

    // todo!();

    // Start a loop
    loop {
        let sum_digit: i32;
        if let Some(node1) = l1.pop_head() {
            if let Some(node2) = l2.pop_head() {
                // We have both digits to add
                let sum = node1 + node2 + carry;

                // Zero the carry
                carry = 0;

                // Handle the carrying.
                if sum > 9 {
                    carry = 1;
                    sum_digit = sum - 10;
                } else {
                    sum_digit = sum;
                }
            } else {
                // Digits for l2 are over so we handle the carry
                let sum = node1 + carry;

                // Zero the carry
                carry = 0;

                // Handle the carrying.
                if sum > 9 {
                    carry = 1;
                    sum_digit = sum - 10;
                } else {
                    sum_digit = sum;
                }
            }
        } else {
            // No mode digits in list 1
            if let Some(node2) = l2.pop_head() {
                // Digits for l2 are over so we handle the carry
                let sum = node2 + carry;

                // Zero the carry
                carry = 0;

                // Handle the carrying.
                if sum > 9 {
                    carry = 1;
                    sum_digit = sum - 10;
                } else {
                    sum_digit = sum;
                }
            } else {
                // Both lists are done

                // If we still have a carry
                if carry == 1 {
                    sum_digit = 1;
                    carry = 0;
                } else {
                    break;
                }
            }
        }

        // Push the sum digit into the answer
        answer.push_head(sum_digit);
    }

    // Reverse
    answer.reverse();

    answer.head
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Convert a slice of digits (least-significant first) into a linked list.
    fn vec_to_list(digits: &[i32]) -> Option<Box<ListNode>> {
        let mut head: Option<Box<ListNode>> = None;
        for &d in digits.iter().rev() {
            head = Some(Box::new(ListNode { val: d, next: head }));
        }
        head
    }

    /// Convenience: turn list back into Vec<i32> for easy assertions.
    fn list_to_vec(mut node: Option<Box<ListNode>>) -> Vec<i32> {
        let mut v = Vec::new();
        while let Some(n) = node {
            v.push(n.val);
            node = n.next;
        }
        v
    }

    #[test] // Example 1 ✓
    fn leet_example_1() {
        let l1 = vec_to_list(&[2, 4, 3]);
        let l2 = vec_to_list(&[5, 6, 4]);
        assert_eq!(
            list_to_vec(Solution::add_two_numbers(l1, l2)),
            vec![7, 0, 8]
        );
    }

    #[test] // Example 2 ✓
    fn leet_example_2() {
        let l1 = vec_to_list(&[0]);
        let l2 = vec_to_list(&[0]);
        assert_eq!(list_to_vec(Solution::add_two_numbers(l1, l2)), vec![0]);
    }

    #[test] // Example 3 ✓
    fn leet_example_3() {
        let l1 = vec_to_list(&[9, 9, 9, 9, 9, 9, 9]);
        let l2 = vec_to_list(&[9, 9, 9, 9]);
        assert_eq!(
            list_to_vec(Solution::add_two_numbers(l1, l2)),
            vec![8, 9, 9, 9, 0, 0, 0, 1]
        );
    }

    #[test] // Different lengths, no carry at MSB
    fn diff_len_no_carry() {
        let l1 = vec_to_list(&[1, 2]); // 21
        let l2 = vec_to_list(&[3]); // 3
        assert_eq!(list_to_vec(Solution::add_two_numbers(l1, l2)), vec![4, 2]); // 24
    }

    #[test] // Carry creates a brand-new most-significant node
    fn carry_creates_new_digit() {
        let l1 = vec_to_list(&[9, 9]); // 99
        let l2 = vec_to_list(&[1]); // 1
        assert_eq!(
            list_to_vec(Solution::add_two_numbers(l1, l2)),
            vec![0, 0, 1]
        ); // 100
    }

    #[test] // Long carry-propagation chain (999 + 1)
    fn long_carry_chain() {
        let l1 = vec_to_list(&[9, 9, 9]);
        let l2 = vec_to_list(&[1]);
        assert_eq!(
            list_to_vec(Solution::add_two_numbers(l1, l2)),
            vec![0, 0, 0, 1]
        );
    }

    #[test] // Alternating carries (8 999 + 2)
    fn alternating_carries() {
        let l1 = vec_to_list(&[8, 9, 9, 9]); // 9 998
        let l2 = vec_to_list(&[2]); // 2
        assert_eq!(
            list_to_vec(Solution::add_two_numbers(l1, l2)),
            vec![0, 0, 0, 0, 1]
        ); // 10 000
    }

    #[test] // List of length 100 filled with 9s + 1
    fn max_len_input() {
        let l1_digits = vec![9; 100]; // 10^100 - 1
        let l1 = vec_to_list(&l1_digits);
        let l2 = vec_to_list(&[1]);
        let result = list_to_vec(Solution::add_two_numbers(l1, l2));
        // Result should be 0 repeated 100 times, followed by 1
        assert_eq!(result.len(), 101);
        assert!(result[..100].iter().all(|&d| d == 0));
        assert_eq!(result[100], 1);
    }

    #[test] // Trailing zeros preserved (1 00 + 0)
    fn trailing_zeros_preserved() {
        let l1 = vec_to_list(&[0, 0, 1]); // represents 100
        let l2 = vec_to_list(&[0]);
        assert_eq!(
            list_to_vec(Solution::add_two_numbers(l1, l2)),
            vec![0, 0, 1]
        );
    }
}
