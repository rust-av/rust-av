pub trait ByteRead {
    fn get_byte(&mut self) -> u8;

    fn get_u16l(&mut self) -> u16;
    fn get_u16b(&mut self) -> u16;

    fn get_u32l(&mut self) -> u32;
    fn get_u32b(&mut self) -> u32;

    fn get_u64l(&mut self) -> u64;
    fn get_u64b(&mut self) -> u64;

    fn get_f32l(&mut self) -> f32;
    fn get_f32b(&mut self) -> f32;

    fn get_f64l(&mut self) -> f64;
    fn get_f64b(&mut self) -> f64;
}
