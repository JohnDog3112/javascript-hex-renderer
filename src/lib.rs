use hex_renderer::{options::{Color, Triangle, CollisionOption, Marker, OverloadOptions, Point, EndPoint, Intersections, Lines, GridPatternOptions, GridOptions}, pattern_utils::{Angle, Direction}, PatternVariant, Pattern, grids::{HexGrid, GridDraw}};
use serde::{Serialize, Deserialize};
use wasm_bindgen::{prelude::wasm_bindgen, JsValue};

#[derive(Serialize, Deserialize)]
struct ColorDef(u8, u8, u8, u8);

impl From<ColorDef> for Color {
    fn from(value: ColorDef) -> Self {
        Self(value.0, value.1, value.2, value.3)
    }
}

#[derive(Serialize, Deserialize)]
#[serde(tag = "type")]
enum LinesDef {
    Monocolor {
        color: ColorDef,
        bent: bool,
    },
    Gradient {
        colors: Vec<ColorDef>,
        segments_per_color: usize,
        bent: bool,
    },
    SegmentColors {
        colors: Vec<ColorDef>,
        triangles: TriangleDef,
        collisions: CollisionOptionDef,
    },
}

impl From<LinesDef> for Lines {
    fn from(value: LinesDef) -> Self {
        match value {
            LinesDef::Monocolor { color, bent } => Self::Monocolor{
                color: color.into(), 
                bent
            },
            LinesDef::Gradient { colors, segments_per_color, bent } => Self::Gradient{
                colors: colors.into_iter().map(Color::from).collect(), 
                segments_per_color, 
                bent
            },
            LinesDef::SegmentColors { colors, triangles, collisions } => Self::SegmentColors { 
                colors: colors.into_iter().map(Color::from).collect(), 
                triangles: triangles.into(), 
                collisions: collisions.into(),
            },
        }
    }
}
#[derive(Serialize, Deserialize)]
#[serde(tag = "type")]
enum TriangleDef {
    None,
    Match { radius: f32 },
    BorderMatch { match_radius: f32, border: MarkerDef },
    BorderStartMatch { match_radius: f32, border: MarkerDef },
}

impl From<TriangleDef> for Triangle {
    fn from(value: TriangleDef) -> Self {
        match value {
            TriangleDef::None => Self::None,
            TriangleDef::Match { radius } => Self::Match{radius},
            TriangleDef::BorderMatch { match_radius, border } => Self::BorderMatch { 
                match_radius, 
                border: border.into()
            },
            TriangleDef::BorderStartMatch { match_radius, border } => Self::BorderStartMatch {
                match_radius,
                border: border.into(),
            },
        }
    }
}

#[derive(Serialize, Deserialize)]
#[serde(tag = "type")]
enum CollisionOptionDef {
    Dashes {
        color: ColorDef
    },
    MatchedDashes,
    ParallelLines,
    OverloadedParallel {
        max_line: usize,
        overload: OverloadOptionsDef,
    },
}

impl From<CollisionOptionDef> for CollisionOption {
    fn from(value: CollisionOptionDef) -> Self {
        match value {
            CollisionOptionDef::Dashes{color} => Self::Dashes(color.into()),
            CollisionOptionDef::MatchedDashes => Self::MatchedDashes,
            CollisionOptionDef::ParallelLines => Self::ParallelLines,
            CollisionOptionDef::OverloadedParallel { max_line, overload } => Self::OverloadedParallel {
                max_line,
                overload: overload.into()
            },
        }
    }
}

#[derive(Serialize, Deserialize)]
#[serde(tag = "type")]
enum OverloadOptionsDef {
    Dashes{ color: ColorDef},
    LabeledDashes { color: ColorDef, label: MarkerDef },
    MatchedDashes,
}

impl From<OverloadOptionsDef> for OverloadOptions {
    fn from(value: OverloadOptionsDef) -> Self {
        match value {
            OverloadOptionsDef::Dashes{color} => Self::Dashes(color.into()),
            OverloadOptionsDef::LabeledDashes { color, label } => Self::LabeledDashes {
                color: color.into(),
                label: label.into()
            },
            OverloadOptionsDef::MatchedDashes => Self::MatchedDashes,
        }
    }
}

#[derive(Serialize, Deserialize)]
#[serde(tag = "type")]
enum PointDef {
    None,
    Single{marker: MarkerDef},
    Double { inner: MarkerDef, outer: MarkerDef },
}

impl From<PointDef> for Point {
    fn from(value: PointDef) -> Self {
        match value {
            PointDef::None => Self::None,
            PointDef::Single{marker} => Self::Single(marker.into()),
            PointDef::Double { inner, outer } => Self::Double {
                inner: inner.into(),
                outer: outer.into()
            },
        }
    }
}

#[derive(Serialize, Deserialize)]

struct MarkerDef {
    pub color: ColorDef,
    pub radius: f32,
}

impl From<MarkerDef> for Marker {
    fn from(value: MarkerDef) -> Self {
        Self {
            color: value.color.into(),
            radius: value.radius
        }
    }
}

#[derive(Serialize, Deserialize)]
#[serde(tag = "type")]
enum EndPointDef {
    Point{ point: PointDef},
    Match { radius: f32 },
    BorderedMatch { match_radius: f32, border: MarkerDef },
}

impl From<EndPointDef> for EndPoint {
    fn from(value: EndPointDef) -> Self {
        match value {
            EndPointDef::Point{point} => Self::Point(point.into()),
            EndPointDef::Match { radius } => Self::Match {radius},
            EndPointDef::BorderedMatch { match_radius, border } => Self::BorderedMatch {
                match_radius,
                border: border.into()
            },
        }
    }
}

#[derive(Serialize, Deserialize)]
#[serde(tag = "type")]
enum IntersectionsDef {
    Nothing,
    UniformPoints{point: PointDef},
    EndsAndMiddle {
        start: EndPointDef,
        end: EndPointDef,
        middle: PointDef,
    },
}

impl From<IntersectionsDef> for Intersections {
    fn from(value: IntersectionsDef) -> Self {
        match value {
            IntersectionsDef::Nothing => Self::Nothing,
            IntersectionsDef::UniformPoints{point} => Self::UniformPoints(point.into()),
            IntersectionsDef::EndsAndMiddle { start, end, middle } => Self::EndsAndMiddle {
                start: start.into(),
                end: end.into(),
                middle: middle.into(),
            },
        }
    }
}

#[derive(Serialize, Deserialize)]
#[serde(tag = "type")]
enum GridPatternOptionsDef {
    Uniform{
        intersections: IntersectionsDef, 
        lines: LinesDef
    },
    Changing {
        variations: Vec<(IntersectionsDef, LinesDef)>,
        intros: Vec<String>,
        retros: Vec<String>,
    },
}

fn convert_angle_sigs(str: String) -> Result<Vec<Angle>, String> {
    str.chars().map(|ch| {
        Angle::try_from(ch).map_err(|_| format!("Invalid angle_sigs! {}", str))
    }).collect()
}
fn convert_to_angle_list(inp: Vec<String>) -> Result<Vec<Vec<Angle>>, String> {
    inp.into_iter().map(convert_angle_sigs).collect()
}
impl TryFrom<GridPatternOptionsDef> for GridPatternOptions {
    type Error = String;

    fn try_from(value: GridPatternOptionsDef) -> Result<Self, Self::Error> {
        Ok(match value {
            GridPatternOptionsDef::Uniform { intersections, lines } => Self::Uniform(
                intersections.into(),
                lines.into()
            ),
            GridPatternOptionsDef::Changing { variations, intros, retros } => Self::Changing {
                variations: variations.into_iter().map(|(a, b)| (a.into(), b.into())).collect(),
                intros: convert_to_angle_list(intros)?,
                retros: convert_to_angle_list(retros)?
            },
        })
    }
}

#[derive(Serialize, Deserialize)]
struct GridOptionsDef {
    pub line_thickness: f32,

    pub pattern_options: GridPatternOptionsDef,

    pub center_dot: PointDef,
}

impl TryFrom<GridOptionsDef> for GridOptions {
    type Error = String;

    fn try_from(value: GridOptionsDef) -> Result<Self, Self::Error> {
        Ok(Self {
            line_thickness: value.line_thickness,
            pattern_options: value.pattern_options.try_into()?,
            center_dot: value.center_dot.into()
        })
    }
}

#[derive(Serialize, Deserialize)]
pub struct PatternVariantDef {
    direction: String,
    angle_sigs: String,
    great_spell: bool,
}

impl TryFrom<PatternVariantDef> for PatternVariant {
    type Error = String;

    fn try_from(value: PatternVariantDef) -> Result<Self, Self::Error> {

        let direction = Direction::try_from(&value.direction[..]).map_err(|_| format!("Invalid Direction! {}", value.direction))?;
        let angle_sigs = convert_angle_sigs(value.angle_sigs)?;
        let pattern = Pattern::new(direction, angle_sigs);

        Ok(if value.great_spell {
            Self::Normal(pattern)
        } else {
            Self::Monocolor(pattern)
        })

    }
}



fn parse_js_vals(grid_options: JsValue, patterns: Vec<JsValue>) -> Result<(GridOptions, Vec<PatternVariant>), String> {
    
    let patterns = patterns
        .into_iter()
        .map(serde_wasm_bindgen::from_value::<PatternVariantDef>)
        .collect::<Result<Vec<_>, _>>()
        .map_err(|a| a.to_string())?;

    let patterns = patterns
        .into_iter()
        .map(PatternVariant::try_from)
        .collect::<Result<Vec<_>, String>>()?;

    let grid_options: GridOptionsDef = serde_wasm_bindgen::from_value(grid_options).map_err(|a| a.to_string())?;

    let grid_options: GridOptions = grid_options.try_into()?;

    Ok((grid_options, patterns))
}

#[wasm_bindgen]
pub fn draw_hex_grid(grid_options: JsValue, patterns: Vec<JsValue>, max_width: usize, scale: f32) -> Result<Vec<u8>, String> {

    let (grid_options, patterns) = parse_js_vals(grid_options, patterns)?;

    let hex_grid = HexGrid::new(patterns, max_width);

    let hex_grid = hex_grid.map_err(|_| "Failed to create grid!".to_string())?;

    hex_grid.draw_grid_png(scale, &grid_options).map_err(|_| "Failed to draw grid!".to_string())
}

#[wasm_bindgen]
pub fn draw_bound_hex_grid(grid_options: JsValue, patterns: Vec<JsValue>, max_width: usize, width: f32, height: f32) -> Result<Vec<u8>, String> {

    let (grid_options, patterns) = parse_js_vals(grid_options, patterns)?;

    let hex_grid = HexGrid::new(patterns, max_width);

    let hex_grid = hex_grid.map_err(|_| "Failed to create grid!".to_string())?;

    let scale = hex_grid.get_bound_scale((width, height), &grid_options);

    hex_grid.draw_grid_png(scale, &grid_options).map_err(|_| "Failed to draw grid!".to_string())
}


#[wasm_bindgen]
pub fn draw_square_grid(grid_options: JsValue, patterns: Vec<JsValue>, max_width: usize, max_scale: f32, x_pad: f32, y_pad: f32, scale: f32) -> Result<Vec<u8>, String> {
    let (grid_options, patterns) = parse_js_vals(grid_options, patterns)?;

    let square_grid = hex_renderer::grids::SquareGrid::new(patterns, max_width, max_scale, x_pad, y_pad).map_err(|_| "Failed to create grid!".to_string())?;

    square_grid.draw_grid_png(scale, &grid_options).map_err(|_| "Failed to draw grid!".to_string())
}

#[wasm_bindgen]
#[allow(clippy::too_many_arguments)]
pub fn draw_bound_square_grid(grid_options: JsValue, patterns: Vec<JsValue>, max_width: usize, max_scale: f32, x_pad: f32, y_pad: f32, width: f32, height: f32) -> Result<Vec<u8>, String> {
    let (grid_options, patterns) = parse_js_vals(grid_options, patterns)?;

    let square_grid = hex_renderer::grids::SquareGrid::new(patterns, max_width, max_scale, x_pad, y_pad).map_err(|_| "Failed to create grid!".to_string())?;

    let scale = square_grid.get_bound_scale((width, height), &grid_options);

    square_grid.draw_grid_png(scale, &grid_options).map_err(|_| "Failed to draw grid!".to_string())
}

#[wasm_bindgen]
pub fn example() -> Result<JsValue, serde_wasm_bindgen::Error> {
    let tmp = GridOptionsDef {
        line_thickness: 0.12,
        pattern_options: GridPatternOptionsDef::Uniform { 
            intersections: IntersectionsDef::UniformPoints{
                point: PointDef::Single{
                    marker: MarkerDef {
                        color: ColorDef(255, 255, 255, 255),
                        radius: 0.07
                    }
                }
            }, 
            lines: LinesDef::Gradient { 
                colors: vec![
                    ColorDef(255, 0, 0, 255),
                    ColorDef(0, 255, 0, 255),
                    ColorDef(0, 0, 255, 255)
                ], 
                segments_per_color: 5, 
                bent: true,
            }
        },
        center_dot: PointDef::None,
    };

    serde_wasm_bindgen::to_value(&tmp)
}