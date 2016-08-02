pub trait Protocol {
	fn decode_buffer(&mut self, buffer: &Vec<u8>);
}