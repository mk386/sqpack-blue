

#[cfg(test)]
mod basic {
    extern crate md5;
    use super::super::*;
//    use std::time::{Duration, SystemTime};

    #[test]
    fn test_initialize_ffxiv() {
        let path = std::env::var("sqpack").unwrap();

        let g =
            FFXIV::new(Path::new(&path));
        g.unwrap();
    }

    #[test]
    fn test_decoding() {
        let mut path = std::env::var("sqpack").unwrap();
        path.push_str("/ffxiv/0a0000.win32.index");

        let mut file = File::open(&path).expect("not found");
        let i = io::read_index_file(&mut file).unwrap();
        let exd = i.get_file(0xE39B7999, 0xa41d4329)
            .expect("couldn't unwrap file in lib.rs");
        assert_eq!(exd.data_offset, 124459392);
    }

    #[test]
    fn test_manual_export() {

        let path = std::env::var("sqpack").unwrap();
        let mut path_index = path.clone();
        let mut path_data = path.clone();
        path_index.push_str("/ffxiv/0c0000.win32.index");
        path_data.push_str("/ffxiv/0c0000.win32.dat0");

        let mut index = File::open(&path_index).expect("not found");
        let index_scd = io::read_index_file(&mut index).unwrap();
        let scd_file_index = index_scd.get_file(0x0AF269D6, 0xe3b71579).unwrap();

        let mut dat_file =
            File::open(&path_data).expect("not found");
        let scd = io::read_data_file(&mut dat_file, scd_file_index).unwrap();

//        use std::io::Write;
        let expected: [u8;16] = [0x43, 0x51, 0x52, 0x41, 0xA8, 0xE7, 0x8E, 0xCC, 0xD5, 0xE1, 0xB3, 0x3A, 0xBE, 0x89, 0xDB, 0xCC];
        let digest:[u8;16] = md5::compute(&scd).into();
        assert_eq!(expected, digest);
    }

    #[test]
    fn test_index_location() {
        let path = std::env::var("sqpack").unwrap();
        let ffxiv = FFXIV::new(Path::new(&path)).unwrap();
        let exfile = ffxiv.get_exfile(&String::from("music/ffxiv/bgm_system_title.scd")).unwrap();
        assert!(
            exfile.get_index_file(
                ffxiv.path.as_path()).as_os_str().eq(
                (path + "\\ffxiv\\0c0000.win32.index").as_str()
            )
//                ,
//            "C:\\Program Files (x86)\\SquareEnix\\FINAL FANTASY XIV - A Realm Reborn\\game\\sqpack\\ffxiv\\0c0000.win32.index"
        );
    }

    #[test]
    fn test_get_index() {
        let path = std::env::var("sqpack").unwrap();
        let ffxiv = FFXIV::new(Path::new(&path)).unwrap();
        ffxiv.get_index(&ffxiv.get_exfile(&String::from("music/ffxiv/bgm_system_title.scd")).unwrap()).unwrap();
    }

    #[test]
    fn test_dat_file_identification() {
        let path = std::env::var("sqpack").unwrap();
        let ffxiv = FFXIV::new(Path::new(&path)).unwrap();
        let exfile = ffxiv.get_exfile(&String::from("music/ffxiv/bgm_system_title.scd")).unwrap();
        let index_file =
            ffxiv.get_index(&exfile).unwrap();
        let phash = exfile.get_sqpack_hashcode();
        let ifl = index_file.get_file(phash.folder_hash, phash.file_hash).unwrap();
        let base_dat_path= exfile.get_dat_file(ffxiv.path.as_path(), ifl.dat_file);
        assert!(
            base_dat_path.as_os_str().eq((path + "\\ffxiv\\0c0000.win32.dat0").as_str())
        );
    }

    #[test]
    fn test_export_raw_data_1() {
        let path = std::env::var("sqpack").unwrap();

        let ffxiv = FFXIV::new(Path::new(&path)).unwrap();
        let v = ffxiv.get_raw_data(
            &ExFileIdentifier::new(
                &String::from("music/ffxiv/bgm_system_title.scd")).unwrap()).unwrap();

//        let mut fi = File::create("exd/bgm_system_title.scd").unwrap();
//        use std::io::Write;
//        fi.write_all(&mut v.0);
        let expected: [u8;16] = [0x43, 0x51, 0x52, 0x41, 0xA8, 0xE7, 0x8E, 0xCC, 0xD5, 0xE1, 0xB3, 0x3A, 0xBE, 0x89, 0xDB, 0xCC];
        let digest:[u8;16] = md5::compute(&v.0).into();
        assert_eq!(expected, digest);
    }

    #[test]
    fn test_export_raw_data_2() {
        let path = std::env::var("sqpack").unwrap();

        let ffxiv = FFXIV::new(Path::new(&path)).unwrap();
        let v = ffxiv.get_raw_data(
            &ExFileIdentifier::new(
                &String::from("music/ffxiv/BGM_PvP_Mogi_01.scd")).unwrap()).unwrap();

//        let mut fi = File::create("exd/bgm_pvp_mogi_01.scd").unwrap();
//        use std::io::Write;
//        fi.write_all(&mut v.0).unwrap();
        let expected: [u8;16] = [0x0D, 0xCC, 0x9B, 0xE6, 0xDE, 0xE5, 0xAE, 0x4B, 0x8F, 0xF3, 0x96, 0xA1, 0xA0, 0xA5, 0x70, 0xBE];
        let digest:[u8;16] = md5::compute(&v.0).into();
        assert_eq!(expected, digest);
    }

    #[test]
    fn sheet_index() {
        let path = std::env::var("sqpack").unwrap();

        let ffxiv = FFXIV::new(Path::new(&path)).unwrap();

        ffxiv.get_sheet_index().unwrap();
    }

    #[test]
    fn sheet_load_no_language() {
        let path = std::env::var("sqpack").unwrap();

        let ffxiv = FFXIV::new(Path::new(&path)).unwrap();

        let s = ffxiv.get_sheet_index().unwrap();
        ffxiv.get_sheet(&String::from("bgm"),
                        ::sheet::ex::SheetLanguage::None, &s).unwrap();

    }

    #[test]
    fn sheet_load_language() {
        let path = std::env::var("sqpack").unwrap();

        let ffxiv = FFXIV::new(Path::new(&path)).unwrap();

        let s = ffxiv.get_sheet_index().unwrap();
        let sheet_result =
            ffxiv.get_sheet(&String::from("achievement"),
                            ::sheet::ex::SheetLanguage::None, &s);

        match sheet_result {
            Ok(_) => panic!("Should not be possible to load without language!"),
            _ => (),
        };

        let sheet_result_2 =
            ffxiv.get_sheet(&String::from("achievement"),
                            ::sheet::ex::SheetLanguage::English, &s);
        let sheet = sheet_result_2.unwrap();
        let row: String = sheet.rows.get(&28).unwrap().read_cell_data(1).unwrap();
        assert_eq!(row, "The Sweet Science V")
    }

    #[test]
    fn sheet_load_items() {
        let path = std::env::var("sqpack").unwrap();

        let ffxiv = FFXIV::new(Path::new(&path)).unwrap();

        let s = ffxiv.get_sheet_index().unwrap();
        let sheet = ffxiv.get_sheet(&String::from("item"),
                        ::sheet::ex::SheetLanguage::English, &s).unwrap();

        let omg: String = sheet.rows[&23991].read_cell_data(0).unwrap();
        assert_eq!(omg, "OMG");
    }

    #[test]
    fn check_float_load() {
        use ::byteorder::BigEndian;
        use ::byteorder::ByteOrder;
        let f: f32 = BigEndian::read_f32(&vec![0x42, 0xb4, 0x00, 0x00]);
        assert_eq!(f, 90.0f32);
    }

    #[test]
    fn csv_ify() {
        let path = std::env::var("sqpack").unwrap();
        let out = std::env::var("out").unwrap();

        let ffxiv = FFXIV::new(Path::new(&path)).unwrap();

        let s = ffxiv.get_sheet_index().unwrap();
        let sheet = ffxiv.get_sheet(&String::from("bgm"),
                                    ::sheet::ex::SheetLanguage::None, &s).unwrap();

        let mut bgm = File::create(out).unwrap();
        sheet::write_csv(&sheet,&mut bgm).unwrap();

    }

//    #[test]
//    fn csv_ify() {
//        let path = std::env::var("sqpack").unwrap();
//        let out = std::env::var("out").unwrap();
//
//        let ffxiv = FFXIV::new(Path::new(&path)).unwrap();
//
//        let s = ffxiv.get_sheet_index().unwrap();
//        let sheet = ffxiv.get_sheet(&String::from("bgm"),
//                        ::sheet::ex::SheetLanguage::None, &s).unwrap();
//
//        let mut bgm = File::create(out).unwrap();
//        use std::io::Write;
//        write!(bgm, "\"index\",").unwrap();
//        sheet.types.iter().enumerate().for_each(|(index, typ)| {
//            if index == sheet.types.len() - 1 {
//                write!(bgm, "\"{}\"", typ.get_header())
//            } else {
//                write!(bgm, "\"{}\",", typ.get_header())
//            }.unwrap();
//        });
//        writeln!(bgm, "").unwrap();
//        sheet.rows.iter().enumerate().for_each(|(index, row)| {
//            write!(bgm, "\"{}\",", index).unwrap();
//            row.types.iter().enumerate().for_each(|(index_typ, typ)| {
//                use ::sheet::ex::SheetDataType;
//                use ::sheet::BitFlags;
//                match typ {
//                    SheetDataType::String(_s_info) =>
//                        write!(bgm, "\"{}\"", row.read_cell_data::<String>(index_typ).unwrap()),
//                    SheetDataType::Bool(_info) =>
//                        write!(bgm, "\"{}\"", row.read_cell_data::<bool>(index_typ).unwrap()),
//                    SheetDataType::Byte(_info) =>
//                        write!(bgm, "\"{}\"", row.read_cell_data::<i8>(index_typ).unwrap()),
//                    SheetDataType::UByte(_info) =>
//                        write!(bgm, "\"{}\"", row.read_cell_data::<u8>(index_typ).unwrap()),
//                    SheetDataType::Short(_info) =>
//                        write!(bgm, "\"{}\"", row.read_cell_data::<i16>(index_typ).unwrap()),
//                    SheetDataType::UShort(_info) =>
//                        write!(bgm, "\"{}\"", row.read_cell_data::<u16>(index_typ).unwrap()),
//                    SheetDataType::Int(_info) =>
//                        write!(bgm, "\"{}\"", row.read_cell_data::<i32>(index_typ).unwrap()),
//                    SheetDataType::UInt(_info) =>
//                        write!(bgm, "\"{}\"", row.read_cell_data::<u32>(index_typ).unwrap()),
//                    SheetDataType::Float(_info) =>
//                        write!(bgm, "\"{}\"", row.read_cell_data::<f32>(index_typ).unwrap()),
//                    SheetDataType::PackedInts(_info) =>
//                        write!(bgm, "\"unsupported\""),
//                    SheetDataType::BitFlags(b_info) =>
//                        write!(bgm, "\"{}\"", row.read_cell_data
//                            ::<BitFlags>(index_typ).unwrap().get_bool(b_info.bit.clone())),
//
//                }.unwrap();
//                if index_typ != row.types.len() - 1 {
//                    write!(bgm, ",").unwrap();
//                }
//            });
//            writeln!(bgm, "").unwrap();
//        });
//
//    }

    #[test]
    fn check_bit_algo() {
        use ::sheet::BitFlags;
        let bf = BitFlags {data: 6u8};
        assert_eq!(bf.get_bool(0), false);
        assert_eq!(bf.get_bool(1), true);
        assert_eq!(bf.get_bool(2), true);
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

#[cfg(test)]
mod expack_test {
    use super::super::*;

    #[test]
    fn test_expack() {
        let path = std::env::var("sqpack").unwrap();
        let apath = Path::new(&path);

        let m = expack::ExFileIdentifier::new(&String::from("music/ex2/BGM_EX2_Dan_D09.scd")).unwrap();
        let pbuff = m.get_index_file(apath);

        let a = pbuff.as_os_str();
        println!("{:?}", a);
        assert!(a.eq(( path.clone() + "\\ex2\\0c0200.win32.index").as_str()));

    }
}

