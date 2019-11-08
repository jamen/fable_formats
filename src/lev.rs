pub mod decode;
pub mod encode;

// Temporary comments from fabletlcmod.com.
//
// Creating Heightmaps and loading heightmaps into 3ds max.
//
// http://www.ogre3d.org/wiki/index.php/3dsmax_Heightmap
//

#[derive(Debug,PartialEq)]
pub struct Lev<'a> {
    header: LevHeader,
    heightmap_cells: Vec<LevHeightmapCell>,
    soundmap_cells: Vec<LevSoundmapCell>,
    navigation_header: LevNavigationHeader,
    navigation_section: LevNavigationSection<'a>
}

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


#[derive(Debug,PartialEq)]
pub struct LevHeightmapCell {
    size: u32,
    version: u8,
    height: u32,
    ground_theme: (u8, u8, u8),
    ground_theme_strength: (u8, u8),
    walkable: bool,
    passover: bool,
    sound_theme: u8,
    shore: bool,
}

#[derive(Debug,PartialEq)]
pub struct LevSoundmapCell {
    size: u32,
    version: u8,
    sound_theme: (u8, u8, u8),
    sound_theme_strength: (u8, u8),
    sound_index: u8,
}

#[derive(Debug,PartialEq)]
pub struct LevNavigationHeader {
    sections_start: u32,
    sections_count: u32,
    sections: Vec<(String, u32)>,
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
pub struct LevNavigationSection<'a> {
    size: u32,
    version: u32,
    level_width: u32,
    level_height: u32,
    interactive_nodes: Vec<LevInteractiveNode>,
    subsets_count: u32,
    level_nodes: Vec<LevNavigationNode<'a>>,
}

#[derive(Debug,PartialEq)]
pub struct LevInteractiveNode {
    x: u32,
    y: u32,
    subset: u32,
}

#[derive(Debug,PartialEq)]
pub enum LevNavigationNode<'a> {
    Regular(LevNavigationRegularNode),
    Navigation(LevNavigationNavigationNode),
    Exit(LevNavigationExitNode),
    Blank(LevNavigationBlankNode),
    Unknown1(LevNavigationUnknown1Node),
    Unknown2(LevNavigationUnknown2Node),
    Unknown3(LevNavigationUnknown3Node),
    Unknown(LevNavigationUnknownNode<'a>),
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

#[derive(Debug,PartialEq)]
pub struct LevNavigationUnknown1Node {
    end: u8
}

#[derive(Debug,PartialEq)]
pub struct LevNavigationUnknown2Node {
    end: u8
}

#[derive(Debug,PartialEq)]
pub struct LevNavigationUnknown3Node {
    end: u8
}

#[derive(Debug,PartialEq)]
pub struct LevNavigationUnknownNode<'a> {
    node_op: &'a [u8],
    end: u8
}

#[derive(Debug,PartialEq)]
pub struct LevNavigationBlankNode {
    root: u8
}