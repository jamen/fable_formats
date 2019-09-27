use nom::IResult;
use nom::number::complete::{le_u8,le_u16,le_u32,le_u64,float};
use nom::bytes::complete::{tag,take};
use nom::sequence::tuple;
use nom::multi::count;
use nom::branch::alt;
use crate::parser::util::parse_rle_string;

#[derive(Debug,PartialEq)]
pub struct Lev {
    header: LevHeader,
    heightmap_cells: Vec<LevHeightmapCell>,
    soundmap_cells: Vec<LevSoundmapCell>,
    // navigation_sections: Vec<>
}

// fn parse_lev(input: &[u8]) -> IResult<&[u8], LevHeader> {

// }

#[derive(Debug,PartialEq)]
pub struct LevHeader {
    pub version: u16,
    pub obsolete_offset: u32,
    pub navigation_offset: u32,
    pub unique_id_count: u64,
    pub width: u32,
    pub height: u32,
    pub map_version: u32,
    // pub heightmap_palette: &'a [u8],
    pub ambient_sound_version: u32,
    // pub sound_palette: &'a [u8],
    pub checksum: u32,
    pub sound_themes: Vec<String>,
}

pub fn parse_header(input: &[u8]) -> IResult<&[u8], LevHeader> {
    let (input, _header_size) = le_u32(input)?;
    let (input, version) = le_u16(input)?;
    let (input, _unknown_1) = take(3usize)(input)?; // fabletlcmod.com: 3 bytes of padding. see checksum.
    let (input, _unknown_2) = le_u32(input)?;
    let (input, obsolete_offset) = le_u32(input)?;
    let (input, _unknown_3) = le_u32(input)?;
    let (input, navigation_offset) = le_u32(input)?;
    let (input, _map_header_size) = le_u8(input)?;
    let (input, map_version) = le_u32(input)?; // fabletlcmod.com:  An 8 bit integer (with 3 bytes of padding)
    let (input, unique_id_count) = le_u64(input)?;
    let (input, width) = le_u32(input)?;
    let (input, height) = le_u32(input)?;
    let (input, _always_true) = le_u8(input)?;

    println!("version {:?}", version);
    println!("obsolete_offset {:?}", obsolete_offset);
    println!("navigation_offset {:?}", navigation_offset);
    println!("_header_size {:?}", _header_size);
    println!("_map_header_size {:?}", _map_header_size);

    let (input, _heightmap_palette) = take(33792usize)(input)?; // TODO: figure this out
    let (input, ambient_sound_version) = le_u32(input)?;
    let (input, sound_themes_count) = le_u32(input)?;
    let (input, _sound_palette) = take(33792usize)(input)?; // TODO: figure this out
    let (input, checksum) = le_u32(input)?; // fabletlcmod.com: only if the map header pad byte 2 is 9.

    let (input, sound_themes) = count(parse_rle_string, (sound_themes_count - 1) as usize)(input)?;

    Ok(
        (
            input,
            LevHeader {
                version: version,
                obsolete_offset: obsolete_offset,
                navigation_offset: navigation_offset,
                unique_id_count: unique_id_count,
                width: width,
                height: height,
                map_version: map_version,
                // heightmap_palette: heightmap_palette,
                ambient_sound_version: ambient_sound_version,
                // sound_palette: sound_palette,
                checksum: checksum,
                sound_themes: sound_themes,
            }
        )
    )
}

#[derive(Debug,PartialEq)]
pub struct LevHeightmapCell {
    size: u32,
    version: u8,
    height: f32,
    ground_theme: (u8, u8, u8),
    ground_theme_strength: (u8, u8),
    walkable: bool,
    passover: bool,
    sound_theme: u8,
    shore: bool,
}

pub fn parse_heightmap_cell(input: &[u8]) -> IResult<&[u8], LevHeightmapCell> {
    let (input, size) = le_u32(input)?;
    let (input, version) = le_u8(input)?;
    let (input, height) = float(input)?;
    let (input, _zero) = le_u32(input)?;
    let (input, ground_theme) = tuple((le_u8, le_u8, le_u8))(input)?;
    let (input, ground_theme_strength) = tuple((le_u8, le_u8))(input)?;
    let (input, walkable) = le_u8(input)?;
    let (input, passover) = le_u8(input)?;
    let (input, sound_theme) = le_u8(input)?;
    let (input, _zero) = le_u8(input)?;
    let (input, shore) = le_u8(input)?;
    let (input, _unknown) = le_u8(input)?;

    Ok(
        (
            input,
            LevHeightmapCell {
                size: size,
                version: version,
                height: height,
                ground_theme: ground_theme,
                ground_theme_strength: ground_theme_strength,
                walkable: walkable != 0,
                passover: passover != 0,
                sound_theme: sound_theme,
                shore: shore != 0,
            }
        )
    )
}

#[derive(Debug,PartialEq)]
pub struct LevSoundmapCell {
    size: u32,
    version: u8,
    sound_theme: (u8, u8, u8),
    sound_theme_strength: (u8, u8),
    sound_index: u8,
}

pub fn parse_soundmap_cell(input: &[u8]) -> IResult<&[u8], LevSoundmapCell> {
    let (input, size) = le_u32(input)?;
    let (input, version) = le_u8(input)?;
    let (input, sound_theme) = tuple((le_u8, le_u8, le_u8))(input)?;
    let (input, sound_theme_strength) = tuple((le_u8, le_u8))(input)?;
    let (input, sound_index) = le_u8(input)?;

    Ok(
        (
            input,
            LevSoundmapCell {
                size: size,
                version: version,
                sound_theme: sound_theme,
                sound_theme_strength: sound_theme_strength,
                sound_index: sound_index,
            }
        )
    )
}

#[derive(Debug,PartialEq)]
pub struct LevNavigationHeader<'a> {
    sections_start: u32,
    sections_count: u32,
    sections: Vec<(&'a [u8], u32)>,
}

pub fn parse_navigation_header(input: &[u8]) -> IResult<&[u8], LevNavigationHeader> {
    let (input, sections_start) = le_u32(input)?;
    let (input, sections_count) = le_u32(input)?;

    let (input, sections) = count(parse_navigation_header_section, sections_count as usize)(input)?;

    Ok(
        (
            input,
            LevNavigationHeader {
                sections_start: sections_start,
                sections_count: sections_count,
                sections: sections,
            }
        )
    )
}

pub fn parse_navigation_header_section(input: &[u8]) -> IResult<&[u8], (&[u8], u32)> {
    let (input, len) = le_u32(input)?;
    let (input, name) = take(len as usize)(input)?;
    let (input, start) = le_u32(input)?;

    Ok( (input, (name, start)) )
}

//
// From fabletlcmod.com:
//
// A Subset has 7 Layers (0-6), each defining blocks of walkable area.
// Layer 0 = 32 X 32
// Layer 1 = 16 X 16
// Layer 2 = 8 X 8
// Layer 3 = 4 X 4
// Layer 4 = 2 X 2
// Layer 5 = 1 X 1
// Layer 6 = 0.5 X 0.5
//

#[derive(Debug,PartialEq)]
pub struct LevNavigationSection {
    size: u32,
    version: u32,
    level_width: u32,
    level_height: u32,
    interactive_nodes: Vec<LevInteractiveNode>,
    subsets_count: u32,
    level_nodes: Vec<LevNavigationNode>,
}

pub fn parse_navigation_section(input: &[u8]) -> IResult<&[u8], LevNavigationSection> {
    let (input, size) = le_u32(input)?;
    let (input, version) = le_u32(input)?;
    let (input, level_width) = le_u32(input)?;
    let (input, level_height) = le_u32(input)?;
    let (input, _unknown_1) = le_u32(input)?; // fabletlcmod.com: Number of levels, see navigation nodes

    let (input, interactive_nodes_count) = le_u32(input)?;
    let (input, interactive_nodes) = count(parse_navigation_interactive_node, interactive_nodes_count as usize)(input)?;

    let (input, subsets_count) = le_u32(input)?;

    let (input, level_nodes_count) = le_u32(input)?;
    let (input, level_nodes) = count(parse_navigation_level_node, level_nodes_count as usize)(input)?;

    Ok(
        (
            input,
            LevNavigationSection {
                size: size,
                version: version,
                level_width: level_width,
                level_height: level_height,
                interactive_nodes: interactive_nodes,
                subsets_count: subsets_count,
                level_nodes: level_nodes,
            }
        )
    )
}

#[derive(Debug,PartialEq)]
pub struct LevInteractiveNode {
    x: u32,
    y: u32,
    subset: u32,
}

pub fn parse_navigation_interactive_node(input: &[u8]) -> IResult<&[u8], LevInteractiveNode> {
    let (input, x) = le_u32(input)?;
    let (input, y) = le_u32(input)?;
    let (input, subset) = le_u32(input)?;

    Ok(
        (
            input,
            LevInteractiveNode {
                x: x,
                y: y,
                subset: subset,
            }
        )
    )
}

#[derive(Debug,PartialEq)]
pub enum LevNavigationNode {
    Regular(LevNavigationRegularNode),
    Navigation(LevNavigationNavigationNode),
    Exit(LevNavigationExitNode),
    Blank(LevNavigationBlankNode),
}

pub fn parse_navigation_level_node(input: &[u8]) -> IResult<&[u8], LevNavigationNode> {
    alt((
        parse_navigation_regular_node,
        parse_navigation_navigation_node,
        parse_navigation_exit_node,
        parse_navigation_blank_node
    ))(input)
}

#[derive(Debug,PartialEq)]
pub struct LevNavigationRegularNode {
    root: u8,
    end: u8,
    layer: u8,
    subset: u8,
    x: f32,
    y: f32,
    node_id: u32,
    child_nodes: (u32, u32, u32, u32) // (top_right, top_left, bottom_right, bottom_left)
}

pub fn parse_navigation_regular_node(input: &[u8]) -> IResult<&[u8], LevNavigationNode> {
    let (input, _node_op) = tag(&[0, 0, 0, 0, 0, 1, 0, 0])(input)?;
    let (input, _unknown_1) = le_u8(input)?;
    let (input, root) = le_u8(input)?;
    let (input, _unknown_2) = le_u8(input)?;
    let (input, end) = le_u8(input)?;
    let (input, layer) = le_u8(input)?;
    let (input, subset) = le_u8(input)?;
    let (input, x) = float(input)?;
    let (input, y) = float(input)?;
    let (input, node_id) = le_u32(input)?;
    let (input, child_nodes) = tuple((le_u32, le_u32, le_u32, le_u32))(input)?;

    Ok(
        (
            input,
            LevNavigationNode::Regular(
                LevNavigationRegularNode {
                    root: root,
                    end: end,
                    layer: layer,
                    subset: subset,
                    x: x,
                    y: y,
                    node_id: node_id,
                    child_nodes: child_nodes,
                }
            )
        )
    )
}

#[derive(Debug,PartialEq)]
pub struct LevNavigationNavigationNode {
    root: u8,
    end: u8,
    layer: u8,
    subset: u8,
    x: f32,
    y: f32,
    node_id: u32,
    node_level: u32,
    nodes: Vec<u32>,
}

pub fn parse_navigation_navigation_node(input: &[u8]) -> IResult<&[u8], LevNavigationNode> {
    let (input, _node_op) = tag(&[0, 0, 0, 1, 0, 1, 0, 1])(input)?;
    let (input, _unknown_1) = le_u8(input)?;
    let (input, root) = le_u8(input)?;
    let (input, _unknown_2) = le_u8(input)?;
    let (input, end) = le_u8(input)?;
    let (input, layer) = le_u8(input)?;
    let (input, subset) = le_u8(input)?;
    let (input, x) = float(input)?;
    let (input, y) = float(input)?;
    let (input, node_id) = le_u32(input)?;
    let (input, node_level) = le_u32(input)?; // fabletlcmod.com: Represents some sort of z level attribute
    let (input, _unknown_3) = le_u8(input)?;  // fabletlcmod.com: So far, Subset 0 = 0 or 128, SubSet 1+ = 64

    let (input, nodes_count) = le_u32(input)?;
    let (input, nodes) = count(le_u32, nodes_count as usize)(input)?;

    Ok(
        (
            input,
            LevNavigationNode::Navigation(
                LevNavigationNavigationNode {
                    root: root,
                    end: end,
                    layer: layer,
                    subset: subset,
                    x: x,
                    y: y,
                    node_id: node_id,
                    node_level: node_level,
                    nodes: nodes,
                }
            )
        )
    )
}

#[derive(Debug,PartialEq)]
pub struct LevNavigationExitNode {
    root: u8,
    end: u8,
    layer: u8,
    subset: u8,
    x: f32,
    y: f32,
    node_id: u32,
    node_level: u32,
    nodes: Vec<u32>,
    uids: Vec<u64>,
}

pub fn parse_navigation_exit_node(input: &[u8]) -> IResult<&[u8], LevNavigationNode> {
    let (input, _node_op) = tag(&[1, 0, 0, 1, 1, 0, 1, 1])(input)?;
    let (input, _unknown_1) = le_u8(input)?;
    let (input, root) = le_u8(input)?;
    let (input, _unknown_2) = le_u8(input)?;
    let (input, end) = le_u8(input)?;
    let (input, layer) = le_u8(input)?;
    let (input, subset) = le_u8(input)?;
    let (input, x) = float(input)?;
    let (input, y) = float(input)?;
    let (input, node_id) = le_u32(input)?;
    let (input, node_level) = le_u32(input)?; // fabletlcmod.com: Represents some sort of z level attribute
    let (input, _unknown_3) = le_u8(input)?;  // fabletlcmod.com: So far, Subset 0 = 0 or 128, SubSet 1+ = 64

    let (input, nodes_count) = le_u32(input)?;
    let (input, nodes) = count(le_u32, nodes_count as usize)(input)?;

    // fabletlcmod.com: Stripped UID to create the real uid add 18446741874686296064
    let (input, uids_count) = le_u32(input)?;
    let (input, uids) = count(le_u64, uids_count as usize)(input)?;

    Ok(
        (
            input,
            LevNavigationNode::Exit(
                LevNavigationExitNode {
                    root: root,
                    end: end,
                    layer: layer,
                    subset: subset,
                    x: x,
                    y: y,
                    node_id: node_id,
                    node_level: node_level,
                    nodes: nodes,
                    uids: uids,
                }
            )
        )
    )
}

#[derive(Debug,PartialEq)]
pub struct LevNavigationBlankNode {
    root: u8
}

pub fn parse_navigation_blank_node(input: &[u8]) -> IResult<&[u8], LevNavigationNode> {
    let (input, _node_op) = tag(&[0, 1, 1])(input)?;
    let (input, _unknown_1) = le_u8(input)?;
    let (input, root) = le_u8(input)?;
    let (input, _unknown_2) = le_u8(input)?;

    Ok(
        (
            input,
            LevNavigationNode::Blank(
                LevNavigationBlankNode {
                    root: root,
                }
            )
        )
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::Read;

    #[test]
    fn test_lev() {
        let file_path = concat!(env!("FABLE"), "/data/Levels/FinalAlbion/LookoutPoint.lev");
        let mut file = File::open(file_path).expect("failed to open file.");

        let mut lev: Vec<u8> = Vec::new();

        file.read_to_end(&mut lev).expect("Failed to read file.");

        let (_, lev) = parse_header(&lev).expect("Failed to parse header.");

        println!("{:#?}", lev);

        // let mut bank_index: Vec<u8> = Vec::new();
        // file.seek(SeekFrom::Start(big_header.bank_address as u64)).expect("Failed to seek file.");
        // file.read_to_end(&mut bank_index).expect("Failed to read file.");

        // let (_, big_bank_index) = parse_bank_index(&bank_index).expect("Failed to parse bank index.");

        // println!("{:?}", big_bank_index);

        // let mut file_index: Vec<u8> = Vec::new();
        // file.seek(SeekFrom::Start(big_bank_index.index_start as u64)).expect("Failed to seek file.");
        // file.take(big_bank_index.index_size as u64).read_to_end(&mut file_index).expect("Failed to read file.");
        // file.read_to_end(&mut file_index).expect("Failed to read file.");

        // let (_, big_file_index) = match parse_file_index(&file_index) {
        //     Ok(value) => value,
        //     Err(nom::Err::Error((_, error))) => return println!("Error {:?}", error),
        //     Err(nom::Err::Failure((_, error))) => return println!("Error {:?}", error),
        //     Err(_) => return println!("Error"),
        // };

        // println!("{:#?}", big_file_index);
    }
}