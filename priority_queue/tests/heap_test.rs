use priority_queue::heap::{Heap, MAX_HEAP_SIZE};
use priority_queue::PriorityQueue;
#[cfg(test)]
mod tests {
    use super::*;
    mod create {
        use super::*;
        #[test]
        fn create_0_element_heap() {
            let h: Heap = PriorityQueue::new();
            assert_eq!(h.size(), 0);
        }
        #[test]
        fn create_empty_heap() {
            let h = Heap::new();
            assert!(h.is_empty());
        }
        mod insert {
            use super::*;
            #[test]
            fn create_1_element_heap() {
                let mut h = Heap::new();
                let _res = h.insert(42);
                assert_eq!(h.size(), 1);
            }
            #[test]
            fn create_non_empty_heap() {
                let mut h = Heap::new();
                let _res = h.insert(42);
                assert!(!h.is_empty());
            }
            #[test]
            fn create_full_heap_check_full() {
                let mut h = Heap::new();
                for _i in 0..MAX_HEAP_SIZE - 1 {
                    let _res = h.insert(42);
                }
                assert!(h.is_full());
            }
            #[test]
            fn create_full_heap() {
                let mut h = Heap::new();
                for _i in 0..MAX_HEAP_SIZE - 1 {
                    let _res = h.insert(42);
                }
                assert_eq!(h.insert(42), Err("Full heap"));
            }
        }
        mod from {
            use super::*;
            #[test]
            fn create_empty_heap() {
                let values: [i32; 0] = [];
                let err: Result<Box<Heap>, &'static str> = PriorityQueue::from(&values);
                assert_eq!(err.unwrap_err(), "Empty slice")
            }
            #[test]
            fn create_1_element_heap() {
                let values: [i32; 1] = [42];
                let h: Heap = *PriorityQueue::from(&values).unwrap();
                assert_eq!(h.get_max(), Ok(42));
            }
            #[test]
            fn create_3_elements_heap() {
                let values: [i32; 3] = [1, 42, 2];
                let h: Heap = *PriorityQueue::from(&values).unwrap();
                assert_eq!(h.get_max(), Ok(42));
            }
            #[test]
            fn create_heap_too_long_slice() {
                let values: [i32; MAX_HEAP_SIZE] = [1; MAX_HEAP_SIZE];
                let err: Result<Box<Heap>, &'static str> = PriorityQueue::from(&values);
                assert_eq!(err.unwrap_err(), "Slice too long")
            }
        }
    }
    mod get_max {
        use super::*;
        #[test]
        fn get_max_zero_element() {
            let h = Heap::new();
            assert_eq!(h.get_max(), Err("Empty heap"));
        }
        #[test]
        fn get_max_one_element() {
            let mut h = Heap::new();
            let _res = h.insert(42);
            assert_eq!(h.get_max(), Ok(42));
        }
        #[test]
        fn get_max_two_elements1() {
            let mut h = Heap::new();
            let _res = h.insert(42);
            let _res = h.insert(0);
            assert_eq!(h.get_max(), Ok(42));
        }
        #[test]
        fn get_max_two_elements2() {
            let mut h = Heap::new();
            let _res = h.insert(0);
            let _res = h.insert(42);
            assert_eq!(h.get_max(), Ok(42));
        }
    }
    mod del_max {
        use super::*;
        #[test]
        fn delete_max_empty_heap() {
            let mut h = Heap::new();
            assert_eq!(h.delete_max(), Err("Empty heap"));
        }
        #[test]
        fn delete_max_one_element_heap() {
            let mut h = Heap::new();
            let _res = h.insert(42);
            assert_eq!(h.delete_max(), Ok(42));
        }
        #[test]
        fn delete_max_one_element_heap_check_empty() {
            let mut h = Heap::new();
            let _res = h.insert(42);
            let _res = h.delete_max();
            assert!(h.is_empty());
        }
        #[test]
        fn delete_max_two_elements_heap() {
            let mut h = Heap::new();
            let _res = h.insert(0);
            let _res = h.insert(42);
            assert_eq!(h.delete_max(), Ok(42));
        }
        #[test]
        fn delete_max_two_elements_heap_check_size() {
            let mut h = Heap::new();
            let _res = h.insert(0);
            let _res = h.insert(42);
            let _res = h.delete_max();
            assert_eq!(h.size(), 1);
        }
        #[test]
        fn delete_max_two_elements_heap_check_max() {
            let mut h = Heap::new();
            let _res = h.insert(2);
            let _res = h.insert(42);
            let _res = h.delete_max();
            assert_eq!(h.get_max(), Ok(2));
        }
    }
}
