

#[cfg(test)]
mod basic {
    use super::super::*;
    use std::time::{Duration, SystemTime};

    #[test]
    fn test_initialize_ffxiv() {
        let g =
            FFXIV::new(Path::new("C:\\Program Files (x86)\\SquareEnix\\FINAL FANTASY XIV - A Realm Reborn\\game\\sqpack"));
        g.unwrap();
    }

    #[test]
    fn test_decoding() {
        let mut file = File::open("C:\\Program Files (x86)\\SquareEnix\\FINAL FANTASY XIV - A Realm Reborn\\game\\sqpack\\ffxiv\\0a0000.win32.index").expect("not found");
        let i = io::read_index_file(&mut file);
        let exd = i.get_file(0xE39B7999, 0xa41d4329)
            .expect("couldn't unwrap file in lib.rs");
        assert_eq!(exd.data_offset, 104770944);
    }

    #[test]
    fn test_export_raw_data() {
        let ffxiv = FFXIV::new(Path::new("C:\\Program Files (x86)\\SquareEnix\\FINAL FANTASY XIV - A Realm Reborn\\game\\sqpack")).unwrap();
        ffxiv.get_raw_data(&ExPath{file_type: 3u8, expansion: GameExpansion::FFXIV}).unwrap_err();
    }

    #[test]
    fn test_scd_export() {
        extern crate md5;

        let mut index = File::open("C:\\Program Files (x86)\\SquareEnix\\FINAL FANTASY XIV - A Realm Reborn\\game\\sqpack\\ffxiv\\0c0000.win32.index").expect("not found");
        let index_scd = io::read_index_file(&mut index);
        let scd_file_index = index_scd.get_file(0x0AF269D6, 0xe3b71579).unwrap();

        let mut dat_file =
            File::open("C:\\Program Files (x86)\\SquareEnix\\FINAL FANTASY XIV - A Realm Reborn\\game\\sqpack\\ffxiv\\0c0000.win32.dat0").expect("not found");
        let scd = io::read_data_file(&mut dat_file, scd_file_index);

        use std::io::Write;
        let expected: [u8;16] = [0x43, 0x51, 0x52, 0x41, 0xA8, 0xE7, 0x8E, 0xCC, 0xD5, 0xE1, 0xB3, 0x3A, 0xBE, 0x89, 0xDB, 0xCC];
        let digest:[u8;16] = md5::compute(&scd).into();
        assert_eq!(expected, digest);
    }

}

#[cfg(test)]
mod hash {
    use super::super::*;

    #[test]
    fn test_hash_file_name() {
        assert_eq!(hash::compute(&String::from("bgm_system_title.scd")), 0xE3B71579)
    }

    #[test]
    fn test_hash_folder_name() {
        assert_eq!(hash::compute(&String::from("music/ffxiv")), 0x0AF269D6)
    }

    #[test]
    fn test_hash_path() {
        let hash::PathHash{folder_hash, file_hash} = hash::compute_path(&String::from("music/ffxiv/bgm_system_title.scd"));
        assert_eq!(folder_hash, 0x0AF269D6);
        assert_eq!(file_hash, 0xE3B71579)
    }

    #[test]
    fn test_hash_lower_eq() {
        assert_eq!(hash::compute(&String::from("bgm_system_title.scd")), hash::compute(&String::from("BGM_System_Title.scd")));
    }

}