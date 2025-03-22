use rust_decimal::Decimal;

#[derive(Clone)]
pub enum NumObject {
	DecNumber(Decimal),
	Operator(String),
}

pub struct NumObjectBuffer {
	pub len: usize,
	pub buffer: Vec<NumObject>,
	pub current_object: Option<NumObject>,
}
impl NumObjectBuffer {
	pub fn default() -> Self {
		NumObjectBuffer {
			len: 0,
			buffer: Vec::new(),
			current_object: None,
		}
	}

	pub fn push(&mut self, num_object: NumObject) {
		self.buffer.push(num_object);
		self.len += 1;
	}

	#[allow(dead_code)]
	pub fn pop(&mut self) -> Option<NumObject> {
		if self.len > 0 {
			self.len -= 1;
			self.buffer.pop()
		} else {
			None
		}
	}

	#[allow(dead_code)]
	pub fn clear(&mut self) {
		self.len = 0;
		self.buffer.clear();
	}
}
