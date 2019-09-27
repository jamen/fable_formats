// From fabletlcmod.com:
//
// ==== Stb Archive Header ====
//
// [4] Bytes - File ID (BBBB)
// [4] Bytes - Build Verion
// [4] Bytes - Unknown
// [4] Bytes - Always (0100 0000)
// [4] Bytes - Header Size
// [4] Bytes - Number of Files
// [4] Bytes - Number of Levels
// [4] Bytes - Offset to Developer Listings
//
// === Developer Header ===
//
// [4] Bytes - Start (0100 0000)
// [4] Bytes - Null
// [4] Bytes - Number of Levels
//
// ==== Developer Listings ====
//
// [4] Bytes - Listing Start (2A00 0000)
// [4] Bytes - File ID
// [4] Bytes - Null
// [4] Bytes - File Size
// [4] Bytes - Offset
// [4] Bytes - Null
// [4] Bytes - Length of String Name
// ~ String
// [4] Bytes - Null
// [4] Bytes - Always 01
// [4] Bytes - Length of Second String Name
// ~ String
// [4] Bytes - Bytes Left in Listing *
//
// *This is Where regular Entries end. Only Engine Listings have data past here.
// [4] Bytes - Always 0C
// [4] Bytes - Always 1600 0000
// [4] Bytes - Null
// [4] Bytes - Unknown (Enumerator or a CRC?)
//
// Last Entry: STATIC_MAP_COMMON_HEADER Needs to be accessed for easier editing on the individual .Lev Files

use nom::IResult;
use nom::number::complete::le_u32;
use nom::bytes::complete::tag;
use crate::parser::util::parse_rle_string;

#[derive(Debug,PartialEq)]
pub struct StbHeader {
    version: u32,
    header_size: u32,
    files_count: u32,
    levels_count: u32,
    developer_listings: u32,
}

pub fn parse_header(input: &[u8]) -> IResult<&[u8], StbHeader> {
    let (input, _magic_number) = tag("BBBB")(input)?;
    let (input, version) = le_u32(input)?;
    let (input, _unknown_1) = le_u32(input)?;
    let (input, _unknown_2) = le_u32(input)?;
    let (input, header_size) = le_u32(input)?;
    let (input, files_count) = le_u32(input)?;
    let (input, levels_count) = le_u32(input)?;
    let (input, developer_listings) = le_u32(input)?;

    Ok(
        (
            input,
            StbHeader {
                version: version,
                header_size: header_size,
                files_count: files_count,
                levels_count: levels_count,
                developer_listings: developer_listings,
            }
        )
    )
}

#[derive(Debug,PartialEq)]
pub struct StbDevHeader {
    listing_start: u32,
    file_id: u32,
    file_size: u32,
    offset: u32,
    file_name: String,
    file_name_2: String,
    bytes_left: u32,
}

pub fn parse_developer_header(input: &[u8]) -> IResult<&[u8], StbDevHeader> {
    let (input, listing_start) = le_u32(input)?;
    let (input, file_id) = le_u32(input)?;
    let (input, _null) = le_u32(input)?;
    let (input, file_size) = le_u32(input)?;
    let (input, offset) = le_u32(input)?;
    let (input, _null) = le_u32(input)?;

    println!("listing_start {:?}", listing_start);
    println!("file_id {:?}", file_id);
    println!("file_size {:?}", file_size);
    println!("offset {:?}", offset);

    let (input, file_name) = parse_rle_string(input)?;

    let (input, _null) = le_u32(input)?;
    let (input, _unknown_1) = le_u32(input)?;

    let (input, file_name_2) = parse_rle_string(input)?;

    let (input, bytes_left) = le_u32(input)?;

    let (input, _unknown_2) = le_u32(input)?;
    let (input, _unknown_3) = le_u32(input)?;
    let (input, _null) = le_u32(input)?;
    let (input, _unknown_4) = le_u32(input)?;

    Ok(
        (
            input,
            StbDevHeader {
                listing_start: listing_start,
                file_id: file_id,
                file_size: file_size,
                offset: offset,
                file_name: file_name,
                file_name_2: file_name_2,
                bytes_left: bytes_left,
            }
        )
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::{Read,Seek};
    use std::io::SeekFrom;

    #[test]
    fn test_stb() {
        let mut file = File::open(concat!(env!("FABLE"), "/data/Levels/FinalAlbion_RT.stb")).expect("failed to open file.");

        let mut header: Vec<u8> = Vec::new();
        let mut developer_header: Vec<u8> = Vec::new();

        let mut file_header = file.take(40);
        file_header.read_to_end(&mut header).expect("Failed to read file.");
        let mut file = file_header.into_inner();

        // println!("header {:?}", header);

        let (_, stb_header) = parse_header(&header).expect("Failed to parse header.");

        println!("{:#?}", stb_header);

        file.seek(SeekFrom::Start(stb_header.developer_listings as u64)).expect("Failed to seek file.");
        file.read_to_end(&mut developer_header).expect("Failed to read file.");

        let (_, stb_developer_header) = match parse_developer_header(&developer_header) {
            Ok(value) => value,
            Err(nom::Err::Error((_, error))) => return println!("Error {:?}", error),
            Err(nom::Err::Failure((_, error))) => return println!("Error {:?}", error),
            Err(_) => return println!("Error"),
        };

        println!("{:#?}", stb_developer_header);
    }
}