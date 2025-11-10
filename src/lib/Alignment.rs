#[derive(Clone)]
pub enum AlignmentEnum {
    _align1 = 1,
    _align2 = 2,
    _align4 = 4,
    _align8 = 8,
    _align16 = 16
}

pub struct Alignment{
    pub align: AlignmentEnum,
    pub 
    _alignNBits: u16,

}

impl Alignment {
    fn new(align: AlignmentEnum, custom: Option<u16>) -> Alignment {
        let n_bits = match custom {
            Some(n_bits) => n_bits,
            None => (align.clone() as u16) * 8
        };
        Alignment{
            _alignNBits: n_bits,
            align,

        }
    }
    pub fn min(&self) -> AlignmentEnum {
        AlignmentEnum::_align1
    }
    pub unsafe fn min_unchecked(&self) -> u16 {
        self._alignNBits
    }
    pub fn max(&self) -> AlignmentEnum {
        AlignmentEnum::_align16
    }

}
