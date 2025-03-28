const NES_TAG: [u8; 4] = [0x4E, 0x45, 0x53, 0x1A];
const PRG_ROM_PAGE_SIZE: usize = 16384;
const CHR_ROM_PAGE_SIZE: usize = 8192;



#[derive(Debug, PartialEq)]
pub enum Mirroring {
   VERTICAL,
   HORIZONTAL,
   FOUR_SCREEN,
}

pub struct Rom {
   pub prg_rom: Vec<u8>,
   pub chr_rom: Vec<u8>,
   pub mapper: u8,
   pub screen_mirroring: Mirroring,
}

impl Rom {
	pub fn new(raw: &Vec<u8>) -> Result<Rom, String> {
			if &raw[0..4] != NES_TAG {
					return Err("File is not in iNES file format".to_string());
			}

			let mapper = (raw[7] & 0b1111_0000) | (raw[6] >> 4);

			let ines_ver = (raw[7] >> 2) & 0b11;
			if ines_ver != 0 {
					return Err("NES2.0 format is not supported".to_string());
			}

			let four_screen = raw[6] & 0b1000 != 0;
			let vertical_mirroring = raw[6] & 0b1 != 0;
			let screen_mirroring = match (four_screen, vertical_mirroring) {
					(true, _) => Mirroring::FOUR_SCREEN,
					(false, true) => Mirroring::VERTICAL,
					(false, false) => Mirroring::HORIZONTAL,
			};

			let prg_rom_size = raw[4] as usize * PRG_ROM_PAGE_SIZE;
			let chr_rom_size = raw[5] as usize * CHR_ROM_PAGE_SIZE;

			let skip_trainer = raw[6] & 0b100 != 0;

			let prg_rom_start = 16 + if skip_trainer { 512 } else { 0 };
			let chr_rom_start = prg_rom_start + prg_rom_size;

			Ok(Rom {
					prg_rom: raw[prg_rom_start..(prg_rom_start + prg_rom_size)].to_vec(),
					chr_rom: raw[chr_rom_start..(chr_rom_start + chr_rom_size)].to_vec(),
					mapper: mapper,
					screen_mirroring: screen_mirroring,
			})
	}
}

pub mod test {
    use super::*;

    struct test_rom {
        header: Vec<u8>,
        trainer: Option<Vec<u8>>,
        pgp_rom: Vec<u8>,
        chr_rom: Vec<u8>,
    }

    fn create_rom(rom: test_rom) -> Vec<u8>{
        let mut result = Vec::with_capacity(
            rom.header.len()
                + rom.trainer.as_ref().map_or(0, |t| t.len())
                + rom.pgp_rom.len()
                + rom.chr_rom.len(),
        );

        result.extend(&rom.header);
        if let Some(t) = rom.trainer {
            result.extend(t);
        }
        result.extend(&rom.pgp_rom);
        result.extend(&rom.chr_rom);

        return result;
    }

    pub fn test_rom(program: Vec<u8>) -> Rom {
        let mut pgp_rom_contents = program;
        pgp_rom_contents.resize(2 * PRG_ROM_PAGE_SIZE, 0);

        let test_rom = create_rom(test_rom {
            header: vec![
                0x4E, 0x45, 0x53, 0x1A, 0x02, 0x01, 0x31, 00, 00, 00, 00, 00, 00, 00, 00, 00,
            ],
            trainer: None,
            pgp_rom: pgp_rom_contents,
            chr_rom: vec![2; 1 * CHR_ROM_PAGE_SIZE],
        });

        Rom::new(&test_rom).unwrap()
    }
}
