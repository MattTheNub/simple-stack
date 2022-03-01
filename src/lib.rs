#[derive(Clone)]
pub struct Stack<T> {
	head: Option<Box<Node<T>>>,
	size: usize,
}

impl<T> Stack<T> {
	/// Creates an empty stack.
	///
	/// # Examples
	/// ```
	/// use simple_stack::Stack;
	/// let stack: Stack<i32> = Stack::new();
	/// ```
	pub fn new() -> Self {
		Self {
			head: None,
			size: 0,
		}
	}

	/// Adds an item to the top of a stack.
	///
	/// # Examples
	/// ```
	/// use simple_stack::Stack;
	///
	/// let mut stack = Stack::new();
	/// stack.push(3);
	///
	/// assert_eq!(stack.peek(), Some(&3));
	/// ```
	pub fn push(&mut self, val: T) {
		self.size += 1;
		self.head = Some(Box::new(Node {
			val,
			next: self.head.take(),
		}));
	}

	/// Removes the top item from a stack and returns it, or <code>None</code>
	/// if the stack is empty.
	///
	/// # Examples
	/// ```
	/// use simple_stack::Stack;
	///
	/// let mut stack = Stack::new();
	/// stack.push(3);
	/// stack.push(7);
	///
	/// assert_eq!(stack.pop(), Some(7));
	/// assert_eq!(stack.pop(), Some(3));
	/// assert_eq!(stack.pop(), None);
	/// ```
	pub fn pop(&mut self) -> Option<T> {
		self.head.take().map(|head| {
			self.size -= 1; // only decrease size if the stack isn't empty
			self.head = head.next;

			head.val
		})
	}

	/// Returns a reference to the top item of a stack, or <code>None</code> if
	/// the stack is empty.
	///
	/// # Examples
	/// ```
	/// use simple_stack::Stack;
	///
	/// let mut stack = Stack::new();
	///
	/// assert_eq!(stack.peek(), None);
	///
	/// stack.push(4);
	///
	/// assert_eq!(stack.peek(), Some(&4));
	/// ```
	pub fn peek(&self) -> Option<&T> {
		self.head.as_ref().map(|head| &head.val)
	}
	/// Returns a mutable reference to the top item of a stack, or <code>None</code> if
	/// the stack is empty.
	///
	/// # Examples
	/// ```
	/// use simple_stack::Stack;
	///
	/// let mut stack = Stack::new();
	///
	/// stack.push(11);
	///
	/// assert_eq!(stack.peek_mut(), Some(&mut 11));
	/// *stack.peek_mut().unwrap() += 1;
	/// assert_eq!(stack.peek_mut(), Some(&mut 12));
	/// ```
	pub fn peek_mut(&mut self) -> Option<&mut T> {
		self.head.as_mut().map(|head| &mut head.val)
	}

	/// Returns the number of items in the stack.
	///
	/// # Examples
	/// ```
	/// use simple_stack::Stack;
	///
	/// let mut stack = Stack::new();
	///
	/// assert_eq!(stack.size(), 0);
	/// stack.push(1);
	/// stack.push(9);
	/// assert_eq!(stack.size(), 2);
	/// ```
	pub fn size(&self) -> usize {
		self.size
	}
}

impl<T> Default for Stack<T> {
	fn default() -> Self {
		Self::new()
	}
}

#[derive(Clone)]
struct Node<T> {
	val: T,
	next: Option<Box<Node<T>>>,
}
