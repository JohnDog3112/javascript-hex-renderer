use hex_renderer::{options, pattern_utils, grids::{HexGrid, GridDraw, SquareGrid}};
use serde::{Serialize, Deserialize};
use wasm_bindgen::prelude::wasm_bindgen;
use tsify::Tsify;

#[derive(Tsify, Serialize, Deserialize)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct Color(pub u8, pub u8, pub u8, pub u8);

impl From<Color> for options::Color {
    fn from(value: Color) -> Self {
        Self(value.0, value.1, value.2, value.3)
    }
}

#[derive(Tsify, Serialize, Deserialize)]
#[tsify(into_wasm_abi, from_wasm_abi)]
#[serde(tag = "type")]
pub enum Lines {
    Monocolor {
        color: Color,
        bent: bool,
    },
    Gradient {
        colors: Vec<Color>,
        segments_per_color: usize,
        bent: bool,
    },
    SegmentColors {
        colors: Vec<Color>,
        triangles: Triangle,
        collisions: CollisionOption,
    },
}

impl From<Lines> for options::Lines {
    fn from(value: Lines) -> Self {
        match value {
            Lines::Monocolor { color, bent } => Self::Monocolor{
                color: color.into(), 
                bent
            },
            Lines::Gradient { colors, segments_per_color, bent } => Self::Gradient{
                colors: colors.into_iter().map(options::Color::from).collect(), 
                segments_per_color, 
                bent
            },
            Lines::SegmentColors { colors, triangles, collisions } => Self::SegmentColors { 
                colors: colors.into_iter().map(options::Color::from).collect(), 
                triangles: triangles.into(), 
                collisions: collisions.into(),
            },
        }
    }
}
#[derive(Tsify, Serialize, Deserialize)]
#[tsify(into_wasm_abi, from_wasm_abi)]
#[serde(tag = "type")]
pub enum Triangle {
    None,
    Match { radius: f32 },
    BorderMatch { match_radius: f32, border: Marker },
    BorderStartMatch { match_radius: f32, border: Marker },
}

impl From<Triangle> for options::Triangle {
    fn from(value: Triangle) -> Self {
        match value {
            Triangle::None => Self::None,
            Triangle::Match { radius } => Self::Match{radius},
            Triangle::BorderMatch { match_radius, border } => Self::BorderMatch { 
                match_radius, 
                border: border.into()
            },
            Triangle::BorderStartMatch { match_radius, border } => Self::BorderStartMatch {
                match_radius,
                border: border.into(),
            },
        }
    }
}

#[derive(Tsify, Serialize, Deserialize)]
#[tsify(into_wasm_abi, from_wasm_abi)]
#[serde(tag = "type")]
pub enum CollisionOption {
    Dashes {
        color: Color
    },
    MatchedDashes,
    ParallelLines,
    OverloadedParallel {
        max_line: usize,
        overload: OverloadOptions,
    },
}

impl From<CollisionOption> for options::CollisionOption {
    fn from(value: CollisionOption) -> Self {
        match value {
            CollisionOption::Dashes{color} => Self::Dashes(color.into()),
            CollisionOption::MatchedDashes => Self::MatchedDashes,
            CollisionOption::ParallelLines => Self::ParallelLines,
            CollisionOption::OverloadedParallel { max_line, overload } => Self::OverloadedParallel {
                max_line,
                overload: overload.into()
            },
        }
    }
}

#[derive(Tsify, Serialize, Deserialize)]
#[tsify(into_wasm_abi, from_wasm_abi)]
#[serde(tag = "type")]
pub enum OverloadOptions {
    Dashes{ color: Color},
    LabeledDashes { color: Color, label: Marker },
    MatchedDashes,
}

impl From<OverloadOptions> for options::OverloadOptions {
    fn from(value: OverloadOptions) -> Self {
        match value {
            OverloadOptions::Dashes{color} => Self::Dashes(color.into()),
            OverloadOptions::LabeledDashes { color, label } => Self::LabeledDashes {
                color: color.into(),
                label: label.into()
            },
            OverloadOptions::MatchedDashes => Self::MatchedDashes,
        }
    }
}

#[derive(Tsify, Serialize, Deserialize)]
#[tsify(into_wasm_abi, from_wasm_abi)]
#[serde(tag = "type")]
pub enum Point {
    None,
    Single{marker: Marker},
    Double { inner: Marker, outer: Marker },
}

impl From<Point> for options::Point {
    fn from(value: Point) -> Self {
        match value {
            Point::None => Self::None,
            Point::Single{marker} => Self::Single(marker.into()),
            Point::Double { inner, outer } => Self::Double {
                inner: inner.into(),
                outer: outer.into()
            },
        }
    }
}

#[derive(Tsify, Serialize, Deserialize)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct Marker {
    pub color: Color,
    pub radius: f32,
}

impl From<Marker> for options::Marker {
    fn from(value: Marker) -> Self {
        Self {
            color: value.color.into(),
            radius: value.radius
        }
    }
}

#[derive(Tsify, Serialize, Deserialize)]
#[tsify(into_wasm_abi, from_wasm_abi)]
#[serde(tag = "type")]
pub enum EndPoint {
    Point{ point: Point},
    Match { radius: f32 },
    BorderedMatch { match_radius: f32, border: Marker },
}

impl From<EndPoint> for options::EndPoint {
    fn from(value: EndPoint) -> Self {
        match value {
            EndPoint::Point{point} => Self::Point(point.into()),
            EndPoint::Match { radius } => Self::Match {radius},
            EndPoint::BorderedMatch { match_radius, border } => Self::BorderedMatch {
                match_radius,
                border: border.into()
            },
        }
    }
}

#[derive(Tsify, Serialize, Deserialize)]
#[tsify(into_wasm_abi, from_wasm_abi)]
#[serde(tag = "type")]
pub enum Intersections {
    Nothing,
    UniformPoints{point: Point},
    EndsAndMiddle {
        start: EndPoint,
        end: EndPoint,
        middle: Point,
    },
}

impl From<Intersections> for options::Intersections {
    fn from(value: Intersections) -> Self {
        match value {
            Intersections::Nothing => Self::Nothing,
            Intersections::UniformPoints{point} => Self::UniformPoints(point.into()),
            Intersections::EndsAndMiddle { start, end, middle } => Self::EndsAndMiddle {
                start: start.into(),
                end: end.into(),
                middle: middle.into(),
            },
        }
    }
}

#[derive(Tsify, Serialize, Deserialize)]
#[tsify(into_wasm_abi, from_wasm_abi)]
#[serde(tag = "type")]
pub enum GridPatternOptions {
    Uniform{
        intersections: Intersections, 
        lines: Lines
    },
    Changing {
        variations: Vec<(Intersections, Lines)>,
        intros: Vec<String>,
        retros: Vec<String>,
    },
}

fn convert_angle_sigs(str: String) -> Result<Vec<pattern_utils::Angle>, String> {
    str.chars().map(|ch| {
        pattern_utils::Angle::try_from(ch).map_err(|_| format!("Invalid angle_sigs! {}", str))
    }).collect()
}
fn convert_to_angle_list(inp: Vec<String>) -> Result<Vec<Vec<pattern_utils::Angle>>, String> {
    inp.into_iter().map(convert_angle_sigs).collect()
}
impl TryFrom<GridPatternOptions> for options::GridPatternOptions {
    type Error = String;

    fn try_from(value: GridPatternOptions) -> Result<Self, Self::Error> {
        Ok(match value {
            GridPatternOptions::Uniform { intersections, lines } => Self::Uniform(
                intersections.into(),
                lines.into()
            ),
            GridPatternOptions::Changing { variations, intros, retros } => Self::Changing {
                variations: variations.into_iter().map(|(a, b)| (a.into(), b.into())).collect(),
                intros: convert_to_angle_list(intros)?,
                retros: convert_to_angle_list(retros)?
            },
        })
    }
}

#[derive(Tsify, Serialize, Deserialize)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct GridOptions {
    pub line_thickness: f32,

    pub pattern_options: GridPatternOptions,

    pub center_dot: Point,
}

impl TryFrom<GridOptions> for options::GridOptions {
    type Error = String;

    fn try_from(value: GridOptions) -> Result<Self, Self::Error> {
        Ok(Self {
            line_thickness: value.line_thickness,
            pattern_options: value.pattern_options.try_into()?,
            center_dot: value.center_dot.into()
        })
    }
}

#[derive(Tsify, Serialize, Deserialize)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct PatternVariant {
    direction: String,
    angle_sigs: String,
    great_spell: bool,
}

impl TryFrom<PatternVariant> for hex_renderer::PatternVariant {
    type Error = String;

    fn try_from(value: PatternVariant) -> Result<Self, Self::Error> {

        let direction = pattern_utils::Direction::try_from(&value.direction[..]).map_err(|_| format!("Invalid Direction! {}", value.direction))?;
        let angle_sigs = convert_angle_sigs(value.angle_sigs)?;
        let pattern = hex_renderer::Pattern::new(direction, angle_sigs);

        Ok(if value.great_spell {
            Self::Monocolor(pattern)
        } else {
            Self::Normal(pattern)
        })

    }
}

#[derive(Tsify, Serialize, Deserialize)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct PatternVariantArray(pub Vec<PatternVariant>);

impl TryFrom<PatternVariantArray> for Vec<hex_renderer::PatternVariant> {
    type Error = String;

    fn try_from(value: PatternVariantArray) -> Result<Self, Self::Error> {
        value.0.into_iter().map(|item| {
            item.try_into()
        }).collect()
    }
}

#[wasm_bindgen(skip_jsdoc)]
/**
Renders that patterns on a hexagonal grid

 @param {GridOptions} grid_options Options for drawing the patterns
 @param {PatternVariantArray} patterns List of patterns
 @param {number} max_width This is the width of the hexagonal grid before patterns are wrapped around
 @param {number} scale Distance between points on the grid (in pixels)
 @returns {Uint8Array} PNG of the image as a byte array
*/
pub fn draw_hex_grid(grid_options: GridOptions, patterns: PatternVariantArray , max_width: usize, scale: f32) -> Result<Vec<u8>, String> {

    let grid_options: options::GridOptions = grid_options.try_into()?;

    let hex_grid = HexGrid::new(patterns.try_into()?, max_width);

    let hex_grid = hex_grid.map_err(|_| "Failed to create grid!".to_string())?;

    hex_grid.draw_grid_png(scale, &grid_options).map_err(|_| "Failed to draw grid!".to_string())
}

#[wasm_bindgen(skip_jsdoc)]
/**
Renders that patterns on a hexagonal grid that fits in a given resolution

 @param {GridOptions} grid_options Options for drawing the patterns
 @param {PatternVariantArray} patterns List of patterns
 @param {number} max_width This is the width of the hexagonal grid before patterns are wrapped around
 @param {number} width Maximum width of output image
 @param {number} height Maximum height of output image
 @returns {Uint8Array} PNG of the image as a byte array
*/
pub fn draw_bound_hex_grid(grid_options: GridOptions, patterns: PatternVariantArray, max_width: usize, width: f32, height: f32) -> Result<Vec<u8>, String> {

    let grid_options: options::GridOptions = grid_options.try_into()?;

    let hex_grid = HexGrid::new(patterns.try_into()?, max_width);

    let hex_grid = hex_grid.map_err(|_| "Failed to create grid!".to_string())?;

    let scale = hex_grid.get_bound_scale((width, height), &grid_options);

    hex_grid.draw_grid_png(scale, &grid_options).map_err(|_| "Failed to draw grid!".to_string())
}


#[wasm_bindgen(skip_jsdoc)]
/**
Renders that patterns on a square tile grid
Every pattern is 1 tile

 @param {GridOptions} grid_options Options for drawing the patterns
 @param {PatternVariantArray} patterns List of patterns
 @param {number} max_width Maximum number of tiles per row
 @param {number} max_scale Maximum size for patterns (Percentage)
 @param {number} x_pad Padding between tiles in the x direction (Percentage)
 @param {number} y_pad Padding between tiles in the y direction (Percentage)
 @param {number} scale Size of each tile (in pixels)
 @returns {Uint8Array} PNG of the image as a byte array
*/
pub fn draw_square_grid(grid_options: GridOptions, patterns: PatternVariantArray, max_width: usize, max_scale: f32, x_pad: f32, y_pad: f32, scale: f32) -> Result<Vec<u8>, String> {

    let grid_options: options::GridOptions = grid_options.try_into()?;

    let square_grid = SquareGrid::new(patterns.try_into()?, max_width, max_scale, x_pad, y_pad).map_err(|_| "Failed to create grid!".to_string())?;

    square_grid.draw_grid_png(scale, &grid_options).map_err(|_| "Failed to draw grid!".to_string())
}

#[allow(clippy::too_many_arguments)]
#[wasm_bindgen(skip_jsdoc)]
/**
Renders that patterns on a square tile grid that fits in the given resolution
Every pattern is 1 tile

 @param {GridOptions} grid_options Options for drawing the patterns
 @param {PatternVariantArray} patterns List of patterns
 @param {number} max_width Maximum number of tiles per row
 @param {number} max_scale Maximum size for patterns (Percentage)
 @param {number} x_pad Padding between tiles in the x direction (Percentage)
 @param {number} y_pad Padding between tiles in the y direction (Percentage)
 @param {number} width Maximum width of output image
 @param {number} height Maximum height of output image
 @returns {Uint8Array} PNG of the image as a byte array
*/
pub fn draw_bound_square_grid(grid_options: GridOptions, patterns: PatternVariantArray, max_width: usize, max_scale: f32, x_pad: f32, y_pad: f32, width: f32, height: f32) -> Result<Vec<u8>, String> {
    
    let grid_options: options::GridOptions = grid_options.try_into()?;

    let square_grid = SquareGrid::new(patterns.try_into()?, max_width, max_scale, x_pad, y_pad).map_err(|_| "Failed to create grid!".to_string())?;

    let scale = square_grid.get_bound_scale((width, height), &grid_options);

    square_grid.draw_grid_png(scale, &grid_options).map_err(|_| "Failed to draw grid!".to_string())
}

#[wasm_bindgen(skip_jsdoc)]
/**
Renders a single pattern bound to a certain image size.
Almost equivalent to draw_bound_square_grid, however, it draws the pattern without padding for the segment renderer arrows.
This allows for single patterns to not change in size if the renderer is all that changes.
 
 @param {GridOptions} grid_options Options for drawing the patterns
 @param {PatternVariant} pattern Pattern to draw
 @parak {number} max_scale Maximum size for the pattern (percentage)
 @param {number} width Maximum width of the output image
 @param {number} height Maximum height of the output image
 */
pub fn draw_bound_pattern(grid_options: GridOptions, pattern: PatternVariant, max_scale: f32, width: f32, height: f32) -> Result<Vec<u8>, String> {
    let grid_options: options::GridOptions = grid_options.try_into()?;

    let pattern: hex_renderer::PatternVariant = pattern.try_into()?;


    let mut alt_options = grid_options.clone();
    
    match &mut alt_options.pattern_options {
        options::GridPatternOptions::Uniform(_, lines) => {
            match lines {
                options::Lines::Monocolor { color:_, bent:_ }
                | options::Lines::Gradient { colors:_, segments_per_color:_, bent:_ } => (),
                options::Lines::SegmentColors { colors:_, triangles, collisions:_ } => {
                    *triangles = options::Triangle::None;
                },
            };
        },
        options::GridPatternOptions::Changing { variations, intros:_, retros:_ } => {
            variations.iter_mut().for_each(|(_, lines)| {
                match lines {
                    options::Lines::Monocolor { color:_, bent:_ }
                    | options::Lines::Gradient { colors:_, segments_per_color:_, bent:_ } => (),
                    options::Lines::SegmentColors { colors:_, triangles, collisions:_ } => {
                        *triangles = options::Triangle::None;
                    },
                };
            });
        },
    };

    
    let square_grid = SquareGrid::new(vec![pattern], 10, max_scale, 0f32, 0f32).map_err(|_| "Failed to create grid!".to_string())?;

    let scale = square_grid.get_bound_scale((width, height), &alt_options);

    let grid = square_grid.draw_grid_with_padding(scale, &grid_options, alt_options.get_max_radius()).map_err(|_| "Failed to draw grid!".to_string())?;

    grid.encode_png().map_err(|_| "Failed to encode grid!".to_string())
}